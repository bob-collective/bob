import { GatewayApiClient } from "../api/client.js";
import { resolveBtcSigner, signBtcWithSpec } from "../signer/btc.js";
import { resolveEvmSigner, waitForNonceClear, signEvmWithSpec } from "../signer/evm.js";
import { pollOrder, type PollCallbacks } from "../polling/poll-order.js";
import { formatOutput, formatConfirmation } from "../output/formatter.js";
import { parseAssetChain } from "../util/asset-chain-parser.js";
import { parseAmount } from "../util/amount-parser.js";
import { fetchFeeRate, findPendingMempoolTx } from "../util/mempool.js";
import { usdToWei } from "../util/price-oracle.js";
import { progress, warn } from "../util/progress.js";
import { retryWithBackoff } from "../util/retry.js";
import { BTC_ZERO_ADDRESS } from "../api/types.js";
import type {
  SwapSuccessJson,
  SwapSubmittedJson,
  SwapMempoolPendingJson,
  QuoteJson,
  ErrorJson,
} from "../output/json-shapes.js";
import { createPublicClient, http } from "viem";
import * as viemChains from "viem/chains";
import { createInterface } from "node:readline";

function getViemChainId(chainName: string): number | null {
  const lookup: Record<string, string> = {
    ethereum: "mainnet",
    arbitrum: "arbitrum",
    base: "base",
    optimism: "optimism",
    polygon: "polygon",
    bsc: "bsc",
    avalanche: "avalanche",
    bob: "bob",
    sonic: "sonic",
    bera: "berachain",
    unichain: "unichain",
    soneium: "soneium",
    telos: "telos",
    swell: "swellchain",
  };
  const viemKey = lookup[chainName.toLowerCase()];
  if (!viemKey) return null;
  const chain = (viemChains as Record<string, { id?: number }>)[viemKey];
  return chain?.id ?? null;
}

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
  timeoutMs: number;
  json: boolean;
}

const RETRY_BACKOFF = [10000, 10000, 10000];

