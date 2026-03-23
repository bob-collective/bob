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
  getRoutes: vi.fn().mockResolvedValue([
    {
      srcChain: "bitcoin",
      dstChain: "base",
      srcToken: "BTC",
      dstToken: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913",
    },
  ]),
}));

vi.mock("../../src/util/input-resolver.js", () => ({
  resolveSwapInputs: vi.fn().mockResolvedValue({
    srcAsset: { chain: "bitcoin", address: "BTC", symbol: "BTC", decimals: 8 },
    dstAsset: { chain: "base", address: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913", symbol: "USDC", decimals: 6 },
    atomicUnits: "5000000",
    display: "0.05 BTC",
  }),
}));

vi.mock("../../src/chains/index.js", () => ({
  getChainFamily: vi.fn((chain: string) => chain === "bitcoin" ? "bitcoin" : "evm"),
  deriveAddress: vi.fn().mockResolvedValue("0xDerived"),
  resolvePrivateKey: vi.fn(() => undefined),
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

  it("derives recipient from EVM_PRIVATE_KEY when --recipient is omitted", async () => {
    mockGetQuote.mockResolvedValueOnce(sdkQuote);
    mockGetRecommendedFees.mockResolvedValueOnce({ fastestFee: 25 });

    const { resolvePrivateKey } = await import("../../src/chains/index.js");
    vi.mocked(resolvePrivateKey).mockReturnValueOnce("0xEvmKey");

    const { deriveAddress } = await import("../../src/chains/index.js");
    vi.mocked(deriveAddress).mockResolvedValueOnce("0xDerivedAddr");

    const result = await handleQuote({
      src: "BTC", dst: "USDC:base", amount: "5000000",
    });

    expect(vi.mocked(deriveAddress)).toHaveBeenCalledWith("base", "0xEvmKey");
    expect(result.confirmation.recipient).toBe("0xDerivedAddr");
  });

  it("explicit --recipient overrides derived address in quote", async () => {
    mockGetQuote.mockResolvedValueOnce(sdkQuote);
    mockGetRecommendedFees.mockResolvedValueOnce({ fastestFee: 25 });

    const result = await handleQuote({
      src: "BTC", dst: "USDC:base", amount: "5000000", recipient: "0xExplicit",
    });

    expect(mockGetQuote).toHaveBeenCalledWith(
      expect.objectContaining({ toUserAddress: "0xExplicit" }),
    );
    expect(result.confirmation.recipient).toBe("0xExplicit");
  });

  it("throws when --recipient omitted and no destination key available in quote", async () => {
    const { resolvePrivateKey } = await import("../../src/chains/index.js");
    vi.mocked(resolvePrivateKey).mockReturnValueOnce(undefined);

    await expect(
      handleQuote({ src: "BTC", dst: "USDC:base", amount: "5000000" }),
    ).rejects.toThrow("--recipient is required");
  });
});
