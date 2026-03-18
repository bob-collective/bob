// src/signer/evm-private-key.test.ts
import { describe, test, expect } from "vitest";
import { signAndBroadcastEvm } from "../../src/signer/evm.js";

describe("signAndBroadcastEvm", () => {
  test("throws on invalid private key", async () => {
    await expect(
      signAndBroadcastEvm(
        { to: "0xabc", data: "0x", value: "0", chainId: 1 },
        "notahexkey",
        "https://rpc.example.com",
      )
    ).rejects.toThrow("invalid private key");
  });
});
