import type { GatewayOrderInfoV2 } from "@gobob/bob-sdk";
import type { Logger } from "../output.js";
import { sleep } from "./sleep.js";

// ─── Types ───────────────────────────────────────────────────────────────────

/**
 * What the gateway's order state machine told us.
 *
 * A swap is an async process whose state the GATEWAY owns: `inProgress → success |
 * failed | refunded`. These are the only three answers there are — the order settled,
 * the order declared failure, or we ran out of budget before it reached either.
 * "In flight" is not an error: the source funds are committed and the order keeps
 * settling whether or not anyone is watching.
 */
export type OrderOutcome =
  /** The order reached `success`. */
  | { kind: "settled"; order: GatewayOrderInfoV2 }
  /** The order itself declared `failed` or `refunded` — the one terminal failure signal. */
  | { kind: "failed"; order: GatewayOrderInfoV2 }
  /**
   * The budget ran out with no terminal status: the order is still settling, or its
   * status could not be read at all. Facts only — `payoutTxId` is set iff the order
   * told us it had broadcast a BTC payout; `lastError` iff the last read failed.
   * What to *call* this (mempool-pending vs in-flight) is the caller's decision.
   */
  | { kind: "inFlight"; payoutTxId?: string; lastError?: string };

/** Timings for the poll loop. A genuine parameter of the watcher — internal to it. */
export interface PollTimings {
  /** Wait between two ordinary in-progress polls. */
  pollIntervalMs: number;
  /** Ceiling for the exponential backoff applied after consecutive read failures. */
  maxBackoffMs: number;
  /** Budget for a SINGLE `getOrder` read, enforced by aborting the request. */
  getOrderTimeoutMs: number;
}

export const DEFAULT_POLL_TIMINGS: PollTimings = {
  pollIntervalMs: 15_000,
  maxBackoffMs: 60_000,
  getOrderTimeoutMs: 30_000,
};

/**
 * What the watcher needs from the outside world: a way to read the order, and a way to
 * say what it is doing. Nothing else — in particular it has no access to a mempool
 * client, so it *cannot* guess a payout tx from an address even if it wanted to.
 */
export interface OrderWatcherDeps {
  getOrder: (id: string, init?: RequestInit) => Promise<GatewayOrderInfoV2>;
  log: Logger;
}

/** V2 order status is a discriminated object union: {success} | {refunded} | {failed} | {inProgress}. */
const hasKey = <K extends string>(s: unknown, k: K): s is Record<K, unknown> =>
  typeof s === "object" && s !== null && k in s;

// ─── Watcher ─────────────────────────────────────────────────────────────────

/**
 * Await the order's terminal state, or report that we could not.
 *
 * Polling is not a feature — it is the machinery for observing a state machine the
 * gateway owns, which is why it lives here rather than smeared through the command
 * that started the swap. The contract is exactly one thing: return what the state
 * machine said ({@link OrderOutcome}). It never throws for a swap-level reason; the
 * caller decides how to present each outcome.
 *
 * By the time this is called the source funds are committed on-chain, so the ONLY
 * authority on whether the swap failed is the order status itself. Failing to *read*
 * that status (gateway 5xx, Cloudflare 403, connection reset, DNS blip) says nothing
 * about the swap and must never become a failure — the order settles regardless of
 * whether we can see it. Read errors are therefore always retried, with backoff, until
 * `signal` aborts; an order that has still not settled by then is `inFlight`.
 *
 * The budget is expressed ONCE, as `signal`. There is no deadline variable, no clock
 * arithmetic and nothing to clamp: the loop is driven by abort state, each read is
 * bounded by its own budget ANDed with the caller's, and the backoff sleep ends early
 * when the caller's signal fires. `startedAt` exists only to report elapsed seconds in
 * the progress line — it never gates a branch.
 *
 * @param orderId  The order to observe.
 * @param signal   The caller's budget — the ONLY deadline.
 * @param deps     How to read the order, and where to report progress.
 * @param timings  Poll interval, backoff ceiling and per-read budget.
 */
export async function watchOrder(
  orderId: string,
  signal: AbortSignal,
  deps: OrderWatcherDeps,
  timings: PollTimings = DEFAULT_POLL_TIMINGS,
): Promise<OrderOutcome> {
  const { getOrder, log } = deps;
  const startedAt = Date.now();

  let readFailures = 0;
  let lastError: string | undefined;
  /** The BTC payout tx the ORDER itself reports having broadcast (offramp only). */
  let payoutTxId: string | undefined;

  while (!signal.aborted) {
    let order: GatewayOrderInfoV2 | undefined;
    // Bound this single read by its own budget as well as the caller's. Passing the
    // signal to the SDK genuinely cancels the request (it spreads initOverrides into
    // fetch), unlike racing a timer, which would leak a socket per attempt.
    const perRead = AbortSignal.timeout(timings.getOrderTimeoutMs);
    const readSignal = AbortSignal.any([signal, perRead]);
    try {
      log.progress(`  Waiting for confirmation... (${Math.round((Date.now() - startedAt) / 1000)}s elapsed)`);
      order = await getOrder(orderId, { signal: readSignal });
      readFailures = 0;
      lastError = undefined;
    } catch (err) {
      // The caller's budget firing mid-read is not a read failure — it is the watch
      // ending. Leave the last genuine read error (if any) intact and fall through.
      if (signal.aborted && !perRead.aborted) break;
      // Anything else — including this read's own abort — is just a failed read: it
      // says nothing about the swap, so it feeds the backoff and never becomes terminal.
      readFailures++;
      lastError = perRead.aborted
        ? `reading order status timed out after ${timings.getOrderTimeoutMs}ms`
        : err instanceof Error ? err.message : String(err);
      log.warn(`could not read status of order ${orderId} (attempt ${readFailures}, retrying): ${lastError}`);
    }

    if (order) {
      if (hasKey(order.status, "success")) return { kind: "settled", order };

      // The order itself declares failure — the one and only terminal failure signal.
      if (hasKey(order.status, "refunded") || hasKey(order.status, "failed")) {
        return { kind: "failed", order };
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
    // as the caller's signal fires, so it cannot overrun the budget.
    const delay = readFailures > 0
      ? Math.min(timings.pollIntervalMs * 2 ** (readFailures - 1), timings.maxBackoffMs)
      : timings.pollIntervalMs;
    await sleep(delay, signal);
  }

  // Budget exhausted with no terminal status. Everything we know, and nothing we don't:
  // when every read failed we have no payout txid, and an order id that can be followed
  // up beats a confidently wrong txid.
  return {
    kind: "inFlight",
    ...(payoutTxId && { payoutTxId }),
    ...(lastError && { lastError }),
  };
}
