import { describe, it, expect, vi } from "vitest";
import { handleMaxSpendable } from "../../src/commands/max-spendable.js";
import { mockMaxSpendable } from "../fixtures/api-responses.js";

const mockGetMaxSpendable = vi.fn();
vi.mock("../../src/api/client.js", () => ({
  GatewayApiClient: vi.fn().mockImplementation(function () {
    return { getMaxSpendable: mockGetMaxSpendable };
  }),
}));

vi.mock("../../src/util/mempool.js", () => ({
  fetchFeeRate: vi.fn().mockResolvedValue(10),
}));

describe("handleMaxSpendable", () => {
  it("returns max spendable as JSON", async () => {
    mockGetMaxSpendable.mockResolvedValueOnce(mockMaxSpendable);
    const result = await handleMaxSpendable({
      apiUrl: "https://example.com",
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
    mockGetMaxSpendable.mockResolvedValueOnce(mockMaxSpendable);
    const result = await handleMaxSpendable({
      apiUrl: "https://example.com",
      address: "bc1qexample",
      json: false,
    });
    expect(result).toContain("Max spendable:");
    expect(result).toContain("BTC");
    expect(result).toContain("sat");
    expect(result).toContain("Fee rate:");
  });

  it("uses provided btcFeeRate instead of fetching", async () => {
    mockGetMaxSpendable.mockResolvedValueOnce(mockMaxSpendable);
    const result = await handleMaxSpendable({
      apiUrl: "https://example.com",
      address: "bc1qexample",
      btcFeeRate: 25,
      json: true,
    });
    const parsed = JSON.parse(result);
    expect(parsed.feeRateSatPerVbyte).toBe(25);
  });
});
