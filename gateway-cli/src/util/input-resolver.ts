import { isAddress, formatUnits } from "viem";
import type { RouteInfo } from "@gobob/bob-sdk";
import { fetchPrice } from "./price-oracle.js";
import { BTC_DECIMALS } from "../config.js";
import { getChainFamily } from "../chains/index.js";
import { getEvmBalances, getTokenMetadata } from "../chains/evm.js";
import { getBtcBalance } from "../chains/bitcoin.js";
import { getSdk } from "../config.js";

// ─── Chain aliases ───────────────────────────────────────────────────────────

/**
 * Map of common chain abbreviations to their canonical names.
 * Users can use these shortcuts when specifying chains (e.g., --src btc).
 */
export const CHAIN_ALIASES: Record<string, string> = {
  btc: "bitcoin",
  eth: "ethereum",
  arb: "arbitrum",
  pol: "polygon",
  bnb: "bsc",
  avax: "avalanche",
};

/**
 * Resolve a chain input to its canonical name.
 * Checks aliases first, then returns lowercase input if no alias matches.
 */
export function resolveChain(input: string): string {
  const lower = input.toLowerCase();
  return CHAIN_ALIASES[lower] ?? lower;
}

// ─── Asset resolution ────────────────────────────────────────────────────────

/** Zero address used as placeholder for BTC (which has no contract address). */
const BTC_ZERO_ADDRESS = "0x0000000000000000000000000000000000000000";

/**
 * Resolved asset information with chain, address, symbol, and decimals.
 * Used internally to represent both BTC and EVM tokens uniformly.
 */
export interface ResolvedAsset {
  chain: string;
  address: string;
  symbol: string;
  decimals: number;
}

interface TokenMeta {
  address: string;
  symbol: string;
  decimals: number;
  chain: string;
}

interface TokenIndex {
  byChainAndSymbol: Map<string, TokenMeta>;
  byChainAndAddress: Map<string, TokenMeta>;
}

/**
 * Build an index of tokens from routes for fast lookup by symbol or address.
 * Caches token metadata to avoid repeated lookups during swap resolution.
 */
export function buildTokenIndex(routes: RouteInfo[]): TokenIndex {
  const byChainAndSymbol = new Map<string, TokenMeta>();
  const byChainAndAddress = new Map<string, TokenMeta>();

  const seen = new Set<string>();
  for (const r of routes) {
    for (const [chain, addr] of [[r.srcChain, r.srcToken], [r.dstChain, r.dstToken]] as const) {
      const dedup = `${chain}:${addr}`;
      if (seen.has(dedup)) continue;
      seen.add(dedup);

      try {
        const meta = getTokenMetadata(addr, chain);
        const t: TokenMeta = { address: addr, symbol: meta.symbol, decimals: meta.decimals, chain };
        byChainAndSymbol.set(`${chain}:${meta.symbol.toUpperCase()}`, t);
        byChainAndAddress.set(`${chain}:${addr.toLowerCase()}`, t);
      } catch (err) {
        // Skip tokens not in tokenlist to avoid guessed decimals in amount calculations.
        // Re-throw unexpected errors (bugs, corrupted data, etc.)
        if (!(err instanceof Error && err.message.startsWith("Unknown token"))) throw err;
      }
    }
  }

  return { byChainAndSymbol, byChainAndAddress };
}

/**
 * Parse an asset string (e.g., "BTC", "USDC:base", "0x...:ethereum") into a ResolvedAsset.
 * BTC without chain defaults to bitcoin. Other tokens require chain specification.
 * @param raw - Asset string in format "SYMBOL" or "SYMBOL:chain" or "address:chain"
 * @param routes - Available routes for token lookup
 * @param index - Optional pre-built token index for performance
 */
export function parseAssetChain(raw: string, routes: RouteInfo[], index?: TokenIndex): ResolvedAsset {
  const idx = index ?? buildTokenIndex(routes);
  const colonIdx = raw.indexOf(":");
  const assetPart = colonIdx === -1 ? raw : raw.slice(0, colonIdx);
  const chainPart = colonIdx === -1 ? undefined : raw.slice(colonIdx + 1);

  if (assetPart.toUpperCase() === "BTC" && !chainPart) {
    return { chain: "bitcoin", address: BTC_ZERO_ADDRESS, symbol: "BTC", decimals: BTC_DECIMALS };
  }

  if (!chainPart) {
    const sym = assetPart.toUpperCase();
    const chains = [...new Set(routes.flatMap(r => {
      const hits: string[] = [];
      if (getTokenMetadata(r.srcToken, r.srcChain, { throwOnUnknown: false }).symbol.toUpperCase() === sym) hits.push(r.srcChain);
      if (getTokenMetadata(r.dstToken, r.dstChain, { throwOnUnknown: false }).symbol.toUpperCase() === sym) hits.push(r.dstChain);
      return hits;
    }))];
    const suggestions = chains.map((c) => `${sym}:${c}`).join("  ");
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
    const available = [...new Set(routes.flatMap(r => {
      const hits: string[] = [];
      if (r.srcChain === chain) hits.push(getTokenMetadata(r.srcToken, r.srcChain, { throwOnUnknown: false }).symbol);
      if (r.dstChain === chain) hits.push(getTokenMetadata(r.dstToken, r.dstChain, { throwOnUnknown: false }).symbol);
      return hits;
    }))].join(", ");
    throw new Error(`unknown token "${assetPart}" on chain "${chain}".\n\n  Available on ${chain}: ${available || "(none)"}\n\n  Run 'gateway-cli routes --tokens ${chain}' to see all tokens.`);
  }
  return { chain, address: token.address, symbol: token.symbol, decimals: token.decimals };
}

