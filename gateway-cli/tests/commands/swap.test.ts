import { describe, it, expect, vi, beforeEach } from "vitest";
import type { Logger } from "../../src/output.js";
import type { EnrichedRoute } from "../../src/util/route-provider.js";

// ─── Mock fixtures ────────────────────────────────────────────────────────────

const btcToken = { address: "BTC", symbol: "BTC", decimals: 8, chain: "bitcoin" };
const usdcToken = { address: "0xUSDC", symbol: "USDC", decimals: 6, chain: "base" };

const enrichedRoutes: EnrichedRoute[] = [
  { srcChain: "bitcoin", dstChain: "base", srcToken: btcToken, dstToken: usdcToken },
];

const onrampQuote = {
  onramp: {
    inputAmount: { amount: "5000000", address: "BTC", chain: "bitcoin" },
    outputAmount: { amount: "4812300000", address: "0xUSDC", chain: "base" },
    sender: "bc1qtest",
  },
};

const onrampOrder = {
  onramp: { orderId: "order-456", address: "bc1qgateway", psbtHex: "70736274ff" },
};

const confirmedOrder = {
  id: "order-456",
  status: "success" as const,
  dstInfo: { amount: "4812300000" },
};

// ─── Mock functions ───────────────────────────────────────────────────────────

const mockGetQuote = vi.fn();
const mockGetRoutes = vi.fn();
const mockGetOrder = vi.fn();
const mockCreateOrder = vi.fn();
const mockRegisterTx = vi.fn();
const mockSignAllInputs = vi.fn();
const mockGetP2WPKHAddress = vi.fn();
const mockGetAddressMempoolTxs = vi.fn();

// ─── Mocks ───────────────────────────────────────────────────────────────────

vi.mock("../../src/config.js", () => ({
  loadConfig: vi.fn(() => ({
    slippageBps: 300,
    timeoutMs: 5000,
    bitcoinPrivateKey: undefined,
    evmPrivateKey: undefined,
  })),
  getSdk: vi.fn(() => ({
    getQuote: mockGetQuote,
    getRoutes: mockGetRoutes,
    getOrder: mockGetOrder,
    api: { createOrder: mockCreateOrder, registerTx: mockRegisterTx },
  })),
  BTC_DECIMALS: 8,
}));

vi.mock("../../src/util/route-provider.js", () => ({
  getEnrichedRoutes: vi.fn(async () => enrichedRoutes),
  getNativeToken: vi.fn((chain: string) => {
    if (chain === "ethereum") return { symbol: "ETH", decimals: 18 };
    throw new Error(`unknown chain "${chain}"`);
  }),
}));

vi.mock("../../src/util/rpc-resolver.js", () => ({
  resolveRpcUrl: vi.fn(() => undefined),
  getViemChain: vi.fn(() => undefined),
}));

vi.mock("@gobob/bob-sdk", () => ({
  getInnerQuote: vi.fn((quote: any) => {
    const variant = quote.onramp ? quote.onramp : quote.layerZero ?? quote.offramp;
    return variant;
  }),
  getQuoteVariant: vi.fn((quote: any) => {
    if (quote.onramp) return "onramp";
    if (quote.layerZero) return "layerZero";
    return "offramp";
  }),
  ScureBitcoinSigner: {
    fromKey: vi.fn(() => ({
      signAllInputs: mockSignAllInputs,
      getP2WPKHAddress: mockGetP2WPKHAddress,
    })),
  },
  MempoolClient: vi.fn().mockImplementation(() => ({
    getAddressMempoolTxs: mockGetAddressMempoolTxs,
  })),
}));

vi.mock("p-retry", () => ({
  default: vi.fn(async (fn: any, opts: any) => {
    let lastError: any;
    const retries = opts?.retries ?? 3;
    for (let i = 1; i <= retries + 1; i++) {
      try {
        return await fn(i);
      } catch (e: any) {
        if (e?.name === "AbortError") throw e.originalError ?? new Error(e.message);
        lastError = e;
      }
    }
    throw lastError;
  }),
  AbortError: class AbortError extends Error {
    readonly name = "AbortError";
    readonly originalError?: Error;
    constructor(msg: string) {
      super(msg);
      this.name = "AbortError";
    }
  },
}));

// ─── Helpers ─────────────────────────────────────────────────────────────────

const silentLogger: Logger = { progress: () => {}, warn: () => {} };

const baseOpts = {
  src: "BTC",
  dst: "USDC:base",
  amount: "5000000",
  recipient: "0xABC",
  unsigned: false,
  wait: true,
  retry: true,
  privateKey: "cTestPrivateKey",
};

// ─── Tests ────────────────────────────────────────────────────────────────────

