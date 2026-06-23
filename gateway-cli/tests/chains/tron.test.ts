import { describe, it, expect, vi } from "vitest";

vi.mock("viem/accounts", () => ({
  privateKeyToAccount: vi.fn(() => ({
    address: "0xAbCdEf0123456789AbCdEf0123456789AbCdEf01",
  })),
}));

import {
  deriveTronAddress,
  hexToTronAddress,
  isValidTronAddress,
  normalizePrivateKey,
  tokenAddressKey,
} from "../../src/chains/tron/addresses.js";
import { getTronTokenMetadata } from "../../src/chains/tron/metadata.js";
import { getTokenMetadata } from "../../src/chains/tokens.js";

const TRON_USDT = "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t";

describe("tron addresses", () => {
  it("normalizes hex private keys with and without 0x", () => {
    const key = "a".repeat(64);
    expect(normalizePrivateKey(key)).toBe(`0x${key}`);
    expect(normalizePrivateKey(`0x${key}`)).toBe(`0x${key}`);
  });

  it("validates known Tron base58 addresses", () => {
    expect(isValidTronAddress(TRON_USDT)).toBe(true);
    expect(isValidTronAddress("0xAbCdEf0123456789AbCdEf0123456789AbCdEf01")).toBe(false);
  });

  it("converts between hex and base58 contract addresses", () => {
    const hex = "0xAbCdEf0123456789AbCdEf0123456789AbCdEf01";
    const base58 = hexToTronAddress(hex);
    expect(isValidTronAddress(base58)).toBe(true);
  });

  it("uses case-sensitive token index keys on Tron", () => {
    expect(tokenAddressKey("tron", TRON_USDT)).toBe(`tron:${TRON_USDT}`);
    expect(tokenAddressKey("ethereum", hexToTronAddress("0xAbCdEf0123456789AbCdEf0123456789AbCdEf01"))).toContain("ethereum:");
  });

  it("derives a base58 Tron address from a hex private key", () => {
    const address = deriveTronAddress("0x" + "1".repeat(64));
    expect(isValidTronAddress(address)).toBe(true);
  });
});

describe("tron token metadata", () => {
  it("returns USDT metadata for the canonical TRC-20 address", () => {
    expect(getTronTokenMetadata(TRON_USDT)).toEqual({ symbol: "USDT", decimals: 6 });
  });

  it("routes Tron tokens through getTokenMetadata", () => {
    expect(getTokenMetadata(TRON_USDT, "tron")).toEqual({ symbol: "USDT", decimals: 6 });
  });

  it("throws for unknown Tron tokens by default", () => {
    expect(() => getTokenMetadata("TUnknownAddress123456789012345678", "tron")).toThrow(/Unknown token/);
  });
});

describe("getChainFamily", () => {
  it("classifies tron separately from evm", async () => {
    const { getChainFamily } = await import("../../src/chains/index.js");
    expect(getChainFamily("tron")).toBe("tron");
    expect(getChainFamily("ethereum")).toBe("evm");
    expect(getChainFamily("bitcoin")).toBe("bitcoin");
  });
});
