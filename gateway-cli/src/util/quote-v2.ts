import type { GatewayQuoteV2, GatewayOnrampQuoteV2, GatewayOfframpQuoteV2, GatewayLayerZeroQuote } from "@gobob/bob-sdk";

/**
 * V2 equivalent of the SDK's `getInnerQuote`. Picks the active variant
 * (onramp / offramp / layerZero) from a `GatewayQuoteV2` discriminated union.
 *
 * The SDK only ships a typed helper for V1 quotes, but the discriminator
 * shape is identical, so we reproduce it here against the V2 types.
 */
export type InnerQuoteV2 = GatewayOnrampQuoteV2 | GatewayOfframpQuoteV2 | GatewayLayerZeroQuote;

export function getInnerQuoteV2(quote: GatewayQuoteV2): InnerQuoteV2 {
  if ("onramp" in quote) return quote.onramp;
  if ("offramp" in quote) return quote.offramp;
  if ("layerZero" in quote) return quote.layerZero;
  throw new Error("Unknown quote variant");
}
