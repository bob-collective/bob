import { createSdkClient } from "../adapter/sdk-client.js";
import { flattenQuote, detectVariant, type QuoteVariant } from "../adapter/quote-flattener.js";
import { enrichRoutes } from "../adapter/route-enricher.js";
import { getOrFetchRoutes } from "../util/route-cache.js";
import { resolveBtcSigner, signBtcWithResult } from "../signer/btc.js";
import { resolveEvmSigner, waitForNonceClear, type EvmSignerResult } from "../signer/evm.js";
import { pollOrder, type PollCallbacks } from "../polling/poll-order.js";
import { formatOutput, formatConfirmation } from "../output/formatter.js";
import { parseAssetChain } from "../util/asset-chain-parser.js";
import { parseAmount } from "../util/amount-parser.js";
import { findPendingMempoolTx } from "../util/mempool.js";
import { usdToWei } from "../util/price-oracle.js";
import { progress, warn } from "../util/progress.js";
import { retryWithBackoff, isTransientError, TransientError } from "../util/retry.js";
import { isJsonMode } from "../util/progress.js";
import { loadConfig } from "../config/index.js";
import type { GetQuoteParams, GatewayQuote } from "@gobob/bob-sdk";
import type {
  SwapSuccessJson,
  SwapSubmittedJson,
  SwapMempoolPendingJson,
  QuoteJson,
  ErrorJson,
} from "../output/json-shapes.js";
import { createInterface } from "node:readline";

export class RegistrationError extends Error {
  constructor(message: string, public readonly orderId: string) {
    super(message);
    this.name = "RegistrationError";
  }
}

export interface SwapOptions {
  apiUrl: string;
  src: string;
  dst: string;
  amount: string;
  recipient: string;
  sender?: string;
  slippageBps: number;
  gasRefillUsd?: number;
  btcFeeRate?: number;
  privateKey?: string;
  keystorePath?: string;
  keystorePassword?: string;
  evmRpcUrl?: string;
  bitcoinPrivateKey?: string;
  evmPrivateKey?: string;
  bitcoinSignerCmd?: string;
  evmSignerCmd?: string;
  autoConfirm: boolean;
  unsigned: boolean;
  dryRun: boolean;
  noWait: boolean;
  noRetry?: boolean;
  timeoutMs: number;
  json: boolean;
}

const RETRY_BACKOFF = [10000, 10000, 10000];
const TRANSIENT_RETRY_BACKOFF = [5000, 10000, 20000, 40000, 80000];
const TRANSIENT_MAX_ATTEMPTS = 5;

