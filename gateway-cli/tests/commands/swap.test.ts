import { describe, it, expect, vi, beforeEach } from "vitest";
import { handleSwap } from "../../src/commands/swap.js";
import {
  mockQuote,
  mockCreateOrderResponse,
  mockOrder,
  mockRoutes,
} from "../fixtures/api-responses.js";

const mockGetQuote = vi.fn();
const mockCreateOrder = vi.fn();
const mockRegisterTx = vi.fn();
const mockGetOrder = vi.fn();
const mockGetRoutes = vi.fn();

vi.mock("../../src/api/client.js", () => ({
  GatewayApiClient: vi.fn().mockImplementation(function () {
    return {
      getQuote: mockGetQuote,
      createOrder: mockCreateOrder,
      registerTx: mockRegisterTx,
      getOrder: mockGetOrder,
      getRoutes: mockGetRoutes,
    };
  }),
}));

const mockSign = vi.fn();
vi.mock("../../src/signer/btc.js", async (importOriginal) => {
  const actual = await importOriginal<typeof import("../../src/signer/btc.js")>();
  return {
    ...actual,
    ExternalSigner: vi.fn().mockImplementation(function () {
      return { sign: mockSign };
    }),
    signBtcWithSpec: vi.fn().mockImplementation(async (_spec: unknown, psbtBase64: string) => {
      return mockSign(psbtBase64);
    }),
  };
});

vi.mock("../../src/util/mempool.js", () => ({
  fetchFeeRate: vi.fn().mockResolvedValue(10),
  findPendingMempoolTx: vi.fn().mockResolvedValue(null),
}));

