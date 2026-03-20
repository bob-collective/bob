import { getEnrichedRoutes } from "../util/route-provider.js";
import { resolveChain } from "../util/input-resolver.js";
import type { EnrichedRoute } from "../util/route-provider.js";

interface ChainJson { canonical: string; aliases: string[]; chainId: number | null; }
interface TokenJson { symbol: string; address: string; decimals: number; }

function deriveChains(routes: EnrichedRoute[]): ChainJson[] {
  const seen = new Set<string>();
  const result: ChainJson[] = [];
  for (const route of routes) {
    for (const chain of [route.srcChain, route.dstChain]) {
      if (!seen.has(chain)) {
        seen.add(chain);
        result.push({ canonical: chain, aliases: [], chainId: null });
      }
    }
  }
  return result.sort((a, b) => a.canonical.localeCompare(b.canonical));
}

function deriveTokens(routes: EnrichedRoute[], chain: string): TokenJson[] {
  const seen = new Set<string>();
  const tokens: TokenJson[] = [];
  for (const route of routes) {
    for (const token of [route.srcToken, route.dstToken]) {
      if (token.chain === chain && !seen.has(token.address)) {
        seen.add(token.address);
        tokens.push({ symbol: token.symbol, address: token.address, decimals: token.decimals });
      }
    }
  }
  if (tokens.length === 0) {
    throw new Error(`no tokens found on chain "${chain}". Run 'gateway-cli routes --chains' to see supported chains.`);
  }
  return tokens;
}

export type RoutesResult =
  | { type: "chains"; data: ChainJson[] }
  | { type: "tokens"; data: TokenJson[] }
  | { type: "routes"; data: EnrichedRoute[] };

export async function handleRoutes(opts: { from?: string; to?: string; chains?: boolean; tokens?: string }): Promise<RoutesResult> {
  const enriched = await getEnrichedRoutes();

  if (opts.chains) return { type: "chains", data: deriveChains(enriched) };

  if (opts.tokens) {
    const canonical = resolveChain(opts.tokens);
    return { type: "tokens", data: deriveTokens(enriched, canonical) };
  }

  let filtered = enriched;
  if (opts.from) {
    const from = opts.from.toLowerCase();
    filtered = filtered.filter((r) => r.srcChain.toLowerCase() === from);
  }
  if (opts.to) {
    const to = opts.to.toLowerCase();
    filtered = filtered.filter((r) => r.dstChain.toLowerCase() === to);
  }

  return { type: "routes", data: filtered };
}
