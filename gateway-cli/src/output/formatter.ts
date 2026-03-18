export type OutputMode = "json" | "human";

export function formatTable(columns: { label: string; width: number }[], rows: string[][]): string {
  const header = columns.map(c => c.label.padEnd(c.width)).join("");
  const sep = "─".repeat(columns.reduce((sum, c) => sum + c.width, 0));
  const body = rows.map(row =>
    row.map((cell, i) => i < columns.length - 1 ? cell.padEnd(columns[i].width) : cell).join("")
  );
  return [header, sep, ...body].join("\n");
}

function camelToTitle(s: string): string {
  return s
    .replace(/([A-Z])/g, " $1")
    .replace(/^./, (c) => c.toUpperCase())
    .trim();
}

function formatObject(obj: Record<string, unknown>): string {
  const lines: string[] = [];
  for (const [key, value] of Object.entries(obj)) {
    const label = camelToTitle(key);
    if (value !== null && typeof value === "object") {
      lines.push(`${label}: ${JSON.stringify(value)}`);
    } else {
      lines.push(`${label}: ${String(value)}`);
    }
  }
  return lines.join("\n");
}

function formatHuman(data: unknown): string {
  if (data === null || data === undefined) {
    return String(data);
  }

  if (Array.isArray(data)) {
    return data
      .map((item, i) => {
        const header = `[${i + 1}]`;
        if (item !== null && typeof item === "object") {
          return `${header}\n${formatObject(item as Record<string, unknown>)}`;
        }
        return `${header} ${String(item)}`;
      })
      .join("\n\n");
  }

  if (typeof data === "object") {
    return formatObject(data as Record<string, unknown>);
  }

  return String(data);
}

export function formatOutput(data: unknown, mode: OutputMode): string {
  if (mode === "json") {
    return JSON.stringify(data, null, 2);
  }
  return formatHuman(data);
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

export function formatConfirmation(data: ConfirmationData): string {
  const lines: string[] = ["Swap"];
  const srcLine = data.srcDisplay
    ? `${data.srcAmount} ${data.srcAsset}  (${data.srcDisplay})`
    : `${data.srcAmount} ${data.srcAsset}`;
  lines.push(`  From       ${srcLine}`);
  lines.push(`  To         ~${data.dstAmount} ${data.dstAsset} on ${data.dstChain}`);
  if (data.feeNetwork) lines.push(`  Fee        ${data.feeNetwork}`);
  if (data.feeRateSatPerVbyte !== undefined)
    lines.push(`  Fee rate   ${data.feeRateSatPerVbyte} sat/vbyte  (mempool.space)`);
  if (data.gasEstimateEth)
    lines.push(`  Gas (est)  ~${data.gasEstimateEth} ETH  (1.3x safety buffer applied)`);
  if (data.gasRefillUsd) lines.push(`  Gas refill $${data.gasRefillUsd} ETH sent to recipient`);
  lines.push(`  Slippage   ${(data.slippageBps / 100).toFixed(1)}% max  (${data.slippageBps} bps)`);
  lines.push(`  Recipient  ${data.recipient}`);
  return lines.join("\n");
}
