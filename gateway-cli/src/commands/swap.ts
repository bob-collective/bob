import type { BitcoinSigner, GetQuoteParams, GatewayCreateOrderV3 } from "@gobob/bob-sdk";
import { getInnerQuoteV3 } from "../util/quote.js";
import { type WalletClient, type PublicClient } from "viem";
import { withRetry, SwapError } from "../errors.js";
import { getRoutes } from "../util/route-provider.js";
import { resolveSwapInputs, parseAssetChain, buildTokenIndex } from "../util/input-resolver.js";
import { loadConfig, getSdk, getApi } from "../config.js";
import { deriveAddress, resolveSigner, getChainFamily, resolvePrivateKey, resolveRecipient } from "../chains/index.js";
import { CHAIN_IDS } from "../chains/evm.js";
import type { Logger, SwapSuccessJson, SwapSubmittedJson, SwapMempoolPendingJson, SwapInFlightJson } from "../output.js";

// ─── Types ───────────────────────────────────────────────────────────────────

export interface SwapOptions {
  src: string;
  dst: string;
  amount: string;
  recipient?: string;
  sender?: string;
  owner?: string;
  slippage?: number;
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
  | { type: "mempoolPending"; data: SwapMempoolPendingJson }
  | { type: "inFlight"; data: SwapInFlightJson };

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

  const ownerAddress = opts.owner ?? (srcFamily === "bitcoin" ? recipient : (senderAddress ?? recipient));

  const quoteParams: GetQuoteParams = {
    fromChain: srcAsset.chain,
    toChain: dstAsset.chain,
    fromToken: srcAsset.address,
    toToken: dstAsset.address,
    toUserAddress: recipient,
    fromUserAddress: senderAddress,
    ownerAddress,
    amount: atomicUnits,
    maxSlippage: slippageBps,
  };

  // ── Quote + execute (retryable) ─────────────────────────────────────────────

  const TRANSIENT = [/TRM screening/i, /429/, /Too Many Requests/i, /rate limit/i, /not yet propagated/i, /BTC propagation/i, /timeout/i, /ECONNRESET/, /ETIMEDOUT/];
  const isTransient = (e: unknown) => TRANSIENT.some(p => p.test(e instanceof Error ? e.message : String(e)));

  // Determine variant from chain families: bitcoin src = onramp, bitcoin dst = offramp, else tokenSwap
  const variant = srcFamily === "bitcoin" ? "onramp"
    : getChainFamily(dstAsset.chain) === "bitcoin" ? "offramp"
    : "tokenSwap";

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
      const quote = await sdk.getQuote(quoteParams);
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
    const quote = await sdk.getQuote(quoteParams);
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
  //
  // The source funds are committed on-chain from here on, so the ONLY authority on
  // whether the swap failed is the order status itself. Failing to *read* that status
  // (gateway 5xx, Cloudflare 403, connection reset, DNS blip) tells us nothing about
  // the swap and must never be reported as a swap failure — the order is settling
  // regardless of whether we can see it. Read errors are therefore always retried,
  // with backoff, until the caller's --timeout is exhausted; if the order still has
  // not settled by then we exit in the non-failure "in flight" state below, carrying
  // the orderId + source txId so the order can be followed up.

  const POLL_INTERVAL_MS = 15_000;
  const MAX_BACKOFF_MS = 60_000;
  // Per-attempt read budget. Without it the loop's deadline is unenforceable: a stalled
  // connection (server accepts, never responds) leaves `await getOrder` open forever and
  // `Date.now() < deadline` is never re-evaluated. Aborting the request cancels it for
  // real — unlike racing a timer, which leaves the socket dangling per attempt.
  const GET_ORDER_TIMEOUT_MS = 30_000;
  const sleep = (ms: number) => new Promise(r => setTimeout(r, ms));

  const startMs = Date.now();
  const deadline = startMs + timeoutMs;
  // V2 status is a discriminated object union: {success} | {refunded} | {failed} | {inProgress}.
  const hasKey = <K extends string>(s: unknown, k: K): s is Record<K, unknown> =>
    typeof s === "object" && s !== null && k in s;

  let readFailures = 0;
  let lastReadError: string | undefined;
  // The BTC payout tx the order itself reports having broadcast (offramp only).
  let pendingBtcPaymentTxId: string | undefined;

