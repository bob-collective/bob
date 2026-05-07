import { describe, it, expect, vi, beforeEach } from "vitest";
import type { Logger } from "../../src/output.js";
import type { RouteInfo } from "@gobob/bob-sdk";

// ─── Mock fixtures ────────────────────────────────────────────────────────────

const mockRoutes: RouteInfo[] = [
  { srcChain: "bitcoin", dstChain: "base", srcToken: "BTC", dstToken: "0xUSDC" },
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

// V2 order status is a discriminated object union: { success } | { refunded } | { failed } | { inProgress }.
const confirmedOrder = {
  id: "order-456",
  status: { success: {} },
  dstInfo: { amount: "4812300000" },
};

// ─── Mock functions ───────────────────────────────────────────────────────────

const mockGetQuote = vi.fn();
const mockGetRoutes = vi.fn();
const mockGetOrder = vi.fn();
const mockExecuteQuote = vi.fn();
const mockCreateOrder = vi.fn();
const mockSignAllInputs = vi.fn();
const mockGetAddressMempoolTxs = vi.fn();

const mockBtcSigner = {
  signAllInputs: mockSignAllInputs,
  getP2WPKHAddress: vi.fn().mockResolvedValue("bc1qtest"),
};

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
    executeQuote: mockExecuteQuote,
  })),
  getApi: vi.fn(() => ({ createOrderV2: mockCreateOrder })),
  BTC_DECIMALS: 8,
}));

vi.mock("../../src/util/route-provider.js", () => ({
  getRoutes: vi.fn(async () => mockRoutes),
}));

vi.mock("../../src/util/input-resolver.js", () => ({
  resolveSwapInputs: vi.fn().mockResolvedValue({
    srcAsset: { chain: "bitcoin", address: "BTC", symbol: "BTC", decimals: 8 },
    dstAsset: { chain: "base", address: "0xUSDC", symbol: "USDC", decimals: 6 },
    atomicUnits: "5000000",
    display: "0.05 BTC",
  }),
  humanToAtomic: vi.fn((human: string, decimals: number) => "0"),
  buildTokenIndex: vi.fn(() => ({ byChainAndSymbol: new Map(), byChainAndAddress: new Map() })),
  parseAssetChain: vi.fn((asset: string) => ({ chain: "bitcoin", address: "BTC", symbol: "BTC", decimals: 8 })),
}));

vi.mock("../../src/util/price-oracle.js", () => ({
  fetchPrice: vi.fn().mockResolvedValue({ priceUsd: 100000, source: "mock" }),
}));

vi.mock("../../src/chains/index.js", () => ({
  getChainFamily: vi.fn((chain: string) => chain === "bitcoin" ? "bitcoin" : "evm"),
  deriveAddress: vi.fn().mockResolvedValue("bc1qtest"),
  resolveSigner: vi.fn().mockResolvedValue({ address: "bc1qtest", signer: mockBtcSigner }),
  getTokenBalance: vi.fn().mockResolvedValue({ total: "5000000", allSpendable: "4900000" }),
  buildRegisterPayload: vi.fn((_src: string, _dst: string, orderId: string, txId: string) => ({
    onramp: { orderId, bitcoinTxHex: txId },
  })),
  resolvePrivateKey: vi.fn((chain: string, privateKey?: string) => privateKey),
  resolveRecipient: vi.fn().mockResolvedValue("bc1qtest"),
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
    fromKey: vi.fn(() => mockBtcSigner),
  },
  MempoolClient: vi.fn().mockImplementation(() => ({
    getAddressMempoolTxs: mockGetAddressMempoolTxs,
  })),
  supportedChainsMapping: {
    base: { id: 8453, name: "base", nativeCurrency: { symbol: "ETH", decimals: 18, name: "Ether" } },
    ethereum: { id: 1, name: "ethereum", nativeCurrency: { symbol: "ETH", decimals: 18, name: "Ether" } },
  },
}));

vi.mock("@gobob/tokenlist/tokenlist.json", () => ({
  default: { tokens: [] },
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
    mockExecuteQuote.mockResolvedValue({ order: onrampOrder, tx: "signed-tx-hex-abc" });
    mockCreateOrder.mockResolvedValue(onrampOrder);
    mockSignAllInputs.mockResolvedValue("signed-tx-hex-abc");
    mockGetOrder.mockResolvedValue(confirmedOrder);
    mockGetAddressMempoolTxs.mockResolvedValue([]);
  });

  it("full onramp BTC flow: quote → executeQuote → poll → confirmed", async () => {
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
    expect(mockExecuteQuote).toHaveBeenCalledOnce();
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
    expect(mockExecuteQuote).not.toHaveBeenCalled();
  });

  it("missing signer throws 'no signer configured for Bitcoin'", async () => {
    const { handleSwap } = await import("../../src/commands/swap.js");
    await expect(
      handleSwap({ ...baseOpts, privateKey: undefined }, silentLogger),
    ).rejects.toThrow("no signer configured for Bitcoin");
  });

  it("transient error: executeQuote fails once with 'TRM screening delay', succeeds on retry", async () => {
    let callCount = 0;
    mockExecuteQuote.mockImplementation(async () => {
      callCount++;
      if (callCount === 1) throw new Error("TRM screening delay");
      return { order: onrampOrder, tx: "signed-tx-hex-abc" };
    });

    const { handleSwap } = await import("../../src/commands/swap.js");
    const result = await handleSwap(baseOpts, silentLogger);

    expect(result.type).toBe("confirmed");
    expect(mockExecuteQuote).toHaveBeenCalledTimes(2);
  });

  it("non-transient error: executeQuote fails with 'Insufficient funds', not retried", async () => {
    mockExecuteQuote.mockRejectedValue(new Error("Insufficient funds"));

    const { handleSwap } = await import("../../src/commands/swap.js");
    await expect(
      handleSwap(baseOpts, silentLogger),
    ).rejects.toThrow("Insufficient funds");

    expect(mockExecuteQuote).toHaveBeenCalledTimes(1);
  });

  it("--no-retry: transient error is not retried", async () => {
    mockExecuteQuote.mockRejectedValue(new Error("TRM screening delay"));

    const { handleSwap } = await import("../../src/commands/swap.js");
    await expect(
      handleSwap({ ...baseOpts, retry: false }, silentLogger),
    ).rejects.toThrow("TRM screening delay");

    expect(mockExecuteQuote).toHaveBeenCalledTimes(1);
  });

});
