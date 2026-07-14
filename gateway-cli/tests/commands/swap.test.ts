import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import type { Logger } from "../../src/output.js";
import type { RouteInfo } from "@gobob/bob-sdk";
import type { PollTimings } from "../../src/commands/swap.js";

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
  getApi: vi.fn(() => ({ createOrderV3: mockCreateOrder })),
  BTC_DECIMALS: 8,
}));

vi.mock("../../src/util/route-provider.js", () => ({
  getRoutes: vi.fn(async () => mockRoutes),
}));

// `parseAssetChain` must answer per-asset: the source and the destination are resolved
// through the SAME resolver, and a mock that returned one fixed asset for both would
// make every swap look like a BTC→BTC swap.
vi.mock("../../src/util/input-resolver.js", () => {
  const assets: Record<string, unknown> = {
    "BTC": { chain: "bitcoin", address: "BTC", symbol: "BTC", decimals: 8 },
    "USDC:base": { chain: "base", address: "0xUSDC", symbol: "USDC", decimals: 6 },
    "USDT:ethereum": { chain: "ethereum", address: "0xUSDT", symbol: "USDT", decimals: 6 },
  };
  return {
    humanToAtomic: vi.fn(() => "0"),
    buildTokenIndex: vi.fn(() => ({ byChainAndSymbol: new Map(), byChainAndAddress: new Map() })),
    parseAssetChain: vi.fn((asset: string) => {
      const resolved = assets[asset];
      if (!resolved) throw new Error(`test fixture: no asset for "${asset}"`);
      return resolved;
    }),
    resolveAmount: vi.fn().mockResolvedValue({ atomicUnits: "5000000", display: "0.05 BTC" }),
  };
});

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
  resolveRecipient: vi.fn().mockResolvedValue("0xEvmRecipient"),
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
  // Mirrors real p-retry: when constructed from an Error, that Error becomes
  // `originalError` (preserving any fields attached to it); from a string, it
  // wraps a fresh Error. p-retry throws `originalError`, so fields survive only
  // when attached to the Error passed in.
  AbortError: class AbortError extends Error {
    readonly name = "AbortError";
    readonly originalError: Error;
    constructor(message: string | Error) {
      super(typeof message === "string" ? message : message.message);
      this.name = "AbortError";
      this.originalError = typeof message === "string" ? new Error(message) : message;
    }
  },
}));

// ─── Helpers ─────────────────────────────────────────────────────────────────

const silentLogger: Logger = { progress: () => {}, warn: () => {} };

/**
 * Poll timings compressed so the loop's REAL machinery — AbortSignal.timeout,
 * AbortSignal.any, the abortable sleep, the SDK's request cancellation — runs
 * unmocked and still finishes in milliseconds. Fake timers cannot drive
 * AbortSignal.timeout (it is not backed by the global timer), and faking the loop's
 * clock is exactly the thing these tests exist to prove the loop no longer depends on.
 */
const FAST: PollTimings = { pollIntervalMs: 10, maxBackoffMs: 40, getOrderTimeoutMs: 50 };

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

