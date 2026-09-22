import type {
    GatewayOfframpQuoteV4,
    GatewayOnrampQuoteV2,
    GatewayQuoteV4,
    GatewayTokenSwapQuoteV4,
} from '../generated-client';

export type InnerQuote = GatewayOnrampQuoteV2 | GatewayOfframpQuoteV4 | GatewayTokenSwapQuoteV4;

/** Unwraps a V4 quote to access common amount, fee, and routing fields. */
export function getInnerQuote(quote: GatewayQuoteV4): InnerQuote {
    if ('onramp' in quote) return quote.onramp;
    if ('offramp' in quote) return quote.offramp;
    if ('tokenSwap' in quote) return quote.tokenSwap;
    throw new Error('Unknown quote variant');
}
