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

const USDC_BASE = "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913";
const USDT_ETH = "0xdAC17F958D2ee523a2206206994597C13D831ec7";

vi.mock("../../src/util/route-provider.js", () => ({
  getRoutes: vi.fn().mockResolvedValue([
    { srcChain: "bitcoin", dstChain: "base", srcToken: "BTC", dstToken: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913" },
    // offramp route, so the EVM-source cases resolve through the real resolver
    { srcChain: "ethereum", dstChain: "bitcoin", srcToken: "0xdAC17F958D2ee523a2206206994597C13D831ec7", dstToken: "BTC" },
  ]),
}));

vi.mock("../../src/chains/evm.js", () => ({
  CHAIN_IDS: { ethereum: 1, base: 8453 } as Record<string, number>,
  getTokenMetadata: vi.fn((address: string, chain: string) => {
    if (chain === "bitcoin" || address === "BTC") return { symbol: "BTC", decimals: 8 };
    if (address.toLowerCase() === "0xdac17f958d2ee523a2206206994597c13d831ec7") return { symbol: "USDT", decimals: 6 };
    return { symbol: "USDC", decimals: 6 };
  }),
  getEvmBalances: vi.fn(),
}));

// Use the REAL parseAssetChain/buildTokenIndex: they are the resolver whose verdict
// handleQuote must defer to, so mocking them would only re-test a mock. Only
// resolveSwapInputs (which reads balances / prices) is stubbed.
vi.mock("../../src/util/input-resolver.js", async (importOriginal) => {
  const actual = await importOriginal<typeof import("../../src/util/input-resolver.js")>();
  return {
    ...actual,
    resolveSwapInputs: vi.fn().mockResolvedValue({
      srcAsset: { chain: "bitcoin", address: "BTC", symbol: "BTC", decimals: 8 },
      dstAsset: { chain: "base", address: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913", symbol: "USDC", decimals: 6 },
      atomicUnits: "5000000",
      display: "0.05 BTC",
    }),
  };
});

vi.mock("../../src/chains/index.js", () => ({
  getChainFamily: vi.fn((chain: string) => chain === "bitcoin" ? "bitcoin" : "evm"),
  deriveAddress: vi.fn().mockResolvedValue("0xDerived"),
  resolvePrivateKey: vi.fn(() => undefined),
  resolveRecipient: vi.fn().mockResolvedValue("0xDerived"),
}));

// ─── Tests ───────────────────────────────────────────────────────────────────

const sdkQuote = {
  onramp: {
    inputAmount: { amount: "5000000" },
    outputAmount: { amount: "4800000000" },
    fee: { amount: "10000" },
  },
};

const offrampSdkQuote = {
  offramp: {
    inputAmount: { amount: "47000000" },
    outputAmount: { amount: "74489" },
  },
};

const BTC_RECIPIENT = "bc1q4xdatls497ea76fmuefu9we4ld4yu2vy8hedne";
const EVM_SENDER = "0xAF91558Ba2B1994530c9cfCcbda5AE9cD2b456bb";

/** Point the mocked resolvers at an EVM→BTC offramp (USDT@ethereum → BTC). */
async function mockOfframpInputs() {
  const { resolveSwapInputs } = await import("../../src/util/input-resolver.js");
  const { resolveRecipient } = await import("../../src/chains/index.js");
  vi.mocked(resolveSwapInputs).mockResolvedValueOnce({
    srcAsset: { chain: "ethereum", address: "0xdAC17F958D2ee523a2206206994597C13D831ec7", symbol: "USDT", decimals: 6 },
    dstAsset: { chain: "bitcoin", address: "BTC", symbol: "BTC", decimals: 8 },
    atomicUnits: "47000000",
    display: "47 USDT",
  } as any);
  vi.mocked(resolveRecipient).mockResolvedValueOnce(BTC_RECIPIENT);
}

describe("handleQuote", () => {
  beforeEach(() => {
    vi.clearAllMocks();
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

  // Regression: ownerAddress is the EVM-side address controlling the order — the
  // sender on an offramp. Deriving the sender only for `--amount ALL` left it
  // undefined here, so ownerAddress fell back to the (Bitcoin) recipient and the
  // API rejected every offramp quote with
  // "INVALID_REQUEST: Expected an EVM address but found a Bitcoin address".
  it("sends the EVM sender as ownerAddress on an offramp quote with a fixed amount", async () => {
    const { resolvePrivateKey, deriveAddress } = await import("../../src/chains/index.js");
    await mockOfframpInputs();
    vi.mocked(resolvePrivateKey).mockReturnValueOnce("0xevmkey");
    vi.mocked(deriveAddress).mockResolvedValueOnce(EVM_SENDER);
    mockGetQuote.mockResolvedValueOnce(offrampSdkQuote);

    await handleQuote({ src: "USDT:ethereum", dst: "BTC", amount: "47000000" });

    expect(mockGetQuote).toHaveBeenCalledWith(
      expect.objectContaining({ ownerAddress: EVM_SENDER, toUserAddress: BTC_RECIPIENT }),
    );
  });

  it("fails with an actionable error rather than quoting an offramp with no EVM sender", async () => {
    const { resolvePrivateKey } = await import("../../src/chains/index.js");
    await mockOfframpInputs();
    vi.mocked(resolvePrivateKey).mockReturnValueOnce(undefined);

    await expect(
      handleQuote({ src: "USDT:ethereum", dst: "BTC", amount: "47000000" }),
    ).rejects.toThrow(/EVM owner address/);
    expect(mockGetQuote).not.toHaveBeenCalled();
  });

  // A bitcoin source needs no EVM key, so it must never derive one — not even when
  // spelled in a case the asset resolver accepts ("Btc", "BTC:Bitcoin"). Reading the
  // source family case-sensitively would classify those as EVM, derive EVM_PRIVATE_KEY,
  // and send an EVM address as the onramp's fromUserAddress.
  it.each(["BTC", "Btc", "BTC:Bitcoin"])(
    "does not derive an EVM key for a BTC onramp quote (src: %s)",
    async (src) => {
      const { resolvePrivateKey } = await import("../../src/chains/index.js");
      mockGetQuote.mockResolvedValueOnce(sdkQuote);

      await handleQuote({
        src, dst: "USDC:base", amount: "5000000", recipient: "0xRecipient", btcFeeRate: 15,
      });

      expect(resolvePrivateKey).not.toHaveBeenCalled();
      expect(mockGetQuote).toHaveBeenCalledWith(expect.objectContaining({ fromUserAddress: undefined }));
    },
  );

  it("does not touch the EVM key when --owner already supplies the owner", async () => {
    // resolveOwnerAddress prefers the explicit --owner, so the derived sender is unused on
    // this path. Deriving anyway makes a malformed EVM_PRIVATE_KEY break a quote that needs
    // no key at all — the very thing the lazy derivation exists to avoid.
    const { resolvePrivateKey, deriveAddress } = await import("../../src/chains/index.js");
    await mockOfframpInputs();
    vi.mocked(resolvePrivateKey).mockReturnValueOnce("0xmalformed");
    vi.mocked(deriveAddress).mockRejectedValueOnce(new Error("invalid private key"));
    mockGetQuote.mockResolvedValueOnce(offrampSdkQuote);

    await handleQuote({
      src: "USDT:ethereum", dst: "BTC", amount: "47000000", ownerAddress: EVM_SENDER,
    });

    expect(deriveAddress).not.toHaveBeenCalled();
    expect(mockGetQuote).toHaveBeenCalledWith(expect.objectContaining({ ownerAddress: EVM_SENDER }));
  });

  it("reports the invalid asset, not a key error, when the source has no chain qualifier", async () => {
    // A bare symbol is not resolvable without a chain. The resolver must reject it *before*
    // any key is touched — otherwise a malformed EVM key masks the real problem with a
    // misleading 'Failed to derive sender address' diagnostic.
    const { resolvePrivateKey, deriveAddress } = await import("../../src/chains/index.js");
    vi.mocked(resolvePrivateKey).mockReturnValueOnce("0xmalformed");
    vi.mocked(deriveAddress).mockRejectedValueOnce(new Error("invalid private key"));

    await expect(
      handleQuote({ src: "USDT", dst: "BTC", amount: "5000000", recipient: "bc1qrecipient" }),
    ).rejects.toThrow(/chain required/);

    expect(deriveAddress).not.toHaveBeenCalled();
  });

});
