import { formatUnits } from "viem";
import { formatBtc } from "@gobob/bob-sdk";
import type { ChainBalance } from "./chains/evm.js";
import { BTC_DECIMALS } from "./config.js";

// ─── Output mode ─────────────────────────────────────────────────────────────

/** Output format mode: JSON for machine-readable, human for formatted text. */
export type OutputMode = "json" | "human";

/** Logger interface for progress and warning messages during operations. */
export interface Logger {
  progress: (msg: string) => void;
  warn: (msg: string) => void;
}

/**
 * Create a logger that outputs to stderr in human mode, silent in JSON mode.
 */
export function createLogger(mode: OutputMode): Logger {
  return {
    progress: (msg) => { if (mode === "human") process.stderr.write(msg + "\n"); },
    warn: (msg) => { if (mode === "human") process.stderr.write(`Warning: ${msg}\n`); },
  };
}

/**
 * Render data to stdout.
 * JSON mode always uses JSON.stringify; human mode uses a custom formatter if provided.
 */
export function render(data: unknown, mode: OutputMode, humanFormatter?: (data: any) => string): void {
  if (mode === "json" || !humanFormatter) {
    console.log(formatJson(data));
  } else {
    console.log(humanFormatter(data));
  }
}

// ─── Types (JSON output shapes) ──────────────────────────────────────────────

/** Successful swap confirmation with full details. */
export interface SwapSuccessJson {
  orderId: string;
  status: "confirmed" | "strategy_skipped" | "strategy_failed";
  srcAmount: string;
  srcAsset: string;
  dstAmount: string;
  dstAsset: string;
  dstChain: string;
  quotedDstAmount: string;
  actualSlippageBps: number;
  txId: string;
  elapsedMs: number;
}

/** Swap submitted successfully, awaiting confirmation. */
export interface SwapSubmittedJson {
  orderId: string;
  status: "submitted";
  srcAmount: string;
  srcAsset: string;
  dstAsset: string;
  dstChain: string;
  txId: string;
}

/** Swap pending in mempool (unconfirmed Bitcoin transaction). */
export interface SwapMempoolPendingJson {
  orderId: string;
  status: "mempool_pending";
  srcAmount: string;
  srcAsset: string;
  dstAsset: string;
  dstChain: string;
  mempoolTxId: string;
}

/** Quote details for preview before swap execution. */
export interface QuoteJson {
  srcAmount: string;
  srcAsset: string;
  dstAmount: string;
  dstAsset: string;
  dstChain: string;
  feeNetworkBtc?: string;
  feeProtocolBps?: number;
  slippageBps: number;
  feeRateSatPerVbyte?: number;
  gasEstimateEth?: string;
  gasRefillEth?: string;
  gasRefillUsd?: string;
}

/** Chain information for routes display. */
export interface ChainJson {
  canonical: string;
  aliases: string[];
  chainId: number | null;
}

/** Token information for routes display. */
export interface TokenJson {
  symbol: string;
  address: string;
  decimals: number;
}

/** Maximum spendable balance information for Bitcoin. */
export interface MaxSpendableJson {
  asset: string;
  chain: string;
  address: string;
  maxSpendableSat: string;
  balanceSat: string;
  estimatedFeeSat: string;
  feeRateSatPerVbyte: number;
}

/** Balance data across multiple chains. */
export interface BalanceJson {
  [chain: string]: {
    address: string;
    balance?: string;
    allSpendable?: string;
    maxSpendable?: string;
    native?: { symbol: string; balance: string; allSpendable?: string };
    tokens?: Array<{ symbol: string; address: string; balance: string; allSpendable?: string }>;
    error?: boolean;
  };
}

/** Data required for swap/quote confirmation display. */
export interface ConfirmationData {
  srcAmount: string;
  srcAsset: string;
  srcDisplay?: string;
  dstAmount: string;
  dstAsset: string;
  dstChain: string;
  feeNetwork?: string;
  feeRateSatPerVbyte?: number;
  gasEstimateEth?: string;
  gasRefillUsd?: string;
  slippageBps: number;
  recipient: string;
}

// ─── Raw balance formatting ─────────────────────────────────────────────────

/**
 * Format raw atomic chain balance data into human-readable BalanceJson entry.
 * Converts BTC balances using formatBtc, EVM balances using formatUnits.
 */
export function formatBalanceRaw(raw: ChainBalance, chain: string): BalanceJson[string] {
  if (raw.error) return { address: raw.address, error: true };

  if (chain === "bitcoin") {
    return {
      address: raw.address,
      balance: raw.balance ? formatBtc(BigInt(raw.balance)) : undefined,
      allSpendable: raw.allSpendable ? formatBtc(BigInt(raw.allSpendable)) : undefined,
    };
  }

  return {
    address: raw.address,
    native: raw.native ? {
      symbol: raw.native.symbol,
      balance: formatUnits(BigInt(raw.native.balance), raw.native.decimals),
      allSpendable: formatUnits(BigInt(raw.native.allSpendable), raw.native.decimals),
    } : undefined,
    tokens: raw.tokens?.map(t => ({
      symbol: t.symbol,
      address: t.address,
      balance: formatUnits(BigInt(t.balance), t.decimals),
      allSpendable: formatUnits(BigInt(t.allSpendable), t.decimals),
    })),
  };
}

