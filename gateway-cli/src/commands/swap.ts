import { MempoolClient } from "@gobob/bob-sdk";
import type { BitcoinSigner, GatewayCreateOrderV3 } from "@gobob/bob-sdk";
import { getInnerQuoteV3 } from "../util/quote.js";
import { type WalletClient, type PublicClient } from "viem";
import { withRetry, SwapError } from "../errors.js";
import { getRoutes } from "../util/route-provider.js";
import { resolveSwapContext, type SwapContextOptions } from "../util/swap-context.js";
import { loadConfig, getSdk, getApi } from "../config.js";
import { resolveSigner } from "../chains/index.js";
import { CHAIN_IDS } from "../chains/evm.js";
import type { Logger, SwapSuccessJson, SwapSubmittedJson, SwapMempoolPendingJson } from "../output.js";

// ─── Types ───────────────────────────────────────────────────────────────────

export interface SwapOptions extends SwapContextOptions {
  btcFeeRate?: number;
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

  // ── Resolve inputs ──────────────────────────────────────────────────────────
  // The SAME context `quote` builds: assets, families, sender, recipient, owner,
  // amount and quote params. `swap` additionally needs the sender address (it is the
  // quote's fromUserAddress, the nonce-check subject and the PSBT's input owner) and,
  // unless --unsigned, the key to sign with.
  const routes = await getRoutes();
  const ctx = await resolveSwapContext(opts, routes, config, {
    signing: !opts.unsigned,
    senderAddress: true,
  });
  const { srcAsset, dstAsset, srcFamily, dstFamily, variant, evmChain, senderAddress, recipient, atomicUnits } = ctx;

  // UX-8: BTC onramp --unsigned requires a sender address to construct the PSBT
  if (opts.unsigned && srcFamily === "bitcoin" && !senderAddress) {
    throw new Error("BTC onramp --unsigned requires --sender or BITCOIN_PRIVATE_KEY to construct the PSBT.");
  }

  if (!opts.recipient) {
    log.progress(`Using recipient: ${recipient} (derived from ${dstFamily === "bitcoin" ? "BITCOIN_PRIVATE_KEY" : "EVM_PRIVATE_KEY"})`);
  }

  const timeoutMs = opts.timeout ? opts.timeout * 1000 : config.timeoutMs;

  const signer = opts.unsigned ? null : await resolveSigner(srcFamily === "bitcoin" ? "bitcoin" : evmChain, ctx.key!);

  // ── Quote + execute (retryable) ─────────────────────────────────────────────

  const TRANSIENT = [/TRM screening/i, /429/, /Too Many Requests/i, /rate limit/i, /not yet propagated/i, /BTC propagation/i, /timeout/i, /ECONNRESET/, /ETIMEDOUT/];
  const isTransient = (e: unknown) => TRANSIENT.some(p => p.test(e instanceof Error ? e.message : String(e)));

  // For BTC source, signer holds a BitcoinSigner; for EVM, walletClient + publicClient.
  const btcSigner = !opts.unsigned && variant === "onramp"
    ? ((signer as any).signer as BitcoinSigner)
    : undefined;
  const evmClients = variant !== "onramp" && !opts.unsigned
    ? (signer as { walletClient: WalletClient; publicClient: PublicClient })
    : undefined;

  // EVM wait-for-pending-nonce — only relevant for the signed EVM path.
  if (evmClients && senderAddress) {
    const addr = senderAddress as `0x${string}`;
    const { publicClient } = evmClients;
    const deadline = Date.now() + 120_000;
    let settled = false;
    while (Date.now() < deadline) {
      const [latest, pending] = await Promise.all([
        publicClient.getTransactionCount({ address: addr, blockTag: "latest" }),
        publicClient.getTransactionCount({ address: addr, blockTag: "pending" }),
      ]);
      if (pending <= latest) { settled = true; break; }
      log.progress(`  Waiting for pending tx to settle (pending nonce: ${pending}, latest: ${latest})...`);
      await new Promise(r => setTimeout(r, 5000));
    }
    if (!settled) {
      throw new Error(`Timed out waiting for pending transactions to settle for ${addr}. There may be stuck transactions — check your wallet before retrying.`);
    }
  }

