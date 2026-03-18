import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";

vi.mock("node:fs", () => ({
  existsSync: vi.fn(),
  readFileSync: vi.fn(),
}));

import { existsSync, readFileSync } from "node:fs";
import { loadConfig } from "../../src/config/index.js";

const mockedExistsSync = vi.mocked(existsSync);
const mockedReadFileSync = vi.mocked(readFileSync);

describe("loadConfig", () => {
  const originalEnv = process.env;

  beforeEach(() => {
    process.env = { ...originalEnv };
    delete process.env.GATEWAY_API_URL;
    delete process.env.BITCOIN_SIGNER;
    delete process.env.EVM_SIGNER;
    mockedExistsSync.mockReturnValue(false);
  });

  afterEach(() => {
    process.env = originalEnv;
    vi.restoreAllMocks();
  });

  it("returns default API URL when no env var or TOML", () => {
    const config = loadConfig();
    expect(config.apiUrl).toBe("https://gateway-api-mainnet.gobob.xyz");
  });

  it("returns undefined signers when no env var or TOML", () => {
    const config = loadConfig();
    expect(config.bitcoinSigner).toBeUndefined();
    expect(config.evmSigner).toBeUndefined();
  });

  it("uses GATEWAY_API_URL env var", () => {
    process.env.GATEWAY_API_URL = "https://custom-api.example.com";
    const config = loadConfig();
    expect(config.apiUrl).toBe("https://custom-api.example.com");
  });

  it("uses BITCOIN_SIGNER env var", () => {
    process.env.BITCOIN_SIGNER = "my-btc-signer";
    const config = loadConfig();
    expect(config.bitcoinSigner).toBe("my-btc-signer");
  });

  it("uses EVM_SIGNER env var", () => {
    process.env.EVM_SIGNER = "my-evm-signer";
    const config = loadConfig();
    expect(config.evmSigner).toBe("my-evm-signer");
  });

  it("reads signers from TOML config file", () => {
    mockedExistsSync.mockReturnValue(true);
    mockedReadFileSync.mockReturnValue(
      '[signers]\nbitcoin = "toml-btc-signer"\nevm = "toml-evm-signer"\n'
    );
    const config = loadConfig();
    expect(config.bitcoinSigner).toBe("toml-btc-signer");
    expect(config.evmSigner).toBe("toml-evm-signer");
  });

  it("env vars override TOML config", () => {
    mockedExistsSync.mockReturnValue(true);
    mockedReadFileSync.mockReturnValue(
      '[signers]\nbitcoin = "toml-btc-signer"\nevm = "toml-evm-signer"\n'
    );
    process.env.BITCOIN_SIGNER = "env-btc-signer";
    process.env.EVM_SIGNER = "env-evm-signer";
    const config = loadConfig();
    expect(config.bitcoinSigner).toBe("env-btc-signer");
    expect(config.evmSigner).toBe("env-evm-signer");
  });

  it("handles malformed TOML gracefully", () => {
    mockedExistsSync.mockReturnValue(true);
    mockedReadFileSync.mockReturnValue("this is not valid toml {{{}}}");
    const config = loadConfig();
    expect(config.apiUrl).toBe("https://gateway-api-mainnet.gobob.xyz");
    expect(config.bitcoinSigner).toBeUndefined();
    expect(config.evmSigner).toBeUndefined();
  });
});
