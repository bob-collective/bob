import { describe, it, expect, vi, beforeEach } from "vitest";
import { handleBalance } from "../../src/commands/balance.js";

// ─── Mocks ───────────────────────────────────────────────────────────────────

const mockGetTokenBalance = vi.fn();

vi.mock("../../src/chains/index.js", () => ({
  getChainFamily: vi.fn((chain: string) => chain === "bitcoin" ? "bitcoin" : "evm"),
  getTokenBalance: (...args: any[]) => mockGetTokenBalance(...args),
}));

vi.mock("../../src/config.js", () => ({
  getSdk: vi.fn(() => ({})),
  BTC_DECIMALS: 8,
}));

vi.mock("@gobob/bob-sdk", async (importOriginal) => {
  const actual = await importOriginal<typeof import("@gobob/bob-sdk")>();
  return { ...actual, formatBtc: actual.formatBtc };
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
  getNativeToken: vi.fn(() => ({ symbol: "ETH", decimals: 18 })),
}));

// ─── Tests ───────────────────────────────────────────────────────────────────

describe("handleBalance", () => {
  beforeEach(() => vi.clearAllMocks());

  it("returns error entry when RPC fails instead of throwing", async () => {
    mockGetTokenBalance.mockRejectedValueOnce(new Error("connection refused"));

    const result = await handleBalance("bc1qtest", { chain: "bitcoin" });

    expect(result.bitcoin.error).toBe(true);
  });
});
