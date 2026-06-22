import { describe, it, expect } from "vitest";
import { psbtBase64ToHex, btcSweepAmount } from "../../src/chains/bitcoin.js";

describe("psbtBase64ToHex", () => {
  it("converts base64 PSBT bytes to lowercase hex", () => {
    const hex = "70736274ff01"; // arbitrary bytes
    const base64 = Buffer.from(hex, "hex").toString("base64");
    expect(psbtBase64ToHex(base64)).toBe(hex);
  });
});

describe("btcSweepAmount", () => {
  it("returns confirmed balance minus the fee", () => {
    expect(btcSweepAmount(100_000n, 2_500n)).toBe(97_500n);
  });

  it("throws when the result is at or below dust (546 sats)", () => {
    // 546 confirmed, 0 fee → sweep 546, not strictly above dust
    expect(() => btcSweepAmount(546n, 0n)).toThrow(/too low to cover/i);
  });

  it("throws when the fee exceeds the balance", () => {
    expect(() => btcSweepAmount(1_000n, 1_200n)).toThrow(/too low to cover/i);
  });
});
