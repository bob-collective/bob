import { describe, it, expect, vi, beforeEach } from "vitest";
import { handleQuote } from "../../src/commands/quote.js";

// ─── Mocks ───────────────────────────────────────────────────────────────────

const mockGetQuote = vi.fn();
const mockGetRecommendedFees = vi.fn();

vi.mock("../../src/config.js", () => ({
  loadConfig: vi.fn(() => ({
    slippageBps: 300,
    btcFeeRate: undefined,
  })),
  getSdk: vi.fn(() => ({
    getQuote: mockGetQuote,
  })),
  BTC_DECIMALS: 8,
}));

vi.mock("@gobob/bob-sdk", async (importOriginal) => {
  const actual = await importOriginal<typeof import("@gobob/bob-sdk")>();
  return {
    ...actual,
    MempoolClient: vi.fn(() => ({
      getRecommendedFees: mockGetRecommendedFees,
    })),
  };
});

vi.mock("../../src/util/route-provider.js", () => ({
  getEnrichedRoutes: vi.fn().mockResolvedValue([
    {
      srcChain: "bitcoin",
      dstChain: "base",
      srcToken: { address: "BTC", symbol: "BTC", decimals: 8, chain: "bitcoin" },
      dstToken: { address: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913", symbol: "USDC", decimals: 6, chain: "base" },
    },
  ]),
}));

vi.mock("../../src/util/input-resolver.js", () => ({
  resolveSwapInputs: vi.fn().mockResolvedValue({
    srcAsset: { chain: "bitcoin", address: "BTC", symbol: "BTC", decimals: 8 },
    dstAsset: { chain: "base", address: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913", symbol: "USDC", decimals: 6 },
    parsed: { type: "atomic", atomicUnits: "5000000", display: "0.05 BTC" },
  }),
}));

// ─── Tests ───────────────────────────────────────────────────────────────────

const sdkQuote = {
  onramp: {
    inputAmount: { amount: "5000000" },
    outputAmount: { amount: "4800000000" },
    fee: { amount: "10000" },
  },
};

describe("handleQuote", () => {
  beforeEach(() => {
    mockGetQuote.mockReset();
    mockGetRecommendedFees.mockReset();
  });

  it("fetches mempool fee rate when src is bitcoin and no btcFeeRate provided", async () => {
    mockGetQuote.mockResolvedValueOnce(sdkQuote);
    mockGetRecommendedFees.mockResolvedValueOnce({ fastestFee: 25 });

    const result = await handleQuote({
      src: "BTC", dst: "USDC:base", amount: "5000000", recipient: "0xRecipient",
    });

    expect(mockGetRecommendedFees).toHaveBeenCalledOnce();
    expect(result.quote.feeRateSatPerVbyte).toBe(25);
  });

  it("uses provided btcFeeRate without fetching from mempool", async () => {
    mockGetQuote.mockResolvedValueOnce(sdkQuote);

    const result = await handleQuote({
      src: "BTC", dst: "USDC:base", amount: "5000000", recipient: "0xRecipient",
      btcFeeRate: 15,
    });

    expect(mockGetRecommendedFees).not.toHaveBeenCalled();
    expect(result.quote.feeRateSatPerVbyte).toBe(15);
  });
});
