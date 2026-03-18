// src/util/price-oracle.test.ts
import { describe, test, expect, vi } from "vitest";
import { fetchPrice } from "../../src/util/price-oracle.js";

describe("fetchPrice", () => {
  test("uses Binance first and returns price + source", async () => {
    vi.stubGlobal("fetch", vi.fn().mockResolvedValueOnce({
      ok: true,
      json: async () => ({ price: "92500.00" }),
    } as Response));
    const result = await fetchPrice("BTC");
    expect(result.priceUsd).toBe(92500);
    expect(result.source).toBe("binance");
    vi.unstubAllGlobals();
  });

  test("fetches stablecoin prices via oracle", async () => {
    vi.stubGlobal("fetch", vi.fn().mockResolvedValueOnce({
      ok: true,
      json: async () => ({ price: "1.00" }),
    } as Response));
    const result = await fetchPrice("USDC");
    expect(result.priceUsd).toBe(1);
    expect(result.source).toBe("binance");
    vi.unstubAllGlobals();
  });

  test("falls back to Coinbase when Binance fails", async () => {
    vi.stubGlobal("fetch", vi.fn()
      .mockRejectedValueOnce(new Error("network error"))
      .mockResolvedValueOnce({
        ok: true,
        json: async () => ({ data: { amount: "92500.00" } }),
      } as Response));
    const result = await fetchPrice("BTC");
    expect(result.priceUsd).toBe(92500);
    expect(result.source).toBe("coinbase");
    vi.unstubAllGlobals();
  });

  test("throws PriceOracleError when both oracles fail", async () => {
    vi.stubGlobal("fetch", vi.fn().mockRejectedValue(new Error("network error")));
    await expect(fetchPrice("BTC")).rejects.toThrow("PriceOracleError");
    vi.unstubAllGlobals();
  });
});
