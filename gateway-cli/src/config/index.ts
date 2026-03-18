import { readFileSync, existsSync } from "node:fs";
import { join } from "node:path";
import { homedir } from "node:os";
import { parse as parseTOML } from "toml";

export interface Config {
  apiUrl: string;
  bitcoinSigner?: string;
  evmSigner?: string;
  bitcoinPrivateKey?: string;
  evmPrivateKey?: string;
  keystorePassword?: string;
  keystorePath?: string;
  evmRpcUrl?: string;
  autoConfirm: boolean;
  noWait: boolean;
  timeoutMs: number;
  slippageBps: number;
  btcFeeRate?: number;
  recipient?: string;
  sender?: string;
  rpc: Record<string, string>;
  cache: { ttl: string };
}

interface TomlConfig {
  signers?: { bitcoin?: string; evm?: string };
  rpc?: Record<string, string>;
  cache?: { ttl?: string };
}

const DEFAULT_API_URL = "https://gateway-api-mainnet.gobob.xyz";
const CONFIG_PATH = process.env.GATEWAY_CONFIG ?? join(homedir(), ".gateway-cli", "config.toml");

function loadTomlConfig(): TomlConfig {
  if (!existsSync(CONFIG_PATH)) return {};
  try {
    return parseTOML(readFileSync(CONFIG_PATH, "utf-8")) as TomlConfig;
  } catch { /* config file is unreadable or invalid TOML — fall back to defaults */ return {}; }
}

export function loadConfig(): Config {
  const toml = loadTomlConfig();
  const timeout = parseInt(process.env.GATEWAY_TIMEOUT ?? "1800", 10);
  const slippage = parseInt(process.env.GATEWAY_SLIPPAGE ?? "300", 10);
  const btcFeeRateRaw = process.env.GATEWAY_BTC_FEE_RATE
    ? parseInt(process.env.GATEWAY_BTC_FEE_RATE, 10)
    : undefined;
  return {
    apiUrl: process.env.GATEWAY_API_URL ?? DEFAULT_API_URL,
    bitcoinSigner: process.env.BITCOIN_SIGNER ?? toml.signers?.bitcoin,
    evmSigner: process.env.EVM_SIGNER ?? toml.signers?.evm,
    bitcoinPrivateKey: process.env.BITCOIN_PRIVATE_KEY,
    evmPrivateKey: process.env.EVM_PRIVATE_KEY,
    keystorePassword: process.env.KEYSTORE_PASSWORD,
    keystorePath: process.env.GATEWAY_KEYSTORE,
    evmRpcUrl: process.env.EVM_RPC_URL,
    recipient: process.env.GATEWAY_RECIPIENT,
    sender: process.env.GATEWAY_SENDER,
    autoConfirm: process.env.GATEWAY_AUTO_CONFIRM === "1",
    noWait: process.env.GATEWAY_NO_WAIT === "1",
    timeoutMs: isNaN(timeout) || timeout <= 0 ? 1800000 : timeout * 1000,
    slippageBps: isNaN(slippage) || slippage < 0 || slippage > 10000 ? 300 : slippage,
    btcFeeRate: btcFeeRateRaw !== undefined && isNaN(btcFeeRateRaw) ? undefined : btcFeeRateRaw,
    rpc: toml.rpc ?? {},
    cache: { ttl: toml.cache?.ttl ?? '24h' },
  };
}
