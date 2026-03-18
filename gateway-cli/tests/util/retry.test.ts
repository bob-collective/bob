import { describe, it, expect, vi } from "vitest";
import { retryWithBackoff } from "../../src/util/retry.js";

describe("retryWithBackoff", () => {
  it("returns value on first success", async () => {
    const fn = vi.fn().mockResolvedValueOnce(42);
    const outcome = await retryWithBackoff(fn, { retries: 3, backoffMs: [0, 0, 0] });
    expect(outcome).toEqual({ value: 42 });
    expect(fn).toHaveBeenCalledTimes(1);
  });

  it("retries on failure and returns value after recovery", async () => {
    const fn = vi
      .fn()
      .mockRejectedValueOnce(new Error("fail 1"))
      .mockRejectedValueOnce(new Error("fail 2"))
      .mockResolvedValueOnce("ok");
    const outcome = await retryWithBackoff(fn, { retries: 2, backoffMs: [0, 0] });
    expect(outcome).toEqual({ value: "ok" });
    // retries=2 means 2 retry attempts + 1 final = 3 total
    expect(fn).toHaveBeenCalledTimes(3);
  });

  it("returns last error after all retries exhausted", async () => {
    const err = new Error("persistent failure");
    const fn = vi.fn().mockRejectedValue(err);
    // retries=2 means 2 retry attempts + 1 final = 3 total calls
    const outcome = await retryWithBackoff(fn, { retries: 2, backoffMs: [0, 0] });
    expect("error" in outcome).toBe(true);
    if ("error" in outcome) {
      expect(outcome.error).toBe(err);
    }
    expect(fn).toHaveBeenCalledTimes(3);
  });

  it("waits between retries but not after the final attempt", async () => {
    const delays: number[] = [];
    const origSetTimeout = globalThis.setTimeout;
    vi.spyOn(globalThis, "setTimeout").mockImplementation((fn: any, ms?: number) => {
      delays.push(ms ?? 0);
      return origSetTimeout(fn, 0) as any;
    });

    const error = new Error("fail");
    const fn = vi.fn().mockRejectedValue(error);
    // retries=2: 2 retry delays, then 1 final attempt (no delay)
    await retryWithBackoff(fn, { retries: 2, backoffMs: [10, 20] });

    expect(delays).toHaveLength(2);
    expect(delays[0]).toBe(10);
    expect(delays[1]).toBe(20);

    vi.restoreAllMocks();
  });

  it("calls onRetry callback for each retry", async () => {
    const onRetry = vi.fn();
    const fn = vi
      .fn()
      .mockRejectedValueOnce(new Error("fail 1"))
      .mockRejectedValueOnce(new Error("fail 2"))
      .mockResolvedValueOnce("ok");
    await retryWithBackoff(fn, { retries: 2, backoffMs: [0, 0] }, onRetry);
    expect(onRetry).toHaveBeenCalledTimes(2);
    expect(onRetry).toHaveBeenNthCalledWith(1, 1);
    expect(onRetry).toHaveBeenNthCalledWith(2, 2);
  });

  it("uses default options when none provided", async () => {
    const fn = vi.fn().mockResolvedValueOnce("default");
    const outcome = await retryWithBackoff(fn);
    expect(outcome).toEqual({ value: "default" });
  });
});
