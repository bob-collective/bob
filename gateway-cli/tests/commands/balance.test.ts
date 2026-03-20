import { describe, it, expect, vi, beforeEach } from "vitest";
import { handleBalance } from "../../src/commands/balance.js";

// ─── Mocks ───────────────────────────────────────────────────────────────────

const mockGetTokenBalance = vi.fn();

vi.mock("../../src/chains/index.js", () => ({
  getChainFamily: vi.fn((chain: string) => chain === "bitcoin" ? "bitcoin" : "evm"),
  getTokenBalance: (...args: any[]) => mockGetTokenBalance(...args),
}));

vi.mock("../../src/config.js", () => ({
  getSdk: vi.fn(() => ({
    getMaxSpendable: vi.fn(),
    getRoutes: vi.fn(),
  })),
  BTC_DECIMALS: 8,
}));

vi.mock("@gobob/bob-sdk", async (importOriginal) => {
  const actual = await importOriginal<typeof import("@gobob/bob-sdk")>();
  return {
    ...actual,
    formatBtc: actual.formatBtc,
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
  getNativeToken: vi.fn((chain: string) => {
    if (chain === "base") return { symbol: "ETH", decimals: 18 };
    return { symbol: "ETH", decimals: 18 };
  }),
}));

vi.mock("../../src/util/rpc-resolver.js", () => ({
  resolveRpcUrl: vi.fn().mockReturnValue(undefined),
  getViemChain: vi.fn().mockReturnValue(undefined),
}));

// ─── Tests ───────────────────────────────────────────────────────────────────

describe("handleBalance", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("returns BTC total balance and allSpendable", async () => {
    mockGetTokenBalance.mockResolvedValueOnce({ total: "500000", allSpendable: "490000" });

    const result = await handleBalance("bc1qtest", { chain: "bitcoin" });

    expect(result).toHaveProperty("bitcoin");
    expect(result.bitcoin.address).toBe("bc1qtest");
    expect(result.bitcoin.balance).toBeDefined();
    expect(result.bitcoin.allSpendable).toBeDefined();
  });

  it("calls getTokenBalance with correct args for BTC", async () => {
    mockGetTokenBalance.mockResolvedValueOnce({ total: "500000", allSpendable: "490000" });

    await handleBalance("bc1qtest", { chain: "bitcoin" });

    expect(mockGetTokenBalance).toHaveBeenCalledWith(
      "bitcoin",
      "bc1qtest",
      { address: "BTC", symbol: "BTC", decimals: 8 },
    );
  });

  it("returns zero balance when balance is zero", async () => {
    mockGetTokenBalance.mockResolvedValueOnce({ total: "0", allSpendable: "0" });

    const result = await handleBalance("bc1qtest", { chain: "bitcoin" });

    expect(result).toHaveProperty("bitcoin");
    expect(result.bitcoin.balance).toBeDefined();
    expect(result.bitcoin.allSpendable).toBeDefined();
  });

  it("returns error entry when RPC fails", async () => {
    mockGetTokenBalance.mockRejectedValueOnce(new Error("connection refused"));

    const result = await handleBalance("bc1qtest", { chain: "bitcoin" });

    expect(result.bitcoin.error).toBe(true);
  });

  it("passes feeToken and feeReserve options for EVM chains", async () => {
    // Native balance call
    mockGetTokenBalance.mockResolvedValueOnce({ total: "1000000000000000000", allSpendable: "900000000000000000" });
    // Token balance call (USDC)
    mockGetTokenBalance.mockResolvedValueOnce({ total: "5000000", allSpendable: "4000000" });

    await handleBalance("0xTestAddr", {
      chain: "base",
      feeToken: "0xUSDC",
      feeReserve: "1000000",
    });

    // Both calls should include feeToken/feeReserve
    expect(mockGetTokenBalance).toHaveBeenCalledWith(
      "base",
      "0xTestAddr",
      expect.any(Object),
      expect.objectContaining({ feeToken: "0xUSDC", feeReserve: "1000000" }),
    );
  });
});