  // --unsigned has no public SDK path: executeQuote always signs.
  // Use the V3 API directly to fetch the order with its PSBT (BTC) or unsigned tx (EVM).
  if (opts.unsigned) {
    const order: GatewayCreateOrderV3 = await withRetry(async () => {
      const quote = await sdk.getQuote(ctx.quoteParams);
      return await getApi().createOrderV3({ gatewayQuoteV3: quote });
    }, { retries, isTransient });
    const orderData = (order as any)[variant];
    if (!orderData?.orderId) {
      throw new Error(`Unexpected API response: order.${variant} is missing or has no orderId.`);
    }
    if (variant === "onramp") {
      const psbtHex = orderData.psbtHex;
      if (!psbtHex) throw new Error("Gateway did not return a PSBT for this onramp order.");
      return { type: "unsigned", orderId: orderData.orderId, psbtBase64: Buffer.from(psbtHex, "hex").toString("base64") };
    }
    return { type: "unsigned", orderId: orderData.orderId, txInfo: orderData.tx };
  }

  // Signed flows go through executeQuote.
  // executeQuote handles createOrder + sign + registerTx in one call.
  // For BTC paths the SDK doesn't access walletClient/publicClient; cast to satisfy types.
  const { order, tx, outputAmount } = await withRetry(async () => {
    const quote = await sdk.getQuote(ctx.quoteParams);
    // For BTC paths walletClient/publicClient are unused inside the SDK; the
    // SDK's type forces them to be present, so we widen via `any` to satisfy it.
    const result = await sdk.executeQuote({
      quote,
      walletClient: evmClients?.walletClient as any,
      publicClient: evmClients?.publicClient as any,
      btcSigner,
    });
    return { ...result, outputAmount: getInnerQuoteV3(quote).outputAmount.amount };
  }, { retries, isTransient });

  const orderData = (order as any)[variant];
  if (!orderData?.orderId) {
    throw new Error(`Unexpected API response: order.${variant} is missing or has no orderId. Response keys: ${Object.keys(order).join(", ")}`);
  }
  const orderId: string = orderData.orderId;
  if (!tx) throw new Error("executeQuote did not return a transaction id for the signed flow.");
  const txId: string = tx;

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
  // V2 status is a discriminated object union: {success} | {refunded} | {failed} | {inProgress}.
  const hasKey = <K extends string>(s: unknown, k: K): s is Record<K, unknown> =>
    typeof s === "object" && s !== null && k in s;

  try {
    const finalOrder = await withRetry(async () => {
      log.progress(`  Waiting for confirmation... (${Math.round((Date.now() - startMs) / 1000)}s elapsed)`);
      const o = await sdk.getOrder(orderId);
      if (hasKey(o.status, "success")) return o;
      if (hasKey(o.status, "refunded") || hasKey(o.status, "failed")) {
        // Carry the on-chain settlement tx hash + route so the --json serializer
        // surfaces them (downstream alerting fetches the receipt to detect
        // out-of-gas). Message unchanged so existing categorization still matches.
        // Only offramp/tokenSwap have an EVM settlement tx; onramp settles on BTC.
        throw new SwapError(`Order ${orderId} failed: ${JSON.stringify(o.status)}`, {
          orderId,
          ...(variant !== "onramp" && {
            txId,
            txParams: { to: orderData.tx?.to, from: senderAddress, chainId: CHAIN_IDS[evmChain], chainName: evmChain },
            srcAsset: { symbol: srcAsset.symbol, chain: srcAsset.chain },
            dstAsset: { symbol: dstAsset.symbol, chain: dstAsset.chain },
          }),
        });
      }
      throw new Error("pending");
    }, { retries: Math.ceil(timeoutMs / 15_000), isTransient: (e) => e instanceof Error && e.message === "pending", minTimeout: 15_000, maxTimeout: 15_000 });

    const elapsedMs = Date.now() - startMs;
    const outAmt = finalOrder.dstInfo?.amount ?? "?";
    const slipBps = outputAmount && BigInt(outputAmount) !== 0n && finalOrder.dstInfo?.amount
      ? Number(10000n - BigInt(finalOrder.dstInfo.amount) * 10000n / BigInt(outputAmount))
      : 0;

    log.progress(`✓ Confirmed — ${outAmt} ${dstAsset.symbol} delivered to ${recipient}`);
    return {
      type: "confirmed",
      data: { orderId, status: "confirmed", srcAmount: atomicUnits, srcAsset: srcAsset.symbol, dstAmount: outAmt, dstAsset: dstAsset.symbol, dstChain: dstAsset.chain, quotedDstAmount: outputAmount, actualSlippageBps: slipBps, txId, elapsedMs },
    };
  } catch (err) {
    if (dstFamily === "bitcoin" && err instanceof Error && err.message === "pending") {
      log.progress(`Poll timeout reached. Checking mempool for pending delivery...`);
      const pendingTxs = await new MempoolClient().getAddressMempoolTxs(recipient).catch(mempoolErr => {
        log.warn(`could not check mempool: ${mempoolErr instanceof Error ? mempoolErr.message : String(mempoolErr)}`);
        return [];
      });
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
