import { describe, it, expect, vi, beforeEach } from "vitest";
import { handleQuote } from "../../src/commands/quote.js";
import { mockQuote, mockRoutes } from "../fixtures/api-responses.js";

const mockGetQuote = vi.fn();
const mockGetRoutes = vi.fn();
vi.mock("../../src/api/client.js", () => ({
  GatewayApiClient: vi.fn().mockImplementation(function () {
    return {
      getQuote: mockGetQuote,
      getRoutes: mockGetRoutes,
    };
  }),
}));

vi.mock("../../src/util/mempool.js", () => ({
  fetchFeeRate: vi.fn().mockResolvedValue(10),
}));

describe("handleQuote", () => {
  beforeEach(() => {
    mockGetQuote.mockReset();
    mockGetRoutes.mockReset();
    mockGetRoutes.mockResolvedValue(mockRoutes);
  });

  it("fetches and formats a quote as JSON shape", async () => {
    mockGetQuote.mockResolvedValueOnce(mockQuote);
    const result = await handleQuote({
      apiUrl: "https://example.com",
      amount: "0.05BTC",
      src: "BTC",
      dst: "USDC:base",
      recipient: "0xABC",
      slippageBps: 300,
      json: true,
    });
    const parsed = JSON.parse(result);
    expect(parsed.srcAsset).toBe("BTC");
    expect(parsed.dstAsset).toBe("USDC");
    expect(parsed.dstChain).toBe("base");
    expect(parsed.slippageBps).toBe(300);
    expect(parsed.dstAmount).toBe(mockQuote.outputAmount);
  });

});
