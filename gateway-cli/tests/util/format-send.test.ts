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
});
