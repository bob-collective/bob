import { describe, it, expect, vi, beforeEach } from "vitest";
import { handleRegister } from "../../src/commands/register.js";

// ─── Mocks ───────────────────────────────────────────────────────────────────

const mockRegisterTx = vi.fn();

vi.mock("../../src/config.js", () => ({
  getSdk: vi.fn(() => ({
    api: {
      registerTx: mockRegisterTx,
    },
  })),
}));

// ─── Tests ───────────────────────────────────────────────────────────────────

describe("handleRegister", () => {
  beforeEach(() => {
    mockRegisterTx.mockReset();
  });

  it("registers an EVM transaction (0x-prefixed) as offramp", async () => {
    const fakeResponse = { success: true };
    mockRegisterTx.mockResolvedValueOnce(fakeResponse);

    const evmTxid = "0xabc123def456789012345678901234567890123456789012345678901234567890";

    const result = await handleRegister({ orderId: "order-42", txid: evmTxid });

    expect(mockRegisterTx).toHaveBeenCalledOnce();
    expect(mockRegisterTx).toHaveBeenCalledWith({
      registerTx: {
        offramp: { orderId: "order-42", evmTxhash: evmTxid },
      },
    });
    expect(result).toEqual(fakeResponse);
  });

  it("registers a Bitcoin transaction (non-0x) as onramp", async () => {
    const fakeResponse = { success: true };
    mockRegisterTx.mockResolvedValueOnce(fakeResponse);

    const btcTxid = "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2";

    const result = await handleRegister({ orderId: "order-99", txid: btcTxid });

    expect(mockRegisterTx).toHaveBeenCalledOnce();
    expect(mockRegisterTx).toHaveBeenCalledWith({
      registerTx: {
        onramp: { orderId: "order-99", bitcoinTxid: btcTxid },
      },
    });
    expect(result).toEqual(fakeResponse);
  });

  it("propagates errors from registerTx", async () => {
    mockRegisterTx.mockRejectedValueOnce(new Error("Registration failed"));

    await expect(
      handleRegister({ orderId: "order-1", txid: "0xfail" }),
    ).rejects.toThrow("Registration failed");
  });
});
