// src/signer/btc-private-key.test.ts
import { describe, test, expect } from "vitest";
import { signPsbtWithPrivateKey } from "../../src/signer/btc.js";

describe("signPsbtWithPrivateKey", () => {
  test("throws on invalid WIF", async () => {
    await expect(
      signPsbtWithPrivateKey("notvalidwif", "dGVzdA==")
    ).rejects.toThrow("invalid private key");
  });

  test("throws on invalid PSBT base64", async () => {
    // mainnet WIF for all-zeros key (test only)
    const testWif = "5HueCGU8rMjxECyDialwujaBkSRp9JM1uRQ6EScHDV3j5x1zYeh";
    await expect(
      signPsbtWithPrivateKey(testWif, "not!!valid!!base64!!")
    ).rejects.toThrow();
  });
});
