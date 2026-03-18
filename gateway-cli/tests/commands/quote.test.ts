import { describe, it, expect, vi, beforeEach } from "vitest";
import { handleQuote } from "../../src/commands/quote.js";
import type { EnrichedRoute } from "../../src/adapter/route-enricher.js";

const mockGetQuote = vi.fn();
const mockGetRoutes = vi.fn();

vi.mock("../../src/adapter/sdk-client.js", () => ({
  createSdkClient: vi.fn(() => ({
    getQuote: mockGetQuote,
    getRoutes: mockGetRoutes,
  })),
}));

vi.mock("../../src/config/index.js", () => ({
  loadConfig: vi.fn(() => ({
    apiUrl: "https://example.com",
    cache: { ttl: "24h" },
  })),
}));

const mockEnrichedRoutes: EnrichedRoute[] = [
  {
    srcChain: "bitcoin",
    dstChain: "base",
    srcToken: {
      address: "0x0000000000000000000000000000000000000000",
      symbol: "BTC",
      decimals: 8,
      chain: "bitcoin",
    },
    dstToken: {
      address: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913",
      symbol: "USDC",
      decimals: 6,
      chain: "base",
    },
  },
];

vi.mock("../../src/util/route-cache.js", () => ({
  getOrFetchRoutes: vi.fn().mockResolvedValue([]),
}));

vi.mock("../../src/adapter/route-enricher.js", () => ({
  enrichRoutes: vi.fn(() => mockEnrichedRoutes),
}));

vi.mock("../../src/adapter/quote-flattener.js", () => ({
  flattenQuote: vi.fn((quote: any) => ({
    variant: "onramp",
    inputAmount: "5000000",
    outputAmount: "4812300000",
    fees: "15000",
    raw: quote,
  })),
}));

vi.mock("../../src/util/mempool.js", () => ({
  fetchFeeRate: vi.fn().mockResolvedValue(10),
}));

describe("handleQuote", () => {
  beforeEach(() => {
    mockGetQuote.mockReset();
    mockGetRoutes.mockReset();
  });

  it("fetches and formats a quote as JSON shape", async () => {
    const sdkQuote = {
      onramp: {
        inputAmount: { amount: "5000000" },
        outputAmount: { amount: "4812300000" },
        fees: { amount: "15000" },
        estimatedTimeInSecs: 600,
      },
    };
    mockGetQuote.mockResolvedValueOnce(sdkQuote);

    const result = await handleQuote({
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
    expect(parsed.dstAmount).toBe("4812300000");
  });

  it("passes SDK-shaped params to getQuote", async () => {
    const sdkQuote = {
      onramp: {
        inputAmount: { amount: "5000000" },
        outputAmount: { amount: "4812300000" },
        fees: { amount: "15000" },
      },
    };
    mockGetQuote.mockResolvedValueOnce(sdkQuote);

    await handleQuote({
      amount: "0.05BTC",
      src: "BTC",
      dst: "USDC:base",
      recipient: "0xRecipient",
      sender: "0xSender",
      slippageBps: 200,
      json: true,
    });

    expect(mockGetQuote).toHaveBeenCalledWith({
      fromChain: "bitcoin",
      toChain: "base",
      fromToken: "0x0000000000000000000000000000000000000000",
      toToken: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913",
      toUserAddress: "0xRecipient",
      fromUserAddress: "0xSender",
      amount: "5000000",
      maxSlippage: 200,
    });
  });

  it("returns human-readable output when json is false", async () => {
    const sdkQuote = {
      onramp: {
        inputAmount: { amount: "5000000" },
        outputAmount: { amount: "4812300000" },
        fees: { amount: "15000" },
      },
    };
    mockGetQuote.mockResolvedValueOnce(sdkQuote);

    const result = await handleQuote({
      amount: "0.05BTC",
      src: "BTC",
      dst: "USDC:base",
      recipient: "0xABC",
      slippageBps: 300,
      json: false,
    });

    expect(result).toContain("Swap");
    expect(result).toContain("BTC");
    expect(result).toContain("USDC");
    expect(result).toContain("base");
  });
});
