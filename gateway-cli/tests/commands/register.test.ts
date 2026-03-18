import { describe, it, expect, vi } from "vitest";
import { handleRegister } from "../../src/commands/register.js";

const mockRegisterTx = vi.fn();
vi.mock("../../src/config/index.js", () => ({
  loadConfig: vi.fn().mockReturnValue({
    apiUrl: "https://example.com",
  }),
}));

vi.mock("../../src/adapter/sdk-client.js", () => ({
  createSdkClient: vi.fn().mockReturnValue({
    api: { registerTx: (...args: unknown[]) => mockRegisterTx(...args) },
  }),
}));

describe("handleRegister", () => {
  it("registers a bitcoin txid", async () => {
    mockRegisterTx.mockResolvedValueOnce({ onramp: { txid: "abc123" } });
    const result = await handleRegister({
      orderId: "order-456",
      txid: "abc123",
      json: true,
    });
    const parsed = JSON.parse(result);
    expect(parsed).toHaveProperty("onramp");
  });

  it("detects EVM tx hash by 0x prefix", async () => {
    mockRegisterTx.mockResolvedValueOnce("ok");
    await handleRegister({
      orderId: "order-456",
      txid: "0xabc123",
      json: true,
    });
    expect(mockRegisterTx).toHaveBeenCalledWith({
      registerTx: { offramp: { orderId: "order-456", evmTxhash: "0xabc123" } },
    });
  });

  it("sends bitcoinTxid for non-0x txids", async () => {
    mockRegisterTx.mockResolvedValueOnce({ onramp: { txid: "deadbeef1234" } });
    await handleRegister({
      orderId: "order-456",
      txid: "deadbeef1234",
      json: true,
    });
    expect(mockRegisterTx).toHaveBeenCalledWith({
      registerTx: { onramp: { orderId: "order-456", bitcoinTxid: "deadbeef1234" } },
    });
  });
});
