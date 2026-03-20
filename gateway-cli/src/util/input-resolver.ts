import { isAddress, formatUnits } from "viem";
import type { RouteInfo } from "@gobob/bob-sdk";
import { fetchPrice } from "./price-oracle.js";
import { BTC_DECIMALS } from "../config.js";
import { getTokenBalance } from "../chains/index.js";
import { getTokenMetadata } from "../chains/evm.js";

// ─── Chain aliases ───────────────────────────────────────────────────────────

const CHAIN_ALIASES: Record<string, string> = {
  btc: "bitcoin",
  eth: "ethereum",
  arb: "arbitrum",
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

interface TokenMeta {
  address: string;
  symbol: string;
  decimals: number;
  chain: string;
}

interface TokenIndex {
  byChainAndSymbol: Map<string, TokenMeta>;
  byChainAndAddress: Map<string, TokenMeta>;
  chainsBySymbol: Map<string, string[]>;
  tokensByChain: Map<string, TokenMeta[]>;
}

export function buildTokenIndex(routes: RouteInfo[]): TokenIndex {
  const byChainAndSymbol = new Map<string, TokenMeta>();
  const byChainAndAddress = new Map<string, TokenMeta>();
  const chainsBySymbol = new Map<string, string[]>();
  const tokensByChain = new Map<string, TokenMeta[]>();

  const seen = new Set<string>();
  for (const r of routes) {
    for (const [chain, addr] of [[r.srcChain, r.srcToken], [r.dstChain, r.dstToken]] as const) {
      const dedup = `${chain}:${addr}`;
      if (seen.has(dedup)) continue;
      seen.add(dedup);

      const meta = getTokenMetadata(addr, chain);
      const t: TokenMeta = { address: addr, symbol: meta.symbol, decimals: meta.decimals, chain };

      const sym = t.symbol.toUpperCase();
      byChainAndSymbol.set(`${chain}:${sym}`, t);
      byChainAndAddress.set(`${chain}:${addr.toLowerCase()}`, t);

      const chains = chainsBySymbol.get(sym);
      chains ? chains.push(chain) : chainsBySymbol.set(sym, [chain]);

      const list = tokensByChain.get(chain);
      list ? list.push(t) : tokensByChain.set(chain, [t]);
    }
  }

  return { byChainAndSymbol, byChainAndAddress, chainsBySymbol, tokensByChain };
}

export function parseAssetChain(raw: string, routes: RouteInfo[], index?: TokenIndex): ResolvedAsset {
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

export interface ResolvedInputs {
  srcAsset: ResolvedAsset;
  dstAsset: ResolvedAsset;
  atomicUnits: string;
  display: string;
}

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
    const bal = await getTokenBalance(srcAsset.chain, opts.senderAddress, srcAsset.address, {
      feeToken: opts.feeToken,
      feeReserve: opts.feeReserve,
    });
    if (BigInt(bal.allSpendable) === 0n) {
      throw new Error(`No ${srcAsset.symbol} balance found for ${opts.senderAddress}`);
    }
    const display = `${formatUnits(BigInt(bal.allSpendable), srcAsset.decimals)} ${srcAsset.symbol} (all spendable)`;
    return { srcAsset, dstAsset, atomicUnits: bal.allSpendable, display };
  }

  return { srcAsset, dstAsset, atomicUnits: parsed.atomicUnits, display: parsed.display };
}
