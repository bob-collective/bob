import type { GatewayQuote } from '../generated-client/models/GatewayQuote';
import type { GatewayOnrampQuote } from '../generated-client/models/GatewayOnrampQuote';
import type { GatewayOfframpQuote } from '../generated-client/models/GatewayOfframpQuote';
import type { GatewayLayerZeroQuote } from '../generated-client/models/GatewayLayerZeroQuote';
import { instanceOfGatewayQuoteOneOf, type GatewayQuoteOneOf } from '../generated-client/models/GatewayQuoteOneOf';
import { instanceOfGatewayQuoteOneOf1, type GatewayQuoteOneOf1 } from '../generated-client/models/GatewayQuoteOneOf1';
import { instanceOfGatewayQuoteOneOf2, type GatewayQuoteOneOf2 } from '../generated-client/models/GatewayQuoteOneOf2';

export type InnerQuote = GatewayOnrampQuote | GatewayOfframpQuote | GatewayLayerZeroQuote;

/**
 * Unwraps a GatewayQuote, returning the inner quote object directly.
 *
 * All three variants share common fields (inputAmount, outputAmount, fees,
 * recipient, estimatedTimeInSecs, sender), so you can access them without
 * caring about the variant:
 *
 * ```ts
 * const inner = getInnerQuote(quote);
 * console.log(inner.inputAmount.amount);
 * console.log(inner.outputAmount.amount);
 * console.log(inner.fees.amount);
 * ```
 */
export function getInnerQuote(quote: GatewayQuote): InnerQuote {
    const q = quote as object;
    if (instanceOfGatewayQuoteOneOf(q)) return (q as GatewayQuoteOneOf).onramp;
    if (instanceOfGatewayQuoteOneOf1(q)) return (q as GatewayQuoteOneOf1).offramp;
    if (instanceOfGatewayQuoteOneOf2(q)) return (q as GatewayQuoteOneOf2).layerZero;
    throw new Error('Unknown quote variant');
}
