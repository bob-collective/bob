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
  BTCB: "BTC",
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
    { signal: AbortSignal.timeout(2500) },
  );
  if (!res.ok) throw new Error(`CoinGecko HTTP ${res.status}`);
  const data = (await res.json()) as Record<string, { usd?: number }>;
  const price = data[id]?.usd;
  if (price === undefined) throw new Error(`CoinGecko: no price for id "${id}"`);
  return price;
}

// ─── In-memory memoization (per-process) ────────────────────────────────────

/**
 * Cache of in-flight/settled price lookups keyed by (symbol, coingeckoId). The
 * CLI is short-lived, so a price is stable for a run — this collapses repeat and
 * concurrent lookups (e.g. ETH or USDC held on several chains) into a single
 * network call for every caller. Rejections are evicted so a transient failure
 * isn't cached for the rest of the process.
 */
const priceCache = new Map<string, Promise<PriceResult>>();

/** Clear the price cache. Intended for test isolation between cases. */
export function clearPriceCache(): void {
  priceCache.clear();
}

/**
 * Fetch USD price for a token, memoized per process (see {@link priceCache}).
 * Exchange spot prices (Binance, then Coinbase) are authoritative and preferred;
 * CoinGecko-by-id is only a fallback for tokens the exchanges don't list under a
 * spot symbol (e.g. USD₮0). Throws PriceOracleError only if every applicable
 * source fails.
 *
 * @param symbol - Token symbol (e.g., "BTC", "ETH")
 * @param coingeckoId - Optional CoinGecko coin id from the tokenlist
 * @returns Price in USD and source
 * @throws PriceOracleError if all sources fail
 */
export function fetchPrice(symbol: string, coingeckoId?: string): Promise<PriceResult> {
  const key = `${symbol.toUpperCase()}|${coingeckoId ?? ""}`;
  const cached = priceCache.get(key);
  if (cached) return cached;
  const pending = fetchPriceUncached(symbol, coingeckoId);
  priceCache.set(key, pending);
  pending.catch(() => priceCache.delete(key));
  return pending;
}

async function fetchPriceUncached(symbol: string, coingeckoId?: string): Promise<PriceResult> {
  const spotSymbol = resolveSpotSymbol(symbol);
  // Exchange spot prices are authoritative for listed tokens: prefer them, so a swap amount
  // is never scaled by an unchecked CoinGecko value when a real exchange price is available.
  const [binance, coinbase] = await Promise.allSettled([
    fromBinance(spotSymbol),
    fromCoinbase(spotSymbol),
  ]);
  if (binance.status === "fulfilled" && binance.value > 0) return { priceUsd: binance.value, source: "binance" };
  if (coinbase.status === "fulfilled" && coinbase.value > 0) return { priceUsd: coinbase.value, source: "coinbase" };

  // Exchanges don't list this token (e.g. USD₮0) — fall back to CoinGecko by id.
  if (coingeckoId) {
    try {
      const price = await fromCoinGecko(coingeckoId);
      if (price > 0) return { priceUsd: price, source: "coingecko" };
    } catch (err) {
      throw new PriceOracleError(symbol, err);
    }
  }

  const cause =
    (binance.status === "rejected" && binance.reason) ||
    (coinbase.status === "rejected" && coinbase.reason) ||
    new Error("all price sources returned a non-positive value");
  throw new PriceOracleError(symbol, cause);
}
