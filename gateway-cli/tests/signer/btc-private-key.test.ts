// tests/signer/btc-private-key.test.ts
import { describe, test, expect, vi } from "vitest";
import { resolveBtcSigner, decodeWif } from "../../src/signer/btc.js";

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

describe("resolveBtcSigner with hex private key", () => {
  const hexKey = "a".repeat(64); // valid 32-byte hex key

  test("returns a BitcoinSigner with signAllInputs", async () => {
    const result = await resolveBtcSigner({
      privateKey: hexKey,
      unsigned: false,
    });
    expect("signer" in result).toBe(true);
    if ("signer" in result) {
      expect(result.signer.signAllInputs).toBeDefined();
      expect(typeof result.signer.signAllInputs).toBe("function");
    }
  });

  test("returns an address for private-key signer", async () => {
    const result = await resolveBtcSigner({
      privateKey: hexKey,
      unsigned: false,
    });
    expect("signer" in result).toBe(true);
    if ("signer" in result) {
      expect(result.address).toBeDefined();
      expect(result.address).toContain("bc1q_mock_");
    }
  });

  test("signAllInputs produces output", async () => {
    const result = await resolveBtcSigner({
      privateKey: hexKey,
      unsigned: false,
    });
    if ("signer" in result && result.signer.signAllInputs) {
      const signed = await result.signer.signAllInputs("deadbeef");
      expect(signed).toBe("signed:deadbeef");
    }
  });

  test("accepts 0x-prefixed hex key", async () => {
    const result = await resolveBtcSigner({
      privateKey: `0x${hexKey}`,
      unsigned: false,
    });
    expect("signer" in result).toBe(true);
  });

  test("throws on invalid key format", async () => {
    await expect(
      resolveBtcSigner({
        privateKey: "not-a-valid-key",
        unsigned: false,
      }),
    ).rejects.toThrow("invalid private key");
  });
});

describe("resolveBtcSigner with --unsigned", () => {
  test("returns { unsigned: true }", async () => {
    const result = await resolveBtcSigner({
      unsigned: true,
    });
    expect(result).toEqual({ unsigned: true });
  });
});

describe("decodeWif", () => {
  test("decodes a mainnet uncompressed WIF key", () => {
    // Known test vector: private key all zeros (0x00...01) — but we use a real one
    // WIF for private key 0000000000000000000000000000000000000000000000000000000000000001
    // mainnet compressed: KwDiBf89QgGbjEhKnhXJuH7LrciVrZi3qYjgd9M7rFU73sVHnoWn
    const hex = decodeWif("KwDiBf89QgGbjEhKnhXJuH7LrciVrZi3qYjgd9M7rFU73sVHnoWn");
    expect(hex).toBe("0000000000000000000000000000000000000000000000000000000000000001");
  });

  test("decodes a mainnet uncompressed WIF", () => {
    // WIF for private key 0000000000000000000000000000000000000000000000000000000000000001
    // mainnet uncompressed: 5HpHagT65TZzG1PH3CSu63k8DbpvD8s5ip4nEB3kEsreAnchuDf
    const hex = decodeWif("5HpHagT65TZzG1PH3CSu63k8DbpvD8s5ip4nEB3kEsreAnchuDf");
    expect(hex).toBe("0000000000000000000000000000000000000000000000000000000000000001");
  });

  test("throws on invalid checksum", () => {
    expect(() => decodeWif("KwDiBf89QgGbjEhKnhXJuH7LrciVrZi3qYjgd9M7rFU73sVHnoWm")).toThrow(
      "invalid checksum",
    );
  });

  test("throws on invalid base58 characters", () => {
    expect(() => decodeWif("0OIl")).toThrow("invalid base58");
  });
});
