import { describe, it, expect } from "vitest";
import { psbtBase64ToHex } from "../../src/chains/bitcoin.js";

describe("psbtBase64ToHex", () => {
  it("converts base64 PSBT bytes to lowercase hex", () => {
    const hex = "70736274ff01"; // arbitrary bytes
    const base64 = Buffer.from(hex, "hex").toString("base64");
    expect(psbtBase64ToHex(base64)).toBe(hex);
  });
});
