const BINANCE_URL = "https://api.binance.com/api/v3/ticker/price";
const COINBASE_URL = "https://api.coinbase.com/v2/prices";

export interface PriceResult {
  priceUsd: number;
  source: "binance" | "coinbase";
}

export class PriceOracleError extends Error {
  constructor(symbol: string, cause: unknown) {
    super(`PriceOracleError: could not fetch price for "${symbol}": ${cause}`);
    this.name = "PriceOracleError";
  }
}

async function fromBinance(symbol: string): Promise<number> {
  const pair = `${symbol.toUpperCase()}USDT`;
  const res = await fetch(
    `${BINANCE_URL}?symbol=${pair}`,
    { signal: AbortSignal.timeout(5000) },
  );
  if (!res.ok) throw new Error(`Binance HTTP ${res.status}`);
  const data = (await res.json()) as { price: string };
  return parseFloat(data.price);
}

async function fromCoinbase(symbol: string): Promise<number> {
  const pair = `${symbol.toUpperCase()}-USD`;
  const res = await fetch(
    `${COINBASE_URL}/${pair}/spot`,
    { signal: AbortSignal.timeout(5000) },
  );
  if (!res.ok) throw new Error(`Coinbase HTTP ${res.status}`);
  const data = (await res.json()) as { data: { amount: string } };
  return parseFloat(data.data.amount);
}

export async function fetchPrice(symbol: string): Promise<PriceResult> {
  const [binance, coinbase] = await Promise.allSettled([
    fromBinance(symbol),
    fromCoinbase(symbol),
  ]);

  if (binance.status === "fulfilled") return { priceUsd: binance.value, source: "binance" };
  if (coinbase.status === "fulfilled") return { priceUsd: coinbase.value, source: "coinbase" };

  throw new PriceOracleError(symbol, coinbase.reason);
}
