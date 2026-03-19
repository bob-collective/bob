import { describe, it, expect, vi, beforeEach } from "vitest";
import type { EnrichedRoute } from "../../src/util/route-provider.js";

// Mock config and price-oracle before importing the module under test
vi.mock("../../src/config.js", () => ({
  BTC_DECIMALS: 8,
}));

vi.mock("../../src/util/price-oracle.js", () => ({
  fetchPrice: vi.fn(),
}));

import {
  resolveChain,
  parseAssetChain,
  resolveAmount,
  buildTokenIndex,
} from "../../src/util/input-resolver.js";
import { fetchPrice } from "../../src/util/price-oracle.js";

// Use real-looking addresses so isAddress() recognises them as EVM addresses
const USDC_ADDR = "0x833589fcd6edb6e08f4c7c32d4f71b54bda02913"; // USDC on Base
const WBTC_ETH_ADDR = "0x2260fac5e5542a773aa44fbcfedf7c193bc2c599"; // WBTC on Ethereum
const WBTC_BOB_ADDR = "0x03c7054bcb39f7b2e5b2c7acb37583e32d70cfa3"; // WBTC on BOB

const routes: EnrichedRoute[] = [
  {
    srcChain: "bitcoin",
    dstChain: "base",
    srcToken: { address: "BTC", symbol: "BTC", decimals: 8, chain: "bitcoin" },
    dstToken: { address: USDC_ADDR, symbol: "USDC", decimals: 6, chain: "base" },
  },
  {
    srcChain: "ethereum",
    dstChain: "bob",
    srcToken: { address: WBTC_ETH_ADDR, symbol: "WBTC", decimals: 8, chain: "ethereum" },
    dstToken: { address: WBTC_BOB_ADDR, symbol: "WBTC", decimals: 8, chain: "bob" },
  },
];

// ─── resolveChain ─────────────────────────────────────────────────────────────

describe("resolveChain", () => {
  it("maps btc to bitcoin", () => {
    expect(resolveChain("btc")).toBe("bitcoin");
  });

  it("maps eth to ethereum", () => {
    expect(resolveChain("eth")).toBe("ethereum");
  });

  it("maps mainnet to ethereum", () => {
    expect(resolveChain("mainnet")).toBe("ethereum");
  });

  it("maps arb to arbitrum", () => {
    expect(resolveChain("arb")).toBe("arbitrum");
  });

  it("maps arb1 to arbitrum", () => {
    expect(resolveChain("arb1")).toBe("arbitrum");
  });

  it("maps arbitrum-one to arbitrum", () => {
    expect(resolveChain("arbitrum-one")).toBe("arbitrum");
  });

  it("maps bas to base", () => {
    expect(resolveChain("bas")).toBe("base");
  });

  it("maps opt to optimism", () => {
    expect(resolveChain("opt")).toBe("optimism");
  });

  it("maps pol to polygon", () => {
    expect(resolveChain("pol")).toBe("polygon");
  });

  it("maps bnb to bsc", () => {
    expect(resolveChain("bnb")).toBe("bsc");
  });

  it("maps avax to avalanche", () => {
    expect(resolveChain("avax")).toBe("avalanche");
  });

  it("passes through unknown chains unchanged", () => {
    expect(resolveChain("bob")).toBe("bob");
    expect(resolveChain("solana")).toBe("solana");
  });

  it("is case-insensitive", () => {
    expect(resolveChain("BTC")).toBe("bitcoin");
    expect(resolveChain("ETH")).toBe("ethereum");
    expect(resolveChain("ARB")).toBe("arbitrum");
  });

  it("passes through already-canonical chain names", () => {
    expect(resolveChain("bitcoin")).toBe("bitcoin");
    expect(resolveChain("ethereum")).toBe("ethereum");
    expect(resolveChain("base")).toBe("base");
  });
});

// ─── parseAssetChain ─────────────────────────────────────────────────────────

