import { getInnerQuote, getQuoteVariant } from "@gobob/bob-sdk";
import { getEnrichedRoutes } from "../util/route-provider.js";
import type { BitcoinSigner } from "@gobob/bob-sdk";
import { formatUnits, type WalletClient, type PublicClient } from "viem";
import { resolveSwapInputs, humanToAtomic } from "../util/input-resolver.js";
import { fetchPrice } from "../util/price-oracle.js";
import { MempoolClient } from "@gobob/bob-sdk";
import pRetry, { AbortError } from "p-retry";
import { loadConfig, getSdk } from "../config.js";
import { deriveAddress, resolveSigner, getTokenBalance, buildRegisterPayload, getChainFamily, resolvePrivateKey } from "../chains/index.js";
import type { GatewayOrderInfo, GatewayOrderStatus, GetQuoteParams } from "@gobob/bob-sdk";
import type { Logger, SwapSuccessJson, SwapSubmittedJson, SwapMempoolPendingJson } from "../output.js";

// ─── Retry ───────────────────────────────────────────────────────────────────

const TRANSIENT_PATTERNS = [
  /TRM screening/i,
  /429/,
  /Too Many Requests/i,
  /rate limit/i,
  /not yet propagated/i,
  /BTC propagation/i,
  /timeout/i,
  /ECONNRESET/,
  /ETIMEDOUT/,
];

function isTransientError(err: unknown): boolean {
  const msg = err instanceof Error ? err.message : String(err);
  return TRANSIENT_PATTERNS.some((p) => p.test(msg));
}

async function withRetry<T>(
  fn: () => Promise<T>,
  opts: { retries: number; log: Logger; label?: string; shouldRetry?: (err: unknown) => boolean },
): Promise<T> {
  return pRetry(
    async (attemptNumber) => {
      try {
        return await fn();
      } catch (err) {
        if (opts.shouldRetry && !opts.shouldRetry(err)) {
          throw new AbortError(err instanceof Error ? err.message : String(err));
        }
        if (attemptNumber > 1 || opts.label) {
          opts.log.progress(`${opts.label ?? 'Retrying'} (${attemptNumber}/${opts.retries + 1})...`);
        }
        throw err;
      }
    },
    { retries: opts.retries },
  );
}

// ─── Types ───────────────────────────────────────────────────────────────────

export interface SwapOptions {
  src: string;
  dst: string;
  amount: string;
  recipient: string;
  sender?: string;
  slippage?: number;
  gasRefillUsd?: number;
  btcFeeRate?: number;
  feeToken?: string;
  feeReserve?: string;
  privateKey?: string;
  unsigned: boolean;
  wait: boolean;
  retry: boolean;
  timeout?: number;
}

export type SwapResult =
  | { type: "unsigned"; orderId: string; psbtBase64?: string; txInfo?: any }
  | { type: "cancelled" }
  | { type: "submitted"; data: SwapSubmittedJson }
  | { type: "confirmed"; data: SwapSuccessJson }
  | { type: "mempoolPending"; data: SwapMempoolPendingJson };

// ─── Helpers ─────────────────────────────────────────────────────────────────

function makeRegistrationError(err: unknown, orderId: string, txId: string): Error {
  const msg = err instanceof Error ? err.message : String(err);
  const error = new Error(
    `CRITICAL: Transaction signed but registration failed. Last error: ${msg}\n` +
    `Order ID: ${orderId}\nManually register with: gateway-cli register ${orderId} ${txId}`,
  );
  (error as any).orderId = orderId;
  (error as any).txId = txId;
  return error;
}

async function createSwapOrder(
  sdk: ReturnType<typeof getSdk>,
  quoteParams: GetQuoteParams,
) {
  const quote = await sdk.getQuote(quoteParams);
  const variant = getQuoteVariant(quote);
  const order = await sdk.api.createOrder({ gatewayQuote: quote });
  const orderId = (order as any)[variant].orderId;
  const outputAmount = getInnerQuote(quote).outputAmount.amount;
  return { quote, variant, order, orderId, outputAmount };
}

type UnsignedSigner = { unsigned: true };

async function resolveSignerForSwap(
  chain: string,
  key: string | undefined,
  unsigned: boolean,
  isBtc: boolean,
): Promise<Awaited<ReturnType<typeof resolveSigner>> | UnsignedSigner> {
  if (unsigned) return { unsigned: true };
  if (!key) {
    const chainType = isBtc ? "Bitcoin" : "EVM";
    const envVar = isBtc ? "BITCOIN_PRIVATE_KEY" : "EVM_PRIVATE_KEY";
    throw new Error(`no signer configured for ${chainType}.\n  Set ${envVar} or pass --private-key.\n  Use --unsigned to output the ${isBtc ? "PSBT" : "unsigned transaction"} without signing.`);
  }
  return resolveSigner(chain, key);
}

// ─── Handler ─────────────────────────────────────────────────────────────────

