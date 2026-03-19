import { getInnerQuote, getQuoteVariant } from "@gobob/bob-sdk";
import { getEnrichedRoutes } from "../util/route-provider.js";
import { ScureBitcoinSigner, type BitcoinSigner } from "@gobob/bob-sdk";
import { createWalletClient, createPublicClient, http, erc20Abi, formatUnits, type WalletClient, type PublicClient, type Hex } from "viem";
import { privateKeyToAccount } from "viem/accounts";
import { resolveSwapInputs, humanToAtomic } from "../util/input-resolver.js";
import { fetchPrice } from "../util/price-oracle.js";
import { getNativeToken, type EnrichedRoute } from "../util/route-provider.js";
import { MempoolClient } from "@gobob/bob-sdk";
import { resolveRpcUrl, getViemChain } from "../util/rpc-resolver.js";
import pRetry, { AbortError } from "p-retry";
import { loadConfig, getSdk } from "../config.js";
import type { GatewayOrderInfo, GatewayOrderStatus, GetQuoteParams } from "@gobob/bob-sdk";
import type { Logger, SwapSuccessJson, SwapSubmittedJson, SwapMempoolPendingJson } from "../output.js";

// ─── Transient error detection + retry ──────────────────────────────────────

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

async function withTransientRetry<T>(
  fn: () => Promise<T>,
  opts: { noRetry?: boolean; log: Logger },
): Promise<T> {
  if (opts.noRetry) return fn();
  return pRetry(
    async (attemptNumber) => {
      try {
        return await fn();
      } catch (err) {
        if (isTransientError(err)) {
          opts.log.progress(`Retrying (${attemptNumber}/6)...`);
          throw err;
        }
        throw new AbortError(err instanceof Error ? err.message : String(err));
      }
    },
    { retries: 5 },
  );
}

async function withRegistrationRetry<T>(
  fn: () => Promise<T>,
  log: Logger,
): Promise<T> {
  return pRetry(
    async (attemptNumber) => {
      if (attemptNumber > 1) log.progress(`  Retrying registration (attempt ${attemptNumber})...`);
      return fn();
    },
    { retries: 3 },
  );
}

// ─── Types ───────────────────────────────────────────────────────────────────

class RegistrationError extends Error {
  constructor(message: string, public readonly orderId: string, public readonly txId?: string) {
    super(message);
    this.name = "RegistrationError";
  }
}

