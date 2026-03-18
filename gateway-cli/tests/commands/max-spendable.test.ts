import { describe, it, expect, vi } from "vitest";
import { handleMaxSpendable } from "../../src/commands/max-spendable.js";

const mockGetMaxSpendable = vi.fn();
vi.mock("../../src/config/index.js", () => ({
  loadConfig: vi.fn().mockReturnValue({
    apiUrl: "https://example.com",
  }),
}));

vi.mock("../../src/adapter/sdk-client.js", () => ({
  createSdkClient: vi.fn().mockReturnValue({
    getMaxSpendable: (...args: unknown[]) => mockGetMaxSpendable(...args),
  }),
}));

vi.mock("../../src/util/mempool.js", () => ({
  fetchFeeRate: vi.fn().mockResolvedValue(10),
}));

// SDK-shaped response: amount is a GatewayTokenAmount object
const sdkMaxSpendable = {
  userAddress: "bc1qexample",
  amount: { address: "BTC", amount: "10000000", chain: "bitcoin" },
};

describe("handleMaxSpendable", () => {
  it("returns max spendable as JSON", async () => {
    mockGetMaxSpendable.mockResolvedValueOnce(sdkMaxSpendable);
    const result = await handleMaxSpendable({
      address: "bc1qexample",
      json: true,
    });
    const parsed = JSON.parse(result);
    expect(parsed.maxSpendableSat).toBe("10000000");
    expect(parsed.asset).toBe("BTC");
    expect(parsed.chain).toBe("bitcoin");
    expect(parsed.feeRateSatPerVbyte).toBe(10);
  });

  it("returns max spendable as human readable text", async () => {
    mockGetMaxSpendable.mockResolvedValueOnce(sdkMaxSpendable);
    const result = await handleMaxSpendable({
      address: "bc1qexample",
      json: false,
    });
    expect(result).toContain("Max spendable:");
    expect(result).toContain("BTC");
    expect(result).toContain("sat");
    expect(result).toContain("Fee rate:");
  });

  it("uses provided btcFeeRate instead of fetching", async () => {
    mockGetMaxSpendable.mockResolvedValueOnce(sdkMaxSpendable);
    const result = await handleMaxSpendable({
      address: "bc1qexample",
      btcFeeRate: 25,
      json: true,
    });
    const parsed = JSON.parse(result);
    expect(parsed.feeRateSatPerVbyte).toBe(25);
  });
});
