import { describe, it, expect, vi, beforeEach } from "vitest";
import { handleSwap } from "../../src/commands/swap.js";

// ─── SDK mock ────────────────────────────────────────────────────────────────

const mockGetQuote = vi.fn();
const mockGetRoutes = vi.fn();
const mockGetOrder = vi.fn();
const mockCreateOrder = vi.fn();
const mockRegisterTx = vi.fn();

vi.mock("../../src/adapter/sdk-client.js", () => ({
  createSdkClient: vi.fn(() => ({
    getQuote: mockGetQuote,
    getRoutes: mockGetRoutes,
    getOrder: mockGetOrder,
    api: {
      createOrder: mockCreateOrder,
      registerTx: mockRegisterTx,
    },
  })),
}));

// ─── Route cache mock ────────────────────────────────────────────────────────

vi.mock("../../src/util/route-cache.js", () => ({
  getOrFetchRoutes: vi.fn(async (fetcher: () => Promise<any>) => fetcher()),
}));

// ─── Route enricher mock (pass-through) ──────────────────────────────────────

vi.mock("../../src/adapter/route-enricher.js", () => ({
  enrichRoutes: vi.fn((routes: any) => routes),
}));

// ─── Quote flattener mock ────────────────────────────────────────────────────

vi.mock("../../src/adapter/quote-flattener.js", () => ({
  flattenQuote: vi.fn((quote: any) => ({
    variant: quote.onramp ? "onramp" : "offramp",
    inputAmount: quote.onramp?.inputAmount?.amount ?? quote.offramp?.inputAmount?.amount ?? "0",
    outputAmount: quote.onramp?.outputAmount?.amount ?? quote.offramp?.outputAmount?.amount ?? "0",
    fees: "0",
    raw: quote,
  })),
  detectVariant: vi.fn((quote: any) => {
    if (quote.onramp) return "onramp";
    if (quote.offramp) return "offramp";
    if (quote.layerZero) return "layerZero";
    throw new Error("Unknown quote variant");
  }),
}));

// ─── Config mock ─────────────────────────────────────────────────────────────

vi.mock("../../src/config/index.js", () => ({
  loadConfig: vi.fn(() => ({
    apiUrl: "https://example.com",
    cache: { ttl: "24h" },
    slippageBps: 300,
    timeoutMs: 1800000,
    autoConfirm: false,
    noWait: false,
    rpc: {},
  })),
}));

// ─── BTC signer mock ────────────────────────────────────────────────────────

const mockSign = vi.fn();
vi.mock("../../src/signer/btc.js", async (importOriginal) => {
  const actual = await importOriginal<typeof import("../../src/signer/btc.js")>();
  return {
    ...actual,
    ExternalSigner: vi.fn().mockImplementation(function () {
      return { sign: mockSign };
    }),
    signBtcWithResult: vi.fn().mockImplementation(async (_result: unknown, psbtBase64: string) => {
      return mockSign(psbtBase64);
    }),
  };
});

// ─── Mempool mock ────────────────────────────────────────────────────────────

vi.mock("../../src/util/mempool.js", () => ({
  fetchFeeRate: vi.fn().mockResolvedValue(10),
  findPendingMempoolTx: vi.fn().mockResolvedValue(null),
}));

// ─── Fixtures ────────────────────────────────────────────────────────────────

const onrampRoutes = [
  {
    srcChain: "bitcoin",
    dstChain: "base",
    srcToken: {
      address: "0x0000000000000000000000000000000000000000",
      symbol: "BTC",
      decimals: 8,
      chain: "bitcoin",
    },
    dstToken: {
      address: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913",
      symbol: "USDC",
      decimals: 6,
      chain: "base",
    },
  },
];

