import { isAddress } from "viem";
import { fetchPrice } from "./price-oracle.js";
import { BTC_DECIMALS } from "../config.js";
import type { EnrichedRoute, EnrichedToken } from "./route-provider.js";

// ─── Chain aliases ───────────────────────────────────────────────────────────

const CHAIN_ALIASES: Record<string, string> = {
  btc: "bitcoin",
  eth: "ethereum",
  mainnet: "ethereum",
  arb: "arbitrum",
  arb1: "arbitrum",
  "arbitrum-one": "arbitrum",
  bas: "base",
  opt: "optimism",
  oeth: "optimism",
  pol: "polygon",
  bnb: "bsc",
  avax: "avalanche",
};

export function resolveChain(input: string): string {
  const lower = input.toLowerCase();
  return CHAIN_ALIASES[lower] ?? lower;
}

// ─── Asset resolution ────────────────────────────────────────────────────────

const BTC_ZERO_ADDRESS = "0x0000000000000000000000000000000000000000";

export interface ResolvedAsset {
  chain: string;
  address: string;
  symbol: string;
  decimals: number;
}

interface TokenIndex {
  byChainAndSymbol: Map<string, EnrichedToken>;
  byChainAndAddress: Map<string, EnrichedToken>;
  chainsBySymbol: Map<string, string[]>;
  tokensByChain: Map<string, EnrichedToken[]>;
}

export function buildTokenIndex(routes: EnrichedRoute[]): TokenIndex {
  const byChainAndSymbol = new Map<string, EnrichedToken>();
  const byChainAndAddress = new Map<string, EnrichedToken>();
  const chainsBySymbol = new Map<string, string[]>();
  const tokensByChain = new Map<string, EnrichedToken[]>();

  const seen = new Set<string>();
  for (const r of routes) {
    for (const t of [r.srcToken, r.dstToken]) {
      const dedup = `${t.chain}:${t.address}`;
      if (seen.has(dedup)) continue;
      seen.add(dedup);

      const sym = t.symbol.toUpperCase();
      byChainAndSymbol.set(`${t.chain}:${sym}`, t);
      byChainAndAddress.set(`${t.chain}:${t.address.toLowerCase()}`, t);

      const chains = chainsBySymbol.get(sym);
      chains ? chains.push(t.chain) : chainsBySymbol.set(sym, [t.chain]);

      const list = tokensByChain.get(t.chain);
      list ? list.push(t) : tokensByChain.set(t.chain, [t]);
    }
  }

  return { byChainAndSymbol, byChainAndAddress, chainsBySymbol, tokensByChain };
}

export function parseAssetChain(raw: string, routes: EnrichedRoute[], index?: TokenIndex): ResolvedAsset {
  const idx = index ?? buildTokenIndex(routes);
  const colonIdx = raw.indexOf(":");
  const assetPart = colonIdx === -1 ? raw : raw.slice(0, colonIdx);
  const chainPart = colonIdx === -1 ? undefined : raw.slice(colonIdx + 1);

  if (assetPart.toUpperCase() === "BTC" && !chainPart) {
    return { chain: "bitcoin", address: BTC_ZERO_ADDRESS, symbol: "BTC", decimals: BTC_DECIMALS };
  }

  if (!chainPart) {
    const chains = idx.chainsBySymbol.get(assetPart.toUpperCase()) ?? [];
    const suggestions = chains.map((c) => `${assetPart.toUpperCase()}:${c}`).join("  ");
    throw new Error(`chain required for token "${assetPart}".\n\n  Specify a chain:  ${suggestions || "(no routes found)"}`);
  }

  const chain = resolveChain(chainPart);

  if (isAddress(assetPart, { strict: false })) {
    const token = idx.byChainAndAddress.get(`${chain}:${assetPart.toLowerCase()}`);
    if (!token) throw new Error(`unknown token address "${assetPart}" on chain "${chain}" — decimals cannot be determined.\n\n  Use a known token symbol instead, or verify the address is correct.`);
    return { chain, address: assetPart, symbol: token.symbol, decimals: token.decimals };
  }

  const token = idx.byChainAndSymbol.get(`${chain}:${assetPart.toUpperCase()}`);
  if (!token) {
    const available = (idx.tokensByChain.get(chain) ?? []).map((t) => t.symbol).join(", ");
    throw new Error(`unknown token "${assetPart}" on chain "${chain}".\n\n  Available on ${chain}: ${available || "(none)"}\n\n  Run 'gateway-cli routes --tokens ${chain}' to see all tokens.`);
  }
  return { chain, address: token.address, symbol: token.symbol, decimals: token.decimals };
}

