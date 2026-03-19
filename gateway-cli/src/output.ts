// ─── Output mode ─────────────────────────────────────────────────────────────

export type OutputMode = "json" | "human";

export interface Logger {
  progress: (msg: string) => void;
  warn: (msg: string) => void;
}

export function createLogger(mode: OutputMode): Logger {
  return {
    progress: (msg) => { if (mode === "human") process.stderr.write(msg + "\n"); },
    warn: (msg) => { if (mode === "human") process.stderr.write(`Warning: ${msg}\n`); },
  };
}

/** Render data to stdout. JSON mode always uses JSON.stringify; human mode uses a custom formatter if provided, otherwise JSON. */
export function render(data: unknown, mode: OutputMode, humanFormatter?: (data: any) => string): void {
  if (mode === "json" || !humanFormatter) {
    console.log(formatJson(data));
  } else {
    console.log(humanFormatter(data));
  }
}

// ─── Types (JSON output shapes) ──────────────────────────────────────────────

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

export interface SwapSubmittedJson {
  orderId: string;
  status: "submitted";
  srcAmount: string;
  srcAsset: string;
  dstAsset: string;
  dstChain: string;
  txId: string;
}

export interface SwapMempoolPendingJson {
  orderId: string;
  status: "mempool_pending";
  srcAmount: string;
  srcAsset: string;
  dstAsset: string;
  dstChain: string;
  mempoolTxId: string;
}

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
}

export interface ChainJson {
  canonical: string;
  aliases: string[];
  chainId: number | null;
}

export interface TokenJson {
  symbol: string;
  address: string;
  decimals: number;
}

export interface MaxSpendableJson {
  asset: string;
  chain: string;
  address: string;
  maxSpendableSat: string;
  balanceSat: string;
  estimatedFeeSat: string;
  feeRateSatPerVbyte: number;
}

export interface BalanceJson {
  [chain: string]: {
    address: string;
    balance?: string;
    maxSpendable?: string;
    native?: { symbol: string; balance: string };
    tokens?: Array<{ symbol: string; address: string; balance: string }>;
  };
}

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

// ─── Formatters (stdout) ─────────────────────────────────────────────────────

/** Render any data as formatted JSON. */
export function formatJson(data: unknown): string {
  return JSON.stringify(data, null, 2);
}

/** Simple column-aligned table. */
export function formatTable(columns: { label: string; width: number }[], rows: string[][]): string {
  const header = columns.map(c => c.label.padEnd(c.width)).join("");
  const sep = "─".repeat(columns.reduce((sum, c) => sum + c.width, 0));
  const body = rows.map(row =>
    row.map((cell, i) => i < columns.length - 1 ? cell.padEnd(columns[i].width) : cell).join("")
  );
  return [header, sep, ...body].join("\n");
}

/** Swap/quote confirmation display. */
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

/** Balance display. */
export function formatBalance(result: BalanceJson): string {
  const lines: string[] = [];
  for (const [chain, data] of Object.entries(result)) {
    lines.push(`${chain}  (${data.address})`);
    if (data.balance !== undefined) lines.push(`  Balance:       ${data.balance} BTC`);
    if (data.maxSpendable !== undefined) lines.push(`  Max spendable: ${data.maxSpendable} BTC`);
    if (data.native) lines.push(`  ${data.native.symbol}: ${data.native.balance}`);
    if (data.tokens) for (const t of data.tokens) lines.push(`  ${t.symbol}: ${t.balance}`);
  }
  return lines.length === 0 ? "No non-zero balances found." : lines.join("\n");
}

/** Chains table. */
export function formatChains(data: ChainJson[]): string {
  return formatTable(
    [{ label: "Chain", width: 20 }, { label: "Chain ID", width: 0 }],
    data.map(c => [c.canonical, String(c.chainId ?? "\u2014")]),
  );
}

/** Tokens table. */
export function formatTokens(data: TokenJson[]): string {
  return formatTable(
    [{ label: "Symbol", width: 10 }, { label: "Address", width: 44 }, { label: "Decimals", width: 0 }],
    data.map(t => [t.symbol, t.address, String(t.decimals)]),
  );
}

/** Routes table. */
export function formatRoutes(data: Array<{ srcChain: string; srcToken: { symbol: string }; dstChain: string; dstToken: { symbol: string } }>): string {
  return formatTable(
    [{ label: "Source", width: 25 }, { label: "Destination", width: 0 }],
    data.map(r => [`${r.srcChain}:${r.srcToken.symbol}`, `${r.dstChain}:${r.dstToken.symbol}`]),
  );
}
