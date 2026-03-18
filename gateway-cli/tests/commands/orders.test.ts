import { describe, it, expect, vi } from "vitest";
import { handleOrders } from "../../src/commands/orders.js";
import { mockOrder } from "../fixtures/api-responses.js";

const mockGetOrders = vi.fn();
vi.mock("../../src/api/client.js", () => ({
  GatewayApiClient: vi.fn().mockImplementation(function () {
    return { getOrders: mockGetOrders };
  }),
}));

describe("handleOrders", () => {
  it("returns all orders as JSON", async () => {
    mockGetOrders.mockResolvedValueOnce([mockOrder]);
    const result = await handleOrders({
      apiUrl: "https://example.com",
      address: "0xABC",
      json: true,
    });
    expect(JSON.parse(result)).toHaveLength(1);
  });
});
