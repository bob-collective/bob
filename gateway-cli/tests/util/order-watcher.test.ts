import { describe, it, expect, vi } from "vitest";
import { watchOrder, type PollTimings } from "../../src/util/order-watcher.js";
import type { Logger } from "../../src/output.js";

// ─── The machinery under test ────────────────────────────────────────────────
//
// A swap is an async process whose state the GATEWAY owns: inProgress → success |
// failed | refunded. `watchOrder` is the machinery for observing that state machine,
// and these tests exercise it unmocked: real AbortSignal.timeout, real AbortSignal.any,
// real request cancellation, real backoff. Nothing here is faked but the gateway.
//
// Once the source funds are committed, the order status is the ONLY authority on
// whether the swap failed. Every test below is a case where a *read* went wrong and the
// watcher must NOT turn that into a failure — each one corresponds to a real incident
// in which the CLI exited 1 and gateway-bot opened a false critical issue for a swap
// that had, in fact, worked.

const silentLogger: Logger = { progress: () => {}, warn: () => {} };

/**
 * Timings compressed so the REAL machinery runs and still finishes in milliseconds.
 * These are a legitimate parameter of the component under test — not a seam punched
 * through some other component's signature. Fake timers cannot drive
 * AbortSignal.timeout (it is not backed by the global timer), and faking the loop's
 * clock is exactly the thing these tests exist to prove the loop no longer depends on.
 */
const FAST: PollTimings = { pollIntervalMs: 10, maxBackoffMs: 40, getOrderTimeoutMs: 50 };

const ORDER_ID = "order-456";

const successOrder = { id: ORDER_ID, status: { success: {} }, dstInfo: { amount: "4812300000" } } as any;
const inProgressOrder = { id: ORDER_ID, status: { inProgress: {} } } as any;

/** Watch with the real machinery, a real overall budget, and a stub gateway. */
const watch = (getOrder: any, timeoutMs = 300, timings: PollTimings = FAST) =>
  watchOrder(ORDER_ID, AbortSignal.timeout(timeoutMs), { getOrder, log: silentLogger }, timings);

