import type { RouteInfo } from '@gobob/bob-sdk';
import { getSdk } from '../config.js';

// ─── Route helpers ──────────────────────────────────────────────────────────

/**
 * Extract unique chain names from all routes.
 * Returns deduplicated list of source and destination chains.
 */
export function getUniqueChains(routes: RouteInfo[]): string[] {
  return [...new Set(routes.flatMap(r => [r.srcChain, r.dstChain]))];
}

/**
 * Get unique token addresses for a specific chain.
 * Excludes BTC placeholder address. Results are lowercase and deduplicated.
 * 
 * @param chain - Chain name to filter tokens
 * @param routes - All available routes
 * @returns Array of token addresses on the chain
 */
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
export async function getTokensForChain(chain: string, routes: RouteInfo[]): Promise<Array<{ address: string; symbol: string; decimals: number }>> {
  // Dynamic import to avoid circular dependency at module load time
  const { getTokenMetadata } = await import('../chains/evm.js');
  const addrs = getTokenAddressesForChain(chain, routes);
  const tokens: Array<{ address: string; symbol: string; decimals: number }> = [];
  for (const addr of addrs) {
    try {
      const meta = getTokenMetadata(addr, chain);
      tokens.push({ address: addr, symbol: meta.symbol, decimals: meta.decimals });
    } catch {
      // Skip tokens not in tokenlist — avoids fabricating decimals for balance display
    }
  }
  return tokens;
}

// ─── Public API ──────────────────────────────────────────────────────────────

/**
 * Fetch available swap routes from Gateway SDK.
 * Returns raw RouteInfo array without enrichment.
 */
export async function getRoutes(): Promise<RouteInfo[]> {
  const sdk = getSdk();
  return sdk.getRoutes();
}
