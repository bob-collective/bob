import { describe, it, expect, vi } from "vitest";

vi.mock("../../src/config.js", () => ({ BTC_DECIMALS: 8 }));

vi.mock("../../src/chains/evm.js", () => ({
  CHAIN_IDS: { ethereum: 1, base: 8453, arbitrum: 42161 },
  getNativeToken: (chain: string) => {
    const N: Record<string, { symbol: string; decimals: number }> = {
      ethereum: { symbol: "ETH", decimals: 18 },
      base: { symbol: "ETH", decimals: 18 },
    };
    if (!N[chain]) throw new Error(`unknown chain "${chain}"`);
    return N[chain];
  },
  getTokenMetadata: () => { throw new Error("not used in this test"); },
}));

import { resolveSendAsset } from "../../src/util/asset-resolver.js";

const USDC_BASE = "0x833589fcd6edb6e08f4c7c32d4f71b54bda02913";
const UNKNOWN = "0x1111111111111111111111111111111111111111";

const deps = {
  lookupToken: (chain: string, key: string) => {
    if (chain === "base" && key === "USDC") return { address: USDC_BASE, symbol: "USDC", decimals: 6 };
    if (chain === "base" && key === USDC_BASE.toLowerCase()) return { address: USDC_BASE, symbol: "USDC", decimals: 6 };
    return undefined;
  },
  readOnChainToken: async (_chain: string, _addr: string) => ({ symbol: "FOO", decimals: 9 }),
};

describe("resolveSendAsset", () => {
  it("resolves bare BTC to bitcoin native", async () => {
    const a = await resolveSendAsset("BTC", deps);
    expect(a).toEqual({ chain: "bitcoin", address: "0x0000000000000000000000000000000000000000", symbol: "BTC", decimals: 8 });
  });

  it("resolves a native EVM token (ETH:base)", async () => {
    const a = await resolveSendAsset("ETH:base", deps);
    expect(a).toEqual({ chain: "base", address: "0x0000000000000000000000000000000000000000", symbol: "ETH", decimals: 18 });
  });

  it("resolves a tokenlist symbol with chain alias", async () => {
    const a = await resolveSendAsset("usdc:base", deps);
    expect(a).toEqual({ chain: "base", address: USDC_BASE, symbol: "USDC", decimals: 6 });
  });

  it("resolves a raw address present in the tokenlist", async () => {
    const a = await resolveSendAsset(`${USDC_BASE}:base`, deps);
    expect(a).toEqual({ chain: "base", address: USDC_BASE, symbol: "USDC", decimals: 6 });
  });

  it("falls back to on-chain decimals for an unknown address", async () => {
    const a = await resolveSendAsset(`${UNKNOWN}:base`, deps);
    expect(a).toEqual({ chain: "base", address: UNKNOWN, symbol: "FOO", decimals: 9 });
  });

  it("throws a helpful error for an unknown symbol", async () => {
    await expect(resolveSendAsset("DOGE:base", deps)).rejects.toThrow(/unknown token "DOGE"/i);
  });

  it("throws when a non-BTC asset has no chain", async () => {
    await expect(resolveSendAsset("USDC", deps)).rejects.toThrow(/chain required/i);
  });
});