export async function handleSwap(opts: SwapOptions, log: Logger): Promise<SwapResult> {
  const config = loadConfig();
  const sdk = getSdk();

  const enriched = await getEnrichedRoutes();
  const { srcAsset, dstAsset, parsed } = await resolveSwapInputs(
    opts.src, opts.dst, opts.amount, enriched,
  );

  let atomicUnits: string;
  let display: string;
  if (parsed.type === "all") {
    const resolved = await resolveAllAmount(srcAsset, opts, config);
    atomicUnits = resolved.atomicUnits;
    display = resolved.display;
  } else {
    atomicUnits = parsed.atomicUnits;
    display = parsed.display;
  }

  const srcFamily = getChainFamily(srcAsset.chain);
  const slippageBps = opts.slippage ?? config.slippageBps;
  const timeoutMs = opts.timeout ? opts.timeout * 1000 : config.timeoutMs;

  const evmChain = srcFamily === "bitcoin" ? dstAsset.chain : srcAsset.chain;
  const signer = await resolveSignerForSwap(
    srcFamily === "bitcoin" ? "bitcoin" : evmChain,
    resolvePrivateKey(srcAsset.chain, opts.privateKey, config),
    opts.unsigned,
    srcFamily === "bitcoin",
  );

  const gasRefillWei = opts.gasRefillUsd
    ? await (async () => {
        const { priceUsd } = await fetchPrice("ETH");
        const ethAmount = opts.gasRefillUsd! / priceUsd;
        return humanToAtomic(ethAmount.toFixed(18), 18);
      })()
    : undefined;

  const quoteParams: GetQuoteParams = {
    fromChain: srcAsset.chain,
    toChain: dstAsset.chain,
    fromToken: srcAsset.address,
    toToken: dstAsset.address,
    toUserAddress: opts.recipient,
    fromUserAddress: opts.sender,
    amount: atomicUnits,
    maxSlippage: slippageBps,
    gasRefill: gasRefillWei ? BigInt(gasRefillWei) : undefined,
  };

  // ─── Unsigned mode ─────────────────────────────────────────────────────────

  if ("unsigned" in signer) {
    const { orderId, variant, order } = await createSwapOrder(sdk, quoteParams);

    if (srcFamily === "bitcoin") {
      const onrampData = (order as any).onramp;
      return { type: "unsigned", orderId, psbtBase64: onrampData?.psbtHex ? Buffer.from(onrampData.psbtHex, "hex").toString("base64") : undefined };
    } else {
      return { type: "unsigned", orderId, txInfo: (order as any)[variant]?.tx };
    }
  }

  // ─── Core flow: quote → order → sign → register ───────────────────────────

  const executeCore = async (): Promise<{ orderId: string; txId: string; outputAmount: string }> => {
    const { orderId, variant, order, outputAmount } = await createSwapOrder(sdk, quoteParams);

    let txId: string;

    if (srcFamily === "bitcoin") {
      const onrampData = (order as any).onramp;
      if (!onrampData?.psbtHex) throw new Error("Gateway did not return a PSBT for this onramp order.");
      const psbtHex = onrampData.psbtHex;
      txId = await (signer as { signer: BitcoinSigner; address: string }).signer.signAllInputs!(psbtHex);

      const registerTx = buildRegisterPayload(srcAsset.chain, dstAsset.chain, orderId, txId);
      try {
        await withRetry(
          () => sdk.api.registerTx({ registerTx }),
          { retries: 5, log, label: '  Retrying registration' },
        );
      } catch (err) { throw makeRegistrationError(err, orderId, txId); }
    } else {
      const { walletClient, publicClient } = signer as { walletClient: WalletClient; publicClient: PublicClient; address: string };

      if (opts.sender) {
        const addr = opts.sender as `0x${string}`;
        const deadline = Date.now() + 120_000;
        while (Date.now() < deadline) {
          const [latest, pending] = await Promise.all([
            publicClient.getTransactionCount({ address: addr, blockTag: "latest" }),
            publicClient.getTransactionCount({ address: addr, blockTag: "pending" }),
          ]);
          if (pending <= latest) break;
          await new Promise(r => setTimeout(r, 5000));
        }
      }

      const txInfo = (order as any)[variant]?.tx;
      if (!txInfo) throw new Error("Gateway did not return EVM transaction data for this order.");

      txId = await walletClient.sendTransaction({
        account: walletClient.account!,
        chain: null,
        to: txInfo.to as `0x${string}`,
        data: txInfo.data as `0x${string}`,
        value: BigInt(txInfo.value || "0"),
      });

      const registerTx = buildRegisterPayload(srcAsset.chain, dstAsset.chain, orderId, txId);
      try {
        await withRetry(
          () => sdk.api.registerTx({ registerTx }),
          { retries: 5, log, label: '  Retrying registration' },
        );
      } catch (err) { throw makeRegistrationError(err, orderId, txId); }
    }

    return { orderId, txId, outputAmount };
  };

  // ─── Execute with retry + poll ─────────────────────────────────────────────

  const result = opts.retry
    ? await withRetry(executeCore, { retries: 5, log, shouldRetry: isTransientError })
    : await executeCore();
  const { orderId, txId, outputAmount: quotedOutputAmount } = result;
  const startMs = Date.now();

  log.progress(`✓ Order submitted  (id: ${orderId})`);

  if (!opts.wait) {
    return {
      type: "submitted",
      data: { orderId, status: "submitted", srcAmount: atomicUnits, srcAsset: srcAsset.symbol, dstAsset: dstAsset.symbol, dstChain: dstAsset.chain, txId },
    };
  }

  try {
    const finalOrder = await pollOrder(sdk, orderId, {
      intervalMs: 15_000,
      timeoutMs: timeoutMs,
      onWaiting: () => log.progress(`  Waiting for confirmation...`),
    });

    const elapsedMs = Date.now() - startMs;
    const outAmt = finalOrder.dstInfo?.amount ?? "?";
    const quotedAmt = quotedOutputAmount;
    const slipBps = quotedAmt && finalOrder.dstInfo?.amount
      ? Math.round((1 - Number(finalOrder.dstInfo.amount) / Number(quotedAmt)) * 10000)
      : 0;

    log.progress(`✓ Confirmed — ${outAmt} ${dstAsset.symbol} delivered to ${opts.recipient}`);
    return {
      type: "confirmed",
      data: { orderId, status: "confirmed", srcAmount: atomicUnits, srcAsset: srcAsset.symbol, dstAmount: outAmt, dstAsset: dstAsset.symbol, dstChain: dstAsset.chain, quotedDstAmount: quotedAmt, actualSlippageBps: slipBps, txId, elapsedMs },
    };
  } catch (err) {
    if (srcFamily !== "bitcoin" && err instanceof Error && err.message === "pending") {
      log.progress(`Poll timeout reached. Checking mempool for pending delivery...`);
      const pendingTxs = await new MempoolClient().getAddressMempoolTxs(opts.recipient).catch(() => []);
      const mempoolTxId = pendingTxs.find(tx => !tx.status.confirmed)?.txid;
      if (mempoolTxId) {
        log.progress(`~ BTC tx found in mempool (unconfirmed): ${mempoolTxId}`);
        return {
          type: "mempoolPending",
          data: { orderId, status: "mempool_pending", srcAmount: atomicUnits, srcAsset: srcAsset.symbol, dstAsset: dstAsset.symbol, dstChain: dstAsset.chain, mempoolTxId },
        };
      }
    }
    throw err;
  }
}

