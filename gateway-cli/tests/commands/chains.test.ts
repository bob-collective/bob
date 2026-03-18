import { describe, test, expect, vi } from "vitest";
import { handleChains } from "../../src/commands/chains.js";

vi.mock("../../src/api/client.js", () => ({
  GatewayApiClient: vi.fn().mockImplementation(function () {
    return {
      getRoutes: vi.fn().mockResolvedValue([
        {
          srcChain: "bitcoin", dstChain: "ethereum",
          srcToken: { address: "0x0000000000000000000000000000000000000000", symbol: "BTC", decimals: 8, chain: "bitcoin" },
          dstToken: { address: "0xA0b86991c6218b36c1D19D4a2e9Eb0cE3606eB48", symbol: "USDC", decimals: 6, chain: "ethereum" },
        },
        {
          srcChain: "bitcoin", dstChain: "bob",
          srcToken: { address: "0x0000000000000000000000000000000000000000", symbol: "BTC", decimals: 8, chain: "bitcoin" },
          dstToken: { address: "0xdeadbeef00000000000000000000000000000001", symbol: "WBTC", decimals: 8, chain: "bob" },
        },
      ]),
    };
  }),
}));

describe("handleChains", () => {
  test("returns table with discovered chains", async () => {
    const result = await handleChains({ apiUrl: "http://test", json: false });
    expect(result).toContain("bitcoin");
    expect(result).toContain("ethereum");
    expect(result).toContain("bob");
  });

  test("returns JSON array with chain data", async () => {
    const result = await handleChains({ apiUrl: "http://test", json: true });
    const parsed = JSON.parse(result);
    expect(parsed).toBeInstanceOf(Array);
    expect(parsed.length).toBe(3); // bitcoin, bob, ethereum (sorted)
    expect(parsed[0]).toHaveProperty("canonical");
    expect(parsed[0]).toHaveProperty("aliases");
    expect(parsed[0]).toHaveProperty("chainId");
  });
});
