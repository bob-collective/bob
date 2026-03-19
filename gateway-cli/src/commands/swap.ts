import { getInnerQuote, getQuoteVariant } from "@gobob/bob-sdk";
import { getEnrichedRoutes } from "../util/route-provider.js";
import { ScureBitcoinSigner, type BitcoinSigner } from "@gobob/bob-sdk";
import { createWalletClient, createPublicClient, http, type WalletClient, type PublicClient, type Hex } from "viem";
import { privateKeyToAccount } from "viem/accounts";
import { resolveSwapInputs, parseAmountUsd } from "../util/input-resolver.js";
import { getNativeToken } from "../util/route-provider.js";
import { MempoolClient } from "@gobob/bob-sdk";
import { resolveRpcUrl, getViemChain } from "../util/rpc-resolver.js";
import { retryWithBackoff, executeWithTransientRetry } from "../util/retry.js";
import { loadConfig, getSdk } from "../config.js";
import type { GatewayOrderInfo, GatewayOrderStatus, GetQuoteParams } from "@gobob/bob-sdk";
import type { Logger, SwapSuccessJson, SwapSubmittedJson, SwapMempoolPendingJson } from "../output.js";

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
  amount?: string;
  amountAtomic?: string;
  amountUsd?: string;
  recipient: string;
  sender?: string;
  slippage?: number;
  gasRefill?: number;
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
    opts.src, opts.dst, { amount: opts.amount, amountAtomic: opts.amountAtomic, amountUsd: opts.amountUsd }, enriched,
  );
  const isBtcOnramp = srcAsset.chain === "bitcoin";
  const slippageBps = opts.slippage ?? config.slippageBps;
  const timeoutMs = opts.timeout ? opts.timeout * 1000 : config.timeoutMs;

  const evmChain = isBtcOnramp ? dstAsset.chain : srcAsset.chain;
  const signer = isBtcOnramp
    ? await resolveBtcSigner(opts.privateKey ?? config.bitcoinPrivateKey, opts.unsigned)
    : resolveEvmSigner(opts.privateKey ?? config.evmPrivateKey, opts.unsigned, evmChain);

  const gasRefillWei = opts.gasRefill
    ? (await parseAmountUsd(String(opts.gasRefill), "ETH", getNativeToken("ethereum").decimals)).atomicUnits
    : undefined;

  const quoteParams: GetQuoteParams = {
    fromChain: srcAsset.chain,
    toChain: dstAsset.chain,
    fromToken: srcAsset.address,
    toToken: dstAsset.address,
    toUserAddress: opts.recipient,
    fromUserAddress: opts.sender,
    amount: parsed.atomicUnits,
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
        await retryWithBackoff(
          () => sdk.api.registerTx({ registerTx: { onramp: { orderId, bitcoinTxHex: txId } } }),
          { onRetry: (attempt) => log.progress(`  Retrying registration (attempt ${attempt})...`) },
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
        await retryWithBackoff(
          () => sdk.api.registerTx(registerPayload),
          { onRetry: (attempt) => log.progress(`  Retrying registration (attempt ${attempt})...`) },
        );
      } catch (err) { throw registrationError(err, orderId, txId); }
    }

    return { orderId, txId, outputAmount };
  };

  // ─── Execute with retry + poll ─────────────────────────────────────────────

  const result = await executeWithTransientRetry(executeCore, { noRetry: !opts.retry, log });
  const { orderId, txId, outputAmount: quotedOutputAmount } = result;
  const startMs = Date.now();

  log.progress(`✓ Order submitted  (id: ${orderId})`);

  if (!opts.wait) {
    return {
      type: "submitted",
      data: { orderId, status: "submitted", srcAmount: parsed.atomicUnits, srcAsset: srcAsset.symbol, dstAsset: dstAsset.symbol, dstChain: dstAsset.chain, txId },
    };
  }

  try {
    const finalOrder = await pollOrder(sdk, orderId, {
      intervalMs: 15_000,
      timeoutMs: timeoutMs,
      callbacks: { onWaiting: (elapsed) => log.progress(`  Waiting for confirmation...  [${formatElapsed(elapsed)}]`) },
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
      data: { orderId, status: "confirmed", srcAmount: parsed.atomicUnits, srcAsset: srcAsset.symbol, dstAmount: outAmt, dstAsset: dstAsset.symbol, dstChain: dstAsset.chain, quotedDstAmount: quotedAmt, actualSlippageBps: slippageBps, txId, elapsedMs },
    };
  } catch (err) {
    if (!isBtcOnramp && err instanceof Error && err.name === "PollTimeoutError") {
      log.progress(`Poll timeout reached. Checking mempool for pending delivery...`);
      const pendingTxs = await new MempoolClient().getAddressMempoolTxs(opts.recipient).catch(() => []);
      const mempoolTxId = pendingTxs.find(tx => !tx.status.confirmed)?.txid;
      if (mempoolTxId) {
        log.progress(`~ BTC tx found in mempool (unconfirmed): ${mempoolTxId}`);
        return {
          type: "mempoolPending",
          data: { orderId, status: "mempool_pending", srcAmount: parsed.atomicUnits, srcAsset: srcAsset.symbol, dstAsset: dstAsset.symbol, dstChain: dstAsset.chain, mempoolTxId },
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

function formatElapsed(ms: number): string {
  const s = Math.floor(ms / 1000);
  const m = Math.floor(s / 60);
  const h = Math.floor(m / 60);
  return `${String(h).padStart(2, "0")}:${String(m % 60).padStart(2, "0")}:${String(s % 60).padStart(2, "0")}`;
}

// ─── Order polling ───────────────────────────────────────────────────────────

class PollTimeoutError extends Error {
  constructor(public readonly orderId: string, public readonly timeoutMs: number) {
    super(`Polling timed out after ${timeoutMs}ms for order "${orderId}"`);
    this.name = "PollTimeoutError";
  }
}

interface PollCallbacks { onWaiting?: (elapsedMs: number) => void; }

function isTerminalSuccess(status: GatewayOrderStatus): boolean {
  return status === "success" || status === "strategy_skipped" || status === "strategy_failed";
}

function isTerminalFailure(status: GatewayOrderStatus): boolean {
  if (status === "refunded") return true;
  if (typeof status === "object" && status !== null && "failed" in status) return true;
  return false;
}

async function pollOrder(
  client: { getOrder(id: string): Promise<GatewayOrderInfo> },
  orderId: string,
  opts: { intervalMs: number; timeoutMs: number; callbacks?: PollCallbacks },
): Promise<GatewayOrderInfo> {
  const start = Date.now();
  const deadline = start + opts.timeoutMs;
  while (true) {
    if (Date.now() >= deadline) throw new PollTimeoutError(orderId, opts.timeoutMs);
    opts.callbacks?.onWaiting?.(Date.now() - start);
    const order = await client.getOrder(orderId);
    if (isTerminalSuccess(order.status)) return order;
    if (isTerminalFailure(order.status)) throw new Error(`Order ${orderId} failed with status: ${JSON.stringify(order.status)}`);
    await new Promise((r) => setTimeout(r, opts.intervalMs));
  }
}
