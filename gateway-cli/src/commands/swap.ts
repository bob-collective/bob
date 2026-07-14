import type { BitcoinSigner, ExecuteQuoteStep, GatewayCreateOrderV3 } from "@gobob/bob-sdk";
import { getInnerQuoteV3 } from "../util/quote.js";
import { type WalletClient, type PublicClient } from "viem";
import { withRetry, SwapError, PointOfNoReturnError } from "../errors.js";
import { getRoutes } from "../util/route-provider.js";
import { resolveSwapContext, type SwapContext, type SwapContextOptions } from "../util/swap-context.js";
import { loadConfig, getSdk, getApi } from "../config.js";
import { resolveSigner } from "../chains/index.js";
import { CHAIN_IDS } from "../chains/evm.js";
import type { Logger, SwapSuccessJson, SwapSubmittedJson, SwapMempoolPendingJson, SwapInFlightJson } from "../output.js";

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
  | { type: "mempoolPending"; data: SwapMempoolPendingJson }
  | { type: "inFlight"; data: SwapInFlightJson };

/** Timings for the post-submission poll loop. Overridable so tests can drive it fast. */
export interface PollTimings {
  /** Wait between two ordinary in-progress polls. */
  pollIntervalMs: number;
  /** Ceiling for the exponential backoff applied after consecutive read failures. */
  maxBackoffMs: number;
  /** Budget for a SINGLE `getOrder` read, enforced by aborting the request. */
  getOrderTimeoutMs: number;
}

const DEFAULT_POLL_TIMINGS: PollTimings = {
  pollIntervalMs: 15_000,
  maxBackoffMs: 60_000,
  getOrderTimeoutMs: 30_000,
};

/** V2 order status is a discriminated object union: {success} | {refunded} | {failed} | {inProgress}. */
const hasKey = <K extends string>(s: unknown, k: K): s is Record<K, unknown> =>
  typeof s === "object" && s !== null && k in s;

/**
 * Sleep that resolves early — never rejects — when `signal` aborts.
 *
 * The poll loop's backoff must not outlive the overall timeout, and the way to
 * express that is to hang the sleep off the same signal that bounds everything
 * else. The alternative, clamping the delay against a computed remaining window,
 * is clock arithmetic — and clock arithmetic is what a hand-maintained deadline turns
 * into a negative delay when two of its reads disagree.
 */
function sleep(ms: number, signal: AbortSignal): Promise<void> {
  return new Promise<void>(resolve => {
    if (signal.aborted) return resolve();
    const done = () => {
      clearTimeout(timer);
      signal.removeEventListener("abort", done);
      resolve();
    };
    const timer = setTimeout(done, ms);
    signal.addEventListener("abort", done, { once: true });
  });
}

// ─── Handler ─────────────────────────────────────────────────────────────────

