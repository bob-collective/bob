import { describe, it, expect, vi } from "vitest";
import { encodeFunctionData, erc20Abi } from "viem";
import { nativeSweepAmount, buildUnsignedEvmTx, prepareNativeSweep, sendEvm } from "../../src/chains/evm.js";
import type { EvmSigner } from "../../src/chains/index.js";
import type { ResolvedAsset } from "../../src/util/input-resolver.js";

const TO = "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913";
const FROM = "0x1111111111111111111111111111111111111111";
const USDC: ResolvedAsset = { chain: "base", address: "0x833589fcd6edb6e08f4c7c32d4f71b54bda02913", symbol: "USDC", decimals: 6 };
const ETH: ResolvedAsset = { chain: "base", address: "0x0000000000000000000000000000000000000000", symbol: "ETH", decimals: 18 };

describe("nativeSweepAmount", () => {
  it("deducts exactly 21000 * maxFeePerGas", () => {
    expect(nativeSweepAmount(1_000_000n, 10n)).toBe(1_000_000n - 21_000n * 10n);
  });
  it("returns 0 when the gas cost exceeds the balance", () => {
    expect(nativeSweepAmount(100n, 10n)).toBe(0n);
  });
});

/** Minimal stand-in for an EvmSigner: only the RPC surface a sweep touches. */
function fakeSigner(balance: bigint, feeData: unknown, block: unknown = { baseFeePerGas: 10_000_000_000n }, gas = 21_000n) {
  const sendTransaction = vi.fn().mockResolvedValue(`0x${"ab".repeat(32)}`);
  const signer = {
    address: FROM,
    walletClient: { account: { address: FROM }, chain: { id: 8453 }, sendTransaction, writeContract: vi.fn() },
    publicClient: {
      getBalance: vi.fn().mockResolvedValue(balance),
      estimateFeesPerGas: vi.fn().mockResolvedValue(feeData),
      getBlock: vi.fn().mockResolvedValue(block),
      estimateGas: vi.fn().mockResolvedValue(gas),
    },
  } as unknown as EvmSigner;
  return { signer, sendTransaction, publicClient: (signer as any).publicClient };
}

const BALANCE = 1_000_000_000_000_000_000n;
const TIP = 1_000_000_000n;
const BASE_FEE = 10_000_000_000n;
// 2x base fee + tip: the cap viem would have quoted is only 1.2x base, leaving no headroom.
const SWEEP_CAP = BASE_FEE * 2n + TIP;

describe("prepareNativeSweep", () => {
  it("sizes the sweep against the fee cap and gas it pins on the send", async () => {
    const { signer, publicClient } = fakeSigner(BALANCE, { maxPriorityFeePerGas: TIP });
    const sweep = await prepareNativeSweep(signer, TO);

    expect(sweep.fees).toEqual({ gas: 21_000n, maxFeePerGas: SWEEP_CAP, maxPriorityFeePerGas: TIP });
    expect(sweep.amount).toBe(BALANCE - sweep.fees.gas * SWEEP_CAP);
    // Sized so value + gas * fee cap can never exceed the balance, whatever viem does later.
    expect(sweep.amount + sweep.fees.gas * SWEEP_CAP).toBeLessThanOrEqual(BALANCE);
    // Gas is estimated for the transfer, not assumed to be a plain 21000 transfer.
    expect(publicClient.estimateGas).toHaveBeenCalledWith(expect.objectContaining({ to: TO, value: BALANCE }));
  });

  it("carries base-fee headroom so the tx stays includable after a fee move", async () => {
    const { signer } = fakeSigner(BALANCE, { maxPriorityFeePerGas: TIP });
    const sweep = await prepareNativeSweep(signer, TO);
    const viemDefaultCap = (BASE_FEE * 12n) / 10n + TIP;
    expect(sweep.fees).toMatchObject({ maxFeePerGas: SWEEP_CAP });
    expect(SWEEP_CAP).toBeGreaterThan(viemDefaultCap);
  });

  it("falls back to gasPrice on legacy chains", async () => {
    const { signer } = fakeSigner(BALANCE, { gasPrice: 20_000_000_000n }, {});
    const sweep = await prepareNativeSweep(signer, TO);
    expect(sweep.fees).toEqual({ gas: 21_000n, gasPrice: 20_000_000_000n });
    expect(sweep.amount).toBe(BALANCE - 21_000n * 20_000_000_000n);
  });
});

describe("sendEvm", () => {
  it("pins the sweep fees on the native send instead of letting viem re-estimate", async () => {
    const { signer, sendTransaction } = fakeSigner(BALANCE, { maxPriorityFeePerGas: TIP });
    const sweep = await prepareNativeSweep(signer, TO);
    await sendEvm(signer, ETH, TO, sweep.amount, sweep.fees);

    expect(sendTransaction).toHaveBeenCalledWith(expect.objectContaining({
      to: TO,
      value: sweep.amount,
      gas: 21_000n,
      maxFeePerGas: SWEEP_CAP,
      maxPriorityFeePerGas: TIP,
    }));
  });

  it("leaves fees alone for a non-sweep native send", async () => {
    const { signer, sendTransaction } = fakeSigner(BALANCE, { maxPriorityFeePerGas: TIP });
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
});