describe("handleSwap", () => {
  beforeEach(() => {
    vi.clearAllMocks();

    mockGetQuote.mockResolvedValue(onrampQuote);
    mockCreateOrder.mockResolvedValue(onrampOrder);
    mockRegisterTx.mockResolvedValue(undefined);
    mockSignAllInputs.mockResolvedValue("signed-tx-hex-abc");
    mockGetP2WPKHAddress.mockResolvedValue("bc1qtest");
    mockGetOrder.mockResolvedValue(confirmedOrder);
    mockGetAddressMempoolTxs.mockResolvedValue([]);
  });

  it("full onramp BTC flow: quote → createOrder → sign → register → poll → confirmed", async () => {
    const { handleSwap } = await import("../../src/commands/swap.js");
    const result = await handleSwap(baseOpts, silentLogger);

    expect(result.type).toBe("confirmed");
    if (result.type !== "confirmed") return;

    expect(result.data.orderId).toBe("order-456");
    expect(result.data.status).toBe("confirmed");
    expect(result.data.txId).toBe("signed-tx-hex-abc");
    expect(result.data.dstAmount).toBe("4812300000");
    expect(result.data.srcAsset).toBe("BTC");
    expect(result.data.dstAsset).toBe("USDC");

    expect(mockGetQuote).toHaveBeenCalledOnce();
    expect(mockCreateOrder).toHaveBeenCalledOnce();
    expect(mockSignAllInputs).toHaveBeenCalledWith("70736274ff");
    expect(mockRegisterTx).toHaveBeenCalledOnce();
    expect(mockGetOrder).toHaveBeenCalledWith("order-456");
  });

  it("--no-wait returns submitted without polling", async () => {
    const { handleSwap } = await import("../../src/commands/swap.js");
    const result = await handleSwap({ ...baseOpts, wait: false }, silentLogger);

    expect(result.type).toBe("submitted");
    if (result.type !== "submitted") return;

    expect(result.data.orderId).toBe("order-456");
    expect(result.data.status).toBe("submitted");
    expect(result.data.txId).toBe("signed-tx-hex-abc");

    expect(mockGetOrder).not.toHaveBeenCalled();
  });

  it("--unsigned returns PSBT without signing", async () => {
    const { handleSwap } = await import("../../src/commands/swap.js");
    const result = await handleSwap({ ...baseOpts, unsigned: true }, silentLogger);

    expect(result.type).toBe("unsigned");
    if (result.type !== "unsigned") return;

    expect(result.orderId).toBe("order-456");
    // psbtHex "70736274ff" decoded from hex → base64
    expect(result.psbtBase64).toBe(Buffer.from("70736274ff", "hex").toString("base64"));

    expect(mockSignAllInputs).not.toHaveBeenCalled();
    expect(mockRegisterTx).not.toHaveBeenCalled();
  });

  it("missing signer throws 'no signer configured for Bitcoin'", async () => {
    const { handleSwap } = await import("../../src/commands/swap.js");
    await expect(
      handleSwap({ ...baseOpts, privateKey: undefined }, silentLogger),
    ).rejects.toThrow("no signer configured for Bitcoin");
  });

  it("registration failure throws error with CRITICAL message", async () => {
    mockRegisterTx.mockRejectedValue(new Error("network error"));

    const { handleSwap } = await import("../../src/commands/swap.js");
    await expect(
      handleSwap(baseOpts, silentLogger),
    ).rejects.toThrow("CRITICAL");
  });

  it("transient error: createOrder fails once with 'TRM screening delay', succeeds on retry", async () => {
    let callCount = 0;
    mockCreateOrder.mockImplementation(async () => {
      callCount++;
      if (callCount === 1) throw new Error("TRM screening delay");
      return onrampOrder;
    });

    const { handleSwap } = await import("../../src/commands/swap.js");
    const result = await handleSwap(baseOpts, silentLogger);

    expect(result.type).toBe("confirmed");
    expect(mockCreateOrder).toHaveBeenCalledTimes(2);
  });

  it("non-transient error: createOrder fails with 'Insufficient funds', not retried", async () => {
    mockCreateOrder.mockRejectedValue(new Error("Insufficient funds"));

    const { handleSwap } = await import("../../src/commands/swap.js");
    await expect(
      handleSwap(baseOpts, silentLogger),
    ).rejects.toThrow("Insufficient funds");

    expect(mockCreateOrder).toHaveBeenCalledTimes(1);
  });

  it("--no-retry: transient error is not retried", async () => {
    mockCreateOrder.mockRejectedValue(new Error("TRM screening delay"));

    const { handleSwap } = await import("../../src/commands/swap.js");
    await expect(
      handleSwap({ ...baseOpts, retry: false }, silentLogger),
    ).rejects.toThrow("TRM screening delay");

    expect(mockCreateOrder).toHaveBeenCalledTimes(1);
  });
});
