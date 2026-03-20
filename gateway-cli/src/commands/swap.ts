import { getInnerQuote, getQuoteVariant, MempoolClient } from "@gobob/bob-sdk";
import type { BitcoinSigner, GatewayOrderInfo, GatewayOrderStatus, GetQuoteParams } from "@gobob/bob-sdk";
import { formatUnits, type WalletClient, type PublicClient } from "viem";
import pRetry, { AbortError } from "p-retry";
import { getRoutes } from "../util/route-provider.js";
import { resolveSwapInputs, humanToAtomic } from "../util/input-resolver.js";
import { fetchPrice } from "../util/price-oracle.js";
import { loadConfig, getSdk } from "../config.js";
import { deriveAddress, resolveSigner, buildRegisterPayload, getChainFamily, resolvePrivateKey } from "../chains/index.js";
import type { Logger, SwapSuccessJson, SwapSubmittedJson, SwapMempoolPendingJson } from "../output.js";

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
  | { type: "submitted"; data: SwapSubmittedJson }
  | { type: "confirmed"; data: SwapSuccessJson }
  | { type: "mempoolPending"; data: SwapMempoolPendingJson };

// ─── Handler ─────────────────────────────────────────────────────────────────

export async function handleSwap(opts: SwapOptions, log: Logger): Promise<SwapResult> {
  const config = loadConfig();
  const sdk = getSdk();
  const retries = opts.retry ? 5 : 0;
  const srcFamily = getChainFamily(opts.src.includes(":") ? opts.src.split(":")[1] : opts.src === "BTC" || opts.src === "btc" ? "bitcoin" : opts.src);

  // ── Resolve inputs ──────────────────────────────────────────────────────────

  const key = resolvePrivateKey(srcFamily === "bitcoin" ? "bitcoin" : "evm", opts.privateKey, config);
  const senderAddress = opts.sender ?? (key ? await deriveAddress(srcFamily === "bitcoin" ? "bitcoin" : "evm", key) : undefined);

  const routes = await getRoutes();
  const { srcAsset, dstAsset, atomicUnits, display } = await resolveSwapInputs(
    opts.src, opts.dst, opts.amount, routes,
    { senderAddress, feeToken: opts.feeToken, feeReserve: opts.feeReserve },
  );

  const slippageBps = opts.slippage ?? config.slippageBps;
  const timeoutMs = opts.timeout ? opts.timeout * 1000 : config.timeoutMs;

  // Resolve signer
  if (!opts.unsigned && !key) {
    const isBtc = getChainFamily(srcAsset.chain) === "bitcoin";
    throw new Error(`no signer configured for ${isBtc ? "Bitcoin" : "EVM"}.\n  Set ${isBtc ? "BITCOIN_PRIVATE_KEY" : "EVM_PRIVATE_KEY"} or pass --private-key.\n  Use --unsigned to output the ${isBtc ? "PSBT" : "unsigned transaction"} without signing.`);
  }
  const evmChain = getChainFamily(srcAsset.chain) === "bitcoin" ? dstAsset.chain : srcAsset.chain;
  const signer = opts.unsigned ? null : await resolveSigner(getChainFamily(srcAsset.chain) === "bitcoin" ? "bitcoin" : evmChain, key!);

  // Gas refill
  const gasRefillWei = opts.gasRefillUsd
    ? humanToAtomic((opts.gasRefillUsd / (await fetchPrice("ETH")).priceUsd).toFixed(18), 18)
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

  // ── Quote + order (retryable) ───────────────────────────────────────────────

  const TRANSIENT = [/TRM screening/i, /429/, /Too Many Requests/i, /rate limit/i, /not yet propagated/i, /BTC propagation/i, /timeout/i, /ECONNRESET/, /ETIMEDOUT/];
  const isTransient = (e: unknown) => TRANSIENT.some(p => p.test(e instanceof Error ? e.message : String(e)));

  const { orderId, variant, order, outputAmount } = await pRetry(async () => {
    try {
      const quote = await sdk.getQuote(quoteParams);
      const variant = getQuoteVariant(quote);
      const order = await sdk.api.createOrder({ gatewayQuote: quote });
      return {
        variant,
        order,
        orderId: (order as any)[variant].orderId as string,
        outputAmount: getInnerQuote(quote).outputAmount.amount as string,
      };
    } catch (err) {
      if (!isTransient(err)) throw new AbortError(err instanceof Error ? err.message : String(err));
      throw err;
    }
  }, { retries });

  // ── Unsigned → done ─────────────────────────────────────────────────────────

  if (opts.unsigned) {
    if (getChainFamily(srcAsset.chain) === "bitcoin") {
      const psbtHex = (order as any).onramp?.psbtHex;
      return { type: "unsigned", orderId, psbtBase64: psbtHex ? Buffer.from(psbtHex, "hex").toString("base64") : undefined };
    }
    return { type: "unsigned", orderId, txInfo: (order as any)[variant]?.tx };
  }

  // ── Sign (no retry) ────────────────────────────────────────────────────────

  let txId: string;
  if (getChainFamily(srcAsset.chain) === "bitcoin") {
    const psbtHex = (order as any).onramp?.psbtHex;
    if (!psbtHex) throw new Error("Gateway did not return a PSBT for this onramp order.");
    txId = await ((signer as any).signer as BitcoinSigner).signAllInputs!(psbtHex);
  } else {
    const { walletClient, publicClient } = signer as { walletClient: WalletClient; publicClient: PublicClient };

    // Wait for pending nonce to settle
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
  }

  // ── Register (retryable) ───────────────────────────────────────────────────

  const registerPayload = buildRegisterPayload(srcAsset.chain, dstAsset.chain, orderId, txId);
  await pRetry(() => sdk.api.registerTx({ registerTx: registerPayload }), { retries })
    .catch(err => {
      const msg = err instanceof Error ? err.message : String(err);
      const error = new Error(
        `Registration failed. Last error: ${msg}\nOrder ID: ${orderId}\nManually register with: gateway-cli register ${orderId} ${txId}`,
      );
      (error as any).orderId = orderId;
      (error as any).txId = txId;
      throw error;
    });

  log.progress(`✓ Order submitted (id: ${orderId})`);

  // ── No-wait → done ─────────────────────────────────────────────────────────

  if (!opts.wait) {
    return {
      type: "submitted",
      data: { orderId, status: "submitted", srcAmount: atomicUnits, srcAsset: srcAsset.symbol, dstAsset: dstAsset.symbol, dstChain: dstAsset.chain, txId },
    };
  }

  // ── Poll ────────────────────────────────────────────────────────────────────

  const startMs = Date.now();
  const TERMINAL_SUCCESS: GatewayOrderStatus[] = ["success", "strategy_skipped", "strategy_failed"];

  try {
    const finalOrder = await pRetry(async () => {
      log.progress(`  Waiting for confirmation...`);
      const o = await sdk.getOrder(orderId);
      if (TERMINAL_SUCCESS.includes(o.status as any)) return o;
      if (o.status === "refunded" || (typeof o.status === "object" && o.status !== null && "failed" in o.status)) {
        throw new AbortError(`Order ${orderId} failed: ${JSON.stringify(o.status)}`);
      }
      throw new Error("pending");
    }, { retries: Math.ceil(timeoutMs / 15_000), minTimeout: 15_000, maxTimeout: 15_000, factor: 1 });

    const elapsedMs = Date.now() - startMs;
    const outAmt = finalOrder.dstInfo?.amount ?? "?";
    const slipBps = outputAmount && finalOrder.dstInfo?.amount
      ? Math.round((1 - Number(finalOrder.dstInfo.amount) / Number(outputAmount)) * 10000)
      : 0;

    log.progress(`✓ Confirmed — ${outAmt} ${dstAsset.symbol} delivered to ${opts.recipient}`);
    return {
      type: "confirmed",
      data: { orderId, status: "confirmed", srcAmount: atomicUnits, srcAsset: srcAsset.symbol, dstAmount: outAmt, dstAsset: dstAsset.symbol, dstChain: dstAsset.chain, quotedDstAmount: outputAmount, actualSlippageBps: slipBps, txId, elapsedMs },
    };
  } catch (err) {
    if (getChainFamily(srcAsset.chain) !== "bitcoin" && err instanceof Error && err.message === "pending") {
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
