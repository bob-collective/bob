import { describe, it, expect, vi, beforeEach } from "vitest";
import { handleBalance } from "../../src/commands/balance.js";

// ─── Mocks ───────────────────────────────────────────────────────────────────

const mockGetMaxSpendable = vi.fn();
const mockGetBalance = vi.fn();
const mockGetRoutes = vi.fn();

vi.mock("../../src/config.js", () => ({
  getSdk: vi.fn(() => ({
    getMaxSpendable: mockGetMaxSpendable,
    getRoutes: mockGetRoutes,
  })),
  BTC_DECIMALS: 8,
}));

vi.mock("@gobob/bob-sdk", async (importOriginal) => {
  const actual = await importOriginal<typeof import("@gobob/bob-sdk")>();
  return {
    ...actual,
    EsploraClient: vi.fn(() => ({
      getBalance: mockGetBalance,
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
  getNativeToken: vi.fn((chain: string) => {
    if (chain === "base") return { symbol: "ETH", decimals: 18 };
    return { symbol: "ETH", decimals: 18 };
  }),
}));

vi.mock("../../src/util/rpc-resolver.js", () => ({
  resolveRpcUrl: vi.fn().mockReturnValue(undefined),
  getViemChain: vi.fn().mockReturnValue(undefined),
}));

vi.mock("viem", async (importOriginal) => {
  const actual = await importOriginal<typeof import("viem")>();
  return {
    ...actual,
    createPublicClient: vi.fn(() => ({
      getBalance: vi.fn().mockResolvedValue(0n),
      multicall: vi.fn().mockResolvedValue([]),
    })),
  };
});

// ─── Tests ───────────────────────────────────────────────────────────────────

describe("handleBalance", () => {
  beforeEach(() => {
    mockGetMaxSpendable.mockReset();
    mockGetBalance.mockReset();
    mockGetRoutes.mockReset();
  });

  it("returns BTC total balance and maxSpendable", async () => {
    mockGetBalance.mockResolvedValueOnce({ confirmed: 500000, unconfirmed: 0 });
    mockGetMaxSpendable.mockResolvedValueOnce({
      amount: { amount: "490000" },
    });

    const result = await handleBalance("bc1qtest", { chain: "bitcoin" });

    expect(result).toHaveProperty("bitcoin");
    expect(result.bitcoin.address).toBe("bc1qtest");
    expect(result.bitcoin.balance).toBeDefined();
    expect(result.bitcoin.maxSpendable).toBeDefined();
  });

  it("sums confirmed + unconfirmed into balance", async () => {
    mockGetBalance.mockResolvedValueOnce({ confirmed: 300000, unconfirmed: 50000 });
    mockGetMaxSpendable.mockResolvedValueOnce({
      amount: { amount: "290000" },
    });

    const result = await handleBalance("bc1qtest", { chain: "bitcoin" });

    expect(result.bitcoin.balance).toBeDefined();
  });

  it("returns zero balance when balance is zero", async () => {
    mockGetBalance.mockResolvedValueOnce({ confirmed: 0, unconfirmed: 0 });
    mockGetMaxSpendable.mockResolvedValueOnce({
      amount: { amount: "0" },
    });

    const result = await handleBalance("bc1qtest", { chain: "bitcoin" });

    expect(result).toHaveProperty("bitcoin");
    expect(result.bitcoin.balance).toBeDefined();
  });

  it("returns error entry when RPC fails", async () => {
    mockGetBalance.mockRejectedValueOnce(new Error("connection refused"));
    mockGetMaxSpendable.mockRejectedValueOnce(new Error("connection refused"));

    const result = await handleBalance("bc1qtest", { chain: "bitcoin" });

    expect(result.bitcoin.error).toBe(true);
  });
});
