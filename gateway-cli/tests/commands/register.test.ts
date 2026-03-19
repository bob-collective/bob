import { describe, it, expect, vi, beforeEach } from "vitest";
import { handleRegister } from "../../src/commands/register.js";

// ─── Mocks ───────────────────────────────────────────────────────────────────

const mockRegisterTx = vi.fn();
const mockGetOrder = vi.fn();

vi.mock("../../src/config.js", () => ({
  getSdk: vi.fn(() => ({
    api: {
      registerTx: mockRegisterTx,
      getOrder: mockGetOrder,
    },
  })),
}));

// ─── Tests ───────────────────────────────────────────────────────────────────

describe("handleRegister", () => {
  beforeEach(() => {
    mockRegisterTx.mockReset();
    mockGetOrder.mockReset();
  });

  it("registers onramp when order srcInfo.chain is bitcoin", async () => {
    mockGetOrder.mockResolvedValueOnce({
      srcInfo: { chain: "bitcoin" },
      dstInfo: { chain: "bob" },
    });
    mockRegisterTx.mockResolvedValueOnce({ success: true });

    await handleRegister({ orderId: "order-1", txid: "abc123" });

    expect(mockRegisterTx).toHaveBeenCalledWith({
      registerTx: { onramp: { orderId: "order-1", bitcoinTxHex: "abc123" } },
    });
  });

  it("registers offramp when one chain is bob", async () => {
    mockGetOrder.mockResolvedValueOnce({
      srcInfo: { chain: "bob" },
      dstInfo: { chain: "bitcoin" },
    });
    mockRegisterTx.mockResolvedValueOnce({ success: true });

    await handleRegister({ orderId: "order-2", txid: "0xdef" });

    expect(mockRegisterTx).toHaveBeenCalledWith({
      registerTx: { offramp: { orderId: "order-2", evmTxhash: "0xdef" } },
    });
  });

  it("registers layerZero when neither chain is bob or bitcoin", async () => {
    mockGetOrder.mockResolvedValueOnce({
      srcInfo: { chain: "ethereum" },
      dstInfo: { chain: "base" },
    });
    mockRegisterTx.mockResolvedValueOnce({ success: true });

    await handleRegister({ orderId: "order-3", txid: "0xabc" });

    expect(mockRegisterTx).toHaveBeenCalledWith({
      registerTx: { layerZero: { orderId: "order-3", evmTxhash: "0xabc" } },
    });
  });

  it("propagates errors from registerTx", async () => {
    mockGetOrder.mockResolvedValueOnce({
      srcInfo: { chain: "bob" },
      dstInfo: { chain: "bitcoin" },
    });
    mockRegisterTx.mockRejectedValueOnce(new Error("Registration failed"));

    await expect(
      handleRegister({ orderId: "order-1", txid: "0xfail" }),
    ).rejects.toThrow("Registration failed");
  });
});
