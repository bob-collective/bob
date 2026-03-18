// src/util/nonce.test.ts
import { describe, test, expect, vi } from "vitest";
import { waitForNonceClear } from "../../src/signer/evm.js";

describe("waitForNonceClear", () => {
  test("returns true immediately when no pending txs", async () => {
    const getCount = vi.fn().mockResolvedValue(5n);
    await expect(waitForNonceClear(getCount, getCount, { maxWaitMs: 1000, intervalMs: 100 }))
      .resolves.toBe(true);
  });

  test("returns true after pending clears", async () => {
    const latest = vi.fn().mockResolvedValue(5n);
    const pending = vi.fn()
      .mockResolvedValueOnce(6n)
      .mockResolvedValue(5n);
    await expect(waitForNonceClear(latest, pending, { maxWaitMs: 5000, intervalMs: 50 }))
      .resolves.toBe(true);
  });

  test("returns false after maxWait if still pending", async () => {
    const latest = vi.fn().mockResolvedValue(5n);
    const pending = vi.fn().mockResolvedValue(6n);
    await expect(waitForNonceClear(latest, pending, { maxWaitMs: 200, intervalMs: 50 }))
      .resolves.toBe(false);
  });
});
