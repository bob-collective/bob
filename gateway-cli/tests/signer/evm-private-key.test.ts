import { describe, test, expect } from "vitest";
import { resolveEvmSigner } from "../../src/signer/evm.js";

describe("resolveEvmSigner (private key)", () => {
  const validKey = "0x" + "ab".repeat(32);
  const rpcUrl = "https://rpc.example.com";

  test("returns walletClient and publicClient for a valid private key", async () => {
    const result = await resolveEvmSigner({
      privateKey: validKey,
      unsigned: false,
      rpcUrl,
    });
    expect("walletClient" in result).toBe(true);
    expect("publicClient" in result).toBe(true);
  });

  test("private-key flag takes priority over env var", async () => {
    const envKey = "0x" + "cd".repeat(32);
    const result = await resolveEvmSigner({
      privateKey: validKey,
      envPrivateKey: envKey,
      unsigned: false,
      rpcUrl,
    });
    expect("walletClient" in result).toBe(true);
  });

  test("env var used when flag not set", async () => {
    const result = await resolveEvmSigner({
      envPrivateKey: validKey,
      unsigned: false,
      rpcUrl,
    });
    expect("walletClient" in result).toBe(true);
  });

  test("throws on invalid private key", async () => {
    await expect(
      resolveEvmSigner({
        privateKey: "notahexkey",
        unsigned: false,
        rpcUrl,
      }),
    ).rejects.toThrow();
  });

  test("throws when rpcUrl is missing for private key", async () => {
    await expect(
      resolveEvmSigner({
        privateKey: validKey,
        unsigned: false,
      }),
    ).rejects.toThrow("EVM_RPC_URL is required");
  });

  test("returns unsigned when --unsigned set", async () => {
    const result = await resolveEvmSigner({ unsigned: true });
    expect("unsigned" in result).toBe(true);
  });

  test("throws with helpful message when nothing configured", async () => {
    await expect(
      resolveEvmSigner({ unsigned: false }),
    ).rejects.toThrow("no signer configured for EVM");
  });
});
