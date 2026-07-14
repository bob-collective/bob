import { describe, it, expect, vi, beforeEach } from "vitest";
import { handleQuote } from "../../src/commands/quote.js";

// ─── Mocks ───────────────────────────────────────────────────────────────────

const mockGetQuote = vi.fn();
const mockGetRecommendedFees = vi.fn();

vi.mock("../../src/config.js", () => ({
  loadConfig: vi.fn(() => ({
    slippageBps: 300,
    btcFeeRate: undefined,
    bitcoinPrivateKey: undefined,
    evmPrivateKey: undefined,
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
    { srcChain: "bitcoin", dstChain: "base", srcToken: "BTC", dstToken: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913" },
    // an offramp route, so the EVM-source cases resolve through the real resolver
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

// The REAL input-resolver is used here on purpose: `parseAssetChain` is the resolver whose
// verdict on the source chain `handleQuote` must defer to, and the bugs below were all
// caused by second-guessing it. Mocking it would only re-test a mock.

vi.mock("../../src/chains/index.js", () => ({
  getChainFamily: vi.fn((chain: string) => chain === "bitcoin" ? "bitcoin" : "evm"),
  deriveAddress: vi.fn().mockResolvedValue("0xDerived"),
  resolvePrivateKey: vi.fn(() => undefined),
  resolveRecipient: vi.fn().mockResolvedValue("0xDerived"),
}));

// ─── Fixtures ────────────────────────────────────────────────────────────────

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

/** Point the mocked recipient at the BTC address a USDT@ethereum → BTC offramp pays out to. */
async function mockOfframpRecipient() {
  const { resolveRecipient } = await import("../../src/chains/index.js");
  vi.mocked(resolveRecipient).mockResolvedValueOnce(BTC_RECIPIENT);
}

// ─── Tests ───────────────────────────────────────────────────────────────────

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

  // ─── A1 ────────────────────────────────────────────────────────────────────
  // `ownerAddress` is the EVM-side address controlling the order — the sender on an
  // offramp. `quote` derived the sender only for `--amount ALL`, so on an ordinary
  // offramp it was undefined and `ownerAddress` fell back to the (Bitcoin) recipient.
  // Staging rejected EVERY offramp quote with
  // "INVALID_REQUEST: Invalid Ethereum address: Expected an EVM address but found a
  // Bitcoin address". `swap` derived it unconditionally, which is why only `quote` broke.

  it("sends the EVM sender as ownerAddress on an offramp quote with a fixed amount", async () => {
    const { resolvePrivateKey, deriveAddress } = await import("../../src/chains/index.js");
    await mockOfframpRecipient();
    vi.mocked(resolvePrivateKey).mockReturnValueOnce("0xevmkey");
    vi.mocked(deriveAddress).mockResolvedValueOnce(EVM_SENDER);
    mockGetQuote.mockResolvedValueOnce(offrampSdkQuote);

    await handleQuote({ src: "USDT:ethereum", dst: "BTC", amount: "47000000" });

    expect(mockGetQuote).toHaveBeenCalledWith(
      expect.objectContaining({ ownerAddress: EVM_SENDER, toUserAddress: BTC_RECIPIENT }),
    );
  });

  it("fails with an actionable error rather than quoting an offramp with no EVM sender", async () => {
    // The API makes ownerAddress mandatory (omitting it → MISSING_OWNER_ADDRESS), and the
    // BTC recipient is not a legal substitute. There is nothing to send, so say so.
    const { resolvePrivateKey } = await import("../../src/chains/index.js");
    await mockOfframpRecipient();
    vi.mocked(resolvePrivateKey).mockReturnValueOnce(undefined);

    await expect(
      handleQuote({ src: "USDT:ethereum", dst: "BTC", amount: "47000000" }),
    ).rejects.toThrow(/EVM owner address/);
    expect(mockGetQuote).not.toHaveBeenCalled();
  });

  // ─── A2 ────────────────────────────────────────────────────────────────────

  it("reports the invalid asset, not a key error, when the source has no chain qualifier", async () => {
    // `quote` re-parsed `--src` with an ad-hoc heuristic, so a bare symbol was classified
    // as an EVM chain and a key was derived BEFORE the asset was validated: a malformed
    // EVM_PRIVATE_KEY surfaced as "Failed to derive sender address" instead of the real
    // problem. The asset resolver's verdict must come first.
    const { resolvePrivateKey, deriveAddress } = await import("../../src/chains/index.js");
    vi.mocked(resolvePrivateKey).mockReturnValue("0xmalformed");
    vi.mocked(deriveAddress).mockRejectedValue(new Error("invalid private key"));

    await expect(
      handleQuote({ src: "USDT", dst: "BTC", amount: "5000000", recipient: BTC_RECIPIENT }),
    ).rejects.toThrow(/chain required/);

    expect(deriveAddress).not.toHaveBeenCalled();
  });

  it.each(["BTC", "Btc", "BTC:Bitcoin"])(
    "does not derive an EVM key for a BTC onramp quote (src: %s)",
    async (src) => {
      // The same ad-hoc parser read the source family case-sensitively, so `--src Btc`
      // was classified as an EVM chain — deriving EVM_PRIVATE_KEY and sending an EVM
      // address as the onramp's fromUserAddress. A BTC source needs no EVM key at all.
      const { resolvePrivateKey } = await import("../../src/chains/index.js");
      mockGetQuote.mockResolvedValueOnce(sdkQuote);

      await handleQuote({
        src, dst: "USDC:base", amount: "5000000", recipient: "0xRecipient", btcFeeRate: 15,
      });

      expect(resolvePrivateKey).not.toHaveBeenCalled();
      expect(mockGetQuote).toHaveBeenCalledWith(expect.objectContaining({ fromUserAddress: undefined }));
    },
  );

  // ─── A3 ────────────────────────────────────────────────────────────────────

  it("does not touch the EVM key when --owner already supplies the owner", async () => {
    // With --owner given, the derived sender is not used for anything on a quote. Deriving
    // it anyway makes a malformed EVM_PRIVATE_KEY break a quote that needs no key — a key
    // must only be touched where it is load-bearing.
    const { resolvePrivateKey, deriveAddress } = await import("../../src/chains/index.js");
    await mockOfframpRecipient();
    vi.mocked(resolvePrivateKey).mockReturnValue("0xmalformed");
    vi.mocked(deriveAddress).mockRejectedValue(new Error("invalid private key"));
    mockGetQuote.mockResolvedValueOnce(offrampSdkQuote);

    await handleQuote({
      src: "USDT:ethereum", dst: "BTC", amount: "47000000", owner: EVM_SENDER,
    });

    expect(deriveAddress).not.toHaveBeenCalled();
    expect(mockGetQuote).toHaveBeenCalledWith(expect.objectContaining({ ownerAddress: EVM_SENDER }));
  });
});
