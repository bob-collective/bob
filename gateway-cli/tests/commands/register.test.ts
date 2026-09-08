import { describe, it, expect, vi, beforeEach } from "vitest";

const mockRegisterTxV3 = vi.fn().mockResolvedValue({ status: "ok" });
const mockGetOrder = vi.fn();

vi.mock("../../src/config.js", () => ({
  BTC_DECIMALS: 8,
  getSdk: vi.fn(() => ({ getOrder: mockGetOrder })),
  getApi: vi.fn(() => ({ registerTxV3: mockRegisterTxV3 })),
}));

const TXID = "a".repeat(64);
const RAW_TX = "0200000001" + "b".repeat(200);

const order = (srcChain: string, dstChain: string) => ({
  srcInfo: { chain: srcChain },
  dstInfo: { chain: dstChain },
});

beforeEach(() => {
  vi.clearAllMocks();
  mockRegisterTxV3.mockResolvedValue({ status: "ok" });
});

// `bitcoin_tx_hex` takes a full serialized tx, `bitcoin_txid` a txid — a txid in the
// hex field is rejected server-side, so the two forms must land in different fields.
describe("buildRegisterPayload", () => {
  it("puts a 64-hex-char txid in bitcoinTxid", async () => {
    const { buildRegisterPayload } = await import("../../src/chains/index.js");
    expect(buildRegisterPayload("bitcoin", "order-1", TXID)).toEqual({
      onramp: { orderId: "order-1", bitcoinTxid: TXID },
    });
  });

  it("puts a serialized transaction in bitcoinTxHex", async () => {
    const { buildRegisterPayload } = await import("../../src/chains/index.js");
    expect(buildRegisterPayload("bitcoin", "order-1", RAW_TX)).toEqual({
      onramp: { orderId: "order-1", bitcoinTxHex: RAW_TX },
    });
  });

  // bob-sdk 5.14.1 dropped registerTx for both EVM-source paths.
  it.each([
    ["offramp", "ethereum", "bitcoin"],
    ["tokenSwap", "ethereum", "base"],
  ])("rejects a %s order — EVM-source txs are not registered", async (_variant, src) => {
    const { buildRegisterPayload } = await import("../../src/chains/index.js");
    expect(() => buildRegisterPayload(src, "order-1", TXID)).toThrow(/nothing to register/i);
    expect(() => buildRegisterPayload(src, "order-1", TXID)).toThrow(new RegExp(src));
  });
});

describe("handleRegister", () => {
  it("registers a Bitcoin-source order", async () => {
    mockGetOrder.mockResolvedValue(order("bitcoin", "base"));

    const { handleRegister } = await import("../../src/commands/register.js");
    const result = await handleRegister({ orderId: "order-1", txid: TXID });

    expect(mockRegisterTxV3).toHaveBeenCalledWith({
      registerTxV3: { onramp: { orderId: "order-1", bitcoinTxid: TXID } },
    });
    expect(result).toEqual({ status: "ok" });
  });

  // Must fail before the request: a no-op call would report success on a stuck order.
  it.each([
    ["offramp", "ethereum", "bitcoin"],
    ["tokenSwap", "ethereum", "base"],
  ])("refuses a %s order without calling the API", async (_variant, src, dst) => {
    mockGetOrder.mockResolvedValue(order(src, dst));

    const { handleRegister } = await import("../../src/commands/register.js");
    await expect(handleRegister({ orderId: "order-1", txid: TXID })).rejects.toThrow(
      /nothing to register/i,
    );
    expect(mockRegisterTxV3).not.toHaveBeenCalled();
  });
});
