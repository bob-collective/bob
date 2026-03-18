import { describe, it, expect, vi, beforeEach } from "vitest";
import { handleRoutes } from "../../src/commands/routes.js";

vi.mock("../../src/config/index.js", () => ({
  loadConfig: vi.fn().mockReturnValue({
    apiUrl: "https://example.com",
    cache: { ttl: "24h" },
  }),
}));

vi.mock("../../src/adapter/sdk-client.js", () => ({
  createSdkClient: vi.fn().mockReturnValue({
    getRoutes: vi.fn(),
  }),
}));

const mockGetOrFetchRoutes = vi.fn();
vi.mock("../../src/util/route-cache.js", () => ({
  getOrFetchRoutes: (...args: unknown[]) => mockGetOrFetchRoutes(...args),
}));

const mockEnrichRoutes = vi.fn();
vi.mock("../../src/adapter/route-enricher.js", () => ({
  enrichRoutes: (...args: unknown[]) => mockEnrichRoutes(...args),
}));

const enrichedRoute = {
  srcChain: "bitcoin", dstChain: "base",
  srcToken: { address: "BTC", symbol: "BTC", decimals: 8, chain: "bitcoin" },
  dstToken: { address: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913", symbol: "USDC", decimals: 6, chain: "base" },
};

const enrichedRouteBob = {
  srcChain: "bob", dstChain: "bitcoin",
  srcToken: { address: "0x1", symbol: "USDC", decimals: 6, chain: "bob" },
  dstToken: { address: "BTC", symbol: "BTC", decimals: 8, chain: "bitcoin" },
};

describe("handleRoutes", () => {
  beforeEach(() => {
    mockGetOrFetchRoutes.mockReset();
    mockEnrichRoutes.mockReset();
  });

  it("returns all routes when no filters", async () => {
    mockGetOrFetchRoutes.mockResolvedValueOnce([
      { srcChain: "bitcoin", dstChain: "base", srcToken: "BTC", dstToken: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913" },
    ]);
    mockEnrichRoutes.mockReturnValueOnce([enrichedRoute]);

    const result = await handleRoutes({ json: true });
    expect(JSON.parse(result)).toHaveLength(1);
  });

  it("filters by --from", async () => {
    mockGetOrFetchRoutes.mockResolvedValueOnce([
      { srcChain: "bitcoin", dstChain: "base", srcToken: "BTC", dstToken: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913" },
      { srcChain: "bob", dstChain: "bitcoin", srcToken: "0x1", dstToken: "BTC" },
    ]);
    mockEnrichRoutes.mockReturnValueOnce([enrichedRoute, enrichedRouteBob]);

    const result = await handleRoutes({ json: true, from: "bitcoin" });
    const parsed = JSON.parse(result);
    expect(parsed).toHaveLength(1);
    expect(parsed[0].srcChain).toBe("bitcoin");
  });
});
