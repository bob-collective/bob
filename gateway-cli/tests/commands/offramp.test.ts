import { describe, it, expect, vi, beforeEach } from "vitest";
import { handleOfframp } from "../../src/commands/offramp.js";

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
    variant: quote.offramp ? "offramp" : "onramp",
    inputAmount: quote.offramp?.inputAmount?.amount ?? "0",
    outputAmount: quote.offramp?.outputAmount?.amount ?? "0",
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

// ─── EVM signer mock ─────────────────────────────────────────────────────────

const mockSendTransaction = vi.fn();
vi.mock("../../src/signer/evm.js", async (importOriginal) => {
  const actual = await importOriginal<typeof import("../../src/signer/evm.js")>();
  return {
    ...actual,
    resolveEvmSigner: vi.fn().mockImplementation(async (opts: {
      unsigned: boolean;
      privateKey?: string;
      envPrivateKey?: string;
      keystorePath?: string;
      externalSignerCmd?: string;
    }) => {
      if (opts.unsigned) return { unsigned: true };
      if (!opts.privateKey && !opts.envPrivateKey && !opts.keystorePath && !opts.externalSignerCmd) {
        throw new Error(
          "no signer configured for EVM.\n" +
          "  Set EVM_PRIVATE_KEY, EVM_SIGNER, or pass --private-key / --keystore.\n" +
          "  Use --unsigned to output the unsigned transaction.",
        );
      }
      return {
        walletClient: {
          sendTransaction: mockSendTransaction,
          account: { address: "0xSender" },
        },
        publicClient: {
          getTransactionCount: vi.fn().mockResolvedValue(0),
        },
      };
    }),
  };
});

// ─── Mempool mock ────────────────────────────────────────────────────────────

vi.mock("../../src/util/mempool.js", () => ({
  fetchFeeRate: vi.fn().mockResolvedValue(10),
  findPendingMempoolTx: vi.fn().mockResolvedValue(null),
}));

// ─── Fixtures ────────────────────────────────────────────────────────────────

const offrampRoutes = [
  {
    srcChain: "base",
    dstChain: "bitcoin",
    srcToken: {
      address: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913",
      symbol: "USDC",
      decimals: 6,
      chain: "base",
    },
    dstToken: {
      address: "0x0000000000000000000000000000000000000000",
      symbol: "BTC",
      decimals: 8,
      chain: "bitcoin",
    },
  },
];

// SDK-style offramp quote
const offrampQuote = {
  offramp: {
    inputAmount: { amount: "5000000000", address: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913", chain: "base" },
    outputAmount: { amount: "4900000", address: "BTC", chain: "bitcoin" },
    fees: { amount: "50000", address: "0x0", chain: "base" },
    feeBreakdown: {
      solverFee: { amount: "50000", address: "0x0", chain: "base" },
      protocolFee: { amount: "10000", address: "0x0", chain: "base" },
    },
    slippage: 300,
    srcChain: "base",
    tokenAddress: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913",
    txTo: "0xGateway",
    recipient: "bc1qexample",
  },
};

// SDK-style offramp order
const offrampOrder = {
  offramp: {
    orderId: "order-offramp",
    tx: {
      to: "0xGateway",
      data: "0xabcdef",
      value: "0",
      chain: "base",
    },
  },
};

// SDK-style order info (from getOrder)
const offrampOrderInfo = {
  id: "order-offramp",
  status: "success",
  srcInfo: { amount: "5000000000", chain: "base", token: "USDC" },
  dstInfo: { amount: "4900000", chain: "bitcoin", token: "BTC" },
  timestamp: 1710230400,
};

describe("handleOfframp (thin alias -> handleSwap, offramp: EVM -> BTC)", () => {
  beforeEach(() => vi.clearAllMocks());

  const baseOpts = {
    apiUrl: "https://example.com",
    src: "USDC:base",
    dst: "BTC",
    amount: "5000USDC",
    recipient: "bc1qexample",
    slippageBps: 300,
    evmSignerCmd: "test-evm-signer",
    autoConfirm: true,
    unsigned: false,
    noWait: false,
    dryRun: false,
    timeoutMs: 5000,
    json: true,
  };

  it("executes full offramp flow", async () => {
    mockGetRoutes.mockResolvedValueOnce(offrampRoutes);
    mockGetQuote.mockResolvedValue(offrampQuote);
    mockCreateOrder.mockResolvedValueOnce(offrampOrder);
    mockSendTransaction.mockResolvedValueOnce("0xsigned");
    mockRegisterTx.mockResolvedValueOnce("ok");
    mockGetOrder.mockResolvedValueOnce(offrampOrderInfo);

    const result = await handleOfframp(baseOpts);

    expect(mockSendTransaction).toHaveBeenCalledWith(
      expect.objectContaining({
        to: "0xGateway",
        data: "0xabcdef",
      }),
    );
    const parsed = JSON.parse(result);
    expect(parsed.status).toBe("confirmed");
    expect(parsed.orderId).toBe("order-offramp");
  });

  it("throws when no EVM signer is configured", async () => {
    mockGetRoutes.mockResolvedValueOnce(offrampRoutes);
    await expect(
      handleOfframp({ ...baseOpts, evmSignerCmd: undefined }),
    ).rejects.toThrow("no signer configured for EVM");
  });

  it("outputs unsigned tx info with --unsigned", async () => {
    mockGetRoutes.mockResolvedValueOnce(offrampRoutes);
    mockGetQuote.mockResolvedValue(offrampQuote);
    mockCreateOrder.mockResolvedValueOnce(offrampOrder);

    const result = await handleOfframp({
      ...baseOpts,
      unsigned: true,
      evmSignerCmd: undefined,
    });

    const parsed = JSON.parse(result);
    expect(parsed.orderId).toBe("order-offramp");
    expect(parsed.txInfo).toBeDefined();
    expect(mockSendTransaction).not.toHaveBeenCalled();
  });

  it("returns submitted info with --no-wait", async () => {
    mockGetRoutes.mockResolvedValueOnce(offrampRoutes);
    mockGetQuote.mockResolvedValue(offrampQuote);
    mockCreateOrder.mockResolvedValueOnce(offrampOrder);
    mockSendTransaction.mockResolvedValueOnce("0xsigned");
    mockRegisterTx.mockResolvedValueOnce("ok");

    const result = await handleOfframp({ ...baseOpts, noWait: true });

    const parsed = JSON.parse(result);
    expect(parsed.orderId).toBe("order-offramp");
    expect(parsed.status).toBe("submitted");
    expect(mockGetOrder).not.toHaveBeenCalled();
  });
});