describe("parseAssetChain", () => {
  it("resolves BTC without a chain to bitcoin", () => {
    const result = parseAssetChain("BTC", routes);
    expect(result.chain).toBe("bitcoin");
    expect(result.symbol).toBe("BTC");
    expect(result.decimals).toBe(8);
  });

  it("resolves 'btc' (lowercase) without chain to bitcoin", () => {
    const result = parseAssetChain("btc", routes);
    expect(result.chain).toBe("bitcoin");
    expect(result.symbol).toBe("BTC");
  });

  it("resolves USDC:base format", () => {
    const result = parseAssetChain("USDC:base", routes);
    expect(result.chain).toBe("base");
    expect(result.symbol).toBe("USDC");
    expect(result.address).toBe(USDC_ADDR);
    expect(result.decimals).toBe(6);
  });

  it("resolves chain alias in asset:chain format (WBTC:eth)", () => {
    const result = parseAssetChain("WBTC:eth", routes);
    expect(result.chain).toBe("ethereum");
    expect(result.symbol).toBe("WBTC");
    expect(result.decimals).toBe(8);
  });

  it("resolves WBTC:ethereum", () => {
    const result = parseAssetChain("WBTC:ethereum", routes);
    expect(result.chain).toBe("ethereum");
    expect(result.symbol).toBe("WBTC");
  });

  it("throws for token without chain when multiple chains have that token", () => {
    expect(() => parseAssetChain("WBTC", routes)).toThrow(/chain required/);
  });

  it("throws for unknown token on a chain", () => {
    expect(() => parseAssetChain("UNKNOWN:base", routes)).toThrow(/unknown token/);
  });

  it("throws for token without chain when no routes found", () => {
    expect(() => parseAssetChain("USDC", routes)).toThrow(/chain required/);
  });

  it("resolves by address when an EVM address is provided", () => {
    const result = parseAssetChain(`${USDC_ADDR}:base`, routes);
    expect(result.chain).toBe("base");
    expect(result.symbol).toBe("USDC");
  });

  it("throws for unknown address on a chain", () => {
    expect(() => parseAssetChain("0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef:base", routes)).toThrow(/unknown token address/);
  });

  it("uses a pre-built index when provided", () => {
    const index = buildTokenIndex(routes);
    const result = parseAssetChain("USDC:base", routes, index);
    expect(result.symbol).toBe("USDC");
  });
});

// ─── resolveAmount ────────────────────────────────────────────────────────────

describe("resolveAmount", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("returns atomic units directly when amountAtomic is provided", async () => {
    const result = await resolveAmount({ amountAtomic: "100000000" }, "BTC", 8);
    expect(result.atomicUnits).toBe("100000000");
    expect(result.display).toContain("atomic");
  });

  it("converts human-readable amount to atomic units", async () => {
    const result = await resolveAmount({ amount: "1" }, "BTC", 8);
    expect(result.atomicUnits).toBe("100000000");
    expect(result.display).toContain("1");
    expect(result.display).toContain("BTC");
  });

  it("converts fractional human amount correctly", async () => {
    const result = await resolveAmount({ amount: "0.001" }, "BTC", 8);
    expect(result.atomicUnits).toBe("100000");
  });

  it("converts USDC human amount (6 decimals)", async () => {
    const result = await resolveAmount({ amount: "10" }, "USDC", 6);
    expect(result.atomicUnits).toBe("10000000");
    expect(result.display).toContain("USDC");
  });

  it("resolves USD amount using fetchPrice", async () => {
    vi.mocked(fetchPrice).mockResolvedValueOnce({ priceUsd: 50000, source: "binance" });
    const result = await resolveAmount({ amountUsd: "100" }, "BTC", 8);
    expect(fetchPrice).toHaveBeenCalledWith("BTC");
    // $100 / $50000 per BTC = 0.002 BTC = 200000 satoshis
    expect(result.atomicUnits).toBe("200000");
    expect(result.display).toContain("BTC");
    expect(result.display).toContain("100");
    expect(result.display).toContain("binance");
  });
});
