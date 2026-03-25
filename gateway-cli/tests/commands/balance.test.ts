import { describe, it, expect, vi, beforeEach } from "vitest";
import { handleBalance } from "../../src/commands/balance.js";

const mockGetAllBalances = vi.fn();
const mockDeriveAddress = vi.fn();

vi.mock("../../src/chains/index.js", () => ({
  getAllBalances: (...args: any[]) => mockGetAllBalances(...args),
  deriveAddress: (...args: any[]) => mockDeriveAddress(...args),
  getChainFamily: vi.fn((chain: string) => chain === "bitcoin" ? "bitcoin" : "evm"),
}));

vi.mock("../../src/util/route-provider.js", () => ({
  getRoutes: vi.fn().mockResolvedValue([
    { srcChain: "bitcoin", dstChain: "base", srcToken: "BTC", dstToken: "0xUSDC" },
  ]),
  getUniqueChains: vi.fn(() => ["bitcoin", "base"]),
}));

vi.mock("../../src/util/input-resolver.js", () => ({
  resolveChain: vi.fn((c: string) => c),
}));

vi.mock("../../src/config.js", () => ({
  loadConfig: vi.fn(() => ({
    bitcoinPrivateKey: undefined,
    evmPrivateKey: undefined,
  })),
}));

vi.mock("viem", async (importOriginal) => {
  const actual = await importOriginal<any>();
  return { ...actual };
});

vi.mock("@gobob/bob-sdk", async (importOriginal) => {
  const actual = await importOriginal<any>();
  return { ...actual };
});

describe("handleBalance", () => {
  beforeEach(() => vi.clearAllMocks());

  it("returns error entry when chain RPC fails", async () => {
    mockGetAllBalances.mockResolvedValueOnce({
      bitcoin: { address: "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4", error: true },
    });

    const result = await handleBalance(["bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4"], { chain: ["bitcoin"] });
    expect(result.bitcoin.error).toBe(true);
  });

  it("classifies EVM address and filters to EVM chains", async () => {
    mockGetAllBalances.mockResolvedValueOnce({});

    await handleBalance(["0xAbCdEf0123456789AbCdEf0123456789AbCdEf01"], {});

    expect(mockGetAllBalances).toHaveBeenCalledWith(
      "0xAbCdEf0123456789AbCdEf0123456789AbCdEf01",
      expect.objectContaining({ chainFamily: "evm" }),
    );
  });

  it("classifies BTC address and filters to bitcoin chain", async () => {
    mockGetAllBalances.mockResolvedValueOnce({});

    await handleBalance(["bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4"], {});

    expect(mockGetAllBalances).toHaveBeenCalledWith(
      "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4",
      expect.objectContaining({ chainFamily: "bitcoin" }),
    );
  });

  it("throws for unsupported address format", async () => {
    await expect(handleBalance(["not-an-address"], {})).rejects.toThrow("Unsupported address format");
  });

  it("throws when no addresses and no keys configured", async () => {
    await expect(handleBalance([], {})).rejects.toThrow("No addresses provided");
  });
});
