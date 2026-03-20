import type { RouteInfo } from '@gobob/bob-sdk';
import { getSdk } from '../config.js';

// ─── Route helpers ──────────────────────────────────────────────────────────

/** Unique chain names across all routes. */
export function getUniqueChains(routes: RouteInfo[]): string[] {
  return [...new Set(routes.flatMap(r => [r.srcChain, r.dstChain]))];
}

/** Unique token addresses on a specific chain (deduped, excludes BTC placeholder). */
export function getTokenAddressesForChain(chain: string, routes: RouteInfo[]): string[] {
  const seen = new Set<string>();
  const addrs: string[] = [];
  for (const r of routes) {
    for (const [c, addr] of [[r.srcChain, r.srcToken], [r.dstChain, r.dstToken]] as const) {
      if (c === chain && addr !== 'BTC' && !seen.has(addr.toLowerCase())) {
        seen.add(addr.toLowerCase());
        addrs.push(addr);
      }
    }
  }
  return addrs;
}

/** Unique tokens on a specific chain with metadata (deduped, excludes BTC placeholder). */
export function getTokensForChain(chain: string, routes: RouteInfo[]): Array<{ address: string; symbol: string; decimals: number }> {
  // Lazy import to avoid circular dependency at module load time
  const { getTokenMetadata } = require('../chains/evm.js') as typeof import('../chains/evm.js');
  const addrs = getTokenAddressesForChain(chain, routes);
  return addrs.map(addr => {
    const meta = getTokenMetadata(addr, chain);
    return { address: addr, symbol: meta.symbol, decimals: meta.decimals };
  });
}

// ─── Public API ──────────────────────────────────────────────────────────────

/** Fetch routes from SDK. Returns raw RouteInfo (no enrichment). */
export async function getRoutes(): Promise<RouteInfo[]> {
  const sdk = getSdk();
  return sdk.getRoutes();
}
