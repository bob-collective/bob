import { describe, it, expect, vi, beforeEach } from "vitest";

// ─── Mock fixtures ──────────────────────────────────────────────────────────

const mockGetBalance = vi.fn();
const mockEstimateFeesPerGas = vi.fn();
const mockReadContract = vi.fn();

vi.mock("viem", async (importOriginal) => {
  const actual = await importOriginal<typeof import("viem")>();
  return {
    ...actual,
    createPublicClient: vi.fn(() => ({
      getBalance: mockGetBalance,
      estimateFeesPerGas: mockEstimateFeesPerGas,
      readContract: mockReadContract,
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

vi.mock("@gobob/bob-sdk", () => ({
  supportedChainsMapping: { base: { id: 8453, name: "base" }, ethereum: { id: 1, name: "ethereum" } },
}));

vi.mock("../../src/util/route-provider.js", () => ({
  getNativeToken: vi.fn((chain: string) => {
    if (chain === "base") return { symbol: "ETH", decimals: 18 };
    if (chain === "ethereum") return { symbol: "ETH", decimals: 18 };
    return { symbol: "ETH", decimals: 18 };
  }),
}));

// ─── Import after mocks ─────────────────────────────────────────────────────

import {
  getEvmNativeBalance,
  getEvmTokenBalance,
  deriveEvmAddress,
  resolveEvmSigner,
  isNativeToken,
  NATIVE_GAS_BUFFER,
} from "../../src/chains/evm.js";

// ─── Tests ──────────────────────────────────────────────────────────────────

describe("getEvmNativeBalance", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("returns total and allSpendable with gas buffer deduction", async () => {
    const balance = 1000000000000000000n; // 1 ETH
    const maxFeePerGas = 50000000000n; // 50 gwei

    mockGetBalance.mockResolvedValue(balance);
    mockEstimateFeesPerGas.mockResolvedValue({ maxFeePerGas, gasPrice: 30000000000n });

    const result = await getEvmNativeBalance("base", "0xTestAddress");

    const expectedGasCost = maxFeePerGas * NATIVE_GAS_BUFFER;
    const expectedSpendable = balance - expectedGasCost;

    expect(result.total).toBe(balance.toString());
    expect(result.allSpendable).toBe(expectedSpendable.toString());
  });

  it("returns zero allSpendable when balance is less than gas cost", async () => {
    const balance = 100n; // tiny balance
    const maxFeePerGas = 50000000000n;

    mockGetBalance.mockResolvedValue(balance);
    mockEstimateFeesPerGas.mockResolvedValue({ maxFeePerGas, gasPrice: 30000000000n });

    const result = await getEvmNativeBalance("base", "0xTestAddress");

    expect(result.total).toBe("100");
    expect(result.allSpendable).toBe("0");
  });

  it("falls back to gasPrice when maxFeePerGas is null", async () => {
    const balance = 1000000000000000000n;
    const gasPrice = 30000000000n;

    mockGetBalance.mockResolvedValue(balance);
    mockEstimateFeesPerGas.mockResolvedValue({ maxFeePerGas: null, gasPrice });

    const result = await getEvmNativeBalance("base", "0xTestAddress");

    const expectedGasCost = gasPrice * NATIVE_GAS_BUFFER;
    const expectedSpendable = balance - expectedGasCost;

    expect(result.total).toBe(balance.toString());
    expect(result.allSpendable).toBe(expectedSpendable.toString());
  });
});

describe("getEvmTokenBalance", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("returns total and allSpendable equal when no fee token", async () => {
    mockReadContract.mockResolvedValue(5000000n); // 5 USDC

    const result = await getEvmTokenBalance("base", "0xTestAddress", "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913", 6);

    expect(result.total).toBe("5000000");
    expect(result.allSpendable).toBe("5000000");
  });

  it("deducts fee reserve when fee token matches token address", async () => {
    mockReadContract.mockResolvedValue(5000000n);

    const result = await getEvmTokenBalance(
      "base", "0xTestAddress", "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913", 6,
      "0x833589fcd6edb6e08f4c7c32d4f71b54bda02913", // fee token (different case)
      "1000000", // fee reserve: 1 USDC
    );

    expect(result.total).toBe("5000000");
    expect(result.allSpendable).toBe("4000000");
  });

  it("does not deduct when fee token does not match", async () => {
    mockReadContract.mockResolvedValue(5000000n);

    const result = await getEvmTokenBalance(
      "base", "0xTestAddress", "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913", 6,
      "0x50c5725949A6F0c72E6C4a641F24049A917DB0Cb", // different token (DAI)
      "1000000",
    );

    expect(result.total).toBe("5000000");
    expect(result.allSpendable).toBe("5000000");
  });

  it("returns zero allSpendable when fee reserve exceeds balance", async () => {
    mockReadContract.mockResolvedValue(500000n);

    const result = await getEvmTokenBalance(
      "base", "0xTestAddress", "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913", 6,
      "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913",
      "1000000", // reserve > balance
    );

    expect(result.total).toBe("500000");
    expect(result.allSpendable).toBe("0");
  });
});

describe("deriveEvmAddress", () => {
  it("returns a 0x address", () => {
    const address = deriveEvmAddress("abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890");

    expect(address).toMatch(/^0x/);
    expect(typeof address).toBe("string");
  });

  it("handles key with 0x prefix", () => {
    const address = deriveEvmAddress("0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890");

    expect(address).toMatch(/^0x/);
  });
});

describe("resolveEvmSigner", () => {
  it("returns walletClient, publicClient, and address", () => {
    const result = resolveEvmSigner("abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890", "base");

    expect(result.address).toMatch(/^0x/);
    expect(result.walletClient).toBeDefined();
    expect(result.publicClient).toBeDefined();
  });
});

describe("isNativeToken", () => {
  it("returns true for native token", () => {
    expect(isNativeToken("base", "ETH")).toBe(true);
    expect(isNativeToken("base", "eth")).toBe(true);
  });

  it("returns false for non-native token", () => {
    expect(isNativeToken("base", "USDC")).toBe(false);
  });
});