export async function handleSwap(
  opts: SwapOptions,
  log: Logger,
  timings: PollTimings = DEFAULT_POLL_TIMINGS,
): Promise<SwapResult> {
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
  const { srcAsset, dstAsset, srcFamily, dstFamily, variant, evmChain, senderAddress, recipient, ownerAddress, atomicUnits } = ctx;

  // UX-8: BTC onramp --unsigned requires a sender address to construct the PSBT
  if (opts.unsigned && srcFamily === "bitcoin" && !senderAddress) {
    throw new Error("BTC onramp --unsigned requires --sender or BITCOIN_PRIVATE_KEY to construct the PSBT.");
  }

  if (!opts.recipient) {
    log.progress(`Using recipient: ${recipient} (derived from ${dstFamily === "bitcoin" ? "BITCOIN_PRIVATE_KEY" : "EVM_PRIVATE_KEY"})`);
  }

  const timeoutMs = opts.timeout ? opts.timeout * 1000 : config.timeoutMs;

  const signer = opts.unsigned ? null : await resolveSigner(srcFamily === "bitcoin" ? "bitcoin" : evmChain, ctx.key!);

  // ── Quote + execute (retryable up to the point of no return) ────────────────

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
    const settleSignal = AbortSignal.timeout(120_000);
    let settled = false;
    while (!settleSignal.aborted) {
      const [latest, pending] = await Promise.all([
        publicClient.getTransactionCount({ address: addr, blockTag: "latest" }),
        publicClient.getTransactionCount({ address: addr, blockTag: "pending" }),
      ]);
      if (pending <= latest) { settled = true; break; }
      log.progress(`  Waiting for pending tx to settle (pending nonce: ${pending}, latest: ${latest})...`);
      await sleep(5000, settleSignal);
    }
    if (!settled) {
      throw new Error(`Timed out waiting for pending transactions to settle for ${addr}. There may be stuck transactions — check your wallet before retrying.`);
    }
  }

  // --unsigned has no public SDK path: executeQuote always signs.
  // Use the V3 API directly to fetch the order with its PSBT (BTC) or unsigned tx (EVM).
  // Nothing here touches a wallet, so it is freely retryable.
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

  // ── Point of no return ─────────────────────────────────────────────────────
  // `executeQuote` is NOT idempotent. Internally it runs:
  //   createOrder → (ERC20 reset/approve) → SIGN + BROADCAST the source tx → registerTx
  // Only that prefix is safe to re-run. If it throws *after* the wallet has been asked
  // to sign — a `registerTx` 5xx, a solver's "504 Gateway Timeout", a dropped socket,
  // i.e. exactly the errors `isTransient` matches — then retrying the closure fetches a
  // fresh quote, creates a SECOND order and broadcasts a SECOND transaction. The user's
  // funds leave the wallet twice.
  //
  // The SDK fires an ExecuteQuoteStep immediately BEFORE every wallet interaction
  // (ResetApproval / Approve / SendTransaction / SignBitcoinTransaction), so the first
  // step of ANY kind is the last instant at which re-running is still safe. We latch on
  // all of them rather than only the two that move the funds: an approve is itself a
  // signed, broadcast tx, and re-entering the closure while it is still pending races
  // its own nonce. Over-latching costs at most one lost retry; under-latching costs the
  // funds.
  let executed;
  try {
    executed = await withRetry(async (guard) => {
      const quote = await sdk.getQuote(ctx.quoteParams);
      // For BTC paths walletClient/publicClient are unused inside the SDK; the
      // SDK's type forces them to be present, so we widen via `any` to satisfy it.
      const result = await sdk.executeQuote({
        quote,
        walletClient: evmClients?.walletClient as any,
        publicClient: evmClients?.publicClient as any,
        btcSigner,
        callback: (step: ExecuteQuoteStep) => guard.pointOfNoReturn(step.type),
      });
      return { ...result, outputAmount: getInnerQuoteV3(quote).outputAmount.amount };
    }, { retries, isTransient });
  } catch (err) {
    // Failed past the point of no return: a tx may be on-chain, but we never got the
    // order id back, so there is nothing to poll and nothing to reconcile automatically.
    // Fail loudly and tell the operator not to do the one thing that would double-send.
    if (err instanceof PointOfNoReturnError) throw pointOfNoReturnError(err, ctx);
    throw err;
  }
  const { order, tx, outputAmount } = executed;

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
  // (gateway 5xx, Cloudflare 403, connection reset, DNS blip) says nothing about the
  // swap and must never be reported as a swap failure — the order settles regardless
  // of whether we can see it. Read errors are therefore always retried, with backoff,
  // until the overall timeout aborts; an order that has still not settled by then exits
  // in the non-failure "in flight" state below, carrying orderId + source txId.
  //
  // The timeout is expressed ONCE, as a signal. There is no deadline variable, no
  // clock arithmetic and nothing to clamp: the loop is driven by abort state, each read
  // is bounded by its own budget ANDed with the overall one, and the backoff sleep ends
  // early when the overall signal fires. `startedAt` exists only to report elapsedMs —
  // it never gates a branch.
  const overall = AbortSignal.timeout(timeoutMs);
  const startedAt = Date.now();

  let readFailures = 0;
  let lastReadError: string | undefined;
  /** The BTC payout tx the ORDER itself reports having broadcast (offramp only). */
  let payoutTxId: string | undefined;

  while (!overall.aborted) {
    let order: Awaited<ReturnType<typeof sdk.getOrder>> | undefined;
    // Bound this single read by its own budget as well as the overall one. Passing the
    // signal to the SDK genuinely cancels the request (it spreads initOverrides into
    // fetch), unlike racing a timer, which would leak a socket per attempt.
    const perAttempt = AbortSignal.timeout(timings.getOrderTimeoutMs);
    const readSignal = AbortSignal.any([overall, perAttempt]);
    try {
      log.progress(`  Waiting for confirmation... (${Math.round((Date.now() - startedAt) / 1000)}s elapsed)`);
      order = await sdk.getOrder(orderId, { signal: readSignal });
      readFailures = 0;
      lastReadError = undefined;
    } catch (err) {
      // The overall timeout firing mid-read is not a read failure — it is the loop
      // ending. Leave the last genuine read error (if any) intact and fall through.
      if (overall.aborted && !perAttempt.aborted) break;
      // Anything else — including this read's own abort — is just a failed read: it
      // says nothing about the swap, so it feeds the backoff and never becomes terminal.
      readFailures++;
      lastReadError = perAttempt.aborted
        ? `reading order status timed out after ${timings.getOrderTimeoutMs}ms`
        : err instanceof Error ? err.message : String(err);
      log.warn(`could not read status of order ${orderId} (attempt ${readFailures}, retrying): ${lastReadError}`);
    }

    if (order) {
      if (hasKey(order.status, "success")) {
        const elapsedMs = Date.now() - startedAt;
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
      // null until it broadcasts, then carrying the exact txid. It is the only sound way
      // to name THIS order's payout — the recipient address cannot identify it, since one
      // address is reused across orders and parallel offramps to it overlap in the mempool.
      const inProgress = hasKey(order.status, "inProgress")
        ? order.status.inProgress as { pendingBtcPayment?: { txid?: string } | null } | undefined
        : undefined;
      if (inProgress?.pendingBtcPayment?.txid) {
        payoutTxId = inProgress.pendingBtcPayment.txid;
      }
    }

    // Back off on consecutive read failures so we don't hammer a struggling API; an
    // ordinary in-progress poll just waits the fixed interval. The sleep ends as soon
    // as the overall signal fires, so it cannot overrun the timeout.
    const delay = readFailures > 0
      ? Math.min(timings.pollIntervalMs * 2 ** (readFailures - 1), timings.maxBackoffMs)
      : timings.pollIntervalMs;
    await sleep(delay, overall);
  }

  // ── Timed out without a terminal status → in flight, NOT a failure ──────────

  const elapsedMs = Date.now() - startedAt;

  // If the order reported a BTC payout while we were polling, that txid is this order's
  // payout — reported by the order, not inferred from the destination address.
  if (payoutTxId) {
    log.progress(`~ BTC payout broadcast, not yet confirmed: ${payoutTxId}`);
    return {
      type: "mempoolPending",
      data: { orderId, status: "mempool_pending", srcAmount: atomicUnits, srcAsset: srcAsset.symbol, dstAsset: dstAsset.symbol, dstChain: dstAsset.chain, mempoolTxId: payoutTxId },
    };
  }

  // Otherwise we have no tx we can honestly attribute to this order — including when
  // every read failed. An order id that can be followed up beats a confidently wrong txid.
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

// ─── Point-of-no-return reporting ────────────────────────────────────────────

/**
 * Turn a {@link PointOfNoReturnError} into the loud, terminal {@link SwapError} an
 * operator sees. It must not assert that funds *were* sent — the latch arms when the
 * wallet is asked to sign, which is before viem prepares the transaction, so a
 * transient RPC failure during gas estimation or nonce fetch also lands here and may
 * mean nothing was broadcast at all. That over-warning is deliberate and stated: it
 * costs one manual check, whereas under-warning costs the funds. (SDK follow-up to
 * narrow the window: bob-collective/bob#1122.)
 *
 * The reconciliation step it points at uses `ownerAddress` — the EVM address orders are
 * indexed by. Naming the sender would print a `bc1q…` address on an onramp, and
 * `gateway-cli orders` rejects BTC addresses outright, leaving the operator with no way
 * to check — at exactly the moment they are tempted to re-run and double-send.
 */
function pointOfNoReturnError(err: PointOfNoReturnError, ctx: SwapContext): SwapError {
  return new SwapError(
    `Swap aborted after the wallet was asked to sign (step: ${err.reason}). ` +
    `A transaction may already have been broadcast — do NOT re-run this swap or you may send twice. ` +
    `(The signature was requested but we never learned whether it reached the chain; a failure during ` +
    `transaction preparation may mean nothing was sent.) ` +
    `Confirm with \`gateway-cli orders ${ctx.ownerAddress}\` before re-running. ` +
    `Underlying error: ${err.originalError.message}`,
    {
      srcAsset: { symbol: ctx.srcAsset.symbol, chain: ctx.srcAsset.chain },
      dstAsset: { symbol: ctx.dstAsset.symbol, chain: ctx.dstAsset.chain },
      ...(ctx.variant !== "onramp" && {
        txParams: { from: ctx.senderAddress, chainId: CHAIN_IDS[ctx.evmChain], chainName: ctx.evmChain },
      }),
    },
  );
}
