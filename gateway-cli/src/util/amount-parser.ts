import { fetchPrice } from "./price-oracle.js";

export interface ParsedAmount {
  atomicUnits: string;
  display: string;
  priceSource?: "binance" | "coinbase" | "stablecoin";
}

function toAtomicUnits(human: number, decimals: number): string {
  const factor = 10n ** BigInt(decimals);
  const scaled = BigInt(Math.round(human * Number(factor)));
  return scaled.toString();
}

export async function parseAmount(
  raw: string,
  srcSymbol: string,
  srcDecimals: number,
): Promise<ParsedAmount> {
  // Raw atomic units passthrough (for agents)
  if (raw.startsWith("raw:")) {
    const units = raw.slice(4);
    if (!/^\d+$/.test(units)) throw new Error(`invalid raw amount "${units}" — must be a positive integer`);
    return { atomicUnits: units, display: `${units} (raw)` };
  }

  // Dollar amount
  if (raw.startsWith("$")) {
    const usdValue = parseFloat(raw.slice(1));
    if (isNaN(usdValue) || usdValue < 0) throw new Error(`invalid dollar amount "${raw}"`);
    const { priceUsd, source } = await fetchPrice(srcSymbol);
    const humanAmount = usdValue / priceUsd;
    return {
      atomicUnits: toAtomicUnits(humanAmount, srcDecimals),
      display: `${humanAmount.toFixed(8).replace(/\.?0+$/, "")} ${srcSymbol} ($${usdValue} via ${source})`,
      priceSource: source,
    };
  }

  // Satoshi shorthand
  if (/^\d+sat$/i.test(raw)) {
    const sats = parseInt(raw, 10);
    return { atomicUnits: String(sats), display: `${sats} sat` };
  }

  // Reject raw integers without suffix
  if (/^\d+$/.test(raw)) {
    throw new Error(
      `suffix required — raw integers are ambiguous. Use "0.1BTC", "100000sat", "100USDC", etc.`,
    );
  }

  // Token suffix: match number + alpha suffix
  const match = raw.match(/^([0-9]*\.?[0-9]+)([A-Za-z]+)$/);
  if (!match) throw new Error(`cannot parse amount "${raw}"`);
  const [, numStr, suffix] = match;
  const suffixUpper = suffix.toUpperCase();
  const human = parseFloat(numStr);
  if (isNaN(human)) throw new Error(`invalid number in amount "${raw}"`);

  if (suffixUpper !== srcSymbol.toUpperCase()) {
    throw new Error(
      `unknown amount suffix "${suffix}". Use ${srcSymbol}, sat, or a "$" prefix for USD.`,
    );
  }

  return {
    atomicUnits: toAtomicUnits(human, srcDecimals),
    display: `${numStr} ${suffixUpper}`,
  };
}
