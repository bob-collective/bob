import type { RouteInfo } from "@gobob/bob-sdk";
import { getRoutes, getUniqueChains, getTokensForChain } from "../util/route-provider.js";
import { resolveChain } from "../util/input-resolver.js";

interface ChainJson { canonical: string; aliases: string[]; chainId: number | null; }
interface TokenJson { symbol: string; address: string; decimals: number; }

export type RoutesResult =
  | { type: "chains"; data: ChainJson[] }
  | { type: "tokens"; data: TokenJson[] }
  | { type: "routes"; data: RouteInfo[] };

export async function handleRoutes(opts: { from?: string; to?: string; chains?: boolean; tokens?: string }): Promise<RoutesResult> {
  const routes = await getRoutes();

  if (opts.chains) {
    const chains = getUniqueChains(routes).sort();
    return { type: "chains", data: chains.map(c => ({ canonical: c, aliases: [], chainId: null })) };
  }

  if (opts.tokens) {
    const canonical = resolveChain(opts.tokens);
    const tokens = getTokensForChain(canonical, routes);
    if (tokens.length === 0) throw new Error(`no tokens found on chain "${canonical}". Run 'gateway-cli routes --chains' to see supported chains.`);
    return { type: "tokens", data: tokens.map(t => ({ symbol: t.symbol, address: t.address, decimals: t.decimals })) };
  }

  let filtered = routes;
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