/**
 * Convert raw balance map to formatted BalanceJson.
 * Iterates over all chains and formats each balance entry.
 */
export function formatAllBalances(raw: Record<string, ChainBalance>): BalanceJson {
  return Object.fromEntries(
    Object.entries(raw).map(([chain, data]) => [chain, formatBalanceRaw(data, chain)]),
  );
}

// ─── Formatters (stdout) ─────────────────────────────────────────────────────

/**
 * Render any data as formatted JSON with 2-space indentation.
 */
export function formatJson(data: unknown): string {
  return JSON.stringify(data, null, 2);
}

/**
 * Render a column-aligned table with header, separator, and rows.
 * @param columns - Column definitions with label and width
 * @param rows - Row data as arrays of cell values
 */
export function formatTable(columns: { label: string; width: number }[], rows: string[][]): string {
  const header = columns.map(c => c.label.padEnd(c.width)).join("");
  const sep = "─".repeat(columns.reduce((sum, c) => sum + c.width, 0));
  const body = rows.map(row =>
    row.map((cell, i) => i < columns.length - 1 ? cell.padEnd(columns[i].width) : cell).join("")
  );
  return [header, sep, ...body].join("\n");
}

/**
 * Format swap/quote confirmation for human-readable display.
 * Shows source/destination amounts, fees, gas estimates, and recipient.
 */
export function formatConfirmation(data: ConfirmationData): string {
  const lines: string[] = ["Swap"];
  const srcLine = data.srcDisplay
    ? `${data.srcAmount} ${data.srcAsset}  (${data.srcDisplay})`
    : `${data.srcAmount} ${data.srcAsset}`;
  lines.push(`  From       ${srcLine}`);
  lines.push(`  To         ~${data.dstAmount} ${data.dstAsset} on ${data.dstChain}`);
  if (data.feeNetwork) lines.push(`  Fee        ${data.feeNetwork}`);
  if (data.feeRateSatPerVbyte !== undefined)
    lines.push(`  Fee rate   ${data.feeRateSatPerVbyte} sat/vbyte`);
  if (data.gasEstimateEth)
    lines.push(`  Gas (est)  ~${data.gasEstimateEth} ETH`);
  if (data.gasRefillUsd) lines.push(`  Gas refill $${data.gasRefillUsd} ETH sent to recipient`);
  lines.push(`  Slippage   ${(data.slippageBps / 100).toFixed(1)}% max  (${data.slippageBps} bps)`);
  lines.push(`  Recipient  ${data.recipient}`);
  return lines.join("\n");
}

/**
 * Format balance data for human-readable display.
 * Shows balances per chain with native tokens and ERC20 tokens.
 */
export function formatBalance(result: BalanceJson): string {
  const lines: string[] = [];
  for (const [chain, data] of Object.entries(result)) {
    lines.push(`${chain}  (${data.address})`);
    if (data.error) {
      lines.push(`  N/A (RPC unreachable)`);
      continue;
    }
    if (data.balance !== undefined) lines.push(`  Balance:       ${data.balance} BTC`);
    if (data.allSpendable !== undefined) lines.push(`  All spendable: ${data.allSpendable} BTC`);
    if (data.maxSpendable !== undefined && data.allSpendable === undefined) lines.push(`  Max spendable: ${data.maxSpendable} BTC`);
    if (data.native) {
      const allSuffix = data.native.allSpendable ? ` (all: ${data.native.allSpendable})` : '';
      lines.push(`  ${data.native.symbol}: ${data.native.balance}${allSuffix}`);
    }
    if (data.tokens) {
      for (const t of data.tokens) {
        const allSuffix = t.allSpendable ? ` (all: ${t.allSpendable})` : '';
        lines.push(`  ${t.symbol}: ${t.balance}${allSuffix}`);
      }
    }
  }
  return lines.length === 0 ? "No chains found." : lines.join("\n");
}

/**
 * Format chain list as a table with canonical names and chain IDs.
 */
export function formatChains(data: ChainJson[]): string {
  return formatTable(
    [{ label: "Chain", width: 20 }, { label: "Chain ID", width: 0 }],
    data.map(c => [c.canonical, String(c.chainId ?? "\u2014")]),
  );
}

/**
 * Format token list as a table with symbol, address, and decimals.
 */
export function formatTokens(data: TokenJson[]): string {
  return formatTable(
    [{ label: "Symbol", width: 10 }, { label: "Address", width: 44 }, { label: "Decimals", width: 0 }],
    data.map(t => [t.symbol, t.address, String(t.decimals)]),
  );
}

/**
 * Format routes as a table showing source and destination pairs.
 * Expects enriched route data with srcSymbol/dstSymbol fields.
 */
export function formatRoutes(data: Array<{ srcChain: string; srcSymbol: string; dstChain: string; dstSymbol: string }>): string {
  return formatTable(
    [{ label: "Source", width: 25 }, { label: "Destination", width: 0 }],
    data.map(r => [`${r.srcChain}:${r.srcSymbol}`, `${r.dstChain}:${r.dstSymbol}`]),
  );
}
