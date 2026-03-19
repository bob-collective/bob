import { isAddress } from "viem";
import { fetchPrice } from "./price-oracle.js";
import { BTC_DECIMALS } from "../config.js";
import type { EnrichedRoute, EnrichedToken } from "./route-provider.js";

// ─── Chain aliases ───────────────────────────────────────────────────────────

const CHAIN_ALIASES: Record<string, string> = {
  btc: "bitcoin", eth: "ethereum", mainnet: "ethereum",
  arb: "arbitrum", arb1: "arbitrum", "arbitrum-one": "arbitrum",
  bas: "base", opt: "optimism", oeth: "optimism",
  pol: "polygon", bnb: "bsc", avax: "avalanche",
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

// ─── Amount resolution ───────────────────────────────────────────────────────

export interface ParsedAmount {
  atomicUnits: string;
  display: string;
}

function toAtomicUnits(human: number, decimals: number): string {
  const factor = 10n ** BigInt(decimals);
  return (BigInt(Math.round(human * Number(factor)))).toString();
}

export async function parseAmountUsd(value: string, srcSymbol: string, srcDecimals: number): Promise<ParsedAmount> {
  const usdValue = parseFloat(value);
  const { priceUsd, source } = await fetchPrice(srcSymbol);
  const humanAmount = usdValue / priceUsd;
  return {
    atomicUnits: toAtomicUnits(humanAmount, srcDecimals),
    display: `${humanAmount.toFixed(8).replace(/\.?0+$/, "")} ${srcSymbol} ($${usdValue} via ${source})`,
  };
}

export interface AmountInput {
  amount?: string;
  amountAtomic?: string;
  amountUsd?: string;
}

export async function resolveAmount(input: AmountInput, srcSymbol: string, srcDecimals: number): Promise<ParsedAmount> {
  if (input.amountAtomic) return { atomicUnits: input.amountAtomic, display: `${input.amountAtomic} (atomic)` };
  if (input.amountUsd) return parseAmountUsd(input.amountUsd, srcSymbol, srcDecimals);
  const human = parseFloat(input.amount!);
  return { atomicUnits: toAtomicUnits(human, srcDecimals), display: `${input.amount} ${srcSymbol}` };
}

// ─── Combined asset + amount resolution ──────────────────────────────────────

export async function resolveSwapInputs(
  src: string,
  dst: string,
  amount: AmountInput,
  enriched: EnrichedRoute[],
) {
  const tokenIndex = buildTokenIndex(enriched);
  const srcAsset = parseAssetChain(src, enriched, tokenIndex);
  const dstAsset = parseAssetChain(dst, enriched, tokenIndex);
  const parsed = await resolveAmount(amount, srcAsset.symbol, srcAsset.decimals);
  return { srcAsset, dstAsset, parsed };
}
