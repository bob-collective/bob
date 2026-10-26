import { describe, it, expect, vi } from "vitest";
import { encodeFunctionData, erc20Abi } from "viem";
import { getChainConfig } from "@gobob/bob-sdk";
import { nativeSweepAmount, buildUnsignedEvmTx, prepareNativeSweep, sendEvm } from "../../src/chains/evm.js";
import type { EvmSigner } from "../../src/chains/index.js";
import type { ResolvedAsset } from "../../src/util/input-resolver.js";

const TO = "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913";
const FROM = "0x1111111111111111111111111111111111111111";
const USDC: ResolvedAsset = { chain: "base", address: "0x833589fcd6edb6e08f4c7c32d4f71b54bda02913", symbol: "USDC", decimals: 6 };
const ETH: ResolvedAsset = { chain: "base", address: "0x0000000000000000000000000000000000000000", symbol: "ETH", decimals: 18 };
// Real chain configs: the OP-stack predeploys live flat on `contracts`, which is exactly the shape
// the L1-fee lookup has to read.
const BASE = getChainConfig("base");
const ETHEREUM = getChainConfig("ethereum");

describe("nativeSweepAmount", () => {
  it("deducts exactly 21000 * maxFeePerGas", () => {
    expect(nativeSweepAmount(1_000_000n, 10n)).toBe(1_000_000n - 21_000n * 10n);
  });

  it("returns 0 when the gas cost exceeds the balance", () => {
    expect(nativeSweepAmount(100n, 10n)).toBe(0n);
  });
});

const BALANCE = 1_000_000_000_000_000_000n;
const TIP = 1_000_000_000n;
const BASE_FEE = 10_000_000_000n;
// 2x base fee + tip: the cap viem would have quoted is only 1.2x base, leaving no headroom.
const SWEEP_CAP = BASE_FEE * 2n + TIP;
const ESTIMATE_GAS = 21_000n;
const PINNED_GAS = ESTIMATE_GAS + (ESTIMATE_GAS * 20n) / 100n;
const ORACLE = BASE.contracts.gasPriceOracle.address;
const L1_FEE = 1_000n;

interface FakeOptions {
  balance?: bigint;
  feeData?: unknown;
  block?: unknown;
  chain?: unknown;
  /** Gas the node reports for a transfer, optionally as a function of the value simulated. */
  gas?: bigint | ((value: bigint) => bigint);
  /** Mirror a contract recipient whose receive/fallback rejects zero-value calls. */
  rejectZeroValue?: boolean;
  l1Fee?: bigint;
}

/**
 * Minimal stand-in for an EvmSigner that behaves like a node where it matters: `estimateGas`
 * enforces the allowance rule geth/reth/anvil do (value + gas * fee cap <= balance, against the
 * caller's balance *after* any state override), and can refuse zero-value calls the way a
 * deposit contract does.
 */
function fakeSigner(opts: FakeOptions = {}) {
  const balance = opts.balance ?? BALANCE;
  const feeData = opts.feeData ?? { maxPriorityFeePerGas: TIP };
  const block = opts.block ?? { baseFeePerGas: BASE_FEE };
  const chain = opts.chain ?? ETHEREUM;
  const gasFor = (value: bigint) => (typeof opts.gas === "function" ? opts.gas(value) : opts.gas ?? ESTIMATE_GAS);

  const estimateGas = vi.fn(async (args: { value?: bigint; stateOverride?: { address: string; balance: bigint }[] }) => {
    const value = args.value ?? 0n;
    const overridden = args.stateOverride?.find(o => o.address.toLowerCase() === FROM.toLowerCase())?.balance;
    if (value + ESTIMATE_GAS * SWEEP_CAP > (overridden ?? balance)) {
      throw new Error("insufficient funds for gas * price + value");
    }
    if (opts.rejectZeroValue && value === 0n) {
      throw new Error("execution reverted: minimum deposit");
    }
    return gasFor(value);
  });

  const sendTransaction = vi.fn().mockResolvedValue(`0x${"ab".repeat(32)}`);
  const signer = {
    address: FROM,
    walletClient: { account: { address: FROM }, chain, sendTransaction, writeContract: vi.fn() },
    publicClient: {
      getBalance: vi.fn().mockResolvedValue(balance),
      estimateFeesPerGas: vi.fn().mockResolvedValue(feeData),
      getBlock: vi.fn().mockResolvedValue(block),
      estimateGas,
      // The L1-fee path goes through viem's readContract → client.request('eth_call').
      request: vi.fn(async () => `0x${(opts.l1Fee ?? 0n).toString(16).padStart(64, "0")}`),
    },
  } as unknown as EvmSigner;

  return { signer, sendTransaction, publicClient: (signer as unknown as { publicClient: Record<string, any> }).publicClient, estimateGas };
}