describe("handleSwap (onramp: BTC -> EVM)", () => {
  beforeEach(() => vi.clearAllMocks());

  const baseOpts = {
    apiUrl: "https://example.com",
    src: "BTC",
    dst: "USDC:base",
    amount: "5000000sat",
    recipient: "0xABC",
    slippageBps: 300,
    bitcoinSignerCmd: "test-signer",
    autoConfirm: true,
    unsigned: false,
    noWait: false,
    timeoutMs: 5000,
    json: true,
  };

  it("executes full onramp flow", async () => {
    mockGetRoutes.mockResolvedValueOnce(mockRoutes);
    mockGetQuote.mockResolvedValueOnce(mockQuote);
    mockCreateOrder.mockResolvedValueOnce(mockCreateOrderResponse);
    mockSign.mockResolvedValueOnce("signed-tx-hex");
    mockRegisterTx.mockResolvedValueOnce({ success: true });
    mockGetOrder.mockResolvedValueOnce(mockOrder);

    const result = await handleSwap(baseOpts);

    expect(mockGetQuote).toHaveBeenCalledTimes(1);
    expect(mockCreateOrder).toHaveBeenCalledTimes(1);
    expect(mockSign).toHaveBeenCalledWith(mockCreateOrderResponse.psbtBase64);
    expect(mockRegisterTx).toHaveBeenCalledTimes(1);
    const parsed = JSON.parse(result);
    expect(parsed.status).toBe("confirmed");
    expect(parsed.orderId).toBe("order-456");
  });

  it("returns submitted info with --no-wait", async () => {
    mockGetRoutes.mockResolvedValueOnce(mockRoutes);
    mockGetQuote.mockResolvedValueOnce(mockQuote);
    mockCreateOrder.mockResolvedValueOnce(mockCreateOrderResponse);
    mockSign.mockResolvedValueOnce("signed-tx-hex");
    mockRegisterTx.mockResolvedValueOnce({ success: true });

    const result = await handleSwap({ ...baseOpts, noWait: true });

    const parsed = JSON.parse(result);
    expect(parsed.orderId).toBe("order-456");
    expect(parsed.status).toBe("submitted");
    expect(mockGetOrder).not.toHaveBeenCalled();
  });

  it("throws when no BTC signer is configured", async () => {
    mockGetRoutes.mockResolvedValueOnce(mockRoutes);
    await expect(
      handleSwap({
        ...baseOpts,
        bitcoinSignerCmd: undefined,
        unsigned: false,
      }),
    ).rejects.toThrow("no signer configured for Bitcoin");
  });

  it("retries register-tx on failure", async () => {
    // Mock retryWithBackoff to avoid 30s delays
    const retryMod = await import("../../src/util/retry.js");
    const retrySpy = vi.spyOn(retryMod, "retryWithBackoff").mockImplementation(
      async (fn) => {
        // Simulate 2 failures then success
        try { await fn(); } catch {}
        try { await fn(); } catch {}
        try {
          const value = await fn();
          return { value };
        } catch (err) {
          return { error: err };
        }
      },
    );

    mockGetRoutes.mockResolvedValueOnce(mockRoutes);
    mockGetQuote.mockResolvedValueOnce(mockQuote);
    mockCreateOrder.mockResolvedValueOnce(mockCreateOrderResponse);
    mockSign.mockResolvedValueOnce("signed-tx-hex");
    mockRegisterTx
      .mockRejectedValueOnce(new Error("network"))
      .mockRejectedValueOnce(new Error("network"))
      .mockResolvedValueOnce({ success: true });
    mockGetOrder.mockResolvedValueOnce(mockOrder);

    const result = await handleSwap(baseOpts);

    expect(mockRegisterTx).toHaveBeenCalledTimes(3);
    const parsed = JSON.parse(result);
    expect(parsed.status).toBe("confirmed");

    retrySpy.mockRestore();
  });

  it("outputs unsigned PSBT with --unsigned", async () => {
    mockGetRoutes.mockResolvedValueOnce(mockRoutes);
    mockGetQuote.mockResolvedValueOnce(mockQuote);
    mockCreateOrder.mockResolvedValueOnce(mockCreateOrderResponse);

    const result = await handleSwap({
      ...baseOpts,
      unsigned: true,
      bitcoinSignerCmd: undefined,
    });

    const parsed = JSON.parse(result);
    expect(parsed.psbtBase64).toBe(mockCreateOrderResponse.psbtBase64);
    expect(parsed.orderId).toBe("order-456");
    expect(mockSign).not.toHaveBeenCalled();
  });

  it("throws on registration failure when json=false", async () => {
    // Mock retryWithBackoff to return error immediately
    const retryMod = await import("../../src/util/retry.js");
    const retrySpy = vi.spyOn(retryMod, "retryWithBackoff").mockResolvedValue({
      error: new Error("registration failed"),
    });

    mockGetRoutes.mockResolvedValueOnce(mockRoutes);
    mockGetQuote.mockResolvedValueOnce(mockQuote);
    mockCreateOrder.mockResolvedValueOnce(mockCreateOrderResponse);
    mockSign.mockResolvedValueOnce("signed-tx-hex");

    await expect(handleSwap({ ...baseOpts, json: false })).rejects.toThrow(
      "CRITICAL",
    );

    retrySpy.mockRestore();
  });

  it("returns error JSON on registration failure when json=true", async () => {
    const retryMod = await import("../../src/util/retry.js");
    const retrySpy = vi.spyOn(retryMod, "retryWithBackoff").mockResolvedValue({
      error: new Error("registration failed"),
    });

    mockGetRoutes.mockResolvedValueOnce(mockRoutes);
    mockGetQuote.mockResolvedValueOnce(mockQuote);
    mockCreateOrder.mockResolvedValueOnce(mockCreateOrderResponse);
    mockSign.mockResolvedValueOnce("signed-tx-hex");

    const result = await handleSwap({ ...baseOpts, json: true });
    const parsed = JSON.parse(result);
    expect(parsed.error.code).toBe(3);
    expect(parsed.error.orderId).toBe("order-456");

    retrySpy.mockRestore();
  });
});
