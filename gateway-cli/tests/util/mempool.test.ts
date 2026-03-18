// src/util/mempool.test.ts
import { describe, test, expect, vi } from "vitest";
import { fetchFeeRate, findPendingMempoolTx } from "../../src/util/mempool.js";

describe("fetchFeeRate", () => {
  test("returns fee rate from mempool.space", async () => {
    vi.stubGlobal("fetch", vi.fn().mockResolvedValueOnce({
      ok: true,
      json: async () => ({ fastestFee: 20, halfHourFee: 15, hourFee: 12, economyFee: 5, minimumFee: 1 }),
    } as Response));
    const rate = await fetchFeeRate();
    expect(rate).toBe(20);
    vi.unstubAllGlobals();
  });

  test("throws when mempool.space unreachable", async () => {
    vi.stubGlobal("fetch", vi.fn().mockRejectedValue(new Error("network error")));
    await expect(fetchFeeRate()).rejects.toThrow("could not fetch fee rate");
    vi.unstubAllGlobals();
  });
});

describe("findPendingMempoolTx", () => {
  test("returns null when no pending tx", async () => {
    vi.stubGlobal("fetch", vi.fn().mockResolvedValueOnce({
      ok: true,
      json: async () => [],
    } as Response));
    const result = await findPendingMempoolTx("bc1qtest");
    expect(result).toBeNull();
    vi.unstubAllGlobals();
  });

  test("returns txid when pending tx found", async () => {
    vi.stubGlobal("fetch", vi.fn().mockResolvedValueOnce({
      ok: true,
      json: async () => [{ txid: "deadbeef", status: { confirmed: false } }],
    } as Response));
    const result = await findPendingMempoolTx("bc1qtest");
    expect(result).toBe("deadbeef");
    vi.unstubAllGlobals();
  });
});
