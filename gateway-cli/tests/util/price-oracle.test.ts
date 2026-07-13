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

  it("resolves cbBTC to BTC spot pair for exchange APIs", async () => {
    vi.stubGlobal("fetch", vi.fn().mockImplementation((url: string) => {
      expect(url).not.toMatch(/CBBTC|cbBTC/i);
      if (url.includes("binance.com") && url.includes("BTCUSDT")) {
        return Promise.resolve({
          ok: true,
          json: async () => ({ price: "99000.00" }),
        });
      }
      if (url.includes("coinbase.com") && url.includes("BTC-USD")) {
        return Promise.resolve({
          ok: true,
          json: async () => ({ data: { amount: "99100.00" } }),
        });
      }
      return Promise.resolve({ ok: false, status: 404 });
    }));

    const result = await fetchPrice("cbBTC");
    expect(result.priceUsd).toBe(99000);
    expect(result.source).toBe("binance");
  });

  it("falls back to CoinGecko when the exchanges don't list the token", async () => {
    vi.stubGlobal("fetch", vi.fn().mockImplementation((url: string) => {
      if (url.includes("coingecko.com")) {
        return Promise.resolve({ ok: true, json: async () => ({ usdt0: { usd: 0.9998 } }) });
      }
      // Exchanges don't list USD₮0 under a spot symbol.
      return Promise.resolve({ ok: false, status: 404 });
    }));

    const result = await fetchPrice("USD₮0", "usdt0");
    expect(result.priceUsd).toBe(0.9998);
    expect(result.source).toBe("coingecko");
  });

  it("prefers the exchange price over CoinGecko when both resolve", async () => {
    vi.stubGlobal("fetch", vi.fn().mockImplementation((url: string) => {
      // CoinGecko returns a deliberately wrong price; the exchange price must win.
      if (url.includes("coingecko.com")) return Promise.resolve({ ok: true, json: async () => ({ ethereum: { usd: 999999 } }) });
      if (url.includes("binance.com")) return Promise.resolve({ ok: true, json: async () => ({ price: "2500.00" }) });
      return Promise.resolve({ ok: false, status: 404 });
    }));

    const result = await fetchPrice("ETH", "ethereum");
    expect(result.priceUsd).toBe(2500);
    expect(result.source).toBe("binance");
  });

  it("returns a price if any single source succeeds (only Coinbase up)", async () => {
    vi.stubGlobal("fetch", vi.fn().mockImplementation((url: string) => {
      if (url.includes("coingecko.com")) return Promise.resolve({ ok: false, status: 429 });
      if (url.includes("binance.com")) return Promise.reject(new Error("network error"));
      return Promise.resolve({ ok: true, json: async () => ({ data: { amount: "3100.00" } }) });
    }));

    const result = await fetchPrice("ETH", "ethereum");
    expect(result.priceUsd).toBe(3100);
    expect(result.source).toBe("coinbase");
  });

  it("surfaces the CoinGecko failure as the cause when every source fails", async () => {
    vi.stubGlobal("fetch", vi.fn().mockImplementation((url: string) => {
      if (url.includes("coingecko.com")) return Promise.resolve({ ok: false, status: 429 });
      return Promise.resolve({ ok: false, status: 404 });
    }));

    // With a coingeckoId, the error should point at the CoinGecko 429, not the exchange 404.
    await expect(fetchPrice("USD₮0", "usdt0")).rejects.toThrow(/CoinGecko HTTP 429/);
  });

});
