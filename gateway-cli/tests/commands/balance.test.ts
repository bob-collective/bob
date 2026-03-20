import { describe, it, expect, vi, beforeEach } from "vitest";
import { handleBalance } from "../../src/commands/balance.js";

const mockGetAllBalances = vi.fn();

vi.mock("../../src/chains/index.js", () => ({
  getAllBalances: (...args: any[]) => mockGetAllBalances(...args),
}));

describe("handleBalance", () => {
  beforeEach(() => vi.clearAllMocks());

  it("returns error entry when chain RPC fails", async () => {
    mockGetAllBalances.mockResolvedValueOnce({
      bitcoin: { address: "bc1qtest", error: true },
    });

    const result = await handleBalance("bc1qtest", { chain: "bitcoin" });
    expect(result.bitcoin.error).toBe(true);
  });

  it("passes feeToken and feeReserve to getAllBalances", async () => {
    mockGetAllBalances.mockResolvedValueOnce({});

    await handleBalance("0xTest", { chain: "base", feeToken: "0xUSDC", feeReserve: "1000000" });

    expect(mockGetAllBalances).toHaveBeenCalledWith("0xTest", {
      chain: "base",
      feeToken: "0xUSDC",
      feeReserve: "1000000",
    });
  });
});