export async function handleSwap(opts: SwapOptions): Promise<string> {
  const config = loadConfig();
  const sdk = createSdkClient(opts.apiUrl);

  // Get enriched routes for token resolution
  const routes = await getOrFetchRoutes(() => sdk.getRoutes(), config.cache.ttl);
  const enriched = enrichRoutes(routes);

  const srcAsset = parseAssetChain(opts.src, enriched);
  const dstAsset = parseAssetChain(opts.dst, enriched);
  const isBtcOnramp = srcAsset.chain === "bitcoin";

  const parsed = await parseAmount(opts.amount, srcAsset.symbol, srcAsset.decimals);

  const signer = isBtcOnramp
    ? await resolveBtcSigner({
        privateKey: opts.privateKey,
        envPrivateKey: opts.bitcoinPrivateKey,
        externalSignerCmd: opts.bitcoinSignerCmd,
        unsigned: opts.unsigned,
      })
    : await resolveEvmSigner({
        privateKey: opts.privateKey,
        envPrivateKey: opts.evmPrivateKey,
        keystorePath: opts.keystorePath,
        keystorePassword: opts.keystorePassword,
        externalSignerCmd: opts.evmSignerCmd,
        unsigned: opts.unsigned,
        rpcUrl: opts.evmRpcUrl,
      });

  const gasRefillWei = opts.gasRefillUsd ? await usdToWei(opts.gasRefillUsd) : undefined;

  // Build SDK quote params
  const quoteParams: GetQuoteParams = {
    fromChain: srcAsset.chain,
    toChain: dstAsset.chain,
    fromToken: srcAsset.address,
    toToken: dstAsset.address,
    toUserAddress: opts.recipient,
    fromUserAddress: opts.sender,
    amount: parsed.atomicUnits,
    maxSlippage: opts.slippageBps,
    gasRefill: gasRefillWei ? BigInt(gasRefillWei) : undefined,
  };

  // ─── Core flow: quote → order → sign → register ────────────────────────────
  // This inner function is what gets retried on transient errors.

  const executeCore = async (): Promise<{ orderId: string; txId: string; flat: ReturnType<typeof flattenQuote>; quote: any }> => {
    const quote = await sdk.getQuote(quoteParams);
    const flat = flattenQuote(quote);

    // Detect quote variant
    const variant = detectVariant(quote);

    // Create order via SDK's lower-level API for full control over signing & registration
    const order = await sdk.api.createOrder({ gatewayQuote: quote });

    // Extract orderId based on variant
    const orderId = extractOrderId(order, variant);

    const isUnsigned = "unsigned" in signer;
    if (isUnsigned) {
      // Unsigned mode doesn't need retry - return early through a special path
      throw new UnsignedEarlyReturn(orderId, order, variant, flat, quote);
    }

    let txId: string;

    if (isBtcOnramp) {
      const onrampData = (order as any).onramp;
      if (!onrampData?.psbtHex) throw new Error("Gateway did not return a PSBT for this onramp order.");

      // signBtcWithResult expects base64, convert hex to base64
      const psbtBase64 = Buffer.from(onrampData.psbtHex, "hex").toString("base64");
      txId = await signBtcWithResult(signer as import("../signer/btc.js").BtcSignerResult, psbtBase64);

      // Register the signed BTC transaction
      const outcome = await retryWithBackoff(
        () => sdk.api.registerTx({
          registerTx: { onramp: { orderId, bitcoinTxHex: txId } },
        }),
        { retries: 3, backoffMs: RETRY_BACKOFF },
        (attempt) => progress(`  Retrying registration (attempt ${attempt}/3, 10s delay)...`),
      );

      if ("error" in outcome) {
        return handleRegistrationError(outcome.error, orderId, txId, opts.json) as never;
      }
    } else {
      const evmSigner = signer as Exclude<EvmSignerResult, { unsigned: true }>;
      const { walletClient, publicClient } = evmSigner;

      if (opts.sender) {
        const addr = opts.sender as `0x${string}`;
        const cleared = await waitForNonceClear(
          () => publicClient.getTransactionCount({ address: addr, blockTag: "latest" }).then(BigInt),
          () => publicClient.getTransactionCount({ address: addr, blockTag: "pending" }).then(BigInt),
          { maxWaitMs: 120_000, intervalMs: 5000 },
        );
        if (!cleared) warn(`pending transactions on ${opts.sender} did not clear within 2 min — proceeding anyway`);
      }

      const txInfo = extractTxInfo(order, variant);
      if (!txInfo) throw new Error("Gateway did not return EVM transaction data for this order.");

      txId = await walletClient.sendTransaction({
        account: walletClient.account!,
        chain: null,
        to: txInfo.to as `0x${string}`,
        data: txInfo.data as `0x${string}`,
        value: BigInt(txInfo.value || "0"),
      });

      // Register the EVM transaction
      const registerPayload = variant === "layerZero"
        ? { registerTx: { layerZero: { orderId, evmTxhash: txId } } }
        : { registerTx: { offramp: { orderId, evmTxhash: txId } } };

      const outcome = await retryWithBackoff(
        () => sdk.api.registerTx(registerPayload),
        { retries: 3, backoffMs: RETRY_BACKOFF },
        (attempt) => progress(`  Retrying registration (attempt ${attempt}/3, 10s delay)...`),
      );

      if ("error" in outcome) {
        return handleRegistrationError(outcome.error, orderId, txId, opts.json) as never;
      }
    }

    return { orderId, txId, flat, quote };
  };

  // ─── Initial quote for dry-run / confirmation ───────────────────────────────

  const initialQuote = await sdk.getQuote(quoteParams);
  const initialFlat = flattenQuote(initialQuote);

  if (opts.dryRun) {
    if (opts.json) {
      const shape: QuoteJson = {
        srcAmount: parsed.atomicUnits,
        srcAsset: srcAsset.symbol,
        dstAmount: initialFlat.outputAmount,
        dstAsset: dstAsset.symbol,
        dstChain: dstAsset.chain,
        slippageBps: opts.slippageBps,
        feeRateSatPerVbyte: opts.btcFeeRate,
      };
      return JSON.stringify(shape, null, 2);
    }
    return formatConfirmation({
      srcAmount: parsed.atomicUnits,
      srcAsset: srcAsset.symbol,
      srcDisplay: parsed.display,
      dstAmount: initialFlat.outputAmount,
      dstAsset: dstAsset.symbol,
      dstChain: dstAsset.chain,
      feeRateSatPerVbyte: opts.btcFeeRate,
      slippageBps: opts.slippageBps,
      recipient: opts.recipient,
      gasRefillUsd: opts.gasRefillUsd ? String(opts.gasRefillUsd) : undefined,
    });
  }

  if (!opts.autoConfirm && !opts.json) {
    const confirmMsg = formatConfirmation({
      srcAmount: parsed.atomicUnits,
      srcAsset: srcAsset.symbol,
      srcDisplay: parsed.display,
      dstAmount: initialFlat.outputAmount,
      dstAsset: dstAsset.symbol,
      dstChain: dstAsset.chain,
      feeRateSatPerVbyte: opts.btcFeeRate,
      slippageBps: opts.slippageBps,
      recipient: opts.recipient,
      gasRefillUsd: opts.gasRefillUsd ? String(opts.gasRefillUsd) : undefined,
    });
    const ok = await confirm(`${confirmMsg}\n\nProceed?`);
    if (!ok) {
      return opts.json
        ? JSON.stringify({ error: { code: 1, message: "cancelled" } } satisfies ErrorJson)
        : "Swap cancelled.";
    }
  }

  // ─── Execute with transient retry ──────────────────────────────────────────

  let result: { orderId: string; txId: string; flat: ReturnType<typeof flattenQuote>; quote: any };

  try {
    result = await executeWithTransientRetry(executeCore, opts);
  } catch (err) {
    if (err instanceof UnsignedEarlyReturn) {
      if (isBtcOnramp) {
        const onrampData = (err.order as any).onramp;
        const payload = { orderId: err.orderId, psbtBase64: onrampData?.psbtHex ? Buffer.from(onrampData.psbtHex, "hex").toString("base64") : undefined };
        return formatOutput(payload, opts.json ? "json" : "human");
      } else {
        const txInfo = extractTxInfo(err.order, err.variant);
        const payload = { orderId: err.orderId, txInfo };
        return formatOutput(payload, opts.json ? "json" : "human");
      }
    }
    if (err instanceof RegistrationFailure && opts.json) {
      const errData: ErrorJson = {
        error: { code: 3, message: `transaction signed but registration failed`, orderId: err.orderId, txId: err.txId },
      };
      return JSON.stringify(errData, null, 2);
    }
    throw err;
  }

  const { orderId, txId, flat } = result;
  const startMs = Date.now();

  progress(`✓ Order submitted  (id: ${orderId})`);

  if (opts.noWait) {
    const submitted: SwapSubmittedJson = {
      orderId,
      status: "submitted",
      srcAmount: parsed.atomicUnits,
      srcAsset: srcAsset.symbol,
      dstAsset: dstAsset.symbol,
      dstChain: dstAsset.chain,
      txId,
    };
    return opts.json ? JSON.stringify(submitted, null, 2) : formatOutput(submitted, "human");
  }

  const callbacks: PollCallbacks = {
    onWaiting: (elapsed) => progress(`  Waiting for confirmation...  [${formatElapsed(elapsed)}]`),
  };

  try {
    const finalOrder = await pollOrder(sdk, orderId, {
      intervalMs: 15_000,
      timeoutMs: opts.timeoutMs,
      callbacks,
    });

    const elapsedMs = Date.now() - startMs;
    const outAmt = finalOrder.dstInfo?.amount ?? "?";
    const quotedAmt = flat.outputAmount;
    const slippageBps = quotedAmt && finalOrder.dstInfo?.amount
      ? Math.round((1 - Number(finalOrder.dstInfo.amount) / Number(quotedAmt)) * 10000)
      : 0;

    const success: SwapSuccessJson = {
      orderId,
      status: "confirmed",
      srcAmount: parsed.atomicUnits,
      srcAsset: srcAsset.symbol,
      dstAmount: outAmt,
      dstAsset: dstAsset.symbol,
      dstChain: dstAsset.chain,
      quotedDstAmount: quotedAmt,
      actualSlippageBps: slippageBps,
      txId,
      elapsedMs,
    };
    progress(`✓ Confirmed — ${outAmt} ${dstAsset.symbol} delivered to ${opts.recipient}`);
    return opts.json ? JSON.stringify(success, null, 2) : formatOutput(success, "human");
  } catch (err) {
    if (!isBtcOnramp && err instanceof Error && err.name === "PollTimeoutError") {
      progress(`Poll timeout reached. Checking mempool for pending delivery...`);
      const mempoolTxId = await findPendingMempoolTx(opts.recipient);
      if (mempoolTxId) {
        progress(`~ BTC tx found in mempool (unconfirmed): ${mempoolTxId}`);
        const pending: SwapMempoolPendingJson = {
          orderId,
          status: "mempool_pending",
          srcAmount: parsed.atomicUnits,
          srcAsset: srcAsset.symbol,
          dstAsset: dstAsset.symbol,
          dstChain: dstAsset.chain,
          mempoolTxId,
        };
        return opts.json ? JSON.stringify(pending, null, 2) : formatOutput(pending, "human");
      }
    }
    throw err;
  }
}

