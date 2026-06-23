import { describe, it, expect, vi } from "vitest";
import type { Logger } from "../../src/output.js";

vi.mock("../../src/config.js", () => ({
  BTC_DECIMALS: 8,
  loadConfig: () => ({ timeoutMs: 1000, slippageBps: 300 }),
  getSdk: () => ({}),
}));

const mockBuildBtcPsbt = vi.fn().mockResolvedValue("cHNidP8B-mock");

vi.mock("../../src/chains/bitcoin.js", () => ({
  buildBtcPsbt: (...args: any[]) => mockBuildBtcPsbt(...args),
  estimateBtcSweepAmount: vi.fn(),
}));

vi.mock("../../src/util/asset-resolver.js", () => ({
  resolveSendAsset: vi.fn().mockResolvedValue({
    chain: "bitcoin", address: "BTC", symbol: "BTC", decimals: 8,
  }),
}));

const noopLog: Logger = { progress: () => {} } as any;

import { resolveSendAmount, handleSend } from "../../src/commands/send.js";

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

const BTC_SENDER = "bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq";
const BTC_TO = "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4";

describe("handleSend BTC --unsigned --from (keyless)", () => {
  it("builds an unsigned PSBT from --from with no private key", async () => {
    const result = await handleSend(
      { asset: "BTC", amount: "1000", to: BTC_TO, from: BTC_SENDER, unsigned: true, wait: false } as any,
      noopLog,
    );
    expect(result.type).toBe("unsigned");
    expect(mockBuildBtcPsbt).toHaveBeenCalledWith(BTC_SENDER, BTC_TO, 1000n, undefined, "default");
    if (result.type === "unsigned") {
      expect((result.data as any).psbtBase64).toBe("cHNidP8B-mock");
    }
  });

  it("rejects an invalid --from address for the asset's chain", async () => {
    await expect(handleSend(
      { asset: "BTC", amount: "1000", to: BTC_TO, from: "0xdeadbeef", unsigned: true, wait: false } as any,
      noopLog,
    )).rejects.toThrow(/valid Bitcoin address/i);
  });

  it("still requires a key when no --from is given", async () => {
    await expect(handleSend(
      { asset: "BTC", amount: "1000", to: BTC_TO, unsigned: true, wait: false } as any,
      noopLog,
    )).rejects.toThrow(/key is required|BITCOIN_PRIVATE_KEY/i);
  });
});