// SDK-style onramp quote
const onrampQuote = {
  onramp: {
    inputAmount: { amount: "5000000", address: "BTC", chain: "bitcoin" },
    outputAmount: { amount: "4812300000", address: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913", chain: "base" },
    fees: { amount: "15000", address: "BTC", chain: "bitcoin" },
    signedQuoteData: "signed-data",
    slippage: "300",
    dstChain: "base",
    dstToken: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913",
    token: "BTC",
    recipient: "0xABC",
    executionFees: { amount: "1000", address: "BTC", chain: "bitcoin" },
    feeBreakdown: {
      solverFee: { amount: "12000", address: "BTC", chain: "bitcoin" },
      protocolFee: { amount: "3000", address: "BTC", chain: "bitcoin" },
    },
  },
};

// SDK-style onramp order
const onrampOrder = {
  onramp: {
    orderId: "order-456",
    address: "bc1qgateway",
    psbtHex: "70736274ff",
  },
};

// SDK-style order info (from getOrder)
const orderInfo = {
  id: "order-456",
  status: "success",
  srcInfo: { amount: "5000000", chain: "bitcoin", token: "BTC" },
  dstInfo: { amount: "4812300000", chain: "base", token: "USDC" },
  timestamp: 1710230400,
};

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
    dryRun: false,
    timeoutMs: 5000,
    json: true,
  };

  it("executes full onramp flow", async () => {
    mockGetRoutes.mockResolvedValueOnce(onrampRoutes);
    mockGetQuote.mockResolvedValue(onrampQuote);
    mockCreateOrder.mockResolvedValueOnce(onrampOrder);
    mockSign.mockResolvedValueOnce("signed-tx-hex");
    mockRegisterTx.mockResolvedValueOnce({ onramp: { txid: "txid-123" } });
    mockGetOrder.mockResolvedValueOnce(orderInfo);

    const result = await handleSwap(baseOpts);

    expect(mockGetQuote).toHaveBeenCalledTimes(2);
    expect(mockCreateOrder).toHaveBeenCalledTimes(1);
    expect(mockSign).toHaveBeenCalled();
    expect(mockRegisterTx).toHaveBeenCalledTimes(1);
    const parsed = JSON.parse(result);
    expect(parsed.status).toBe("confirmed");
    expect(parsed.orderId).toBe("order-456");
  });

  it("returns submitted info with --no-wait", async () => {
    mockGetRoutes.mockResolvedValueOnce(onrampRoutes);
    mockGetQuote.mockResolvedValue(onrampQuote);
    mockCreateOrder.mockResolvedValueOnce(onrampOrder);
    mockSign.mockResolvedValueOnce("signed-tx-hex");
    mockRegisterTx.mockResolvedValueOnce({ onramp: { txid: "txid-123" } });

    const result = await handleSwap({ ...baseOpts, noWait: true });

    const parsed = JSON.parse(result);
    expect(parsed.orderId).toBe("order-456");
    expect(parsed.status).toBe("submitted");
    expect(mockGetOrder).not.toHaveBeenCalled();
  });

  it("throws when no BTC signer is configured", async () => {
    mockGetRoutes.mockResolvedValueOnce(onrampRoutes);
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

    mockGetRoutes.mockResolvedValueOnce(onrampRoutes);
    mockGetQuote.mockResolvedValue(onrampQuote);
    mockCreateOrder.mockResolvedValueOnce(onrampOrder);
    mockSign.mockResolvedValueOnce("signed-tx-hex");
    mockRegisterTx
      .mockRejectedValueOnce(new Error("network"))
      .mockRejectedValueOnce(new Error("network"))
      .mockResolvedValueOnce({ onramp: { txid: "txid-123" } });
    mockGetOrder.mockResolvedValueOnce(orderInfo);

    const result = await handleSwap(baseOpts);

    expect(mockRegisterTx).toHaveBeenCalledTimes(3);
    const parsed = JSON.parse(result);
    expect(parsed.status).toBe("confirmed");

    retrySpy.mockRestore();
  });

  it("outputs unsigned PSBT with --unsigned", async () => {
    mockGetRoutes.mockResolvedValueOnce(onrampRoutes);
    mockGetQuote.mockResolvedValue(onrampQuote);
    mockCreateOrder.mockResolvedValueOnce(onrampOrder);

    const result = await handleSwap({
      ...baseOpts,
      unsigned: true,
      bitcoinSignerCmd: undefined,
    });

    const parsed = JSON.parse(result);
    // psbtHex "70736274ff" in base64 is "cHNidP8="
    expect(parsed.psbtBase64).toBe(Buffer.from("70736274ff", "hex").toString("base64"));
    expect(parsed.orderId).toBe("order-456");
    expect(mockSign).not.toHaveBeenCalled();
  });

  it("throws on registration failure when json=false", async () => {
    // Mock retryWithBackoff to return error immediately
    const retryMod = await import("../../src/util/retry.js");
    const retrySpy = vi.spyOn(retryMod, "retryWithBackoff").mockResolvedValue({
      error: new Error("registration failed"),
    });

    mockGetRoutes.mockResolvedValueOnce(onrampRoutes);
    mockGetQuote.mockResolvedValue(onrampQuote);
    mockCreateOrder.mockResolvedValueOnce(onrampOrder);
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

    mockGetRoutes.mockResolvedValueOnce(onrampRoutes);
    mockGetQuote.mockResolvedValue(onrampQuote);
    mockCreateOrder.mockResolvedValueOnce(onrampOrder);
    mockSign.mockResolvedValueOnce("signed-tx-hex");

    const result = await handleSwap({ ...baseOpts, json: true });
    const parsed = JSON.parse(result);
    expect(parsed.error.code).toBe(3);
    expect(parsed.error.orderId).toBe("order-456");

    retrySpy.mockRestore();
  });

  it("throws TransientError with --no-retry on transient error", async () => {
    mockGetRoutes.mockResolvedValueOnce(onrampRoutes);
    // Initial quote succeeds, core quote fails with transient error
    mockGetQuote
      .mockResolvedValueOnce(onrampQuote)
      .mockRejectedValueOnce(new Error("TRM screening delay"));

    const { TransientError } = await import("../../src/util/retry.js");

    await expect(
      handleSwap({ ...baseOpts, noRetry: true }),
    ).rejects.toThrow(TransientError);
  });

  it("does not retry non-transient errors with --no-retry", async () => {
    mockGetRoutes.mockResolvedValueOnce(onrampRoutes);
    mockGetQuote
      .mockResolvedValueOnce(onrampQuote)
      .mockRejectedValueOnce(new Error("Insufficient funds"));

    await expect(
      handleSwap({ ...baseOpts, noRetry: true }),
    ).rejects.toThrow("Insufficient funds");
  });

  it("retries transient errors by default (no --no-retry)", async () => {
    // Use vi.useFakeTimers to avoid real delays
    vi.useFakeTimers();

    mockGetRoutes.mockResolvedValueOnce(onrampRoutes);
    // Initial quote for confirmation, then 1 transient failure, then success
    mockGetQuote.mockResolvedValue(onrampQuote);
    mockCreateOrder
      .mockRejectedValueOnce(new Error("TRM screening delay"))
      .mockResolvedValueOnce(onrampOrder);
    mockSign.mockResolvedValue("signed-tx-hex");
    mockRegisterTx.mockResolvedValue({ onramp: { txid: "txid-123" } });
    mockGetOrder.mockResolvedValueOnce(orderInfo);

    const swapPromise = handleSwap(baseOpts);

    // Advance past the retry delay
    await vi.advanceTimersByTimeAsync(6000);

    const result = await swapPromise;
    const parsed = JSON.parse(result);
    expect(parsed.status).toBe("confirmed");
    // createOrder called twice: once failed (transient), once succeeded
    expect(mockCreateOrder).toHaveBeenCalledTimes(2);

    vi.useRealTimers();
  });

  it("throws TransientError after all retry attempts exhausted", async () => {
    vi.useFakeTimers();

    mockGetRoutes.mockResolvedValueOnce(onrampRoutes);
    mockGetQuote.mockResolvedValue(onrampQuote);
    // All createOrder calls fail with transient error
    mockCreateOrder.mockRejectedValue(new Error("429 Too Many Requests"));

    const { TransientError } = await import("../../src/util/retry.js");

    // Capture the promise and attach a rejection handler immediately
    // to avoid unhandled rejection warnings
    const swapPromise = handleSwap(baseOpts).catch((e) => e);

    // Advance past all retry delays (5s + 10s + 20s + 40s + 80s = 155s)
    for (let i = 0; i < 10; i++) {
      await vi.advanceTimersByTimeAsync(20000);
    }

    const error = await swapPromise;
    expect(error).toBeInstanceOf(TransientError);
    expect(error.message).toBe("429 Too Many Requests");

    vi.useRealTimers();
  });
});