export interface SwapOptions {
  src: string;
  dst: string;
  amount: string;
  recipient: string;
  sender?: string;
  slippage?: number;
  gasRefillUsd?: number;
  btcFeeRate?: number;
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
    const resolved = await resolveAllAmount(srcAsset, opts, config, enriched);
    atomicUnits = resolved.atomicUnits;
    display = resolved.display;
  } else {
    atomicUnits = parsed.atomicUnits;
    display = parsed.display;
  }

  const isBtcOnramp = srcAsset.chain === "bitcoin";
  const slippageBps = opts.slippage ?? config.slippageBps;
  const timeoutMs = opts.timeout ? opts.timeout * 1000 : config.timeoutMs;

  const evmChain = isBtcOnramp ? dstAsset.chain : srcAsset.chain;
  const signer = isBtcOnramp
    ? await resolveBtcSigner(opts.privateKey ?? config.bitcoinPrivateKey, opts.unsigned)
    : resolveEvmSigner(opts.privateKey ?? config.evmPrivateKey, opts.unsigned, evmChain);

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
    const quote = await sdk.getQuote(quoteParams);
    const variant = getQuoteVariant(quote);
    const order = await sdk.api.createOrder({ gatewayQuote: quote });
    const orderId = (order as any)[variant].orderId;

    if (isBtcOnramp) {
      const onrampData = (order as any).onramp;
      return { type: "unsigned", orderId, psbtBase64: onrampData?.psbtHex ? Buffer.from(onrampData.psbtHex, "hex").toString("base64") : undefined };
    } else {
      return { type: "unsigned", orderId, txInfo: (order as any)[variant]?.tx };
    }
  }

  // ─── Core flow: quote → order → sign → register ───────────────────────────

  const executeCore = async (): Promise<{ orderId: string; txId: string; outputAmount: string }> => {
    const quote = await sdk.getQuote(quoteParams);
    const outputAmount = getInnerQuote(quote).outputAmount.amount;
    const variant = getQuoteVariant(quote);
    const order = await sdk.api.createOrder({ gatewayQuote: quote });
    const orderId = (order as any)[variant].orderId;

    let txId: string;

    if (isBtcOnramp) {
      const onrampData = (order as any).onramp;
      if (!onrampData?.psbtHex) throw new Error("Gateway did not return a PSBT for this onramp order.");
      const psbtHex = onrampData.psbtHex;
      txId = await (signer as { signer: BitcoinSigner }).signer.signAllInputs!(psbtHex);

      try {
        await withRegistrationRetry(
          () => sdk.api.registerTx({ registerTx: { onramp: { orderId, bitcoinTxHex: txId } } }),
          log,
        );
      } catch (err) { throw registrationError(err, orderId, txId); }
    } else {
      const { walletClient, publicClient } = signer as { walletClient: WalletClient; publicClient: PublicClient };

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

      const registerPayload = variant === "layerZero"
        ? { registerTx: { layerZero: { orderId, evmTxhash: txId } } }
        : { registerTx: { offramp: { orderId, evmTxhash: txId } } };

      try {
        await withRegistrationRetry(
          () => sdk.api.registerTx(registerPayload),
          log,
        );
      } catch (err) { throw registrationError(err, orderId, txId); }
    }

    return { orderId, txId, outputAmount };
  };

  // ─── Execute with retry + poll ─────────────────────────────────────────────

  const result = await withTransientRetry(executeCore, { noRetry: !opts.retry, log });
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
    const slippageBps = quotedAmt && finalOrder.dstInfo?.amount
      ? Math.round((1 - Number(finalOrder.dstInfo.amount) / Number(quotedAmt)) * 10000)
      : 0;

    log.progress(`✓ Confirmed — ${outAmt} ${dstAsset.symbol} delivered to ${opts.recipient}`);
    return {
      type: "confirmed",
      data: { orderId, status: "confirmed", srcAmount: atomicUnits, srcAsset: srcAsset.symbol, dstAmount: outAmt, dstAsset: dstAsset.symbol, dstChain: dstAsset.chain, quotedDstAmount: quotedAmt, actualSlippageBps: slippageBps, txId, elapsedMs },
    };
  } catch (err) {
    if (!isBtcOnramp && err instanceof Error && err.message === "pending") {
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

// ─── Helpers ─────────────────────────────────────────────────────────────────

type BtcSignerResult = { signer: BitcoinSigner; address: string } | { unsigned: true };
type EvmSignerResult = { walletClient: WalletClient; publicClient: PublicClient } | { unsigned: true };

async function resolveBtcSigner(key: string | undefined, unsigned: boolean): Promise<BtcSignerResult> {
  if (unsigned) return { unsigned: true };
  if (!key) throw new Error("no signer configured for Bitcoin.\n  Set BITCOIN_PRIVATE_KEY or pass --private-key.\n  Use --unsigned to output the PSBT without signing.");
  const signer = ScureBitcoinSigner.fromKey(key);
  return { signer, address: await signer.getP2WPKHAddress() };
}

function resolveEvmSigner(key: string | undefined, unsigned: boolean, chainName: string): EvmSignerResult {
  if (unsigned) return { unsigned: true };
  if (!key) throw new Error("no signer configured for EVM.\n  Set EVM_PRIVATE_KEY or pass --private-key.\n  Use --unsigned to output the unsigned transaction.");
  const rpcUrl = resolveRpcUrl(chainName);
  const viemChain = getViemChain(chainName);
  const account = privateKeyToAccount((key.startsWith("0x") ? key : `0x${key}`) as Hex);
  const transport = http(rpcUrl);
  return {
    walletClient: createWalletClient({ account, chain: viemChain, transport }),
    publicClient: createPublicClient({ chain: viemChain, transport }),
  };
}

function registrationError(err: unknown, orderId: string, txId: string): RegistrationError {
  const msg = err instanceof Error ? err.message : String(err);
  return new RegistrationError(
    `CRITICAL: Transaction signed but registration failed after 3 retries. Last error: ${msg}\n` +
    `Order ID: ${orderId}\nManually register with: gateway-cli register ${orderId} ${txId}`,
    orderId, txId,
  );
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

const NATIVE_GAS_BUFFER = 900_000n;

async function resolveAllAmount(
  srcAsset: { chain: string; address: string; symbol: string; decimals: number },
  opts: { privateKey?: string; sender?: string; unsigned: boolean },
  config: { bitcoinPrivateKey?: string; evmPrivateKey?: string },
  enriched: EnrichedRoute[],
): Promise<{ atomicUnits: string; display: string }> {
  const isBtc = srcAsset.chain === "bitcoin";

  // Derive sender address
  let senderAddress: string | undefined = opts.sender;
  if (!senderAddress) {
    const key = isBtc
      ? (opts.privateKey ?? config.bitcoinPrivateKey)
      : (opts.privateKey ?? config.evmPrivateKey);
    if (!key) {
      throw new Error("--amount ALL requires a sender address. Use --private-key, --sender, or set BITCOIN_PRIVATE_KEY / EVM_PRIVATE_KEY.");
    }
    if (isBtc) {
      const signer = ScureBitcoinSigner.fromKey(key);
      senderAddress = await signer.getP2WPKHAddress();
    } else {
      const account = privateKeyToAccount((key.startsWith("0x") ? key : `0x${key}`) as Hex);
      senderAddress = account.address;
    }
  }

  let resolvedAtomic: string;
  if (isBtc) {
    const sdk = getSdk();
    const maxSpendable = await sdk.getMaxSpendable(senderAddress);
    resolvedAtomic = maxSpendable.amount.amount;
  } else if (isNativeToken(srcAsset)) {
    // Native EVM token (ETH, etc.) — reserve gas
    const rpcUrl = resolveRpcUrl(srcAsset.chain);
    const client = createPublicClient({ chain: getViemChain(srcAsset.chain), transport: http(rpcUrl) });
    const [balance, feeData] = await Promise.all([
      client.getBalance({ address: senderAddress as `0x${string}` }),
      client.estimateFeesPerGas(),
    ]);
    const gasCost = (feeData.maxFeePerGas ?? feeData.gasPrice ?? 0n) * NATIVE_GAS_BUFFER;
    const available = balance > gasCost ? balance - gasCost : 0n;
    resolvedAtomic = available.toString();
  } else {
    // ERC20 token
    const rpcUrl = resolveRpcUrl(srcAsset.chain);
    const client = createPublicClient({ chain: getViemChain(srcAsset.chain), transport: http(rpcUrl) });
    const balance = await client.readContract({
      address: srcAsset.address as `0x${string}`,
      abi: erc20Abi,
      functionName: "balanceOf",
      args: [senderAddress as `0x${string}`],
    });
    resolvedAtomic = balance.toString();
  }

  if (BigInt(resolvedAtomic) === 0n) {
    throw new Error(`No ${srcAsset.symbol} balance found for ${senderAddress}`);
  }

  const humanDisplay = formatUnits(BigInt(resolvedAtomic), srcAsset.decimals);
  return { atomicUnits: resolvedAtomic, display: `${humanDisplay} ${srcAsset.symbol} (max spendable)` };
}

function isNativeToken(asset: { chain: string; symbol: string }): boolean {
  const nt = getNativeToken(asset.chain);
  return asset.symbol.toUpperCase() === nt.symbol.toUpperCase();
}
