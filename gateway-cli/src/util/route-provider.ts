import type { RouteInfo } from '@gobob/bob-sdk';
import { supportedChainsMapping } from '@gobob/bob-sdk';
import tokenlistJson from '@gobob/tokenlist/tokenlist.json';
import { BTC_DECIMALS } from '../config.js';
import { getSdk } from '../config.js';

// ─── Types ───────────────────────────────────────────────────────────────────

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

// ─── Tokenlist index ─────────────────────────────────────────────────────────

type TokenEntry = { address: string; symbol: string; decimals: number; chainId: number };

const BTC: EnrichedToken = { address: 'BTC', symbol: 'BTC', decimals: BTC_DECIMALS, chain: 'bitcoin' };

interface AddressEntry {
  canonical: TokenEntry;
  uniform: boolean;
  byChainId: Map<number, TokenEntry>;
}

const tokenIndex = new Map<string, AddressEntry>();
for (const t of tokenlistJson.tokens as TokenEntry[]) {
  const key = t.address.toLowerCase();
  const existing = tokenIndex.get(key);
  if (existing) {
    existing.byChainId.set(t.chainId, t);
    if (existing.uniform && (t.symbol !== existing.canonical.symbol || t.decimals !== existing.canonical.decimals)) {
      existing.uniform = false;
    }
  } else {
    tokenIndex.set(key, {
      canonical: t,
      uniform: true,
      byChainId: new Map([[t.chainId, t]]),
    });
  }
}

// ─── Route enrichment ────────────────────────────────────────────────────────

function buildChainIdMap(routes: RouteInfo[]): Record<string, number> {
  const map: Record<string, number> = {};
  for (const { srcChain, srcToken, dstChain, dstToken } of routes) {
    for (const [chain, addr] of [[srcChain, srcToken], [dstChain, dstToken]] as const) {
      if (chain === 'bitcoin' || chain in map) continue;
      const entry = tokenIndex.get(addr.toLowerCase());
      if (entry?.byChainId.size === 1) map[chain] = entry.canonical.chainId;
    }
  }
  return map;
}

function resolveToken(address: string, chain: string, chainIds: Record<string, number>): EnrichedToken | null {
  if (chain === 'bitcoin' || address === 'BTC') return BTC;

  const entry = tokenIndex.get(address.toLowerCase());
  if (!entry) {
    process.stderr.write(`Warning: Token ${address} on "${chain}" not found in tokenlist — skipping route\n`);
    return null;
  }

  if (entry.uniform) {
    return { address: entry.canonical.address, symbol: entry.canonical.symbol, decimals: entry.canonical.decimals, chain };
  }

  const chainId = chainIds[chain];
  const token = chainId !== undefined ? entry.byChainId.get(chainId) : undefined;
  if (token) return { address: token.address, symbol: token.symbol, decimals: token.decimals, chain };

  process.stderr.write(`Warning: Token ${address} on "${chain}" is ambiguous in tokenlist and chain ID is unknown — skipping route\n`);
  return null;
}

function enrichRoutes(routes: RouteInfo[]): EnrichedRoute[] {
  const chainIds = buildChainIdMap(routes);
  const result: EnrichedRoute[] = [];
  for (const r of routes) {
    const srcToken = resolveToken(r.srcToken, r.srcChain, chainIds);
    const dstToken = resolveToken(r.dstToken, r.dstChain, chainIds);
    if (srcToken && dstToken) {
      result.push({ srcChain: r.srcChain, dstChain: r.dstChain, srcToken, dstToken });
    }
  }
  return result;
}

// ─── Native token metadata ───────────────────────────────────────────────────

const NATIVE_TOKENS: Record<string, { symbol: string; decimals: number }> = Object.fromEntries(
  Object.entries(supportedChainsMapping).map(([name, chain]) => [
    name,
    { symbol: chain.nativeCurrency.symbol, decimals: chain.nativeCurrency.decimals },
  ]),
);

export function getNativeToken(chain: string): { symbol: string; decimals: number } {
  const token = NATIVE_TOKENS[chain];
  if (!token) throw new Error(`unknown chain "${chain}" — cannot determine native token`);
  return token;
}

// ─── Public API ──────────────────────────────────────────────────────────────

/** Fetch routes from SDK and enrich with tokenlist metadata. */
export async function getEnrichedRoutes(): Promise<EnrichedRoute[]> {
  const sdk = getSdk();
  const routes = await sdk.getRoutes();
  return enrichRoutes(routes);
}