// ─── Order polling ───────────────────────────────────────────────────────────

const TERMINAL_SUCCESS: GatewayOrderStatus[] = ["success", "strategy_skipped", "strategy_failed"];

async function pollOrder(
  sdk: { getOrder(id: string): Promise<GatewayOrderInfo> },
  orderId: string,
  opts: { intervalMs: number; timeoutMs: number; onWaiting?: () => void },
): Promise<GatewayOrderInfo> {
  return pRetry(
    async () => {
      opts.onWaiting?.();
      const order = await sdk.getOrder(orderId);
      if (TERMINAL_SUCCESS.includes(order.status as any)) return order;
      if (order.status === "refunded" || (typeof order.status === "object" && order.status !== null && "failed" in order.status)) {
        throw new AbortError(`Order ${orderId} failed: ${JSON.stringify(order.status)}`);
      }
      throw new Error("pending");
    },
    { retries: Math.ceil(opts.timeoutMs / opts.intervalMs), minTimeout: opts.intervalMs, maxTimeout: opts.intervalMs, factor: 1 },
  );
}

// ─── ALL amount resolution ──────────────────────────────────────────────────

async function resolveAllAmount(
  srcAsset: { chain: string; address: string; symbol: string; decimals: number },
  opts: { privateKey?: string; sender?: string; unsigned: boolean; feeToken?: string; feeReserve?: string },
  config: { bitcoinPrivateKey?: string; evmPrivateKey?: string },
): Promise<{ atomicUnits: string; display: string }> {
  // Derive sender address
  let senderAddress: string | undefined = opts.sender;
  if (!senderAddress) {
    const key = resolvePrivateKey(srcAsset.chain, opts.privateKey, config);
    if (!key) {
      throw new Error("--amount ALL requires a sender address. Use --private-key, --sender, or set BITCOIN_PRIVATE_KEY / EVM_PRIVATE_KEY.");
    }
    senderAddress = await deriveAddress(srcAsset.chain, key);
  }

  const bal = await getTokenBalance(srcAsset.chain, senderAddress, srcAsset.address, {
    feeToken: opts.feeToken,
    feeReserve: opts.feeReserve,
  });
  const resolvedAtomic = bal.allSpendable;

  if (BigInt(resolvedAtomic) === 0n) {
    throw new Error(`No ${srcAsset.symbol} balance found for ${senderAddress}`);
  }

  const humanDisplay = formatUnits(BigInt(resolvedAtomic), srcAsset.decimals);
  return { atomicUnits: resolvedAtomic, display: `${humanDisplay} ${srcAsset.symbol} (max spendable)` };
}
