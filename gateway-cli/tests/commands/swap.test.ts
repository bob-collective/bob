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
    api: { createOrder: mockCreateOrder, registerTx: mockRegisterTx },
  })),
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
    mockCreateOrder.mockResolvedValue(onrampOrder);
    mockRegisterTx.mockResolvedValue(undefined);
    mockSignAllInputs.mockResolvedValue("signed-tx-hex-abc");
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

  it("registration failure throws error with recovery instructions", async () => {
    mockRegisterTx.mockRejectedValue(new Error("network error"));

    const { handleSwap } = await import("../../src/commands/swap.js");
    await expect(
      handleSwap(baseOpts, silentLogger),
    ).rejects.toThrow("Registration failed");
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

  it("EVM sendTransaction revert attaches enriched context to error", async () => {
    // Reconfigure mocks for an EVM offramp flow (not BTC onramp)
    const { resolveSwapInputs } = await import("../../src/util/input-resolver.js");
    (resolveSwapInputs as any).mockResolvedValue({
      srcAsset: { chain: "base", address: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913", symbol: "USDC", decimals: 6 },
      dstAsset: { chain: "bitcoin", address: "BTC", symbol: "BTC", decimals: 8 },
      atomicUnits: "10000000",
      display: "10 USDC",
    });

    const { parseAssetChain } = await import("../../src/util/input-resolver.js");
    (parseAssetChain as any).mockImplementation((asset: string) =>
      asset.includes("BTC") ? { chain: "bitcoin", address: "BTC", symbol: "BTC", decimals: 8 }
                             : { chain: "base", address: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913", symbol: "USDC", decimals: 6 }
    );

    const { getChainFamily, resolveSigner, resolveRecipient } = await import("../../src/chains/index.js");
    (getChainFamily as any).mockImplementation((chain: string) => chain === "bitcoin" ? "bitcoin" : "evm");
    (resolveRecipient as any).mockResolvedValue("bc1qtest");

    const mockWalletClient = {
      account: { address: "0xAF91558Ba2B1994530c9cfCcbda5AE9cD2b456bb" },
      chain: { id: 8453, name: "Base" },
      sendTransaction: vi.fn().mockRejectedValue(
        Object.assign(new Error("Execution reverted for an unknown reason."), {
          cause: { data: "0xdeadbeef" },
        })
      ),
    };
    const mockPublicClient = {
      getTransactionCount: vi.fn().mockResolvedValue(0),
    };
    (resolveSigner as any).mockResolvedValue({ walletClient: mockWalletClient, publicClient: mockPublicClient });

    const mockOfframpQuote = {
      offramp: {
        inputAmount: { amount: "10000000", address: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913", chain: "base" },
        outputAmount: { amount: "15000", address: "BTC", chain: "bitcoin" },
        sender: "0xAF91558Ba2B1994530c9cfCcbda5AE9cD2b456bb",
      },
    };
    const mockOfframpOrder = {
      offramp: {
        orderId: "revert-order-123",
        tx: { to: "0x96C33FB0b058c341F5567b0b91E1Fc5F2E5cB1be", data: "0xe5c65df5aabbccdd", value: "369441375065843" },
      },
    };
    mockGetQuote.mockResolvedValue(mockOfframpQuote);
    mockCreateOrder.mockResolvedValue(mockOfframpOrder);

    const { handleSwap } = await import("../../src/commands/swap.js");
    try {
      await handleSwap({ ...baseOpts, src: "USDC:base", dst: "BTC", privateKey: "0xTestEvmKey" }, silentLogger);
      expect.unreachable("should have thrown");
    } catch (err: any) {
      expect(err.message).toContain("Execution reverted");
      expect(err.orderId).toBe("revert-order-123");
      expect(err.txParams).toEqual({
        to: "0x96C33FB0b058c341F5567b0b91E1Fc5F2E5cB1be",
        from: "0xAF91558Ba2B1994530c9cfCcbda5AE9cD2b456bb",
        value: "369441375065843",
        data: "0xe5c65df5aabbccdd",
        chainId: 8453,
        chainName: "Base",
      });
      expect(err.srcAsset).toEqual({ symbol: "USDC", address: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913", chain: "base" });
      expect(err.dstAsset).toEqual({ symbol: "BTC", address: "BTC", chain: "bitcoin" });
      expect(err.functionSelector).toBe("0xe5c65df5");
      expect(err.revertData).toBe("0xdeadbeef");
    }
  });

});
