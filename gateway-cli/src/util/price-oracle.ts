const BINANCE_URL = process.env.BINANCE_API_URL ?? "https://api.binance.com/api/v3/ticker/price";
const COINBASE_URL = process.env.COINBASE_API_URL ?? "https://api.coinbase.com/v2/prices";

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
  try {
    return { priceUsd: await fromBinance(symbol), source: "binance" };
  } catch { /* Binance failed — fall back to Coinbase */
    try {
      return { priceUsd: await fromCoinbase(symbol), source: "coinbase" };
    } catch (err) {
      throw new PriceOracleError(symbol, err);
    }
  }
}

export async function usdToWei(usdAmount: number): Promise<string> {
  const { priceUsd } = await fetchPrice("ETH");
  return String(BigInt(Math.round((usdAmount / priceUsd) * 1e18)));
}
