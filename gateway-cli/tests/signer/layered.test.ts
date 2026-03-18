// tests/signer/layered.test.ts
import { describe, test, expect, vi } from "vitest";
import { resolveBtcSigner } from "../../src/signer/btc.js";
import { resolveEvmSigner } from "../../src/signer/evm.js";

// Mock the SDK's ScureBitcoinSigner
vi.mock("@gobob/bob-sdk", () => {
  class MockScureBitcoinSigner {
    private keyHex: string;
    constructor(privateKeyHex: string) {
      this.keyHex = privateKeyHex;
    }
    async signAllInputs(psbtHex: string): Promise<string> {
      return `signed:${psbtHex}`;
    }
    async getP2WPKHAddress(): Promise<string> {
      return `bc1q_mock_${this.keyHex.slice(0, 8)}`;
    }
  }
  return { ScureBitcoinSigner: MockScureBitcoinSigner };
});

describe("resolveBtcSigner", () => {
  const hexKey = "a".repeat(64);

  test("--private-key flag takes priority over env var", async () => {
    const result = await resolveBtcSigner({
      privateKey: hexKey,
      envPrivateKey: "b".repeat(64),
      externalSignerCmd: undefined,
      unsigned: false,
    });
    expect("signer" in result).toBe(true);
    if ("signer" in result) {
      expect(result.address).toContain("bc1q_mock_aaaaaaaa");
    }
  });

  test("env var used when flag not set", async () => {
    const result = await resolveBtcSigner({
      privateKey: undefined,
      envPrivateKey: "b".repeat(64),
      externalSignerCmd: undefined,
      unsigned: false,
    });
    expect("signer" in result).toBe(true);
    if ("signer" in result) {
      expect(result.address).toContain("bc1q_mock_bbbbbbbb");
    }
  });

  test("external signer used when no key configured", async () => {
    const result = await resolveBtcSigner({
      privateKey: undefined,
      envPrivateKey: undefined,
      externalSignerCmd: "/path/to/signer",
      unsigned: false,
    });
    expect("signer" in result).toBe(true);
    if ("signer" in result) {
      expect(result.signer.signAllInputs).toBeDefined();
      expect(result.address).toBeUndefined();
    }
  });

  test("unsigned mode returned when --unsigned set", async () => {
    const result = await resolveBtcSigner({
      privateKey: undefined,
      envPrivateKey: undefined,
      externalSignerCmd: undefined,
      unsigned: true,
    });
    expect("unsigned" in result).toBe(true);
  });

  test("throws with helpful message when nothing configured", async () => {
    await expect(
      resolveBtcSigner({ privateKey: undefined, envPrivateKey: undefined, externalSignerCmd: undefined, unsigned: false })
    ).rejects.toThrow("no signer configured for Bitcoin");
  });
});

describe("resolveEvmSigner", () => {
  const validKey = "0x" + "ab".repeat(32);
  const rpcUrl = "https://rpc.example.com";

  test("returns walletClient + publicClient for private key", async () => {
    const result = await resolveEvmSigner({
      privateKey: validKey,
      unsigned: false,
      rpcUrl,
    });
    expect("walletClient" in result).toBe(true);
    expect("publicClient" in result).toBe(true);
  });

  test("returns walletClient + publicClient for external signer", async () => {
    const result = await resolveEvmSigner({
      externalSignerCmd: "echo signed",
      unsigned: false,
      rpcUrl,
    });
    expect("walletClient" in result).toBe(true);
    expect("publicClient" in result).toBe(true);
  });

  test("returns unsigned when --unsigned set", async () => {
    const result = await resolveEvmSigner({ unsigned: true });
    expect("unsigned" in result).toBe(true);
  });

  test("throws with helpful message when nothing configured", async () => {
    await expect(
      resolveEvmSigner({ unsigned: false })
    ).rejects.toThrow("no signer configured for EVM");
  });
});
