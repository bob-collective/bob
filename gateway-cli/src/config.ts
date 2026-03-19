import { GatewaySDK } from "@gobob/bob-sdk";

export const BTC_DECIMALS = 8;

export interface Config {
  apiUrl?: string;
  bitcoinPrivateKey?: string;
  evmPrivateKey?: string;
  timeoutMs: number;
  slippageBps: number;
  btcFeeRate?: number;
}

let _config: Config | null = null;

export function loadConfig(): Config {
  if (_config) return _config;
  const feeRate = process.env.BTC_FEE_RATE ? parseInt(process.env.BTC_FEE_RATE, 10) : undefined;
  _config = {
    apiUrl: process.env.GATEWAY_API_URL,
    bitcoinPrivateKey: process.env.BITCOIN_PRIVATE_KEY,
    evmPrivateKey: process.env.EVM_PRIVATE_KEY,
    timeoutMs: 1_800_000,
    slippageBps: 300,
    btcFeeRate: feeRate && !isNaN(feeRate) ? feeRate : undefined,
  };
  return _config;
}

let _sdk: InstanceType<typeof GatewaySDK> | null = null;

export function getSdk(): InstanceType<typeof GatewaySDK> {
  if (!_sdk) {
    const { apiUrl } = loadConfig();
    _sdk = apiUrl ? new GatewaySDK(apiUrl) : new GatewaySDK();
  }
  return _sdk;
}
