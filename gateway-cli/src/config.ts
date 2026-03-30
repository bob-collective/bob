import { GatewaySDK } from "@gobob/bob-sdk";

/** Bitcoin uses 8 decimal places (satoshis). */
export const BTC_DECIMALS = 8;

/**
 * Application configuration loaded from environment variables.
 * All values are optional except timeoutMs and slippageBps which have defaults.
 */
export interface Config {
  /** Gateway API base URL (default: production endpoint). */
  apiUrl?: string;
  /** Bitcoin private key in WIF or hex format. */
  bitcoinPrivateKey?: string;
  /** EVM private key in hex format. */
  evmPrivateKey?: string;
  /** Polling timeout in milliseconds (default: 30 minutes). */
  timeoutMs: number;
  /** Default slippage tolerance in basis points (default: 300 = 3%). */
  slippageBps: number;
  /** Bitcoin fee rate in sat/vbyte (default: from mempool.space). */
  btcFeeRate?: number;
}

let _config: Config | null = null;

/**
 * Load configuration from environment variables.
 * Cached after first call for performance.
 */
export function loadConfig(): Config {
  if (_config) return _config;
  const feeRate = process.env.BTC_FEE_RATE ? parseInt(process.env.BTC_FEE_RATE, 10) : undefined;
  _config = {
    apiUrl: process.env.GATEWAY_API_URL,
    bitcoinPrivateKey: process.env.BITCOIN_PRIVATE_KEY,
    evmPrivateKey: process.env.EVM_PRIVATE_KEY,
    timeoutMs: 1_800_000,
    slippageBps: 300,
    btcFeeRate: feeRate != null && !isNaN(feeRate) ? feeRate : undefined,
  };
  return _config;
}

let _sdk: InstanceType<typeof GatewaySDK> | null = null;

/**
 * Get or create Gateway SDK instance.
 * Uses configured API URL or defaults to production.
 * Cached after first call for performance.
 */
export function getSdk(): InstanceType<typeof GatewaySDK> {
  if (!_sdk) {
    const { apiUrl } = loadConfig();
    _sdk = apiUrl ? new GatewaySDK(apiUrl) : new GatewaySDK();
  }
  return _sdk;
}
