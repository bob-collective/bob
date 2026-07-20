import { describe, it, expect } from "vitest";
import { swapSchema } from "../../src/schemas.js";

const base = { src: "BTC", dst: "USDC:base", amount: "5000000" };

describe("swapSchema --timeout", () => {
  it("accepts an ordinary polling timeout", () => {
    expect(swapSchema.parse({ ...base, timeout: "5400" })).toMatchObject({ timeout: 5400 });
  });

  it("rejects a timeout large enough to make AbortSignal.timeout throw", () => {
    // The poll timeout becomes `AbortSignal.timeout(timeoutMs)`, created AFTER the source tx
    // is broadcast. Above 2^32-1 ms it throws RangeError *synchronously*, which would report
    // a swap whose funds are already committed as a hard failure — the exact failure mode the
    // signal-driven poll loop exists to make impossible. Reject it at the boundary instead,
    // where nothing has been sent yet.
    expect(() => swapSchema.parse({ ...base, timeout: "99999999" })).toThrow(/86400/);
    // Sanity: the rejected value really is the one that would blow up.
    expect(() => AbortSignal.timeout(99_999_999 * 1000)).toThrow(RangeError);
    expect(() => AbortSignal.timeout(86_400 * 1000)).not.toThrow();
  });
});
