import { describe, it, expect, vi, beforeEach } from "vitest";
import { handleStatus } from "../../src/commands/status.js";

// ─── Mocks ───────────────────────────────────────────────────────────────────

const mockGetOrder = vi.fn();

vi.mock("../../src/config.js", () => ({
  getSdk: vi.fn(() => ({
    api: {
      getOrder: mockGetOrder,
    },
  })),
}));

// ─── Tests ───────────────────────────────────────────────────────────────────

describe("handleStatus", () => {
  beforeEach(() => {
    mockGetOrder.mockReset();
  });

  it("calls sdk.api.getOrder with the provided orderId", async () => {
    const fakeOrder = { id: "order-123", status: "pending" };
    mockGetOrder.mockResolvedValueOnce(fakeOrder);

    const result = await handleStatus({ orderId: "order-123" });

    expect(mockGetOrder).toHaveBeenCalledOnce();
    expect(mockGetOrder).toHaveBeenCalledWith({ id: "order-123" });
    expect(result).toEqual(fakeOrder);
  });

  it("propagates errors from getOrder", async () => {
    mockGetOrder.mockRejectedValueOnce(new Error("Order not found"));

    await expect(handleStatus({ orderId: "nonexistent" })).rejects.toThrow("Order not found");
  });
});
