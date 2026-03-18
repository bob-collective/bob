import { describe, it, expect, vi } from "vitest";
import { handleStatus } from "../../src/commands/status.js";

const mockGetOrder = vi.fn();
vi.mock("../../src/config/index.js", () => ({
  loadConfig: vi.fn().mockReturnValue({
    apiUrl: "https://example.com",
  }),
}));

vi.mock("../../src/adapter/sdk-client.js", () => ({
  createSdkClient: vi.fn().mockReturnValue({
    api: { getOrder: (...args: unknown[]) => mockGetOrder(...args) },
  }),
}));

// SDK-shaped order data
const sdkOrder = {
  srcInfo: { chain: "bitcoin", token: "BTC", amount: "5000000", txHash: null },
  dstInfo: { chain: "base", token: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913", amount: "4812300000", txHash: "0xabc" },
  status: "success",
  timestamp: 1710244800,
};

describe("handleStatus", () => {
  it("returns order info as JSON", async () => {
    mockGetOrder.mockResolvedValueOnce(sdkOrder);
    const result = await handleStatus({
      orderId: "order-456",
      json: true,
    });
    expect(JSON.parse(result).status).toBe("success");
  });

  it("calls sdk.api.getOrder with correct id", async () => {
    mockGetOrder.mockResolvedValueOnce(sdkOrder);
    await handleStatus({
      orderId: "order-456",
      json: true,
    });
    expect(mockGetOrder).toHaveBeenCalledWith({ id: "order-456" });
  });
});
