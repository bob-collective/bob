import { describe, it, expect, beforeAll, afterAll } from "vitest";
import { spawn, type ChildProcess } from "node:child_process";
import { existsSync } from "node:fs";
import { createPublicClient, createWalletClient, http, parseEther, formatEther } from "viem";
import { privateKeyToAccount } from "viem/accounts";
import { anvil } from "viem/chains";
import { prepareNativeSweep, sendEvm } from "../../src/chains/evm.js";
import type { ResolvedAsset } from "../../src/util/input-resolver.js";

/**
 * End-to-end sweep against a real node, because the thing under test is the node's balance
 * rule (`value + gas * fee cap (+ L1 fee) <= balance`), which a mocked estimateGas cannot model.
 * Runs only where `anvil` is installed; CI without Foundry skips it.
 */
const ANVIL_BIN = process.env.ANVIL_BIN ?? `${process.env.HOME}/.foundry/bin/anvil`;
const RPC_PORT = 8599;
const RPC = `http://localhost:${RPC_PORT}`;
const KEY = ("0x" + "11".repeat(32)) as `0x${string}`;
const TO = "0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC";
const ETH: ResolvedAsset = { chain: "ethereum", address: "0x0000000000000000000000000000000000000000", symbol: "ETH", decimals: 18 };
const GWEI = 1_000_000_000n;

const hasAnvil = existsSync(ANVIL_BIN);

describe.skipIf(!hasAnvil)("native ALL sweep on a real node", () => {
  let node: ChildProcess | undefined;
  const account = privateKeyToAccount(KEY);
  const publicClient = createPublicClient({ chain: anvil, transport: http(RPC) });
  const walletClient = createWalletClient({ account, chain: anvil, transport: http(RPC) });

  beforeAll(async () => {
    node = spawn(ANVIL_BIN, ["--port", String(RPC_PORT), "--base-fee", (10n * GWEI).toString()], { stdio: "ignore" });
    for (let i = 0; i < 100; i++) {
      try {
        await publicClient.getBlockNumber();
        return;
      } catch {
        await new Promise(r => setTimeout(r, 100));
      }
    }
    throw new Error("anvil did not start");
  }, 30_000);

  afterAll(() => {
    node?.kill("SIGKILL");
  });

  it("estimating the whole balance as value is rejected while a fee cap is attached", async () => {
    await publicClient.request({ method: "anvil_setBalance" as never, params: [account.address, "0xde0b6b3a7640000"] as never });
    const balance = await publicClient.getBalance({ address: account.address });

    await expect(publicClient.request({
      method: "eth_estimateGas",
      params: [{ from: account.address, to: TO, value: `0x${balance.toString(16)}`, maxFeePerGas: `0x${(21n * GWEI).toString(16)}` }] as never,
    })).rejects.toThrow();
  });

  it("sizes and broadcasts a full-balance sweep", async () => {
    await publicClient.request({ method: "anvil_setBalance" as never, params: [account.address, "0xde0b6b3a7640000"] as never });
    await publicClient.request({ method: "anvil_setBalance" as never, params: [TO, "0x0"] as never });
    const balance = await publicClient.getBalance({ address: account.address });

    const sweep = await prepareNativeSweep({ address: account.address, publicClient, walletClient } as never, TO);
    const feeCap = "maxFeePerGas" in sweep.fees ? sweep.fees.maxFeePerGas : sweep.fees.gasPrice;
    expect(sweep.amount + sweep.fees.gas * feeCap).toBeLessThanOrEqual(balance);

    const hash = await sendEvm({ address: account.address, publicClient, walletClient } as never, ETH, TO, sweep.amount, sweep.fees);
    const receipt = await publicClient.waitForTransactionReceipt({ hash });
    expect(receipt.status).toBe("success");

    expect(await publicClient.getBalance({ address: TO })).toBe(sweep.amount);
    // Headroom is dust: it stays in the wallet rather than being charged.
    console.log(`swept ${formatEther(sweep.amount)} ETH, left ${formatEther(await publicClient.getBalance({ address: account.address }))} ETH`);
    expect(await publicClient.getBalance({ address: account.address })).toBeLessThan(parseEther("0.01"));
  }, 60_000);
});
