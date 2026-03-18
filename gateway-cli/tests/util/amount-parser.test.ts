// src/util/amount-parser.test.ts
import { describe, test, expect, vi } from "vitest";
import { parseAmount } from "../../src/util/amount-parser.js";

describe("parseAmount", () => {
  test("parses BTC suffix to satoshis", async () => {
    const result = await parseAmount("0.1BTC", "BTC", 8);
    expect(result.atomicUnits).toBe("10000000");
    expect(result.display).toBe("0.1 BTC");
  });

  test("parses sat suffix directly", async () => {
    const result = await parseAmount("100sat", "BTC", 8);
    expect(result.atomicUnits).toBe("100");
    expect(result.display).toBe("100 sat");
  });

  test("parses USDC suffix with 6 decimals", async () => {
    const result = await parseAmount("100USDC", "USDC", 6);
    expect(result.atomicUnits).toBe("100000000");
    expect(result.display).toBe("100 USDC");
  });

  test("parses ETH suffix to wei", async () => {
    const result = await parseAmount("1.5ETH", "ETH", 18);
    expect(result.atomicUnits).toBe("1500000000000000000");
    expect(result.display).toBe("1.5 ETH");
  });

  test("rejects raw unitless integers", async () => {
    await expect(parseAmount("100000000", "BTC", 8)).rejects.toThrow("suffix required");
  });

  test("rejects suffix that doesn't match srcSymbol", async () => {
    await expect(parseAmount("100XYZ", "BTC", 8)).rejects.toThrow("unknown amount suffix");
  });

  test("resolves $ amount via oracle (BTC)", async () => {
    vi.spyOn(await import("../../src/util/price-oracle.js"), "fetchPrice").mockResolvedValue({
      priceUsd: 100000,
      source: "binance",
    });
    const result = await parseAmount("$100", "BTC", 8);
    expect(result.atomicUnits).toBe("100000"); // $100 at $100k/BTC = 0.001 BTC = 100000 sat
    expect(result.priceSource).toBe("binance");
    vi.restoreAllMocks();
  });

  test("resolves $ stablecoin via oracle", async () => {
    vi.spyOn(await import("../../src/util/price-oracle.js"), "fetchPrice").mockResolvedValue({
      priceUsd: 1,
      source: "binance",
    });
    const result = await parseAmount("$50", "USDC", 6);
    expect(result.atomicUnits).toBe("50000000"); // 50 USDC = 50_000_000 units
    expect(result.priceSource).toBe("binance");
    vi.restoreAllMocks();
  });

  test("passes through raw: prefix as atomic units", async () => {
    const result = await parseAmount("raw:100000000", "BTC", 8);
    expect(result.atomicUnits).toBe("100000000");
    expect(result.display).toBe("100000000 (raw)");
  });

  test("rejects non-integer raw amounts", async () => {
    await expect(parseAmount("raw:abc", "BTC", 8)).rejects.toThrow("invalid raw amount");
  });

  test("rejects empty raw amount", async () => {
    await expect(parseAmount("raw:", "BTC", 8)).rejects.toThrow("invalid raw amount");
  });

  test("rejects negative dollar amounts", async () => {
    await expect(parseAmount("$-100", "BTC", 8)).rejects.toThrow();
  });

  test("handles very small BTC amounts without precision loss", async () => {
    const result = await parseAmount("0.00000001BTC", "BTC", 8);
    expect(result.atomicUnits).toBe("1");
  });

  test("handles zero amount", async () => {
    const result = await parseAmount("0BTC", "BTC", 8);
    expect(result.atomicUnits).toBe("0");
  });
});