  while (Date.now() < deadline) {
    // The try wraps ONLY the read: everything below it interprets a status we did
    // successfully read, and a bug there must surface, not be retried as a read error.
    let order: Awaited<ReturnType<typeof sdk.getOrder>> | undefined;
    // Never let a single read outlive the caller's --timeout.
    const attemptTimeoutMs = Math.min(GET_ORDER_TIMEOUT_MS, deadline - Date.now());
    const signal = AbortSignal.timeout(attemptTimeoutMs);
    try {
      log.progress(`  Waiting for confirmation... (${Math.round((Date.now() - startMs) / 1000)}s elapsed)`);
      order = await sdk.getOrder(orderId, { signal });
      readFailures = 0;
      lastReadError = undefined;
    } catch (err) {
      // An aborted read is just another failed read: it says nothing about the swap,
      // so it feeds the same backoff and never becomes a terminal failure.
      readFailures++;
      lastReadError = signal.aborted
        ? `reading order status timed out after ${Math.round(attemptTimeoutMs / 1000)}s`
        : err instanceof Error ? err.message : String(err);
      log.warn(`could not read status of order ${orderId} (attempt ${readFailures}, retrying): ${lastReadError}`);
    }

    if (order) {
      if (hasKey(order.status, "success")) {
        const elapsedMs = Date.now() - startMs;
        const outAmt = order.dstInfo?.amount ?? "?";
        const slipBps = outputAmount && BigInt(outputAmount) !== 0n && order.dstInfo?.amount
          ? Number(10000n - BigInt(order.dstInfo.amount) * 10000n / BigInt(outputAmount))
          : 0;

        log.progress(`✓ Confirmed — ${outAmt} ${dstAsset.symbol} delivered to ${recipient}`);
        return {
          type: "confirmed",
          data: { orderId, status: "confirmed", srcAmount: atomicUnits, srcAsset: srcAsset.symbol, dstAmount: outAmt, dstAsset: dstAsset.symbol, dstChain: dstAsset.chain, quotedDstAmount: outputAmount, actualSlippageBps: slipBps, txId, elapsedMs },
        };
      }

      if (hasKey(order.status, "refunded") || hasKey(order.status, "failed")) {
        // The order itself declares failure — the one and only terminal signal.
        // Carry the on-chain settlement tx hash + route so the --json serializer
        // surfaces them (downstream alerting fetches the receipt to detect
        // out-of-gas). Message unchanged so existing categorization still matches.
        // Only offramp/tokenSwap have an EVM settlement tx; onramp settles on BTC.
        throw new SwapError(`Order ${orderId} failed: ${JSON.stringify(order.status)}`, {
          orderId,
          ...(variant !== "onramp" && {
            txId,
            txParams: { to: orderData.tx?.to, from: senderAddress, chainId: CHAIN_IDS[evmChain], chainName: evmChain },
            srcAsset: { symbol: srcAsset.symbol, chain: srcAsset.chain },
            dstAsset: { symbol: dstAsset.symbol, chain: dstAsset.chain },
          }),
        });
      }

      // inProgress → keep polling, but remember the BTC payout if the order has one.
      // `pendingBtcPayment` is the order's own record of the tx the gateway broadcast:
      // null until it broadcasts, then carrying the exact txid. It is the only sound
      // way to name this order's payout — the recipient address cannot identify it,
      // since one address is reused across orders and parallel offramps to it overlap.
      const inProgress = hasKey(order.status, "inProgress")
        ? order.status.inProgress as { pendingBtcPayment?: { txid?: string } | null } | undefined
        : undefined;
      if (inProgress?.pendingBtcPayment?.txid) {
        pendingBtcPaymentTxId = inProgress.pendingBtcPayment.txid;
      }
    }

    // Back off on consecutive read failures so we don't hammer a struggling API;
    // a normal in-progress poll just waits the fixed interval. Never sleep past
    // the caller's deadline.
    const delay = readFailures > 0
      ? Math.min(POLL_INTERVAL_MS * 2 ** (readFailures - 1), MAX_BACKOFF_MS)
      : POLL_INTERVAL_MS;
    const remaining = deadline - Date.now();
    if (remaining <= 0) break;
    await sleep(Math.min(delay, remaining));
  }

  // ── Timed out without a terminal status → in flight, NOT a failure ──────────

  // If the order reported a BTC payout while we were polling, that txid is this order's
  // payout — reported by the order, not inferred. If it never did (including when every
  // read failed), we have no tx we can honestly attribute to this order, so we report
  // none: an in-flight order id beats a confidently wrong txid.
  if (pendingBtcPaymentTxId) {
    log.progress(`~ BTC payout broadcast, not yet confirmed: ${pendingBtcPaymentTxId}`);
    return {
      type: "mempoolPending",
      data: { orderId, status: "mempool_pending", srcAmount: atomicUnits, srcAsset: srcAsset.symbol, dstAsset: dstAsset.symbol, dstChain: dstAsset.chain, mempoolTxId: pendingBtcPaymentTxId },
    };
  }

  const elapsedMs = Date.now() - startMs;
  log.progress(
    `~ Still in flight after ${Math.round(elapsedMs / 1000)}s — order ${orderId} has not settled yet.\n` +
    `  The source funds are committed (tx ${txId}). This is not a failure.\n` +
    `  Follow it up with: gateway-cli status ${orderId}`,
  );
  return {
    type: "inFlight",
    data: {
      orderId, status: "in_flight", srcAmount: atomicUnits, srcAsset: srcAsset.symbol,
      dstAsset: dstAsset.symbol, dstChain: dstAsset.chain, txId, elapsedMs,
      ...(lastReadError && { lastError: lastReadError }),
    },
  };
}
