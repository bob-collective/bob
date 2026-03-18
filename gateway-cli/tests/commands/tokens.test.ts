import { describe, test, expect, vi } from "vitest";
import { handleTokens } from "../../src/commands/tokens.js";

vi.mock("../../src/api/client.js", () => ({
  GatewayApiClient: vi.fn().mockImplementation(function () {
    return {
      getRoutes: vi.fn().mockResolvedValue([
        {
          srcChain: "bitcoin", dstChain: "ethereum",
          srcToken: { address: "0x0000000000000000000000000000000000000000", symbol: "BTC", decimals: 8, chain: "bitcoin" },
          dstToken: { address: "0xA0b86991c6218b36c1D19D4a2e9Eb0cE3606eB48", symbol: "USDC", decimals: 6, chain: "ethereum" },
        },
      ]),
    };
  }),
}));

describe("handleTokens", () => {
  test("returns tokens for a chain", async () => {
    const result = await handleTokens({ apiUrl: "http://test", chain: "ethereum", json: false });
    expect(result).toContain("USDC");
    expect(result).toContain("0xA0b86991c6218b36c1D19D4a2e9Eb0cE3606eB48");
  });

  test("returns JSON array", async () => {
    const result = await handleTokens({ apiUrl: "http://test", chain: "ethereum", json: true });
    const parsed = JSON.parse(result);
    expect(parsed).toBeInstanceOf(Array);
    expect(parsed[0]).toHaveProperty("symbol");
    expect(parsed[0]).toHaveProperty("address");
    expect(parsed[0]).toHaveProperty("decimals");
  });

  test("resolves chain aliases", async () => {
    const result = await handleTokens({ apiUrl: "http://test", chain: "eth", json: true });
    const parsed = JSON.parse(result);
    expect(parsed.length).toBeGreaterThan(0);
  });

  test("throws for chain with no tokens", async () => {
    await expect(handleTokens({ apiUrl: "http://test", chain: "bob", json: false }))
      .rejects.toThrow('no tokens found on chain "bob"');
  });
});