// ─── Amount parsing ─────────────────────────────────────────────────────────

/**
 * Parsed amount result. Either atomic units (for specific amounts) or "all" (for max balance).
 */
export type ParsedAmount =
  | { type: "atomic"; atomicUnits: string; display: string }
  | { type: "all" };

/**
 * Convert a human-readable decimal amount to atomic units (smallest denomination).
 * Uses string manipulation to avoid floating-point precision issues.
 * @param human - Human-readable amount (e.g., "0.05" for 0.05 ETH)
 * @param decimals - Token decimals (e.g., 18 for ETH, 6 for USDC)
 * @returns Atomic units as string (e.g., "50000000000000000" for 0.05 ETH)
 */
export function humanToAtomic(human: string, decimals: number): string {
  const [intPart, fracPart = ""] = human.split(".");
  const padded = fracPart.padEnd(decimals, "0").slice(0, decimals);
  const raw = intPart + padded;
  return BigInt(raw).toString();
}

/** User-facing help message for valid amount formats. */
const AMOUNT_HELP = `Expected one of:
  0.05BTC     Amount in BTC (e.g., 0.05, 1.5, 10)
  100USD      USD dollar amount (converted at market price)
  5000000     Raw atomic units (satoshis for BTC, wei for ETH)
  ALL         Your entire spendable balance`;

/**
 * Parse an amount string into atomic units.
 * Supports multiple formats: token suffix (0.05BTC), USD suffix (100USD),
 * atomic units (5000000), or ALL for max balance.
 * @param raw - Raw amount string from user input
 * @param srcSymbol - Source token symbol for suffix matching
 * @param srcDecimals - Source token decimals for conversion
 */
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

  // ALL → max spendable balance
  if (upper === "ALL") return { type: "all" };

  // USD suffix → fetch price and convert
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

  // Token symbol suffix → human-readable amount
  if (upper.endsWith(srcSymbol.toUpperCase())) {
    const numStr = trimmed.slice(0, -srcSymbol.length);
    if (/^\d+(\.\d+)?$/.test(numStr)) {
      const num = parseFloat(numStr);
      if (isNaN(num) || num <= 0) throw new Error(`Invalid amount "${raw}". ${AMOUNT_HELP}`);
      const atomicUnits = humanToAtomic(numStr, srcDecimals);
      return { type: "atomic", atomicUnits, display: `${numStr} ${srcSymbol}` };
    }
  }

  // Bare integer → atomic units (no conversion)
  if (/^\d+$/.test(trimmed)) {
    const val = BigInt(trimmed);
    if (val <= 0n) throw new Error(`Invalid amount "${raw}". Amount must be positive.`);
    return { type: "atomic", atomicUnits: val.toString(), display: `${trimmed} (atomic)` };
  }

  throw new Error(`Invalid amount "${raw}". ${AMOUNT_HELP}`);
}

// ─── Combined asset + amount resolution ──────────────────────────────────────

/**
 * Combined result of resolving swap inputs: source/destination assets and atomic amount.
 */
export interface ResolvedInputs {
  srcAsset: ResolvedAsset;
  dstAsset: ResolvedAsset;
  atomicUnits: string;
  display: string;
}

/**
 * Resolve all swap inputs (source, destination, amount) into structured data.
 * Handles asset parsing, amount conversion, and "ALL" balance lookup.
 * @param src - Source asset string (e.g., "BTC", "USDC:base")
 * @param dst - Destination asset string
 * @param amount - Amount string (e.g., "0.05BTC", "100USD", "ALL")
 * @param routes - Available routes for token lookup
 * @param opts - Optional sender address and fee token/reserve for balance calculations
 */
export async function resolveSwapInputs(
  src: string,
  dst: string,
  amount: string,
  routes: RouteInfo[],
  opts?: { senderAddress?: string; feeToken?: string; feeReserve?: string },
): Promise<ResolvedInputs> {
  const tokenIndex = buildTokenIndex(routes);
  const srcAsset = parseAssetChain(src, routes, tokenIndex);
  const dstAsset = parseAssetChain(dst, routes, tokenIndex);
  const parsed = await parseAmount(amount, srcAsset.symbol, srcAsset.decimals);

  if (parsed.type === "all") {
    if (!opts?.senderAddress) {
      throw new Error("--amount ALL requires a sender address. Use --private-key, --sender, or set BITCOIN_PRIVATE_KEY / EVM_PRIVATE_KEY.");
    }
    let allSpendable: string;
    if (getChainFamily(srcAsset.chain) === 'bitcoin') {
      const bal = await getBtcBalance(opts.senderAddress, getSdk());
      allSpendable = bal.allSpendable;
    } else {
      const isNative = srcAsset.address === '0x0000000000000000000000000000000000000000';
      const tokens = isNative ? [] : [{ address: srcAsset.address, symbol: srcAsset.symbol, decimals: srcAsset.decimals }];
      const result = await getEvmBalances(srcAsset.chain, opts.senderAddress, tokens, {
        includeNative: isNative,
        feeToken: opts.feeToken,
        feeReserve: opts.feeReserve,
      });
      allSpendable = isNative ? result.native!.allSpendable : result.tokens![0].allSpendable;
    }
    if (BigInt(allSpendable) === 0n) {
      throw new Error(`No ${srcAsset.symbol} balance found for ${opts.senderAddress}`);
    }
    const display = `${formatUnits(BigInt(allSpendable), srcAsset.decimals)} ${srcAsset.symbol} (all spendable)`;
    return { srcAsset, dstAsset, atomicUnits: allSpendable, display };
  }

  return { srcAsset, dstAsset, atomicUnits: parsed.atomicUnits, display: parsed.display };
}
