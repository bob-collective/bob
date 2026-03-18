import { describe, test, expect, vi } from "vitest";
import { handleTokens } from "../../src/commands/tokens.js";

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
  ]),
}));

vi.mock("../../src/adapter/route-enricher.js", () => ({
  enrichRoutes: vi.fn().mockReturnValue([
    {
      srcChain: "bitcoin", dstChain: "ethereum",
      srcToken: { address: "0x0000000000000000000000000000000000000000", symbol: "BTC", decimals: 8, chain: "bitcoin" },
      dstToken: { address: "0xA0b86991c6218b36c1D19D4a2e9Eb0cE3606eB48", symbol: "USDC", decimals: 6, chain: "ethereum" },
    },
  ]),
}));

describe("handleTokens", () => {
  test("returns tokens for a chain", async () => {
    const result = await handleTokens({ chain: "ethereum", json: false });
    expect(result).toContain("USDC");
    expect(result).toContain("0xA0b86991c6218b36c1D19D4a2e9Eb0cE3606eB48");
  });

  test("returns JSON array", async () => {
    const result = await handleTokens({ chain: "ethereum", json: true });
    const parsed = JSON.parse(result);
    expect(parsed).toBeInstanceOf(Array);
    expect(parsed[0]).toHaveProperty("symbol");
    expect(parsed[0]).toHaveProperty("address");
    expect(parsed[0]).toHaveProperty("decimals");
  });

  test("resolves chain aliases", async () => {
    const result = await handleTokens({ chain: "eth", json: true });
    const parsed = JSON.parse(result);
    expect(parsed.length).toBeGreaterThan(0);
  });

  test("throws for chain with no tokens", async () => {
    await expect(handleTokens({ chain: "bob", json: false }))
      .rejects.toThrow('no tokens found on chain "bob"');
  });
});
