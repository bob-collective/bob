import { describe, it, expect } from "vitest";
import { encodeFunctionData, erc20Abi } from "viem";
import { nativeSweepAmount, buildUnsignedEvmTx } from "../../src/chains/evm.js";
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
