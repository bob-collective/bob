// src/util/asset-chain-parser.test.ts
import { describe, test, expect } from "vitest";
import { parseAssetChain } from "../../src/util/asset-chain-parser.js";
import type { RouteInfo } from "../../src/api/types.js";

const mockRoutes: RouteInfo[] = [
  {
    srcChain: "bitcoin", dstChain: "ethereum",
    srcToken: { address: "0x0000000000000000000000000000000000000000", symbol: "BTC", decimals: 8, chain: "bitcoin" },
    dstToken: { address: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", symbol: "USDC", decimals: 6, chain: "ethereum" },
  },
  {
    srcChain: "ethereum", dstChain: "bitcoin",
    srcToken: { address: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", symbol: "USDC", decimals: 6, chain: "ethereum" },
    dstToken: { address: "0x0000000000000000000000000000000000000000", symbol: "BTC", decimals: 8, chain: "bitcoin" },
  },
];

describe("parseAssetChain", () => {
  test("bare BTC resolves to bitcoin zero address", () => {
    const result = parseAssetChain("BTC", mockRoutes);
    expect(result.chain).toBe("bitcoin");
    expect(result.address).toBe("0x0000000000000000000000000000000000000000");
    expect(result.symbol).toBe("BTC");
    expect(result.decimals).toBe(8);
  });

  test("bare lowercase btc resolves to bitcoin", () => {
    const result = parseAssetChain("btc", mockRoutes);
    expect(result.chain).toBe("bitcoin");
  });

  test("USDC:ethereum resolves token address", () => {
    const result = parseAssetChain("USDC:ethereum", mockRoutes);
    expect(result.chain).toBe("ethereum");
    expect(result.address).toBe("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48");
    expect(result.symbol).toBe("USDC");
    expect(result.decimals).toBe(6);
  });

  test("USDC:eth resolves via chain alias", () => {
    const result = parseAssetChain("USDC:eth", mockRoutes);
    expect(result.chain).toBe("ethereum");
  });

  test("0x address passthrough with chain", () => {
    const result = parseAssetChain("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48:ethereum", mockRoutes);
    expect(result.address).toBe("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48");
    expect(result.chain).toBe("ethereum");
  });

  test("bare non-BTC symbol throws with chain suggestion", () => {
    expect(() => parseAssetChain("USDC", mockRoutes)).toThrow("chain required");
    expect(() => parseAssetChain("USDC", mockRoutes)).toThrow("USDC:ethereum");
  });

  test("unknown token on chain throws with available tokens", () => {
    expect(() => parseAssetChain("WBTC:ethereum", mockRoutes)).toThrow("unknown token");
    expect(() => parseAssetChain("WBTC:ethereum", mockRoutes)).toThrow("USDC");
  });
});
