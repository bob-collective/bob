import { describe, it, expect, vi } from "vitest";
import { handleOrders } from "../../src/commands/orders.js";

const mockGetOrders = vi.fn();
vi.mock("../../src/config/index.js", () => ({
  loadConfig: vi.fn().mockReturnValue({
    apiUrl: "https://example.com",
  }),
}));

vi.mock("../../src/adapter/sdk-client.js", () => ({
  createSdkClient: vi.fn().mockReturnValue({
    getOrders: (...args: unknown[]) => mockGetOrders(...args),
  }),
}));

// SDK-shaped order data
const sdkOrder = {
  srcInfo: { chain: "bitcoin", token: "BTC", amount: "5000000", txHash: null },
  dstInfo: { chain: "base", token: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913", amount: "4812300000", txHash: "0xabc" },
  status: "success",
  timestamp: 1710244800,
};

describe("handleOrders", () => {
  it("returns all orders as JSON", async () => {
    mockGetOrders.mockResolvedValueOnce([sdkOrder]);
    const result = await handleOrders({
      address: "0xABC",
      json: true,
    });
    expect(JSON.parse(result)).toHaveLength(1);
  });
});
