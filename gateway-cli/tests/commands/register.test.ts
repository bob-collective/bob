import { describe, it, expect, vi } from "vitest";
import { handleRegister } from "../../src/commands/register.js";

const mockRegisterTx = vi.fn();
vi.mock("../../src/api/client.js", () => ({
  GatewayApiClient: vi.fn().mockImplementation(function () {
    return { registerTx: mockRegisterTx };
  }),
}));

describe("handleRegister", () => {
  it("registers a bitcoin txid", async () => {
    mockRegisterTx.mockResolvedValueOnce({ success: true });
    const result = await handleRegister({
      apiUrl: "https://example.com",
      orderId: "order-456",
      txid: "abc123",
      json: true,
    });
    expect(JSON.parse(result).success).toBe(true);
  });

  it("detects EVM tx hash by 0x prefix", async () => {
    mockRegisterTx.mockResolvedValueOnce({ success: true });
    await handleRegister({
      apiUrl: "https://example.com",
      orderId: "order-456",
      txid: "0xabc123",
      json: true,
    });
    expect(mockRegisterTx).toHaveBeenCalledWith(
      expect.objectContaining({ evmTxhash: "0xabc123" }),
    );
  });

  it("sends bitcoinTxid for non-0x txids", async () => {
    mockRegisterTx.mockResolvedValueOnce({ success: true });
    await handleRegister({
      apiUrl: "https://example.com",
      orderId: "order-456",
      txid: "deadbeef1234",
      json: true,
    });
    expect(mockRegisterTx).toHaveBeenCalledWith(
      expect.objectContaining({ bitcoinTxid: "deadbeef1234" }),
    );
  });
});
