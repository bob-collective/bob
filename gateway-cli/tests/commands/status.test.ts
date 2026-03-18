import { describe, it, expect, vi } from "vitest";
import { handleStatus } from "../../src/commands/status.js";
import { mockOrder } from "../fixtures/api-responses.js";

const mockGetOrder = vi.fn();
vi.mock("../../src/api/client.js", () => ({
  GatewayApiClient: vi.fn().mockImplementation(function () {
    return { getOrder: mockGetOrder };
  }),
}));

describe("handleStatus", () => {
  it("returns order info as JSON", async () => {
    mockGetOrder.mockResolvedValueOnce(mockOrder);
    const result = await handleStatus({
      apiUrl: "https://example.com",
      orderId: "order-456",
      json: true,
    });
    expect(JSON.parse(result).status).toBe("success");
  });
});