// ─── Transient retry logic ────────────────────────────────────────────────────

async function executeWithTransientRetry<T>(
  fn: () => Promise<T>,
  opts: Pick<SwapOptions, "noRetry" | "json">,
): Promise<T> {
  if (opts.noRetry) {
    try {
      return await fn();
    } catch (err) {
      if (isTransientError(err)) {
        const msg = err instanceof Error ? err.message : String(err);
        throw new TransientError(msg);
      }
      throw err;
    }
  }

  // Default: retry transient errors with exponential backoff
  let lastError: unknown;
  for (let attempt = 1; attempt <= TRANSIENT_MAX_ATTEMPTS; attempt++) {
    try {
      return await fn();
    } catch (err) {
      if (!isTransientError(err)) throw err; // non-transient → fail immediately
      lastError = err;
      if (attempt < TRANSIENT_MAX_ATTEMPTS) {
        const delay = TRANSIENT_RETRY_BACKOFF[attempt - 1] ?? 80000;
        const delaySec = Math.round(delay / 1000);
        const reason = err instanceof Error ? err.message : String(err);
        emitRetryEvent(attempt + 1, TRANSIENT_MAX_ATTEMPTS, delaySec, reason, opts.json);
        await new Promise((r) => setTimeout(r, delay));
      }
    }
  }
  // All retries exhausted — throw as TransientError
  const msg = lastError instanceof Error ? (lastError as Error).message : String(lastError);
  throw new TransientError(msg);
}

