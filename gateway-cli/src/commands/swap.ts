import { MempoolClient } from "@gobob/bob-sdk";
import type { BitcoinSigner, GetQuoteParams, GatewayCreateOrder } from "@gobob/bob-sdk";
import { getInnerQuoteV2 } from "../util/quote-v2.js";
import { type WalletClient, type PublicClient } from "viem";
import pRetry, { AbortError } from "p-retry";
import { getRoutes } from "../util/route-provider.js";
import { resolveSwapInputs, humanToAtomic, parseAssetChain, buildTokenIndex } from "../util/input-resolver.js";
import { fetchPrice } from "../util/price-oracle.js";
import { loadConfig, getSdk, getApi } from "../config.js";
import { deriveAddress, resolveSigner, getChainFamily, resolvePrivateKey, resolveRecipient } from "../chains/index.js";
import type { Logger, SwapSuccessJson, SwapSubmittedJson, SwapMempoolPendingJson } from "../output.js";

// ─── Types ───────────────────────────────────────────────────────────────────

export interface SwapOptions {
  src: string;
  dst: string;
  amount: string;
  recipient?: string;
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

  // ── Resolve inputs ──────────────────────────────────────────────────────────

  // Fetch routes first so we can accurately determine the source chain family
  // via the full alias/token resolution path rather than ad-hoc string parsing.
  const routes = await getRoutes();
  const tokenIndex = buildTokenIndex(routes);
  const srcFamily = getChainFamily(parseAssetChain(opts.src, routes, tokenIndex).chain);

  const key = resolvePrivateKey(srcFamily === "bitcoin" ? "bitcoin" : "evm", opts.privateKey, config);
  const derivedAddress = key ? await deriveAddress(srcFamily === "bitcoin" ? "bitcoin" : "evm", key) : undefined;
  const senderAddress = opts.sender ?? derivedAddress;

  // Reject --sender that doesn't match the signing key — the order would be created for one address but signed by another
  if (opts.sender && derivedAddress && opts.sender.toLowerCase() !== derivedAddress.toLowerCase()) {
    throw new Error(
      `--sender ${opts.sender} does not match the signing key (${derivedAddress}).\n` +
      `  The order would be created for one address but signed by another.\n` +
      `  Remove --sender to use the key-derived address, or use --unsigned to skip signing.`,
    );
  }

  // UX-8: BTC onramp --unsigned requires a sender address to construct the PSBT
  if (opts.unsigned && srcFamily === "bitcoin" && !senderAddress) {
    throw new Error("BTC onramp --unsigned requires --sender or BITCOIN_PRIVATE_KEY to construct the PSBT.");
  }
  const { srcAsset, dstAsset, atomicUnits, display } = await resolveSwapInputs(
    opts.src, opts.dst, opts.amount, routes,
    { senderAddress, feeToken: opts.feeToken, feeReserve: opts.feeReserve },
  );

  // ── Resolve recipient ────────────────────────────────────────────────────
  const recipient = await resolveRecipient(dstAsset.chain, opts.recipient, config);
  if (!opts.recipient) {
    const dstFamily = getChainFamily(dstAsset.chain);
    log.progress(`Using recipient: ${recipient} (derived from ${dstFamily === "bitcoin" ? "BITCOIN_PRIVATE_KEY" : "EVM_PRIVATE_KEY"})`);
  }

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
    toUserAddress: recipient,
    fromUserAddress: senderAddress,
    amount: atomicUnits,
    maxSlippage: slippageBps,
    gasRefill: gasRefillWei ? BigInt(gasRefillWei) : undefined,
  };

  // ── Quote + execute (retryable) ─────────────────────────────────────────────

  const TRANSIENT = [/TRM screening/i, /429/, /Too Many Requests/i, /rate limit/i, /not yet propagated/i, /BTC propagation/i, /timeout/i, /ECONNRESET/, /ETIMEDOUT/];
  const isTransient = (e: unknown) => TRANSIENT.some(p => p.test(e instanceof Error ? e.message : String(e)));

  // Determine variant from chain families: bitcoin src = onramp, bitcoin dst = offramp, else layerZero
  const variant = srcFamily === "bitcoin" ? "onramp"
    : getChainFamily(dstAsset.chain) === "bitcoin" ? "offramp"
    : "layerZero";

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
  // Use the V2 API directly to fetch the order with its PSBT (BTC) or unsigned tx (EVM).
  if (opts.unsigned) {
    const order: GatewayCreateOrder = await pRetry(async () => {
      try {
        const quote = await sdk.getQuote(quoteParams);
        return await getApi().createOrderV2({ gatewayQuoteV2: quote });
      } catch (err) {
        if (!isTransient(err)) {
          const abort = new AbortError(err instanceof Error ? err.message : String(err));
          if (err instanceof Error) abort.cause = err;
          throw abort;
        }
        throw err;
      }
    }, { retries });
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
  const { order, tx, outputAmount } = await pRetry(async () => {
    try {
      const quote = await sdk.getQuote(quoteParams);
      // For BTC paths walletClient/publicClient are unused inside the SDK; the
      // SDK's type forces them to be present, so we widen via `any` to satisfy it.
      const result = await sdk.executeQuote({
        quote,
        walletClient: evmClients?.walletClient as any,
        publicClient: evmClients?.publicClient as any,
        btcSigner,
      });
      return { ...result, outputAmount: getInnerQuoteV2(quote).outputAmount.amount };
    } catch (err) {
      if (!isTransient(err)) {
        const abort = new AbortError(err instanceof Error ? err.message : String(err));
        if (err instanceof Error) abort.cause = err;
        throw abort;
      }
      throw err;
    }
  }, { retries });

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
    const finalOrder = await pRetry(async () => {
      log.progress(`  Waiting for confirmation... (${Math.round((Date.now() - startMs) / 1000)}s elapsed)`);
      const o = await sdk.getOrder(orderId);
      if (hasKey(o.status, "success")) return o;
      if (hasKey(o.status, "refunded") || hasKey(o.status, "failed")) {
        throw new AbortError(`Order ${orderId} failed: ${JSON.stringify(o.status)}`);
      }
      throw new Error("pending");
    }, { retries: Math.ceil(timeoutMs / 15_000), minTimeout: 15_000, maxTimeout: 15_000, factor: 1 });

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
    if (getChainFamily(dstAsset.chain) === "bitcoin" && err instanceof Error && err.message === "pending") {
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
