import { describe, it, expect, vi, beforeEach } from "vitest";
import { fetchPrice, PriceOracleError } from "../../src/util/price-oracle.js";

// ─── fetchPrice ───────────────────────────────────────────────────────────────

describe("fetchPrice", () => {
  beforeEach(() => {
    vi.restoreAllMocks();
  });

  it("returns price from Binance when it succeeds", async () => {
    vi.stubGlobal("fetch", vi.fn().mockImplementation((url: string) => {
      if (url.includes("binance.com")) {
        return Promise.resolve({
          ok: true,
          json: async () => ({ price: "50000.00" }),
        });
      }
      // Coinbase — also succeeds but Binance should take priority
      return Promise.resolve({
        ok: true,
        json: async () => ({ data: { amount: "51000.00" } }),
      });
    }));

    const result = await fetchPrice("BTC");
    expect(result.priceUsd).toBe(50000);
    expect(result.source).toBe("binance");
  });

  it("falls back to Coinbase when Binance fails", async () => {
    vi.stubGlobal("fetch", vi.fn().mockImplementation((url: string) => {
      if (url.includes("binance.com")) {
        return Promise.resolve({ ok: false, status: 400 });
      }
      return Promise.resolve({
        ok: true,
        json: async () => ({ data: { amount: "51000.50" } }),
      });
    }));

    const result = await fetchPrice("BTC");
    expect(result.priceUsd).toBe(51000.5);
    expect(result.source).toBe("coinbase");
  });

  it("falls back to Coinbase when Binance throws", async () => {
    vi.stubGlobal("fetch", vi.fn().mockImplementation((url: string) => {
      if (url.includes("binance.com")) {
        return Promise.reject(new Error("network error"));
      }
      return Promise.resolve({
        ok: true,
        json: async () => ({ data: { amount: "2000.00" } }),
      });
    }));

    const result = await fetchPrice("ETH");
    expect(result.priceUsd).toBe(2000);
    expect(result.source).toBe("coinbase");
  });

  it("throws PriceOracleError when both Binance and Coinbase fail", async () => {
    vi.stubGlobal("fetch", vi.fn().mockImplementation((url: string) => {
      if (url.includes("binance.com")) {
        return Promise.resolve({ ok: false, status: 503 });
      }
      return Promise.resolve({ ok: false, status: 503 });
    }));

    await expect(fetchPrice("BTC")).rejects.toThrow(PriceOracleError);
  });

  it("PriceOracleError message includes the symbol", async () => {
    vi.stubGlobal("fetch", vi.fn().mockResolvedValue({ ok: false, status: 500 }));

    await expect(fetchPrice("WBTC")).rejects.toThrow(/WBTC/);
  });

  it("uses correct Binance URL with symbol pair", async () => {
    const mockFetch = vi.fn().mockResolvedValue({
      ok: true,
      json: async () => ({ price: "3000.00" }),
    });
    vi.stubGlobal("fetch", mockFetch);

    await fetchPrice("ETH");

    expect(mockFetch).toHaveBeenCalledWith(
      expect.stringContaining("ETHUSDT"),
      expect.anything(),
    );
  });

  it("uses correct Coinbase URL with symbol pair", async () => {
    const mockFetch = vi.fn().mockImplementation((url: string) => {
      if (url.includes("binance.com")) {
        return Promise.resolve({ ok: false, status: 400 });
      }
      return Promise.resolve({
        ok: true,
        json: async () => ({ data: { amount: "3000.00" } }),
      });
    });
    vi.stubGlobal("fetch", mockFetch);

    await fetchPrice("ETH");

    expect(mockFetch).toHaveBeenCalledWith(
      expect.stringContaining("ETH-USD"),
      expect.anything(),
    );
  });
});