describe("watchOrder", () => {
  // ─── Terminal states ───────────────────────────────────────────────────────

  it("returns settled when the order reaches success", async () => {
    const getOrder = vi.fn().mockResolvedValue(successOrder);

    const outcome = await watch(getOrder, 2_000);

    expect(outcome.kind).toBe("settled");
    if (outcome.kind !== "settled") return;
    expect(outcome.order.dstInfo?.amount).toBe("4812300000");
    // Every read carries an abort signal, so no single read can outlive the budget.
    expect(getOrder).toHaveBeenCalledWith(ORDER_ID, { signal: expect.any(AbortSignal) });
  });

  it("returns failed when the order declares `failed`", async () => {
    const outcome = await watch(vi.fn().mockResolvedValue({ id: ORDER_ID, status: { failed: {} } }), 2_000);
    expect(outcome.kind).toBe("failed");
  });

  it("returns failed when the order declares `refunded`", async () => {
    const outcome = await watch(vi.fn().mockResolvedValue({ id: ORDER_ID, status: { refunded: {} } }), 2_000);
    expect(outcome.kind).toBe("failed");
  });

  // ─── A failed READ is never a failed SWAP ──────────────────────────────────

  it("a transient gateway 5xx while polling does not fail an already-settling swap", async () => {
    // Incident (staging order cbf7296e-340a-438a-981c-75980fafc40c): get-order returned
    // "bungee api error: status=504", yet the swap SUCCEEDED — 74,130 sats confirmed on
    // Bitcoin in tx 48abaaf1…, block 957986. The CLI exited 1 and gateway-bot opened a
    // false critical issue for a swap that worked.
    const getOrder = vi.fn()
      .mockRejectedValueOnce(new Error("bungee api error: status=504 Gateway Timeout, message=error code: 504\n"))
      .mockResolvedValue(successOrder);

    const outcome = await watch(getOrder, 2_000);

    expect(outcome.kind).toBe("settled");
    expect(getOrder).toHaveBeenCalledTimes(2);
  });

  it("a generic fetch failure while polling reports inFlight, not failure", async () => {
    // Incident (staging order a9a49f67-a2de-429f-bb44-2f32034a6f22): the SDK runtime threw
    // its generic FetchError (underlying cause: a local Cloudflare 403) on every get-order.
    // 44.6 USDC had already left the wallet; the order was `inProgress` and later settled
    // fine. An unreadable status is not a failed swap — it is a swap we cannot see.
    const getOrder = vi.fn().mockRejectedValue(
      new Error("The request failed and the interceptors did not return an alternative response"),
    );

    const outcome = await watch(getOrder);

    expect(outcome.kind).toBe("inFlight");
    if (outcome.kind !== "inFlight") return;
    expect(outcome.lastError).toContain("interceptors did not return");
    // Retried rather than bailing out on the first read error.
    expect(getOrder.mock.calls.length).toBeGreaterThan(1);
  });

  it("retrying read errors does not mask a genuine order failure", async () => {
    // The order status stays the ONE authority on failure: surviving a transient read
    // error must not make us swallow the `failed` status that follows it.
    const getOrder = vi.fn()
      .mockRejectedValueOnce(new Error("bungee api error: status=504 Gateway Timeout"))
      .mockResolvedValue({ id: ORDER_ID, status: { failed: {} } });

    const outcome = await watch(getOrder, 2_000);

    expect(outcome.kind).toBe("failed");
  });

  it("backs off between consecutive read failures instead of hammering the API", async () => {
    // A gateway that is failing reads is usually a gateway under strain — retrying at the
    // plain poll interval hammers it. Consecutive failures back off exponentially from
    // pollIntervalMs (10 → 20 → 40), capped at maxBackoffMs (40).
    //
    // The gaps must GROW: with the backoff removed every gap would sit at the 10ms poll
    // interval, so the assertions below are written to fail in that case rather than to
    // be satisfied by timer jitter.
    const at: number[] = [];
    const getOrder = vi.fn().mockImplementation(async () => {
      at.push(Date.now());
      throw new Error("bungee api error: status=504");
    });

    const outcome = await watch(getOrder, 300);

    expect(outcome.kind).toBe("inFlight");
    const gaps = at.slice(1).map((t, i) => t - at[i]!);
    expect(gaps.length).toBeGreaterThanOrEqual(3);
    // 1st retry waits ~10ms (the plain interval); by the 3rd it waits ~40ms — a delay a
    // flat 10ms interval could never produce.
    expect(gaps[0]!).toBeLessThan(20);
    expect(gaps[2]!).toBeGreaterThanOrEqual(30);
  });

  // ─── The budget is a signal; nothing does clock arithmetic ─────────────────

  it("an order still inProgress at the end of the budget is inFlight, not failed", async () => {
    // Previously the poll loop exhausted its retries and threw its internal "pending"
    // sentinel, which surfaced as {"error":{"message":"pending"}} with exit 1 — a terminal
    // failure for a swap that was simply still settling.
    const getOrder = vi.fn().mockResolvedValue(inProgressOrder);

    const outcome = await watch(getOrder);

    expect(outcome.kind).toBe("inFlight");
    if (outcome.kind !== "inFlight") return;
    // The status was readable throughout — nothing to report as a read error.
    expect(outcome.lastError).toBeUndefined();
    expect(outcome.payoutTxId).toBeUndefined();
    expect(getOrder.mock.calls.length).toBeGreaterThan(1);
  });

  it("a stalled getOrder read is aborted, so the watch still ends at the budget", async () => {
    // A stalled TCP connection — the server accepts the request and never answers. Like
    // undici, the stub settles ONLY when its AbortSignal fires. Without a per-read signal
    // the await never returns and the budget is unenforceable; racing a timer instead
    // would leak the socket. The read must be genuinely cancelled.
    const getOrder = vi.fn().mockImplementation((_id: string, init?: { signal?: AbortSignal }) =>
      new Promise((_resolve, reject) => {
        init?.signal?.addEventListener("abort", () => reject(init.signal!.reason));
      }),
    );

    const startedAt = Date.now();
    const outcome = await watch(getOrder, 300);
    const elapsedMs = Date.now() - startedAt;

    expect(outcome.kind).toBe("inFlight");
    expect(elapsedMs).toBeLessThan(3_000);
    if (outcome.kind !== "inFlight") return;
    expect(outcome.lastError).toContain("timed out");
    // Every read was cancelled by its own budget, so the loop kept trying.
    expect(getOrder.mock.calls.length).toBeGreaterThan(1);
  }, 10_000);

  it("the per-read budget cancels the request itself, it does not merely stop waiting", async () => {
    // The signal handed to getOrder must actually abort: the SDK spreads initOverrides
    // into fetch, so an aborted signal cancels the HTTP request and frees the socket.
    let observed: AbortSignal | undefined;
    const getOrder = vi.fn().mockImplementation((_id: string, init?: { signal?: AbortSignal }) => {
      observed = init?.signal;
      return new Promise((_r, reject) => init?.signal?.addEventListener("abort", () => reject(init.signal!.reason)));
    });

    await watchOrder(ORDER_ID, AbortSignal.timeout(300), { getOrder, log: silentLogger },
      { ...FAST, getOrderTimeoutMs: 20 });

    expect(observed?.aborted).toBe(true);
  });

  it("survives a clock that jumps past any plausible deadline (no RangeError)", async () => {
    // An earlier fix kept a `deadline` and derived the per-read budget from
    // `deadline - Date.now()`, reading the clock separately from the loop guard. A stall
    // between the two reads makes the window negative, and AbortSignal.timeout(negative)
    // throws RangeError *synchronously* — outside the read's try — so a swap whose funds
    // are already committed exits 1 as a hard failure.
    //
    // There is no clock arithmetic left to break: the loop is driven by abort state
    // alone, so it must survive a clock that lies. Date.now() may only be *reported*.
    const nowSpy = vi.spyOn(Date, "now");
    try {
      let call = 0;
      nowSpy.mockImplementation(() => {
        call++;
        if (call === 1) return 1_000;
        if (call === 2) return 1_100;
        return 999_999_999; // stalled: every later read is "past" any deadline
      });

      const outcome = await watch(vi.fn().mockResolvedValue(inProgressOrder));

      expect(outcome.kind).toBe("inFlight");
    } finally {
      nowSpy.mockRestore();
    }
  });

  // ─── The BTC payout comes from the ORDER, never from the address ───────────
  //
  // The only sound source for this order's payout tx is the order's own
  // `pendingBtcPayment`. Scanning the recipient address for an unconfirmed tx is not
  // correlation: gateway-bot reuses ONE BTC address for every swap and runs offramp legs
  // in parallel, so several of its own payouts to that address are unconfirmed at once.
  // The watcher is given no mempool client at all, so it *cannot* guess.

  it("captures the payout txid the order says it broadcast", async () => {
    const OUR_PAYOUT_TXID = "48abaaf1000000000000000000000000000000000000000000000000000000ff";
    const getOrder = vi.fn().mockResolvedValue({
      id: ORDER_ID,
      status: { inProgress: { pendingBtcPayment: { txid: OUR_PAYOUT_TXID, amount: "74130" } } },
    });

    const outcome = await watch(getOrder);

    expect(outcome.kind).toBe("inFlight");
    if (outcome.kind !== "inFlight") return;
    expect(outcome.payoutTxId).toBe(OUR_PAYOUT_TXID);
  });

  it("reports no payout txid when no status was ever read", async () => {
    // Every read failed, so we know nothing about a payout — including whether one exists.
    // An order id that can be followed up beats a confident, wrong txid.
    const getOrder = vi.fn().mockRejectedValue(new Error("Cloudflare 403"));

    const outcome = await watch(getOrder);

    expect(outcome.kind).toBe("inFlight");
    if (outcome.kind !== "inFlight") return;
    expect(outcome.payoutTxId).toBeUndefined();
    expect(outcome.lastError).toContain("403");
  });

  it("keeps the payout txid the order reported even if later reads fail", async () => {
    // The gateway broadcast the payout and then went dark. The txid is still this
    // order's payout — a read failure does not un-broadcast a transaction.
    const OUR_PAYOUT_TXID = "48abaaf1000000000000000000000000000000000000000000000000000000ff";
    const getOrder = vi.fn()
      .mockResolvedValueOnce({
        id: ORDER_ID,
        status: { inProgress: { pendingBtcPayment: { txid: OUR_PAYOUT_TXID } } },
      })
      .mockRejectedValue(new Error("bungee api error: status=504"));

    const outcome = await watch(getOrder);

    expect(outcome.kind).toBe("inFlight");
    if (outcome.kind !== "inFlight") return;
    expect(outcome.payoutTxId).toBe(OUR_PAYOUT_TXID);
    expect(outcome.lastError).toContain("504");
  });

  // ─── The caller's budget is the only deadline ──────────────────────────────

  it("returns immediately when the caller's signal is already aborted", async () => {
    const getOrder = vi.fn().mockResolvedValue(successOrder);

    const outcome = await watchOrder(ORDER_ID, AbortSignal.abort(), { getOrder, log: silentLogger }, FAST);

    expect(outcome.kind).toBe("inFlight");
    expect(getOrder).not.toHaveBeenCalled();
  });
});