describe("prepareNativeSweep", () => {
  it("sizes the sweep against the fee cap and gas it pins on the send", async () => {
    const { signer, estimateGas } = fakeSigner();
    const sweep = await prepareNativeSweep(signer, TO);

    expect(sweep.fees).toEqual({ gas: PINNED_GAS, maxFeePerGas: SWEEP_CAP, maxPriorityFeePerGas: TIP });
    expect(sweep.amount).toBe(BALANCE - PINNED_GAS * SWEEP_CAP);
    // Sized so value + gas * fee cap can never exceed the balance, whatever viem does later.
    expect(sweep.amount + sweep.fees.gas * SWEEP_CAP).toBeLessThanOrEqual(BALANCE);
  });

  it("probes with the whole balance while topping the sender up, so the node's allowance check passes", async () => {
    const { signer, estimateGas } = fakeSigner();
    await prepareNativeSweep(signer, TO);

    const probe = estimateGas.mock.calls[0][0];
    expect(probe.value).toBe(BALANCE);
    expect(probe.stateOverride).toEqual([{ address: FROM, balance: expect.any(BigInt) }]);
    expect(probe.stateOverride![0].balance).toBeGreaterThan(BALANCE);

    // The second estimate runs at the amount that will actually be sent, over the real balance.
    const sent = estimateGas.mock.calls[1][0];
    expect(sent.value).toBeGreaterThan(0n);
    expect(sent.stateOverride).toBeUndefined();
    expect((sent.value as bigint) + ESTIMATE_GAS * SWEEP_CAP).toBeLessThanOrEqual(BALANCE);
  });

  it("does not abort a sweep to a recipient that rejects zero-value calls", async () => {
    const { signer } = fakeSigner({ rejectZeroValue: true });
    const sweep = await prepareNativeSweep(signer, TO);

    expect(sweep.amount).toBe(BALANCE - PINNED_GAS * SWEEP_CAP);
    expect(sweep.fees.gas).toBe(PINNED_GAS);
  });

  it("re-estimates at the swept value and buffers, so a value-sensitive recipient cannot run short", async () => {
    // A recipient that spends more gas the larger the value it receives: a single estimate at a
    // small value (25000) would pin an insufficient limit for the ~1 ETH transfer it actually gets.
    const { signer, estimateGas } = fakeSigner({ gas: value => (value > 500_000_000_000_000_000n ? 40_000n : 25_000n) });
    const sweep = await prepareNativeSweep(signer, TO);

    expect(estimateGas).toHaveBeenCalledTimes(2);
    expect(sweep.fees.gas).toBe(48_000n); // 40000 at the real value, +20%
    expect(sweep.amount + sweep.fees.gas * SWEEP_CAP).toBeLessThanOrEqual(BALANCE);
  });

  it("carries base-fee headroom so the tx stays includable after a fee move", async () => {
    const { signer } = fakeSigner();
    const sweep = await prepareNativeSweep(signer, TO);
    const viemDefaultCap = (BASE_FEE * 12n) / 10n + TIP;
    expect(sweep.fees).toMatchObject({ maxFeePerGas: SWEEP_CAP });
    expect(SWEEP_CAP).toBeGreaterThan(viemDefaultCap);
  });

  it("reserves the L1 data fee on OP-stack chains", async () => {
    const { signer, publicClient } = fakeSigner({ chain: BASE, l1Fee: L1_FEE });
    const sweep = await prepareNativeSweep(signer, TO);

    // The oracle's quote is reserved twice (L1 gas price can move before inclusion).
    expect(publicClient.request).toHaveBeenCalledTimes(1);
    const oracleCall = publicClient.request.mock.calls[0][0];
    expect(oracleCall.method).toBe("eth_call");
    expect(oracleCall.params[0].to).toBe(ORACLE);
    expect(sweep.amount).toBe(BALANCE - PINNED_GAS * SWEEP_CAP - L1_FEE * 2n);
  });

  // The real chain configs, so a lookup that silently finds nothing (the regression this
  // guards) fails here instead of shipping as a zero reserve on every OP-stack chain.
  it.each(["base", "optimism", "bob", "unichain", "soneium", "ethereum", "arbitrum", "bsc"])(
    "reserves the L1 fee on %s exactly when its config has a GasPriceOracle",
    async chainName => {
      const chain = getChainConfig(chainName);
      const { signer, publicClient } = fakeSigner({ chain, l1Fee: L1_FEE });
      const sweep = await prepareNativeSweep(signer, TO);
      const hasOracle = Boolean(chain.contracts?.gasPriceOracle);

      expect(publicClient.request).toHaveBeenCalledTimes(hasOracle ? 1 : 0);
      expect(sweep.amount).toBe(BALANCE - PINNED_GAS * SWEEP_CAP - (hasOracle ? L1_FEE * 2n : 0n));
    },
  );

  it("does not reserve an L1 fee on chains with no GasPriceOracle", async () => {
    const { signer, publicClient } = fakeSigner({ chain: ETHEREUM, l1Fee: L1_FEE });
    const sweep = await prepareNativeSweep(signer, TO);
    expect(publicClient.request).not.toHaveBeenCalled();
    expect(sweep.amount).toBe(BALANCE - PINNED_GAS * SWEEP_CAP);
  });

  it("falls back to gasPrice on legacy chains", async () => {
    const { signer } = fakeSigner({ feeData: { gasPrice: 20_000_000_000n }, block: {} });
    const sweep = await prepareNativeSweep(signer, TO);
    expect(sweep.fees).toEqual({ gas: PINNED_GAS, gasPrice: 20_000_000_000n });
    expect(sweep.amount).toBe(BALANCE - PINNED_GAS * 20_000_000_000n);
  });
});

