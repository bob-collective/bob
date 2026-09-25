import { GatewaySDK, type V4Api } from "@gobob/bob-sdk";

/** Bitcoin uses 8 decimal places (satoshis). */
export const BTC_DECIMALS = 8;

/**
 * Application configuration loaded from environment variables.
 * All values are optional except timeoutMs and slippageBps which have defaults.
 */
export interface Config {
  /** Gateway API base URL (default: production endpoint). */
  apiUrl?: string;
  /** Gateway API key for authenticated requests. */
  apiKey?: string;
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
    apiKey: process.env.GATEWAY_API_KEY?.trim() || undefined,
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
    const { apiUrl, apiKey } = loadConfig();
    _sdk = new GatewaySDK({ basePath: apiUrl, apiKey });
  }
  return _sdk;
}

/**
 * Typed accessor for the SDK's underlying V4 generated client.
 *
 * The SDK doesn't re-expose `createOrderV4` or `registerTxV4` on its own surface,
 * but we need both for the CLI's unsigned/manual signing flows. Centralize the
 * reach-through here so call sites get full V4Api typing without sprinkling
 * `any` around.
 */
export function getApi(): V4Api {
  return getSdk().api;
}
