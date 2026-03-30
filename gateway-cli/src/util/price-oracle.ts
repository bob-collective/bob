/** Binance API endpoint for token price in USDT. */
const BINANCE_URL = "https://api.binance.com/api/v3/ticker/price";
/** Coinbase API endpoint prefix for token price in USD. */
const COINBASE_URL = "https://api.coinbase.com/v2/prices";

/** Price oracle result with USD price and source exchange. */
export interface PriceResult {
  priceUsd: number;
  source: "binance" | "coinbase";
}

/**
 * Error thrown when price oracle fails to fetch from all sources.
 * Includes the symbol that failed and the underlying cause.
 */
export class PriceOracleError extends Error {
  constructor(symbol: string, cause: unknown) {
    super(`PriceOracleError: could not fetch price for "${symbol}": ${cause}`);
    this.name = "PriceOracleError";
  }
}

/**
 * Fetch price from Binance API (USDT pair).
 * @param symbol - Token symbol (e.g., "BTC", "ETH")
 * @returns Price in USDT
 */
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

/**
 * Fetch price from Coinbase API (USD spot price).
 * @param symbol - Token symbol (e.g., "BTC", "ETH")
 * @returns Price in USD
 */
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

/**
 * Fetch USD price for a token symbol from Binance or Coinbase.
 * Tries Binance first (USDT pair), falls back to Coinbase (USD pair).
 * Returns first successful result; throws PriceOracleError if both fail.
 * 
 * @param symbol - Token symbol (e.g., "BTC", "ETH")
 * @returns Price in USD and source exchange
 * @throws PriceOracleError if both sources fail
 */
export async function fetchPrice(symbol: string): Promise<PriceResult> {
  const [binance, coinbase] = await Promise.allSettled([
    fromBinance(symbol),
    fromCoinbase(symbol),
  ]);

  if (binance.status === "fulfilled" && binance.value > 0) return { priceUsd: binance.value, source: "binance" };
  if (coinbase.status === "fulfilled" && coinbase.value > 0) return { priceUsd: coinbase.value, source: "coinbase" };

  const cause = coinbase.status === "rejected" ? coinbase.reason : binance.status === "rejected" ? binance.reason : new Error("price was zero");
  throw new PriceOracleError(symbol, cause);
}
