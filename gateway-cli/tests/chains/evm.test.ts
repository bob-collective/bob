import { describe, it, expect, vi, beforeEach } from "vitest";

// ─── Mock fixtures ──────────────────────────────────────────────────────────

const mockGetBalance = vi.fn();
const mockEstimateFeesPerGas = vi.fn();
const mockMulticall = vi.fn();

vi.mock("viem", async (importOriginal) => {
  const actual = await importOriginal<typeof import("viem")>();
  return {
    ...actual,
    createPublicClient: vi.fn(() => ({
      getBalance: mockGetBalance,
      estimateFeesPerGas: mockEstimateFeesPerGas,
      multicall: mockMulticall,
    })),
    createWalletClient: vi.fn(() => ({
      account: { address: "0xTestAddress" },
      sendTransaction: vi.fn(),
    })),
    privateKeyToAccount: vi.fn(() => ({
      address: "0xTestAddress",
    })),
    http: vi.fn(),
  };
});

vi.mock("viem/accounts", () => ({
  privateKeyToAccount: vi.fn(() => ({
    address: "0xAbCdEf0123456789AbCdEf0123456789AbCdEf01",
  })),
}));

vi.mock("@gobob/bob-sdk", async (importOriginal) => {
  const actual = await importOriginal<typeof import("@gobob/bob-sdk")>();
  return {
    ...actual,
    supportedChainsMapping: {
      base: { id: 8453, name: "base", nativeCurrency: { symbol: "ETH", decimals: 18, name: "Ether" } },
      ethereum: { id: 1, name: "ethereum", nativeCurrency: { symbol: "ETH", decimals: 18, name: "Ether" } },
    },
    getChainConfig: vi.fn((chain: string) => {
      const chains: Record<string, any> = {
        base: { id: 8453, name: "base", nativeCurrency: { symbol: "ETH", decimals: 18, name: "Ether" } },
        ethereum: { id: 1, name: "ethereum", nativeCurrency: { symbol: "ETH", decimals: 18, name: "Ether" } },
      };
      return chains[chain];
    }),
  };
});

vi.mock("@gobob/tokenlist/tokenlist.json", () => ({
  default: { tokens: [] },
}));

vi.mock("../../src/util/rpc-resolver.js", () => ({
  resolveRpcUrl: vi.fn().mockResolvedValue(undefined),
}));

// ─── Import after mocks ─────────────────────────────────────────────────────

import { getEvmBalances, NATIVE_GAS_BUFFER } from "../../src/chains/evm.js";

// ─── Tests ──────────────────────────────────────────────────────────────────

describe("getEvmBalances", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  // ─── Native only (includeNative: true, no tokens) ──────────────────────

  it("native only: returns gas buffer deduction", async () => {
    const balance = 1000000000000000000n; // 1 ETH
    const maxFeePerGas = 50000000000n; // 50 gwei

    mockGetBalance.mockResolvedValue(balance);
    mockEstimateFeesPerGas.mockResolvedValue({ maxFeePerGas, gasPrice: 30000000000n });

    const result = await getEvmBalances("base", "0xTestAddress", [], { includeNative: true });

    const expectedGasCost = maxFeePerGas * NATIVE_GAS_BUFFER;
    const expectedSpendable = balance - expectedGasCost;

    expect(result.native!.balance).toBe(balance.toString());
    expect(result.native!.allSpendable).toBe(expectedSpendable.toString());
    expect(result.tokens).toBeUndefined();
    expect(mockMulticall).not.toHaveBeenCalled();
  });

  it("native only: zero allSpendable when balance < gas cost", async () => {
    mockGetBalance.mockResolvedValue(100n);
    mockEstimateFeesPerGas.mockResolvedValue({ maxFeePerGas: 50000000000n, gasPrice: 30000000000n });

    const result = await getEvmBalances("base", "0xTestAddress", [], { includeNative: true });

    expect(result.native!.balance).toBe("100");
    expect(result.native!.allSpendable).toBe("0");
  });

  it("native only: falls back to gasPrice when maxFeePerGas is null", async () => {
    const balance = 1000000000000000000n;
    const gasPrice = 30000000000n;

    mockGetBalance.mockResolvedValue(balance);
    mockEstimateFeesPerGas.mockResolvedValue({ maxFeePerGas: null, gasPrice });

    const result = await getEvmBalances("base", "0xTestAddress", [], { includeNative: true });

    const expectedGasCost = gasPrice * NATIVE_GAS_BUFFER;
    expect(result.native!.allSpendable).toBe((balance - expectedGasCost).toString());
  });

  // ─── Tokens only (no includeNative) ────────────────────────────────────

  it("tokens only: returns balances via multicall", async () => {
    mockMulticall.mockResolvedValue([{ status: 'success', result: 5000000n }]);

    const tokens = [{ address: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913", symbol: "USDC", decimals: 6 }];
    const result = await getEvmBalances("base", "0xTestAddress", tokens);

    expect(result.native).toBeUndefined();
    expect(result.tokens).toHaveLength(1);
    expect(result.tokens![0].balance).toBe("5000000");
    expect(result.tokens![0].allSpendable).toBe("5000000");
    expect(mockGetBalance).not.toHaveBeenCalled();
  });

  it("tokens only: applies fee reserve deduction", async () => {
    mockMulticall.mockResolvedValue([{ status: 'success', result: 5000000n }]);

    const tokens = [{ address: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913", symbol: "USDC", decimals: 6 }];
    const result = await getEvmBalances("base", "0xTestAddress", tokens, {
      feeToken: "0x833589fcd6edb6e08f4c7c32d4f71b54bda02913",
      feeReserve: "1000000",
    });

    expect(result.tokens![0].balance).toBe("5000000");
    expect(result.tokens![0].allSpendable).toBe("4000000");
  });

  it("tokens only: no deduction when fee token does not match", async () => {
    mockMulticall.mockResolvedValue([{ status: 'success', result: 5000000n }]);

    const tokens = [{ address: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913", symbol: "USDC", decimals: 6 }];
    const result = await getEvmBalances("base", "0xTestAddress", tokens, {
      feeToken: "0x50c5725949A6F0c72E6C4a641F24049A917DB0Cb",
      feeReserve: "1000000",
    });

    expect(result.tokens![0].allSpendable).toBe("5000000");
  });

  it("tokens only: zero allSpendable when fee reserve exceeds balance", async () => {
    mockMulticall.mockResolvedValue([{ status: 'success', result: 500000n }]);

    const tokens = [{ address: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913", symbol: "USDC", decimals: 6 }];
    const result = await getEvmBalances("base", "0xTestAddress", tokens, {
      feeToken: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913",
      feeReserve: "1000000",
    });

    expect(result.tokens![0].allSpendable).toBe("0");
  });
});
