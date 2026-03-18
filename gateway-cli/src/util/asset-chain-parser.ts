import { isAddress } from "viem";
import { resolveChain } from "./chain-ids.js";
import type { EnrichedRoute, EnrichedToken } from "../adapter/route-enricher.js";

const BTC_ZERO_ADDRESS = "0x0000000000000000000000000000000000000000";

export interface ResolvedAsset {
  chain: string;
  address: string;
  symbol: string;
  decimals: number;
}

export function parseAssetChain(raw: string, routes: EnrichedRoute[]): ResolvedAsset {
  const [assetPart, chainPart] = splitAssetChain(raw);

  // BTC is always Bitcoin — the only bare-symbol shortcut
  if (assetPart.toUpperCase() === "BTC" && !chainPart) {
    return { chain: "bitcoin", address: BTC_ZERO_ADDRESS, symbol: "BTC", decimals: 8 };
  }

  // Bare non-BTC symbols require a chain
  if (!chainPart) {
    const chains = findChainsForSymbol(assetPart, routes);
    const suggestions = chains.map((c) => `${assetPart.toUpperCase()}:${c}`).join("  ");
    throw new Error(
      `chain required for token "${assetPart}".\n\n  Specify a chain:  ${suggestions || "(no routes found)"}`,
    );
  }

  const chain = resolveChain(chainPart);

  // 0x address passthrough
  if (isAddress(assetPart, { strict: false })) {
    const token = findByAddress(assetPart, chain, routes);
    return {
      chain,
      address: assetPart,
      symbol: token?.symbol ?? assetPart,
      decimals: token?.decimals ?? 18,
    };
  }

  // Symbol lookup
  const token = findBySymbol(assetPart, chain, routes);
  if (!token) {
    const available = getTokensOnChain(chain, routes).map((t) => t.symbol).join(", ");
    throw new Error(
      `unknown token "${assetPart}" on chain "${chain}".\n\n  Available on ${chain}: ${available || "(none)"}\n\n  Run 'gateway-cli tokens --chain ${chain}' to see all tokens.`,
    );
  }
  return { chain, address: token.address, symbol: token.symbol, decimals: token.decimals };
}

function splitAssetChain(raw: string): [string, string | undefined] {
  const colonIdx = raw.indexOf(":");
  if (colonIdx === -1) return [raw, undefined];
  return [raw.slice(0, colonIdx), raw.slice(colonIdx + 1)];
}

function allTokens(routes: EnrichedRoute[]): EnrichedToken[] {
  const seen = new Set<string>();
  const result: EnrichedToken[] = [];
  for (const r of routes) {
    for (const t of [r.srcToken, r.dstToken]) {
      const key = `${t.chain}:${t.address}`;
      if (!seen.has(key)) { seen.add(key); result.push(t); }
    }
  }
  return result;
}

function getTokensOnChain(chain: string, routes: EnrichedRoute[]): EnrichedToken[] {
  return allTokens(routes).filter((t) => t.chain === chain);
}

function findBySymbol(symbol: string, chain: string, routes: EnrichedRoute[]): EnrichedToken | undefined {
  return getTokensOnChain(chain, routes).find(
    (t) => t.symbol.toUpperCase() === symbol.toUpperCase(),
  );
}

function findByAddress(address: string, chain: string, routes: EnrichedRoute[]): EnrichedToken | undefined {
  return getTokensOnChain(chain, routes).find(
    (t) => t.address.toLowerCase() === address.toLowerCase(),
  );
}

function findChainsForSymbol(symbol: string, routes: EnrichedRoute[]): string[] {
  return [...new Set(
    allTokens(routes)
      .filter((t) => t.symbol.toUpperCase() === symbol.toUpperCase())
      .map((t) => t.chain),
  )];
}
