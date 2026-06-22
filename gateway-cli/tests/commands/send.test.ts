import { describe, it, expect, vi } from "vitest";

vi.mock("../../src/config.js", () => ({
  BTC_DECIMALS: 8,
  loadConfig: () => ({ timeoutMs: 1000, slippageBps: 300 }),
  getSdk: () => ({}),
}));

import { resolveSendAmount } from "../../src/commands/send.js";

describe("resolveSendAmount", () => {
  it("returns parsed atomic units for a concrete amount", async () => {
    const out = await resolveSendAmount(
      { amount: "1000" } as any,
      { chain: "base", address: "0x0000000000000000000000000000000000000000", symbol: "ETH", decimals: 18 },
      undefined,
      { spendable: async () => 999n },
    );
    expect(out).toBe(1000n);
  });

  it("uses the spendable callback for ALL", async () => {
    const out = await resolveSendAmount(
      { amount: "ALL" } as any,
      { chain: "base", address: "0x0000000000000000000000000000000000000000", symbol: "ETH", decimals: 18 },
      undefined,
      { spendable: async () => 12345n },
    );
    expect(out).toBe(12345n);
  });

  it("throws on ALL with zero spendable", async () => {
    await expect(resolveSendAmount(
      { amount: "ALL" } as any,
      { chain: "base", address: "0x0", symbol: "ETH", decimals: 18 },
      undefined,
      { spendable: async () => 0n },
    )).rejects.toThrow(/no .*balance/i);
  });
});
