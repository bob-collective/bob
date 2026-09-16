import type {
  GatewayQuoteV4,
  GatewayOnrampQuoteV2,
  GatewayOfframpQuoteV4,
  GatewayTokenSwapQuoteV4,
} from "@gobob/bob-sdk";

/**
 * V4 equivalent of the SDK's `getInnerQuote`. Picks the active variant
 * (onramp / offramp / tokenSwap) from a `GatewayQuoteV4` discriminated union.
 *
 * The SDK's exported `getInnerQuote` is typed against a different quote union
 * (onramp / offramp / layerZero), so we reproduce the unwrap here against the
 * V4 types actually returned by `sdk.getQuote`. Onramp still carries the V2 inner
 * shape; offramp and tokenSwap moved to V4 inner shapes.
 */
export type InnerQuoteV4 = GatewayOnrampQuoteV2 | GatewayOfframpQuoteV4 | GatewayTokenSwapQuoteV4;

export function getInnerQuoteV4(quote: GatewayQuoteV4): InnerQuoteV4 {
  if ("onramp" in quote) return quote.onramp;
  if ("offramp" in quote) return quote.offramp;
  if ("tokenSwap" in quote) return quote.tokenSwap;
  throw new Error("Unknown quote variant");
}
