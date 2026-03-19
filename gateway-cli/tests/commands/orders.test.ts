import { describe, it, expect, vi, beforeEach } from "vitest";
import { handleOrders } from "../../src/commands/orders.js";

// ─── Mocks ───────────────────────────────────────────────────────────────────

const mockGetOrders = vi.fn();

vi.mock("../../src/config.js", () => ({
  getSdk: vi.fn(() => ({
    getOrders: mockGetOrders,
  })),
}));

// ─── Tests ───────────────────────────────────────────────────────────────────

describe("handleOrders", () => {
  beforeEach(() => {
    mockGetOrders.mockReset();
  });

  it("calls sdk.getOrders with the provided address", async () => {
    const fakeOrders = [{ id: "order-1" }, { id: "order-2" }];
    mockGetOrders.mockResolvedValueOnce(fakeOrders);

    const result = await handleOrders({ address: "0xDeadBeef" });

    expect(mockGetOrders).toHaveBeenCalledOnce();
    expect(mockGetOrders).toHaveBeenCalledWith("0xDeadBeef");
    expect(result).toEqual(fakeOrders);
  });

  it("returns empty array when no orders exist", async () => {
    mockGetOrders.mockResolvedValueOnce([]);

    const result = await handleOrders({ address: "0xEmpty" });

    expect(result).toEqual([]);
  });

  it("propagates errors from getOrders", async () => {
    mockGetOrders.mockRejectedValueOnce(new Error("Network error"));

    await expect(handleOrders({ address: "0xFail" })).rejects.toThrow("Network error");
  });
});