export async function handleSwap(opts: SwapOptions): Promise<string> {
  const client = new GatewayApiClient(opts.apiUrl);
  const routes = await client.getRoutes();

  const srcAsset = parseAssetChain(opts.src, routes);
  const dstAsset = parseAssetChain(opts.dst, routes);
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
      });

  let feeRate = opts.btcFeeRate;
  if (isBtcOnramp && !feeRate) {
    feeRate = await fetchFeeRate();
  }

  const gasRefillWei = opts.gasRefillUsd ? await usdToWei(opts.gasRefillUsd) : undefined;

  const quote = await client.getQuote({
    srcChain: srcAsset.chain,
    dstChain: dstAsset.chain,
    srcToken: srcAsset.address,
    dstToken: dstAsset.address,
    amount: parsed.atomicUnits,
    recipient: opts.recipient,
    sender: opts.sender,
    slippage: opts.slippageBps,
    gasRefill: gasRefillWei,
  });

  if (opts.dryRun) {
    if (opts.json) {
      const shape: QuoteJson = {
        srcAmount: parsed.atomicUnits,
        srcAsset: srcAsset.symbol,
        dstAmount: quote.outputAmount,
        dstAsset: dstAsset.symbol,
        dstChain: dstAsset.chain,
        slippageBps: opts.slippageBps,
        feeRateSatPerVbyte: feeRate,
        gasRefillEth: gasRefillWei,
      };
      return JSON.stringify(shape, null, 2);
    }
    return formatConfirmation({
      srcAmount: parsed.atomicUnits,
      srcAsset: srcAsset.symbol,
      srcDisplay: parsed.display,
      dstAmount: quote.outputAmount,
      dstAsset: dstAsset.symbol,
      dstChain: dstAsset.chain,
      feeRateSatPerVbyte: feeRate,
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
      dstAmount: quote.outputAmount,
      dstAsset: dstAsset.symbol,
      dstChain: dstAsset.chain,
      feeRateSatPerVbyte: feeRate,
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

  const order = await client.createOrder(quote);

  if (signer.type === "unsigned") {
    const payload = isBtcOnramp
      ? { orderId: order.orderId, psbtBase64: order.psbtBase64 }
      : { orderId: order.orderId, txInfo: order.txInfo };
    return formatOutput(payload, opts.json ? "json" : "human");
  }

  const startMs = Date.now();
  let txId: string;

  if (isBtcOnramp) {
    if (!order.psbtBase64) throw new Error("Gateway did not return a PSBT for this onramp order.");
    txId = await signBtcWithSpec(signer, order.psbtBase64);
  } else {
    if (opts.sender && opts.evmRpcUrl) {
      const rpc = createPublicClient({ transport: http(opts.evmRpcUrl) });
      const addr = opts.sender as `0x${string}`;
      const cleared = await waitForNonceClear(
        () => rpc.getTransactionCount({ address: addr, blockTag: "latest" }).then(BigInt),
        () => rpc.getTransactionCount({ address: addr, blockTag: "pending" }).then(BigInt),
        { maxWaitMs: 120_000, intervalMs: 5000 },
      );
      if (!cleared) warn(`pending transactions on ${opts.sender} did not clear within 2 min — proceeding anyway`);
    }

    if (!order.txInfo) throw new Error("Gateway did not return EVM transaction data for this offramp order.");
    const chainId = getViemChainId(order.txInfo.chain);
    if (chainId === null) throw new Error(`Cannot get chain ID for "${order.txInfo.chain}"`);

    if (signer.type === "private-key" && !opts.evmRpcUrl) {
      throw new Error("EVM_RPC_URL is required when using --private-key for EVM signing.");
    }
    txId = await signEvmWithSpec(
      signer,
      { to: order.txInfo.to, data: order.txInfo.data, value: order.txInfo.value, chainId },
      opts.evmRpcUrl ?? "",
    );
  }

  const outcome = await retryWithBackoff(
    () => client.registerTx({
      orderId: order.orderId,
      ...(isBtcOnramp ? { bitcoinTxHex: txId } : { evmTxhash: txId }),
    }),
    { retries: 3, backoffMs: RETRY_BACKOFF },
    (attempt) => progress(`  Retrying registration (attempt ${attempt}/3, 10s delay)...`),
  );

  if ("error" in outcome) {
    const msg = outcome.error instanceof Error ? outcome.error.message : String(outcome.error);
    const errData: ErrorJson = {
      error: { code: 3, message: `transaction signed but registration failed: ${msg}`, orderId: order.orderId, txId },
    };
    if (opts.json) return JSON.stringify(errData, null, 2);
    throw new RegistrationError(
      `CRITICAL: Transaction signed but registration failed after 3 retries. Last error: ${msg}\n` +
      `Order ID: ${order.orderId}\nManually register with: gateway-cli register ${order.orderId} ${txId}`,
      order.orderId,
    );
  }

  progress(`✓ Order submitted  (id: ${order.orderId})`);

  if (opts.noWait) {
    const submitted: SwapSubmittedJson = {
      orderId: order.orderId,
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
    const finalOrder = await pollOrder(client, order.orderId, {
      intervalMs: 15_000,
      timeoutMs: opts.timeoutMs,
      callbacks,
    });

    const elapsedMs = Date.now() - startMs;
    const outAmt = finalOrder.outputAmount ?? "?";
    const quotedAmt = quote.outputAmount;
    const slippageBps = quotedAmt && finalOrder.outputAmount
      ? Math.round((1 - Number(finalOrder.outputAmount) / Number(quotedAmt)) * 10000)
      : 0;

    const success: SwapSuccessJson = {
      orderId: order.orderId,
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
          orderId: order.orderId,
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

// Re-export BTC_ZERO_ADDRESS so it's available for external consumers via this module
export { BTC_ZERO_ADDRESS };