describe("sendEvm", () => {
  it("pins the sweep fees on the native send instead of letting viem re-estimate", async () => {
    const { signer, sendTransaction } = fakeSigner();
    const sweep = await prepareNativeSweep(signer, TO);
    await sendEvm(signer, ETH, TO, sweep.amount, sweep.fees);

    expect(sendTransaction).toHaveBeenCalledWith(expect.objectContaining({
      to: TO,
      value: sweep.amount,
      gas: PINNED_GAS,
      maxFeePerGas: SWEEP_CAP,
      maxPriorityFeePerGas: TIP,
    }));
  });

  it("leaves fees alone for a non-sweep native send", async () => {
    const { signer, sendTransaction } = fakeSigner();
    await sendEvm(signer, ETH, TO, 500n);
    expect(sendTransaction).toHaveBeenCalledWith(expect.objectContaining({ value: 500n }));
    expect((sendTransaction.mock.calls[0][0] as Record<string, unknown>).maxFeePerGas).toBeUndefined();
  });
});

describe("buildUnsignedEvmTx", () => {
  it("builds a native transfer with empty calldata", () => {
    const tx = buildUnsignedEvmTx(ETH, TO, 500n, FROM, 8453);
    expect(tx).toEqual({ from: FROM, to: TO, value: "500", data: "0x", chainId: 8453 });
  });

  it("builds an ERC20 transfer with encoded calldata and zero value", () => {
    const tx = buildUnsignedEvmTx(USDC, TO, 500n, FROM, 8453);
    const expectedData = encodeFunctionData({ abi: erc20Abi, functionName: "transfer", args: [TO as `0x${string}`, 500n] });
    expect(tx).toEqual({ from: FROM, to: USDC.address, value: "0", data: expectedData, chainId: 8453 });
  });

  it("carries the fees a swept amount was sized against", () => {
    const tx = buildUnsignedEvmTx(ETH, TO, 500n, FROM, 8453, { gas: PINNED_GAS, maxFeePerGas: SWEEP_CAP, maxPriorityFeePerGas: TIP });
    expect(tx).toEqual({
      from: FROM, to: TO, value: "500", data: "0x", chainId: 8453,
      gas: PINNED_GAS.toString(), maxFeePerGas: SWEEP_CAP.toString(), maxPriorityFeePerGas: TIP.toString(),
    });
  });

  it("carries gasPrice when the sweep was priced on a legacy chain", () => {
    const tx = buildUnsignedEvmTx(ETH, TO, 500n, FROM, 8453, { gas: PINNED_GAS, gasPrice: 20_000_000_000n });
    expect(tx).toMatchObject({ gas: PINNED_GAS.toString(), gasPrice: "20000000000" });
  });
});
