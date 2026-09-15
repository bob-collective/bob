import { describe, it, expect, vi, beforeEach } from "vitest";

const mockRegisterTxV4 = vi.fn().mockResolvedValue({ status: "ok" });
const mockGetOrder = vi.fn();

vi.mock("../../src/config.js", () => ({
  BTC_DECIMALS: 8,
  getSdk: vi.fn(() => ({ getOrder: mockGetOrder })),
  getApi: vi.fn(() => ({ registerTxV4: mockRegisterTxV4 })),
}));

const TXID = "a".repeat(64);
const RAW_TX = "0200000001" + "b".repeat(200);

const order = (srcChain: string, dstChain: string) => ({
  srcInfo: { chain: srcChain },
  dstInfo: { chain: dstChain },
});

beforeEach(() => {
  vi.clearAllMocks();
  mockRegisterTxV4.mockResolvedValue({ status: "ok" });
});

// The v4 endpoint carries only `bitcoin_tx_hex` — v3's `bitcoin_txid` alternative is
// gone. A txid must therefore be refused locally: the generated client drops unknown
// fields, so sending one would reach the gateway as an absent tx and come back as an
// opaque 4xx naming nothing the operator typed.
describe("buildRegisterPayload", () => {
  it("refuses a 64-hex-char txid — v4 registers by raw hex only", async () => {
    const { buildRegisterPayload } = await import("../../src/chains/index.js");
    expect(() => buildRegisterPayload("bitcoin", "order-1", TXID)).toThrow(/txid/i);
    expect(() => buildRegisterPayload("bitcoin", "order-1", TXID)).toThrow(/raw hex/i);
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
    const result = await handleRegister({ orderId: "order-1", txid: RAW_TX });

    expect(mockRegisterTxV4).toHaveBeenCalledWith({
      registerTxV4: { onramp: { orderId: "order-1", bitcoinTxHex: RAW_TX } },
    });
    expect(result).toEqual({ status: "ok" });
  });

  // The local refusal must also short-circuit the request, for the same reason the
  // EVM-source refusal does: a dropped field would be reported as a successful register.
  it("refuses a bare txid without calling the API", async () => {
    mockGetOrder.mockResolvedValue(order("bitcoin", "base"));

    const { handleRegister } = await import("../../src/commands/register.js");
    await expect(handleRegister({ orderId: "order-1", txid: TXID })).rejects.toThrow(/raw hex/i);
    expect(mockRegisterTxV4).not.toHaveBeenCalled();
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
    expect(mockRegisterTxV4).not.toHaveBeenCalled();
  });
});
