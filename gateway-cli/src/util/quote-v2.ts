import type { GatewayQuoteV2, GatewayOnrampQuoteV2, GatewayOfframpQuoteV2, GatewayTokenSwapQuoteV2 } from "@gobob/bob-sdk";

/**
 * V2 equivalent of the SDK's `getInnerQuote`. Picks the active variant
 * (onramp / offramp / tokenSwap) from a `GatewayQuoteV2` discriminated union.
 *
 * The SDK only ships a typed helper for V1 quotes, but the discriminator
 * shape is identical, so we reproduce it here against the V2 types.
 */
export type InnerQuoteV2 = GatewayOnrampQuoteV2 | GatewayOfframpQuoteV2 | GatewayTokenSwapQuoteV2;

export function getInnerQuoteV2(quote: GatewayQuoteV2): InnerQuoteV2 {
  if ("onramp" in quote) return quote.onramp;
  if ("offramp" in quote) return quote.offramp;
  if ("tokenSwap" in quote) return quote.tokenSwap;
  throw new Error("Unknown quote variant");
}
