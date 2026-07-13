import type { BalanceJson } from "../output.js";

/** USD prices keyed by token symbol (uppercase), e.g. { BTC: 62000, USDT: 1 }. */
export type PriceBySymbol = Record<string, number>;

function usdOf(balance: string, priceUsd: number): number {
  return parseFloat(balance) * priceUsd;
}

/**
 * Attach `priceUsd` and `usdValue` (priceUsd × balance) to each priceable asset
 * in a balance result. The top-level Bitcoin balance is priced as "BTC"; EVM
 * native and ERC20 tokens are priced by their symbol. Assets without a price in
 * `prices`, and chains that errored, are left untouched. Pure: returns a new
 * object and does not mutate the input.
 */
export function annotateBalancesUsd(result: BalanceJson, prices: PriceBySymbol): BalanceJson {
  const out: BalanceJson = {};
  for (const [chain, data] of Object.entries(result)) {
    if (data.error) {
      out[chain] = data;
      continue;
    }
    const entry: BalanceJson[string] = { ...data };

    if (entry.balance !== undefined && prices.BTC !== undefined) {
      entry.priceUsd = prices.BTC;
      entry.usdValue = usdOf(entry.balance, prices.BTC);
    }

    if (entry.native) {
      const price = prices[entry.native.symbol.toUpperCase()];
      entry.native = price !== undefined
        ? { ...entry.native, priceUsd: price, usdValue: usdOf(entry.native.balance, price) }
        : { ...entry.native };
    }

    if (entry.tokens) {
      entry.tokens = entry.tokens.map(t => {
        const price = prices[t.symbol.toUpperCase()];
        return price !== undefined
          ? { ...t, priceUsd: price, usdValue: usdOf(t.balance, price) }
          : { ...t };
      });
    }

    out[chain] = entry;
  }
  return out;
}
