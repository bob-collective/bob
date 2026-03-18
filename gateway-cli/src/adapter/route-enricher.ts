import type { RouteInfo } from '@gobob/bob-sdk';
import tokenlistJson from '@gobob/tokenlist/tokenlist.json';

export interface EnrichedToken {
  address: string;
  symbol: string;
  decimals: number;
  chain: string;
}

export interface EnrichedRoute {
  srcChain: string;
  dstChain: string;
  srcToken: EnrichedToken;
  dstToken: EnrichedToken;
}

// Hardcoded SDK chain name → chainId mapping.
// Avoids importing SUPPORTED_CHAIN_MAP from tokenlist (which pulls in lodash
// as a transitive devDependency that wouldn't be installed).
const SDK_CHAIN_TO_ID: Record<string, number> = {
  bob: 60808,
  ethereum: 1,
  base: 8453,
  arbitrum: 42161,
  optimism: 10,
  bsc: 56,
  polygon: 137,
  avalanche: 43114,
  sei: 1329,
  soneium: 1868,
  berachain: 80094,
  sonic: 146,
  swellchain: 1923,
  unichain: 130,
  telos: 40,
};

// tokenlist.json is Uniswap-format: { tokens: [...] }
const tokens = tokenlistJson.tokens as Array<{
  address: string; symbol: string; decimals: number; chainId: number;
}>;

const BTC_TOKEN: EnrichedToken = {
  address: 'BTC', symbol: 'BTC', decimals: 8, chain: 'bitcoin',
};

function resolveToken(address: string, sdkChainName: string): EnrichedToken {
  if (sdkChainName === 'bitcoin' || address === 'BTC') return BTC_TOKEN;

  const chainId = SDK_CHAIN_TO_ID[sdkChainName];
  if (chainId) {
    const token = tokens.find(
      t => t.chainId === chainId && t.address.toLowerCase() === address.toLowerCase()
    );
    if (token) {
      return { address: token.address, symbol: token.symbol, decimals: token.decimals, chain: sdkChainName };
    }
  }
  // Fallback: use truncated address as symbol, assume 18 decimals
  return { address, symbol: address.slice(0, 10) + '...', decimals: 18, chain: sdkChainName };
}

export function enrichRoute(route: RouteInfo): EnrichedRoute {
  return {
    srcChain: route.srcChain,
    dstChain: route.dstChain,
    srcToken: resolveToken(route.srcToken, route.srcChain),
    dstToken: resolveToken(route.dstToken, route.dstChain),
  };
}

export function enrichRoutes(routes: RouteInfo[]): EnrichedRoute[] {
  return routes.map(enrichRoute);
}
