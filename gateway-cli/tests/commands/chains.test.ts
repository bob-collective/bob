import { describe, test, expect, vi } from "vitest";
import { handleChains } from "../../src/commands/chains.js";

vi.mock("../../src/config/index.js", () => ({
  loadConfig: vi.fn().mockReturnValue({
    apiUrl: "http://test",
    cache: { ttl: "24h" },
  }),
}));

vi.mock("../../src/adapter/sdk-client.js", () => ({
  createSdkClient: vi.fn().mockReturnValue({
    getRoutes: vi.fn(),
  }),
}));

vi.mock("../../src/util/route-cache.js", () => ({
  getOrFetchRoutes: vi.fn().mockResolvedValue([
    { srcChain: "bitcoin", dstChain: "ethereum", srcToken: "BTC", dstToken: "0xA0b86991c6218b36c1D19D4a2e9Eb0cE3606eB48" },
    { srcChain: "bitcoin", dstChain: "bob", srcToken: "BTC", dstToken: "0xdeadbeef00000000000000000000000000000001" },
  ]),
}));

vi.mock("../../src/adapter/route-enricher.js", () => ({
  enrichRoutes: vi.fn().mockReturnValue([
    {
      srcChain: "bitcoin", dstChain: "ethereum",
      srcToken: { address: "BTC", symbol: "BTC", decimals: 8, chain: "bitcoin" },
      dstToken: { address: "0xA0b86991c6218b36c1D19D4a2e9Eb0cE3606eB48", symbol: "USDC", decimals: 6, chain: "ethereum" },
    },
    {
      srcChain: "bitcoin", dstChain: "bob",
      srcToken: { address: "BTC", symbol: "BTC", decimals: 8, chain: "bitcoin" },
      dstToken: { address: "0xdeadbeef00000000000000000000000000000001", symbol: "WBTC", decimals: 8, chain: "bob" },
    },
  ]),
}));

describe("handleChains", () => {
  test("returns table with discovered chains", async () => {
    const result = await handleChains({ json: false });
    expect(result).toContain("bitcoin");
    expect(result).toContain("ethereum");
    expect(result).toContain("bob");
  });

  test("returns JSON array with chain data", async () => {
    const result = await handleChains({ json: true });
    const parsed = JSON.parse(result);
    expect(parsed).toBeInstanceOf(Array);
    expect(parsed.length).toBe(3); // bitcoin, bob, ethereum (sorted)
    expect(parsed[0]).toHaveProperty("canonical");
    expect(parsed[0]).toHaveProperty("aliases");
    expect(parsed[0]).toHaveProperty("chainId");
  });
});
