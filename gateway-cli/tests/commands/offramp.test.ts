import { describe, it, expect, vi, beforeEach } from "vitest";
import { handleOfframp } from "../../src/commands/offramp.js";
import { mockRoutes } from "../fixtures/api-responses.js";

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
      // Check if any signer source is configured
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
        },
        publicClient: {
          getTransactionCount: vi.fn().mockResolvedValue(0),
        },
      };
    }),
  };
});

vi.mock("../../src/util/mempool.js", () => ({
  fetchFeeRate: vi.fn().mockResolvedValue(10),
  findPendingMempoolTx: vi.fn().mockResolvedValue(null),
}));

// A minimal EVM-side routes fixture for the offramp direction
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

const offrampQuote = {
  quoteId: "quote-offramp",
  inputAmount: "5000000000",
  outputAmount: "4900000",
  feeBreakdown: {
    solverFee: { amount: "50000", address: "0x0", chain: "base" },
    protocolFee: { amount: "10000", address: "0x0", chain: "base" },
  },
  estimatedTimeInSecs: 600,
};

const offrampOrder = {
  orderId: "order-offramp",
  txInfo: {
    to: "0xGateway",
    data: "0xabcdef",
    value: "0",
    chain: "base",
  },
};

describe("handleOfframp (thin alias → handleSwap, offramp: EVM -> BTC)", () => {
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
    timeoutMs: 5000,
    json: true,
  };

  it("executes full offramp flow", async () => {
    mockGetRoutes.mockResolvedValueOnce(offrampRoutes);
    mockGetQuote.mockResolvedValueOnce(offrampQuote);
    mockCreateOrder.mockResolvedValueOnce(offrampOrder);
    mockSendTransaction.mockResolvedValueOnce("0xsigned");
    mockRegisterTx.mockResolvedValueOnce({ success: true });
    mockGetOrder.mockResolvedValueOnce({
      orderId: "order-offramp",
      status: "success",
      inputAmount: "5000000000",
      outputAmount: "4900000",
      srcChain: "base",
      dstChain: "bitcoin",
      createdAt: "2026-03-12T10:00:00Z",
    });

    const result = await handleOfframp(baseOpts);

    expect(mockSendTransaction).toHaveBeenCalledWith(
      expect.objectContaining({
        to: "0xGateway",
        data: "0xabcdef",
        chainId: 8453,
      }),
    );
    const parsed = JSON.parse(result);
    expect(parsed.status).toBe("confirmed");
    expect(parsed.orderId).toBe("order-offramp");
  });

  it("throws when no EVM signer is configured", async () => {
    // getRoutes needed to parse src/dst before signer resolution
    mockGetRoutes.mockResolvedValueOnce(offrampRoutes);
    await expect(
      handleOfframp({ ...baseOpts, evmSignerCmd: undefined }),
    ).rejects.toThrow("no signer configured for EVM");
  });

  it("outputs unsigned tx info with --unsigned", async () => {
    mockGetRoutes.mockResolvedValueOnce(offrampRoutes);
    mockGetQuote.mockResolvedValueOnce(offrampQuote);
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
    mockGetQuote.mockResolvedValueOnce(offrampQuote);
    mockCreateOrder.mockResolvedValueOnce(offrampOrder);
    mockSendTransaction.mockResolvedValueOnce("0xsigned");
    mockRegisterTx.mockResolvedValueOnce({ success: true });

    const result = await handleOfframp({ ...baseOpts, noWait: true });

    const parsed = JSON.parse(result);
    expect(parsed.orderId).toBe("order-offramp");
    expect(parsed.status).toBe("submitted");
    expect(mockGetOrder).not.toHaveBeenCalled();
  });
});