// ─── Amount parsing ─────────────────────────────────────────────────────────

export type ParsedAmount =
  | { type: "atomic"; atomicUnits: string; display: string }
  | { type: "all" };

/** Convert human-readable decimal string to atomic units using string math (no floating-point). */
export function humanToAtomic(human: string, decimals: number): string {
  const [intPart, fracPart = ""] = human.split(".");
  const padded = fracPart.padEnd(decimals, "0").slice(0, decimals);
  const raw = intPart + padded;
  return BigInt(raw).toString(); // strips leading zeros
}

const AMOUNT_HELP = `Expected one of:
  0.05BTC       human-readable (converted to atomic)
  100USD        USD value (converted via price oracle)
  5000000       atomic units (satoshis, wei, etc.)
  ALL           max spendable balance`;

export async function parseAmount(
  raw: string,
  srcSymbol: string,
  srcDecimals: number,
): Promise<ParsedAmount> {
  const trimmed = raw.trim();
  if (!trimmed || trimmed.includes(" ")) {
    throw new Error(`Invalid amount "${raw}". ${AMOUNT_HELP}`);
  }

  const upper = trimmed.toUpperCase();

  // ALL → sentinel
  if (upper === "ALL") return { type: "all" };

  // Ends with USD → price oracle
  if (upper.endsWith("USD")) {
    const numStr = trimmed.slice(0, -3);
    const usdValue = parseFloat(numStr);
    if (isNaN(usdValue) || usdValue <= 0) throw new Error(`Invalid amount "${raw}". ${AMOUNT_HELP}`);
    const { priceUsd, source } = await fetchPrice(srcSymbol);
    const humanAmount = usdValue / priceUsd;
    const atomicUnits = humanToAtomic(humanAmount.toFixed(srcDecimals), srcDecimals);
    return {
      type: "atomic",
      atomicUnits,
      display: `${humanAmount.toFixed(8).replace(/\.?0+$/, "")} ${srcSymbol} ($${usdValue} via ${source})`,
    };
  }

  // Ends with source token symbol → human-readable
  if (upper.endsWith(srcSymbol.toUpperCase())) {
    const numStr = trimmed.slice(0, -srcSymbol.length);
    const num = parseFloat(numStr);
    if (isNaN(num) || num <= 0) throw new Error(`Invalid amount "${raw}". ${AMOUNT_HELP}`);
    const atomicUnits = humanToAtomic(numStr, srcDecimals);
    return { type: "atomic", atomicUnits, display: `${numStr} ${srcSymbol}` };
  }

  // Bare integer → atomic units
  if (/^\d+$/.test(trimmed)) {
    const val = BigInt(trimmed);
    if (val <= 0n) throw new Error(`Invalid amount "${raw}". Amount must be positive.`);
    return { type: "atomic", atomicUnits: val.toString(), display: `${trimmed} (atomic)` };
  }

  throw new Error(`Invalid amount "${raw}". ${AMOUNT_HELP}`);
}

// ─── Combined asset + amount resolution ──────────────────────────────────────

export async function resolveSwapInputs(
  src: string,
  dst: string,
  amount: string,
  enriched: EnrichedRoute[],
) {
  const tokenIndex = buildTokenIndex(enriched);
  const srcAsset = parseAssetChain(src, enriched, tokenIndex);
  const dstAsset = parseAssetChain(dst, enriched, tokenIndex);
  const parsed = await parseAmount(amount, srcAsset.symbol, srcAsset.decimals);
  return { srcAsset, dstAsset, parsed };
}
