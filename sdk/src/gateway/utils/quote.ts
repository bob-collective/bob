import type { GatewayQuote } from '../generated-client/models/GatewayQuote';
import type { GatewayOnrampQuote } from '../generated-client/models/GatewayOnrampQuote';
import type { GatewayOfframpQuote } from '../generated-client/models/GatewayOfframpQuote';
import type { GatewayLayerZeroQuote } from '../generated-client/models/GatewayLayerZeroQuote';
import { instanceOfGatewayQuoteOneOf } from '../generated-client/models/GatewayQuoteOneOf';
import { instanceOfGatewayQuoteOneOf1 } from '../generated-client/models/GatewayQuoteOneOf1';
import { instanceOfGatewayQuoteOneOf2 } from '../generated-client/models/GatewayQuoteOneOf2';

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
    if (instanceOfGatewayQuoteOneOf(quote as object)) return (quote as any).onramp;
    if (instanceOfGatewayQuoteOneOf1(quote as object)) return (quote as any).offramp;
    if (instanceOfGatewayQuoteOneOf2(quote as object)) return (quote as any).layerZero;
    throw new Error('Unknown quote variant');
}
