import {
    instanceOfGatewayOrderInfoOneOf,
    instanceOfGatewayOrderInfoOneOf1,
    instanceOfGatewayOrderInfoOneOf2,
    instanceOfGatewayQuoteOneOf,
    instanceOfGatewayQuoteOneOf1,
    instanceOfGatewayQuoteOneOf2,
} from './gateway';

export const isOnrampQuote = instanceOfGatewayQuoteOneOf;
export const isOfframpQuote = instanceOfGatewayQuoteOneOf1;
export const isLayerZeroQuote = instanceOfGatewayQuoteOneOf2;

export const isOnrampOrder = instanceOfGatewayOrderInfoOneOf;
export const isOfframpOrder = instanceOfGatewayOrderInfoOneOf1;
export const isLayerZeroOrder = instanceOfGatewayOrderInfoOneOf2;
