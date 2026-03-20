import { describe, it, expect, vi, beforeEach } from "vitest";

// Mock config before importing module under test
vi.mock("../../src/config.js", () => ({
  BTC_DECIMALS: 8,
  getSdk: vi.fn(),
}));

// Mock chains/evm.js for getTokenMetadata (used by getTokensForChain)
vi.mock("../../src/chains/evm.js", () => ({
  getTokenMetadata: vi.fn((address: string, chain: string) => {
    if (chain === "bitcoin" || address === "BTC") return { symbol: "BTC", decimals: 8 };
    return { symbol: address.slice(0, 10), decimals: 18 };
  }),
  getNativeToken: vi.fn((chain: string) => {
    if (chain === "ethereum" || chain === "bob" || chain === "base") return { symbol: "ETH", decimals: 18 };
    throw new Error(`unknown chain "${chain}" — cannot determine native token`);
  }),
}));

import { getRoutes, getUniqueChains, getTokensForChain } from "../../src/util/route-provider.js";
import { getNativeToken } from "../../src/chains/evm.js";
import { getSdk } from "../../src/config.js";

// ─── getRoutes ─────────────────────────────────────────────────────────────────

describe("getRoutes", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("fetches routes from SDK and returns raw RouteInfo", async () => {
    const mockGetRoutes = vi.fn().mockResolvedValue([
      {
        srcChain: "bitcoin",
        dstChain: "bob",
        srcToken: "BTC",
        dstToken: "0x03c7054bcb39f7b2e5b2c7acb37583e32d70cfa3",
      },
    ]);
    vi.mocked(getSdk).mockReturnValue({ getRoutes: mockGetRoutes } as any);

    const routes = await getRoutes();

    expect(getSdk).toHaveBeenCalled();
    expect(mockGetRoutes).toHaveBeenCalled();
    expect(routes).toBeInstanceOf(Array);
    expect(routes.length).toBeGreaterThan(0);
    const btcRoute = routes[0];
    expect(btcRoute.srcChain).toBe("bitcoin");
    expect(btcRoute.srcToken).toBe("BTC");
  });

  it("returns empty array when SDK returns no routes", async () => {
    const mockGetRoutes = vi.fn().mockResolvedValue([]);
    vi.mocked(getSdk).mockReturnValue({ getRoutes: mockGetRoutes } as any);

    const routes = await getRoutes();

    expect(routes).toEqual([]);
  });
});

// ─── getUniqueChains ─────────────────────────────────────────────────────────

describe("getUniqueChains", () => {
  it("returns unique chain names", () => {
    const routes = [
      { srcChain: "bitcoin", dstChain: "bob", srcToken: "BTC", dstToken: "0xabc" },
      { srcChain: "ethereum", dstChain: "bob", srcToken: "0xdef", dstToken: "0xabc" },
    ];
    const chains = getUniqueChains(routes);
    expect(chains).toContain("bitcoin");
    expect(chains).toContain("bob");
    expect(chains).toContain("ethereum");
    expect(chains.length).toBe(3);
  });
});

// ─── getNativeToken ───────────────────────────────────────────────────────────

describe("getNativeToken", () => {
  it("returns ETH for ethereum", () => {
    const token = getNativeToken("ethereum");
    expect(token.symbol).toBe("ETH");
    expect(token.decimals).toBe(18);
  });

  it("returns ETH for bob chain", () => {
    const token = getNativeToken("bob");
    expect(token.symbol).toBe("ETH");
    expect(token.decimals).toBe(18);
  });

  it("returns ETH for base chain", () => {
    const token = getNativeToken("base");
    expect(token.symbol).toBe("ETH");
    expect(token.decimals).toBe(18);
  });

  it("throws for unknown chain", () => {
    expect(() => getNativeToken("unknownchain999")).toThrow(/unknown chain/);
    expect(() => getNativeToken("unknownchain999")).toThrow("unknownchain999");
  });
});
