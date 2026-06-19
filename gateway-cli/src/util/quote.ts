import type {
  GatewayQuoteV3,
  GatewayOnrampQuoteV2,
  GatewayOfframpQuoteV3,
  GatewayTokenSwapQuoteV2,
} from "@gobob/bob-sdk";

/**
 * V3 equivalent of the SDK's `getInnerQuote`. Picks the active variant
 * (onramp / offramp / tokenSwap) from a `GatewayQuoteV3` discriminated union.
 *
 * The SDK's exported `getInnerQuote` is typed against a different quote union
 * (onramp / offramp / layerZero), so we reproduce the unwrap here against the
 * V3 types actually returned by `sdk.getQuote`. Onramp and tokenSwap still
 * carry V2 inner shapes; offramp moved to the V3 inner shape.
 */
export type InnerQuoteV3 = GatewayOnrampQuoteV2 | GatewayOfframpQuoteV3 | GatewayTokenSwapQuoteV2;

export function getInnerQuoteV3(quote: GatewayQuoteV3): InnerQuoteV3 {
  if ("onramp" in quote) return quote.onramp;
  if ("offramp" in quote) return quote.offramp;
  if ("tokenSwap" in quote) return quote.tokenSwap;
  throw new Error("Unknown quote variant");
}
