/** Minimal shape of a balance entry that can carry USD annotations. */
export interface UsdAnnotatable {
  priceUsd?: number;
  usdValue?: number;
}

/**
 * Annotate a single asset with its USD price and value (priceUsd × balance).
 * No-op when `priceUsd` is undefined, so an asset we couldn't price is simply
 * left unannotated. Mutates `asset` in place.
 *
 * Pricing is done per asset (each with its own symbol/coingeckoId) rather than
 * through a shared symbol-keyed map, so tokens that share a symbol across chains
 * cannot be cross-contaminated.
 */
export function applyUsd(asset: UsdAnnotatable, balance: string, priceUsd: number | undefined): void {
  if (priceUsd === undefined) return;
  asset.priceUsd = priceUsd;
  asset.usdValue = parseFloat(balance) * priceUsd;
}
