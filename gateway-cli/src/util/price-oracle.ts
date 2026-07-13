/** Binance API endpoint for token price in USDT. */
const BINANCE_URL = "https://api.binance.com/api/v3/ticker/price";
/** Coinbase API endpoint prefix for token price in USD. */
const COINBASE_URL = "https://api.coinbase.com/v2/prices";
/** CoinGecko simple-price endpoint (price by coin id, not exchange symbol). */
const COINGECKO_URL = "https://api.coingecko.com/api/v3/simple/price";

/**
 * Map token symbols to a major spot pair symbol when exchanges do not list
 * the token directly (e.g. cbBTC → BTC for USD amount conversion).
 */
const SPOT_SYMBOL_ALIASES: Record<string, string> = {
  CBBTC: "BTC",
};

function resolveSpotSymbol(symbol: string): string {
  const key = symbol.toUpperCase();
  return SPOT_SYMBOL_ALIASES[key] ?? symbol;
}

/** Price oracle result with USD price and source. */
export interface PriceResult {
  priceUsd: number;
  source: "binance" | "coinbase" | "coingecko";
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
 * Fetch USD price from CoinGecko by coin id.
 * Used for tokens the exchanges do not list under a spot symbol (e.g. USD₮0),
 * where the tokenlist provides a `coingeckoId`.
 * @param id - CoinGecko coin id (e.g. "usdt0", "hyperliquid")
 * @returns Price in USD
 */
async function fromCoinGecko(id: string): Promise<number> {
  const res = await fetch(
    `${COINGECKO_URL}?ids=${encodeURIComponent(id)}&vs_currencies=usd`,
    { signal: AbortSignal.timeout(5000) },
  );
  if (!res.ok) throw new Error(`CoinGecko HTTP ${res.status}`);
  const data = (await res.json()) as Record<string, { usd?: number }>;
  const price = data[id]?.usd;
  if (price === undefined) throw new Error(`CoinGecko: no price for id "${id}"`);
  return price;
}

/**
 * Fetch USD price for a token. When a `coingeckoId` is given it is tried first
 * (CoinGecko by id), which covers tokens exchanges don't list under a spot
 * symbol (e.g. USD₮0); otherwise/failing that, tries Binance then Coinbase by
 * symbol. Returns the first successful result; throws PriceOracleError if all fail.
 *
 * @param symbol - Token symbol (e.g., "BTC", "ETH")
 * @param coingeckoId - Optional CoinGecko coin id from the tokenlist
 * @returns Price in USD and source
 * @throws PriceOracleError if all sources fail
 */
export async function fetchPrice(symbol: string, coingeckoId?: string): Promise<PriceResult> {
  // Prefer CoinGecko by id when the tokenlist provides one: exchanges don't list
  // tokens like USD₮0 under a spot symbol, so the symbol-based sources 404.
  if (coingeckoId) {
    try {
      const price = await fromCoinGecko(coingeckoId);
      if (price > 0) return { priceUsd: price, source: "coingecko" };
    } catch {
      // Fall through to symbol-based sources.
    }
  }

  const spotSymbol = resolveSpotSymbol(symbol);
  const [binance, coinbase] = await Promise.allSettled([
    fromBinance(spotSymbol),
    fromCoinbase(spotSymbol),
  ]);

  if (binance.status === "fulfilled" && binance.value > 0) return { priceUsd: binance.value, source: "binance" };
  if (coinbase.status === "fulfilled" && coinbase.value > 0) return { priceUsd: coinbase.value, source: "coinbase" };

  const cause = coinbase.status === "rejected" ? coinbase.reason : binance.status === "rejected" ? binance.reason : new Error("price was zero");
  throw new PriceOracleError(symbol, cause);
}
