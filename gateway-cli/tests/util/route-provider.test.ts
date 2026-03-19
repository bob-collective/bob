import { describe, it, expect, vi, beforeEach } from "vitest";

// Mock config before importing module under test
vi.mock("../../src/config.js", () => ({
  BTC_DECIMALS: 8,
  getSdk: vi.fn(),
}));

import { getEnrichedRoutes, getNativeToken } from "../../src/util/route-provider.js";
import { getSdk } from "../../src/config.js";

// ─── getEnrichedRoutes ────────────────────────────────────────────────────────

describe("getEnrichedRoutes", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("fetches routes from SDK and returns enriched results", async () => {
    const mockGetRoutes = vi.fn().mockResolvedValue([
      {
        srcChain: "bitcoin",
        dstChain: "bob",
        srcToken: "BTC",
        dstToken: "BTC",
      },
    ]);
    vi.mocked(getSdk).mockReturnValue({ getRoutes: mockGetRoutes } as any);

    const routes = await getEnrichedRoutes();

    expect(getSdk).toHaveBeenCalled();
    expect(mockGetRoutes).toHaveBeenCalled();
    expect(routes).toBeInstanceOf(Array);
    // BTC route should come through enriched
    expect(routes.length).toBeGreaterThan(0);
    const btcRoute = routes[0];
    expect(btcRoute.srcChain).toBe("bitcoin");
    expect(btcRoute.srcToken.symbol).toBe("BTC");
    expect(btcRoute.srcToken.decimals).toBe(8);
  });

  it("returns empty array when SDK returns no routes", async () => {
    const mockGetRoutes = vi.fn().mockResolvedValue([]);
    vi.mocked(getSdk).mockReturnValue({ getRoutes: mockGetRoutes } as any);

    const routes = await getEnrichedRoutes();

    expect(routes).toEqual([]);
  });

  it("skips routes where token is not in tokenlist", async () => {
    const mockGetRoutes = vi.fn().mockResolvedValue([
      {
        srcChain: "ethereum",
        dstChain: "bob",
        srcToken: "0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef",
        dstToken: "0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef",
      },
    ]);
    vi.mocked(getSdk).mockReturnValue({ getRoutes: mockGetRoutes } as any);

    const routes = await getEnrichedRoutes();

    expect(routes).toEqual([]);
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
