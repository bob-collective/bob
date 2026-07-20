import { describe, it, expect } from "vitest";
import { formatSend } from "../../src/output.js";

describe("formatSend", () => {
  it("formats a confirmed send with the txid", () => {
    const out = formatSend({ asset: "USDC", chain: "base", amount: "100", to: "0xabc", txId: "0xdead", status: "confirmed" });
    expect(out).toContain("0xdead");
    expect(out).toContain("USDC");
    expect(out).toContain("base");
  });
  it("formats an unsigned BTC PSBT result", () => {
    const out = formatSend({ unsigned: true, chain: "bitcoin", asset: "BTC", amount: "1000", to: "bc1q", psbtBase64: "cHNidA==" });
    expect(out).toContain("cHNidA==");
  });
  it("formats a broadcast (unconfirmed) send with the txid", () => {
    const out = formatSend({ asset: "ETH", chain: "ethereum", amount: "1", to: "0xfoo", txId: "0xbar", status: "broadcast" });
    expect(out).toContain("0xbar");
    expect(out).toContain("Broadcast");
  });
  it("formats an unsigned EVM tx result", () => {
    const out = formatSend({ unsigned: true, chain: "ethereum", asset: "WETH", amount: "1", to: "0xfoo", tx: { from: "0xaaa", to: "0xbbb", value: "1", data: "0x", chainId: 1 } });
    expect(out).toContain("0xbbb");
    expect(out).toContain("1");
  });
});
