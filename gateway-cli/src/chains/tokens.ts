import { getTronTokenMetadata } from './tron/metadata.js';
import { getTokenMetadata as getEvmTokenMetadata } from './evm.js';
import { BTC_DECIMALS } from '../config.js';

export function getTokenMetadata(
  address: string,
  chain: string,
  opts?: { throwOnUnknown?: boolean },
): { symbol: string; decimals: number } {
  if (chain === 'bitcoin' || address === 'BTC') {
    return { symbol: 'BTC', decimals: BTC_DECIMALS };
  }

  if (chain === 'tron') {
    return getTronTokenMetadata(address, opts);
  }

  return getEvmTokenMetadata(address, chain, opts);
}