function emitRetryEvent(attempt: number, maxAttempts: number, nextRetryIn: number, reason: string, json: boolean): void {
  if (json) {
    process.stderr.write(JSON.stringify({ event: "retry", reason, attempt, maxAttempts, nextRetryIn }) + "\n");
  } else {
    process.stderr.write(`Retrying (${attempt}/${maxAttempts}): ${reason}, waiting ${nextRetryIn}s...\n`);
  }
}

// Sentinel for unsigned early-return (avoids duplicating the unsigned path logic)
class UnsignedEarlyReturn {
  constructor(
    public readonly orderId: string,
    public readonly order: any,
    public readonly variant: QuoteVariant,
    public readonly flat: ReturnType<typeof flattenQuote>,
    public readonly quote: any,
  ) {}
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

function extractOrderId(order: any, variant: QuoteVariant): string {
  if (variant === "onramp" && order.onramp) return order.onramp.orderId;
  if (variant === "offramp" && order.offramp) return order.offramp.orderId;
  if (variant === "layerZero" && order.layerZero) return order.layerZero.orderId;
  throw new Error(`Cannot extract orderId from order for variant "${variant}"`);
}

function extractTxInfo(order: any, variant: QuoteVariant): { to: string; data: string; value: string; chain: string } | undefined {
  if (variant === "offramp" && order.offramp) return order.offramp.tx;
  if (variant === "layerZero" && order.layerZero) return order.layerZero.tx;
  return undefined;
}

class RegistrationFailure extends RegistrationError {
  readonly txId: string;
  constructor(message: string, orderId: string, txId: string) {
    super(message, orderId);
    this.txId = txId;
    this.name = "RegistrationFailure";
  }
}

function handleRegistrationError(error: unknown, orderId: string, txId: string, _json: boolean): never {
  const msg = error instanceof Error ? error.message : String(error);
  throw new RegistrationFailure(
    `CRITICAL: Transaction signed but registration failed after 3 retries. Last error: ${msg}\n` +
    `Order ID: ${orderId}\nManually register with: gateway-cli register ${orderId} ${txId}`,
    orderId,
    txId,
  );
}

async function confirm(message: string): Promise<boolean> {
  const rl = createInterface({ input: process.stdin, output: process.stderr });
  return new Promise((resolve) => {
    rl.question(`${message} [y/N] `, (answer) => {
      rl.close();
      resolve(answer.toLowerCase() === "y" || answer.toLowerCase() === "yes");
    });
  });
}

function formatElapsed(ms: number): string {
  const s = Math.floor(ms / 1000);
  const m = Math.floor(s / 60);
  const h = Math.floor(m / 60);
  return `${String(h).padStart(2, "0")}:${String(m % 60).padStart(2, "0")}:${String(s % 60).padStart(2, "0")}`;
}