/** USDT@ethereum → BTC offramp paying the ONE BTC address gateway-bot reuses for every swap. */
async function setupOfframp() {
  const { resolveSigner, deriveAddress, resolveRecipient } = await import("../../src/chains/index.js");
  const publicClient = { getTransactionCount: vi.fn().mockResolvedValue(0) } as any;
  vi.mocked(resolveSigner).mockResolvedValue({ address: "0xSender", walletClient: {} as any, publicClient } as any);
  vi.mocked(deriveAddress).mockResolvedValue("0xSender");
  vi.mocked(resolveRecipient).mockResolvedValue("bc1qshared");

  mockExecuteQuote.mockResolvedValue({
    order: { offramp: { orderId: "order-btc-1", tx: { to: "0xGateway" } } },
    tx: "0xsettlementhash",
  });

  return { ...baseOpts, src: "USDT:ethereum", dst: "BTC", privateKey: "0xevmkey" };
}

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
    // Every read carries an abort signal, so no single read can outlive --timeout.
    expect(mockGetOrder).toHaveBeenCalledWith("order-456", { signal: expect.any(AbortSignal) });
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

  it("tokenSwap order-failed error carries txId + txParams.chainId for receipt lookup", async () => {
    // An EVM→EVM tokenSwap settles via an on-chain tx. When the order fails,
    // the thrown error must carry the settlement tx hash + chainId so downstream
    // alerting can fetch the receipt and detect out-of-gas — without these the
    // failure is undiagnosable.
    const { resolveSigner, deriveAddress, resolveRecipient } = await import("../../src/chains/index.js");
    // EVM signed path uses { walletClient, publicClient }; the pending-nonce check
    // calls publicClient.getTransactionCount (latest >= pending → settled).
    const publicClient = { getTransactionCount: vi.fn().mockResolvedValue(0) } as any;
    vi.mocked(resolveSigner).mockResolvedValue({ address: "0xSender", walletClient: {} as any, publicClient } as any);
    vi.mocked(deriveAddress).mockResolvedValue("0xSender");
    vi.mocked(resolveRecipient).mockResolvedValue("0xRecipient");

    const tokenSwapOrder = { tokenSwap: { orderId: "order-789", tx: { to: "0xGatewayContract" } } };
    mockExecuteQuote.mockResolvedValue({ order: tokenSwapOrder, tx: "0xsettlementhash" });
    mockGetOrder.mockResolvedValue({ id: "order-789", status: { failed: {} } });

    const { handleSwap } = await import("../../src/commands/swap.js");
    const { SwapError } = await import("../../src/errors.js");

    const caught: any = await handleSwap(
      { ...baseOpts, src: "USDT:ethereum", dst: "USDC:base", privateKey: "0xevmkey" },
      silentLogger,
    ).catch(e => e);

    expect(caught).toBeInstanceOf(SwapError);
    // Message preserved so existing categorization still matches as a fallback.
    expect(caught.message).toContain("order-789 failed");
    expect(caught.context.orderId).toBe("order-789");
    expect(caught.context.txId).toBe("0xsettlementhash");
    expect(caught.context.txParams.chainId).toBe(1); // ethereum
    expect(caught.context.txParams.to).toBe("0xGatewayContract");
    expect(caught.context.srcAsset).toEqual({ symbol: "USDT", chain: "ethereum" });
    expect(caught.context.dstAsset).toEqual({ symbol: "USDC", chain: "base" });
  });

  // ─── C1 / A4 — never retry a swap once the wallet has been asked to sign ────
  //
  // `executeQuote` is not idempotent: createOrder → sign + BROADCAST → registerTx.
  // Re-running it after the broadcast sends the user's funds a SECOND time. The SDK
  // fires its step callback immediately before it asks the wallet to sign, which is
  // the signal these tests rely on. Reachable via `--retry` only (gateway-bot CI does
  // not pass it), but `--retry` defaults to on for anyone running the CLI by hand.

  it("does NOT retry executeQuote when a transient error is thrown AFTER the source tx was broadcast", async () => {
    // Reproduced from the real incident: the solver returned 504 *after* the source funds
    // had already left the wallet. "504 Gateway Timeout" matches /timeout/i in TRANSIENT,
    // so the closure was re-entered → a second quote, a second order, a second broadcast.
    // Six broadcasts (1 + retries: 5) from one 504.
    mockExecuteQuote.mockImplementation(async ({ callback }: any) => {
      callback?.({ step: 1, type: "send_transaction", totalSteps: 1 });
      throw new Error("bungee api error: status=504 Gateway Timeout");
    });

    const { handleSwap } = await import("../../src/commands/swap.js");
    const { SwapError } = await import("../../src/errors.js");

    const caught: any = await handleSwap({ ...baseOpts, retry: true }, silentLogger).catch(e => e);

    // The funds-safety invariant: one broadcast, one attempt. Never two.
    expect(mockExecuteQuote).toHaveBeenCalledTimes(1);

    expect(caught).toBeInstanceOf(SwapError);
    // The operator must not be told "it failed, try again" while funds may be in flight.
    expect(caught.message).toMatch(/do NOT re-run/i);
    expect(caught.message).toContain("send_transaction");
    // Original error text preserved as a substring so gateway-bot's categorization still matches.
    expect(caught.message).toContain("504 Gateway Timeout");
  });

  it("does NOT retry after a BTC signature was requested, even on a transient error", async () => {
    mockExecuteQuote.mockImplementation(async ({ callback }: any) => {
      callback?.({ step: 1, type: "sign_bitcoin_transaction", totalSteps: 1 });
      throw new Error("ECONNRESET");
    });

    const { handleSwap } = await import("../../src/commands/swap.js");
    const caught: any = await handleSwap({ ...baseOpts, retry: true }, silentLogger).catch(e => e);

    expect(mockExecuteQuote).toHaveBeenCalledTimes(1);
    expect(caught.message).toMatch(/do NOT re-run/i);
  });

  it("does NOT retry after an ERC20 approve was signed (approve is a broadcast tx too)", async () => {
    // An approve alone is not a double-send, but re-entering the closure while it is still
    // pending races its own nonce — and the next step would broadcast the funds. The line
    // is drawn at the FIRST wallet interaction of any kind.
    mockExecuteQuote.mockImplementation(async ({ callback }: any) => {
      callback?.({ step: 1, type: "approve", totalSteps: 2 });
      throw new Error("rate limit exceeded");
    });

    const { handleSwap } = await import("../../src/commands/swap.js");
    const caught: any = await handleSwap({ ...baseOpts, retry: true }, silentLogger).catch(e => e);

    expect(mockExecuteQuote).toHaveBeenCalledTimes(1);
    expect(caught.message).toContain("approve");
  });

  it("reports the FURTHEST step reached, not the first, when several steps fired", async () => {
    // An EVM ERC20 swap fires approve → send_transaction. If the error names the first
    // step, an operator reads "only the approve went out" and concludes it is safe to
    // re-run — while the funds tx is already on-chain. That misreading is the exact
    // double-send this fix exists to prevent, so the furthest step must win.
    mockExecuteQuote.mockImplementation(async ({ callback }: any) => {
      callback?.({ step: 1, type: "approve", totalSteps: 2 });
      callback?.({ step: 2, type: "send_transaction", totalSteps: 2 });
      throw new Error("registerTx failed: 503 Service Unavailable");
    });

    const { handleSwap } = await import("../../src/commands/swap.js");
    const caught: any = await handleSwap({ ...baseOpts, retry: true }, silentLogger).catch(e => e);

    expect(mockExecuteQuote).toHaveBeenCalledTimes(1);
    expect(caught.message).toContain("send_transaction");
    expect(caught.message).not.toContain("step: approve");
  });

  it("DOES still retry a transient error raised BEFORE anything was broadcast", async () => {
    // The other half of the invariant: the fix must not disable retries wholesale.
    // Nothing has been signed yet here, so re-quoting is free of side effects.
    let quoteCalls = 0;
    mockGetQuote.mockImplementation(async () => {
      if (++quoteCalls === 1) throw new Error("429 Too Many Requests");
      return onrampQuote;
    });

    const { handleSwap } = await import("../../src/commands/swap.js");
    const result = await handleSwap({ ...baseOpts, retry: true }, silentLogger);

    expect(result.type).toBe("confirmed");
    expect(quoteCalls).toBe(2);
    expect(mockExecuteQuote).toHaveBeenCalledTimes(1);
  });

  it("names the EVM owner, not the BTC sender, in the recovery hint on a BTC onramp", async () => {
    // A4: `gateway-cli orders` rejects BTC addresses outright ("BTC address lookups are not
    // supported"), and orders are indexed by the EVM-side owner anyway. Naming the BTC
    // sender leaves the operator unable to check whether an order exists — at exactly the
    // moment they are tempted to re-run, and double-send.
    const { deriveAddress, resolveRecipient } = await import("../../src/chains/index.js");
    vi.mocked(deriveAddress).mockResolvedValue("bc1qsender");
    vi.mocked(resolveRecipient).mockResolvedValue("0xEvmOwner");
    mockExecuteQuote.mockImplementation(async ({ callback }: any) => {
      callback?.({ step: 1, type: "sign_bitcoin_transaction", totalSteps: 1 });
      throw new Error("ECONNRESET");
    });

    const { handleSwap } = await import("../../src/commands/swap.js");
    const caught: any = await handleSwap({ ...baseOpts, retry: true }, silentLogger).catch(e => e);

    expect(caught.message).toContain("gateway-cli orders 0xEvmOwner");
    expect(caught.message).not.toContain("bc1qsender");
  });

  // ─── B1/B2/B3 — polling never reports a committed swap as failed ────────────
  //
  // Once executeQuote returns, the source funds are committed on-chain. The order status
  // is then the ONLY authority on whether the swap failed: an error while *reading* that
  // status says nothing about the swap. Reporting those reads as terminal failures
  // produced false "swap failed" alarms on swaps that had already succeeded — and, via
  // gateway-bot, false critical GitHub issues.

  describe("polling never reports a committed swap as failed", () => {
    it("a transient gateway 5xx while polling does not fail an already-settling swap", async () => {
      // Incident (staging order cbf7296e-340a-438a-981c-75980fafc40c): get-order returned
      // "bungee api error: status=504", yet the swap SUCCEEDED — 74,130 sats confirmed on
      // Bitcoin in tx 48abaaf1…, block 957986. The CLI exited 1 and gateway-bot opened a
      // false critical issue for a swap that worked.
      mockGetOrder
        .mockRejectedValueOnce(new Error("bungee api error: status=504 Gateway Timeout, message=error code: 504\n"))
        .mockResolvedValue(confirmedOrder);

      const { handleSwap } = await import("../../src/commands/swap.js");
      const result = await handleSwap({ ...baseOpts, timeout: 2 }, silentLogger, FAST);

      expect(result.type).toBe("confirmed");
      expect(mockGetOrder).toHaveBeenCalledTimes(2);
    });

    it("a generic fetch failure while polling reports in_flight, not failure", async () => {
      // Incident (staging order a9a49f67-a2de-429f-bb44-2f32034a6f22): the SDK runtime threw
      // its generic FetchError (underlying cause: a local Cloudflare 403) on every get-order.
      // 44.6 USDC had already left the wallet; the order was `inProgress` and later settled
      // fine. The CLI must not call that a failure — and must hand back the orderId + source
      // tx so the in-flight funds can be followed up.
      mockGetOrder.mockRejectedValue(
        new Error("The request failed and the interceptors did not return an alternative response"),
      );

      const { handleSwap } = await import("../../src/commands/swap.js");
      const result = await handleSwap({ ...baseOpts, timeout: 0.3 }, silentLogger, FAST);

      expect(result.type).toBe("inFlight");
      if (result.type !== "inFlight") return;
      expect(result.data.status).toBe("in_flight");
      expect(result.data.orderId).toBe("order-456");
      expect(result.data.txId).toBe("signed-tx-hex-abc");
      expect(result.data.lastError).toContain("interceptors did not return");
      // Retried rather than bailing out on the first read error.
      expect(mockGetOrder.mock.calls.length).toBeGreaterThan(1);
    });

    it("an order still inProgress at --timeout reports in_flight, not failure", async () => {
      // Previously the poll loop exhausted its retries and threw its internal "pending"
      // sentinel, which surfaced as {"error":{"message":"pending"}} with exit 1 — a terminal
      // failure for a swap that was simply still settling.
      mockGetOrder.mockResolvedValue({ id: "order-456", status: { inProgress: {} } });

      const { handleSwap } = await import("../../src/commands/swap.js");
      const result = await handleSwap({ ...baseOpts, timeout: 0.3 }, silentLogger, FAST);

      expect(result.type).toBe("inFlight");
      if (result.type !== "inFlight") return;
      expect(result.data.orderId).toBe("order-456");
      expect(result.data.txId).toBe("signed-tx-hex-abc");
      // The status was readable throughout — nothing to report as a read error.
      expect(result.data.lastError).toBeUndefined();
      expect(mockGetOrder.mock.calls.length).toBeGreaterThan(1);
    });

    it("retrying read errors does not mask a genuine order failure", async () => {
      // The order status stays the ONE authority on failure: surviving a transient read
      // error must not make us swallow the `failed` status that follows it.
      mockGetOrder
        .mockRejectedValueOnce(new Error("bungee api error: status=504 Gateway Timeout"))
        .mockResolvedValue({ id: "order-456", status: { failed: {} } });

      const { handleSwap } = await import("../../src/commands/swap.js");
      const { SwapError } = await import("../../src/errors.js");

      await expect(
        handleSwap({ ...baseOpts, timeout: 2 }, silentLogger, FAST),
      ).rejects.toThrow(SwapError);
    });

    it("a stalled getOrder read is aborted, so the loop still ends at --timeout", async () => {
      // B2: a stalled TCP connection — the server accepts the request and never answers. Like
      // undici, the mock settles ONLY when its AbortSignal fires. Without a per-attempt signal
      // the await never returns and --timeout is unenforceable; racing a timer instead would
      // leak the socket. The read must be genuinely cancelled.
      mockGetOrder.mockImplementation((_id: string, init?: { signal?: AbortSignal }) =>
        new Promise((_resolve, reject) => {
          init?.signal?.addEventListener("abort", () => reject(init.signal!.reason));
        }),
      );

      const { handleSwap } = await import("../../src/commands/swap.js");
      const startedAt = Date.now();
      const result = await handleSwap({ ...baseOpts, timeout: 0.3 }, silentLogger, FAST);
      const elapsedMs = Date.now() - startedAt;

      // Bounded by --timeout, and the unreadable status is in_flight — never a failure.
      expect(result.type).toBe("inFlight");
      expect(elapsedMs).toBeLessThan(3_000);
      expect(mockGetOrder).toHaveBeenCalledWith("order-456", { signal: expect.any(AbortSignal) });
      if (result.type !== "inFlight") return;
      expect(result.data.lastError).toContain("timed out");
    }, 10_000);

    it("survives a clock that jumps past the deadline mid-iteration (no RangeError)", async () => {
      // B3: the first attempt at B2 kept a `deadline` and derived the per-attempt budget from
      // `deadline - Date.now()`, reading the clock separately from the loop guard. A stall
      // between the two reads makes the window negative, and AbortSignal.timeout(negative)
      // throws RangeError *synchronously* — outside the read's try — so a swap whose funds are
      // already committed exits 1 as a hard failure. That is B1 reintroduced through the clock.
      //
      // There is no clock arithmetic left to break: the loop is driven by abort state alone, so
      // it must survive a clock that lies. Date.now() may now only be *reported*, never obeyed.
      const nowSpy = vi.spyOn(Date, "now");
      try {
        let call = 0;
        nowSpy.mockImplementation(() => {
          call++;
          if (call === 1) return 1_000;  // startedAt
          if (call === 2) return 1_100;  // still inside any plausible window
          return 999_999_999;            // stalled: every later read is "past" any deadline
        });
        mockGetOrder.mockResolvedValue({ id: "order-456", status: { inProgress: {} } });

        const { handleSwap } = await import("../../src/commands/swap.js");
        const result = await handleSwap({ ...baseOpts, timeout: 0.3 }, silentLogger, FAST);

        // Exits cleanly as in-flight rather than throwing RangeError — or "pending".
        expect(result.type).toBe("inFlight");
      } finally {
        nowSpy.mockRestore();
      }
    });
  });

  // ─── B4 — the BTC payout on timeout comes from the order, never from the address ──
  //
  // The only sound source for this order's payout tx is the order's own
  // `pendingBtcPayment`. Scanning the recipient address for an unconfirmed tx is not
  // correlation: gateway-bot reuses ONE BTC address for every swap and runs offramp legs
  // in parallel, so several of its own payouts to that address are unconfirmed at once.

  describe("BTC payout on timeout is taken from the order, never guessed from the address", () => {
    /** A concurrent leg's payout to the same reused address, unconfirmed right now. */
    const OTHER_LEGS_TX = { txid: "9c1d0b7e00000000000000000000000000000000000000000000000000000000", status: { confirmed: false } };
    const OUR_PAYOUT_TXID = "48abaaf1000000000000000000000000000000000000000000000000000000ff";

    it("reports the txid the order says it broadcast, not the first unconfirmed tx to the address", async () => {
      const opts = await setupOfframp();

      // The order tells us exactly which tx it broadcast for THIS order.
      mockGetOrder.mockResolvedValue({
        id: "order-btc-1",
        status: { inProgress: { pendingBtcPayment: { txid: OUR_PAYOUT_TXID, amount: "74130" } } },
      });
      // Meanwhile another leg's payout to the same address is also unconfirmed. An address
      // scan would pick this one and report someone else's tx as our payout.
      mockGetAddressMempoolTxs.mockResolvedValue([OTHER_LEGS_TX]);

      const { handleSwap } = await import("../../src/commands/swap.js");
      const result = await handleSwap({ ...opts, timeout: 0.3 }, silentLogger, FAST);

      expect(result.type).toBe("mempoolPending");
      if (result.type !== "mempoolPending") return;
      expect(result.data.mempoolTxId).toBe(OUR_PAYOUT_TXID);
      expect(result.data.orderId).toBe("order-btc-1");
      // The address heuristic is gone, not merely gated.
      expect(mockGetAddressMempoolTxs).not.toHaveBeenCalled();
    });

    it("reports in_flight with no txid when no status was ever read", async () => {
      const opts = await setupOfframp();

      // Every read failed, so we know nothing about a payout — including whether one exists.
      mockGetOrder.mockRejectedValue(
        new Error("The request failed and the interceptors did not return an alternative response"),
      );
      mockGetAddressMempoolTxs.mockResolvedValue([OTHER_LEGS_TX]);

      const { handleSwap } = await import("../../src/commands/swap.js");
      const result = await handleSwap({ ...opts, timeout: 0.3 }, silentLogger, FAST);

      // An order id that can be followed up beats a confident, wrong txid.
      expect(result.type).toBe("inFlight");
      if (result.type !== "inFlight") return;
      expect(result.data.orderId).toBe("order-btc-1");
      expect(result.data.txId).toBe("0xsettlementhash");
      expect(result.data).not.toHaveProperty("mempoolTxId");
      expect(mockGetAddressMempoolTxs).not.toHaveBeenCalled();
    });
  });
});
