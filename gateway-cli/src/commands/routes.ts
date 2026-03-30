import type { RouteInfo } from "@gobob/bob-sdk";
import { getRoutes, getUniqueChains, getTokensForChain } from "../util/route-provider.js";
import { resolveChain, CHAIN_ALIASES } from "../util/input-resolver.js";
import { CHAIN_IDS, getTokenMetadata } from "../chains/evm.js";

interface ChainJson { canonical: string; aliases: string[]; chainId: number | null; }
interface TokenJson { symbol: string; address: string; decimals: number; }
interface RouteJson { srcChain: string; srcToken: string; srcSymbol: string; dstChain: string; dstToken: string; dstSymbol: string; }

/** Routes command result: chains list, tokens list, or routes list. */
export type RoutesResult =
  | { type: "chains"; data: ChainJson[] }
  | { type: "tokens"; data: TokenJson[] }
  | { type: "routes"; data: RouteJson[] };

/**
 * Handle the routes command: list available swap routes, chains, or tokens.
 * Supports filtering by source/destination chain and tokens-by-chain view.
 * 
 * @param opts - Command options including filters and display mode flags
 * @returns Chains, tokens, or routes data based on options
 */
export async function handleRoutes(opts: { from?: string; to?: string; chains?: boolean; tokens?: string }): Promise<RoutesResult> {
  const routes = await getRoutes();
  const knownChains = getUniqueChains(routes).sort();

  // UX-4: Warn when conflicting flags are combined
  if (opts.chains && opts.tokens) {
    console.warn("Warning: --chains overrides --tokens; --tokens will be ignored");
  }
  if (opts.tokens && (opts.from || opts.to)) {
    console.warn("Warning: --tokens ignores --src-chain/--dst-chain filters");
  }

  if (opts.chains) {
    // Build reverse alias map: canonical → [alias1, alias2, ...]
    const aliasMap = new Map<string, string[]>();
    for (const [alias, canonical] of Object.entries(CHAIN_ALIASES)) {
      const arr = aliasMap.get(canonical) ?? [];
      arr.push(alias);
      aliasMap.set(canonical, arr);
    }
    return {
      type: "chains",
      data: knownChains.map(c => ({
        canonical: c,
        aliases: aliasMap.get(c) ?? [],
        chainId: CHAIN_IDS[c] ?? null,
      })),
    };
  }

  if (opts.tokens) {
    const canonical = resolveChain(opts.tokens);
    const tokens = getTokensForChain(canonical, routes);
    if (tokens.length === 0) throw new Error(`no tokens found on chain "${canonical}". Run 'gateway-cli routes --chains' to see supported chains.`);
    return { type: "tokens", data: tokens.map(t => ({ symbol: t.symbol, address: t.address, decimals: t.decimals })) };
  }

  let filtered = routes;
  if (opts.from) {
    const from = resolveChain(opts.from);
    if (!knownChains.includes(from)) {
      console.warn(`Warning: unknown chain '${opts.from}'. Known chains: ${knownChains.join(", ")}`);
      process.exitCode = 1;
    }
    filtered = filtered.filter((r) => r.srcChain.toLowerCase() === from);
  }
  if (opts.to) {
    const to = resolveChain(opts.to);
    if (!knownChains.includes(to)) {
      console.warn(`Warning: unknown chain '${opts.to}'. Known chains: ${knownChains.join(", ")}`);
      process.exitCode = 1;
    }
    filtered = filtered.filter((r) => r.dstChain.toLowerCase() === to);
  }

  return {
    type: "routes",
    data: filtered.map(r => ({
      srcChain: r.srcChain,
      srcToken: r.srcToken,
      srcSymbol: getTokenMetadata(r.srcToken, r.srcChain).symbol,
      dstChain: r.dstChain,
      dstToken: r.dstToken,
      dstSymbol: getTokenMetadata(r.dstToken, r.dstChain).symbol,
    })),
  };
}
