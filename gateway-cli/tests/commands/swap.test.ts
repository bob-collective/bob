import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
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
  getApi: vi.fn(() => ({ createOrderV3: mockCreateOrder })),
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

/** Drive the poll loop's backoff sleeps under fake timers until it settles. */
async function drivePoll<T>(promise: Promise<T>): Promise<T> {
  let settled = false;
  const tracked = promise.then(
    (v) => { settled = true; return v; },
    (e) => { settled = true; throw e; },
  );
  tracked.catch(() => {}); // the assertion below owns the rejection
  for (let i = 0; !settled && i < 200; i++) {
    await vi.advanceTimersByTimeAsync(15_000);
  }
  if (!settled) throw new Error("poll loop did not settle — it is not bounded by --timeout");
  return tracked;
}

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
    // Every read carries a per-attempt abort so no single read can outlive --timeout.
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
    const { resolveSwapInputs, parseAssetChain } = await import("../../src/util/input-resolver.js");
    // srcFamily is derived from parseAssetChain(opts.src) → drives variant.
    // Make src EVM so variant resolves to "tokenSwap" (not onramp).
    vi.mocked(parseAssetChain).mockReturnValueOnce({ chain: "ethereum", address: "0xUSDT", symbol: "USDT", decimals: 6 } as any);
    vi.mocked(resolveSwapInputs).mockResolvedValueOnce({
      srcAsset: { chain: "ethereum", address: "0xUSDT", symbol: "USDT", decimals: 6 },
      dstAsset: { chain: "base", address: "0xUSDC", symbol: "USDC", decimals: 6 },
      atomicUnits: "50000000",
      display: "50 USDT",
    } as any);
    // EVM src means the signing-key path runs: getChainFamily("ethereum") → "evm".
    const { getChainFamily, resolveSigner, deriveAddress, resolveRecipient } = await import("../../src/chains/index.js");
    vi.mocked(getChainFamily).mockImplementation((chain: string) => chain === "bitcoin" ? "bitcoin" : "evm");
    // EVM signed path uses { walletClient, publicClient }; pending-nonce check
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

    let caught: any;
    try {
      await handleSwap(
        { ...baseOpts, src: "USDT:ethereum", dst: "USDC:base", privateKey: "0xevmkey" },
        silentLogger,
      );
    } catch (e) {
      caught = e;
    }

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

  // ─── Post-submission polling ───────────────────────────────────────────────
  //
  // Once executeQuote returns, the source funds are committed on-chain. The order
  // status is then the ONLY authority on whether the swap failed: an error while
  // *reading* that status says nothing about the swap. Reporting those reads as
  // terminal failures produced false "swap failed" alarms on swaps that had already
  // succeeded (or were still settling) — see the two incidents cited below.

  describe("polling never reports a committed swap as failed", () => {
    afterEach(() => {
      vi.useRealTimers();
    });

    it("survives the clock crossing the deadline mid-iteration (no RangeError)", async () => {
      // The loop guard and the per-attempt timeout must not read the clock separately: a
      // stall between the two reads makes the remaining window negative, and
      // AbortSignal.timeout(negative) throws RangeError *synchronously* — outside the read's
      // try — so a swap whose source funds are already committed exits 1 as a hard failure.
      // Real timers here: we drive Date.now ourselves to place the stall deterministically.
      const nowSpy = vi.spyOn(Date, "now");
      try {
        let call = 0;
        nowSpy.mockImplementation(() => {
          call++;
          if (call === 1) return 1_000;   // startMs   → deadline = 2_000 (timeout: 1s)
          if (call === 2) return 1_500;   // loop guard: still inside the window
          return 999_999;                 // stalled — every later read is past the deadline
        });
        mockGetOrder.mockResolvedValue({ id: "order-456", status: { inProgress: {} } });

        const { handleSwap } = await import("../../src/commands/swap.js");
        const result = await handleSwap({ ...baseOpts, timeout: 1 }, silentLogger);

        // Exits cleanly as in-flight rather than throwing RangeError.
        expect(result.type).toBe("inFlight");
      } finally {
        nowSpy.mockRestore();
      }
    });

    it("a transient gateway 5xx while polling does not fail an already-settling swap", async () => {
      // Incident (staging order cbf7296e-340a-438a-981c-75980fafc40c): get-order
      // returned "bungee api error: status=504", yet the source tx was confirmed and
      // the BTC payout was broadcast — the order reached `success`. The CLI exited 1
      // and gateway-bot opened a false critical issue for a swap that worked.
      vi.useFakeTimers();
      mockGetOrder
        .mockRejectedValueOnce(new Error("bungee api error: status=504 Gateway Timeout, message=error code: 504\n"))
        .mockResolvedValue(confirmedOrder);

      const { handleSwap } = await import("../../src/commands/swap.js");
      const result = await drivePoll(handleSwap({ ...baseOpts, timeout: 120 }, silentLogger));

      expect(result.type).toBe("confirmed");
      expect(mockGetOrder).toHaveBeenCalledTimes(2);
    });

    it("a generic fetch failure while polling reports in_flight, not failure", async () => {
      // Incident (staging order a9a49f67-a2de-429f-bb44-2f32034a6f22): the SDK runtime
      // threw its generic FetchError (underlying cause: a local Cloudflare 403) on every
      // get-order. 44.6 USDC had already left the wallet and the order was `inProgress`.
      // The CLI must not call that a failure, and must hand back the orderId + source tx
      // so the in-flight funds can be followed up.
      vi.useFakeTimers();
      mockGetOrder.mockRejectedValue(
        new Error("The request failed and the interceptors did not return an alternative response"),
      );

      const { handleSwap } = await import("../../src/commands/swap.js");
      const result = await drivePoll(handleSwap({ ...baseOpts, timeout: 120 }, silentLogger));

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
      // sentinel, which surfaced as {"error":{"message":"pending"}} with exit 1 — a
      // terminal failure for a swap that was simply still settling.
      vi.useFakeTimers();
      mockGetOrder.mockResolvedValue({
        id: "order-456",
        status: { inProgress: { pending_btc_payment: {} } },
      });

      const { handleSwap } = await import("../../src/commands/swap.js");
      const result = await drivePoll(handleSwap({ ...baseOpts, timeout: 60 }, silentLogger));

      expect(result.type).toBe("inFlight");
      if (result.type !== "inFlight") return;
      expect(result.data.orderId).toBe("order-456");
      expect(result.data.txId).toBe("signed-tx-hex-abc");
      // The status was readable throughout — nothing to report as a read error.
      expect(result.data.lastError).toBeUndefined();
      expect(mockGetOrder.mock.calls.length).toBeGreaterThan(1);
    });

    it("retrying read errors does not mask a genuine order failure", async () => {
      // The order status stays the one authority on failure: surviving a transient read
      // error must not make us swallow the `failed` status that follows it.
      vi.useFakeTimers();
      mockGetOrder
        .mockRejectedValueOnce(new Error("bungee api error: status=504 Gateway Timeout"))
        .mockResolvedValue({ id: "order-456", status: { failed: {} } });

      const { handleSwap } = await import("../../src/commands/swap.js");
      const { SwapError } = await import("../../src/errors.js");

      await expect(
        drivePoll(handleSwap({ ...baseOpts, timeout: 120 }, silentLogger)),
      ).rejects.toThrow(SwapError);
    });
  });

  // ─── Per-attempt read timeout ─────────────────────────────────────────────
  //
  // --timeout is a promise to the caller (CI budgets a run against it). The loop's
  // wall-clock deadline only enforces that promise if no single read can outlive it.

  it("a stalled getOrder read is aborted, so the loop still ends at --timeout", async () => {
    // A stalled TCP connection: the server accepts the request and never answers. The
    // request only rejects when its AbortSignal fires — so, like undici, the mock settles
    // on abort and never otherwise. Without a per-attempt signal it never settles at all
    // and the deadline check at the top of the loop is never reached again.
    mockGetOrder.mockImplementation((_id: string, init?: { signal?: AbortSignal }) =>
      new Promise((_resolve, reject) => {
        init?.signal?.addEventListener("abort", () => reject(init.signal!.reason));
      }),
    );

    const { handleSwap } = await import("../../src/commands/swap.js");
    const startedAt = Date.now();
    // Real timers: AbortSignal.timeout is backed by a native timer that fake timers
    // do not control, so the abort has to be observed in real time. 1s keeps it quick.
    const result = await handleSwap({ ...baseOpts, timeout: 1 }, silentLogger);
    const elapsedMs = Date.now() - startedAt;

    // Bounded by --timeout, and the unreadable status is in_flight — never a failure.
    expect(result.type).toBe("inFlight");
    expect(elapsedMs).toBeLessThan(5_000);
    // The read was actually cancelled, not merely raced.
    expect(mockGetOrder).toHaveBeenCalledWith("order-456", { signal: expect.any(AbortSignal) });
    if (result.type !== "inFlight") return;
    expect(result.data.lastError).toContain("timed out");
  }, 10_000);

  // ─── BTC payout attribution on timeout ────────────────────────────────────
  //
  // On an offramp timeout the CLI may report the BTC payout tx. The only sound source
  // for it is the order's own `pendingBtcPayment`. Scanning the recipient address for
  // an unconfirmed tx is not correlation: gateway-bot reuses ONE BTC address for every
  // swap and runs offramp legs in parallel, so several of its own payouts to that
  // address are unconfirmed at the same time.

  describe("BTC payout on timeout is taken from the order, never guessed from the address", () => {
    afterEach(() => {
      vi.useRealTimers();
    });

    /** USDT@ethereum → BTC offramp paying the one BTC address the bot reuses for every swap. */
    async function setupOfframp() {
      const { resolveSwapInputs, parseAssetChain } = await import("../../src/util/input-resolver.js");
      vi.mocked(parseAssetChain).mockReturnValueOnce({ chain: "ethereum", address: "0xUSDT", symbol: "USDT", decimals: 6 } as any);
      vi.mocked(resolveSwapInputs).mockResolvedValueOnce({
        srcAsset: { chain: "ethereum", address: "0xUSDT", symbol: "USDT", decimals: 6 },
        dstAsset: { chain: "bitcoin", address: "BTC", symbol: "BTC", decimals: 8 },
        atomicUnits: "50000000",
        display: "50 USDT",
      } as any);

      const { getChainFamily, resolveSigner, deriveAddress, resolveRecipient } = await import("../../src/chains/index.js");
      vi.mocked(getChainFamily).mockImplementation((chain: string) => chain === "bitcoin" ? "bitcoin" : "evm");
      const publicClient = { getTransactionCount: vi.fn().mockResolvedValue(0) } as any;
      vi.mocked(resolveSigner).mockResolvedValue({ address: "0xSender", walletClient: {} as any, publicClient } as any);
      vi.mocked(deriveAddress).mockResolvedValue("0xSender");
      vi.mocked(resolveRecipient).mockResolvedValue("bc1qshared");

      mockExecuteQuote.mockResolvedValue({
        order: { offramp: { orderId: "order-btc-1", tx: { to: "0xGateway" } } },
        tx: "0xsettlementhash",
      });

      return { ...baseOpts, src: "USDT:ethereum", dst: "BTC", privateKey: "0xevmkey", timeout: 60 };
    }

    /** A concurrent leg's payout to the same reused address, unconfirmed right now. */
    const OTHER_LEGS_TX = { txid: "9c1d0b7e00000000000000000000000000000000000000000000000000000000", status: { confirmed: false } };
    const OUR_PAYOUT_TXID = "48abaaf1000000000000000000000000000000000000000000000000000000ff";

    it("reports the txid the order says it broadcast, not the first unconfirmed tx to the address", async () => {
      vi.useFakeTimers();
      const opts = await setupOfframp();

      // The order tells us exactly which tx it broadcast for THIS order.
      mockGetOrder.mockResolvedValue({
        id: "order-btc-1",
        status: { inProgress: { pendingBtcPayment: { txid: OUR_PAYOUT_TXID, amount: "74130" } } },
      });
      // Meanwhile another leg's payout to the same address is also unconfirmed. An
      // address scan would pick this one and report someone else's tx as our payout.
      mockGetAddressMempoolTxs.mockResolvedValue([OTHER_LEGS_TX]);

      const { handleSwap } = await import("../../src/commands/swap.js");
      const result = await drivePoll(handleSwap(opts, silentLogger));

      expect(result.type).toBe("mempoolPending");
      if (result.type !== "mempoolPending") return;
      expect(result.data.mempoolTxId).toBe(OUR_PAYOUT_TXID);
      expect(result.data.orderId).toBe("order-btc-1");
      // The address heuristic is gone, not merely gated.
      expect(mockGetAddressMempoolTxs).not.toHaveBeenCalled();
    });

    it("reports in_flight with no txid when no status was ever read", async () => {
      vi.useFakeTimers();
      const opts = await setupOfframp();

      // Every read failed, so we know nothing about a payout — including whether one exists.
      mockGetOrder.mockRejectedValue(
        new Error("The request failed and the interceptors did not return an alternative response"),
      );
      mockGetAddressMempoolTxs.mockResolvedValue([OTHER_LEGS_TX]);

      const { handleSwap } = await import("../../src/commands/swap.js");
      const result = await drivePoll(handleSwap(opts, silentLogger));

      // An order id we can follow up beats a confident, wrong txid.
      expect(result.type).toBe("inFlight");
      if (result.type !== "inFlight") return;
      expect(result.data.orderId).toBe("order-btc-1");
      expect(result.data.txId).toBe("0xsettlementhash");
      expect(result.data).not.toHaveProperty("mempoolTxId");
      expect(mockGetAddressMempoolTxs).not.toHaveBeenCalled();
    });
  });
});
