// Lean gateway entry for quote/order consumers that don't need the wallet adapters.
// Importing the adapters (OkxWalletAdapter) pulls the whole wallet/ordinals/runes
// stack (and @magiceden-oss/runestone-lib) into the bundle; this entry omits them so
// bundlers only pull the quote/execute client + its generated types. The full surface
// (including adapters) stays available from the package root and `@gobob/bob-sdk`.
export { ExecuteQuoteResult, GatewayApiClient as GatewaySDK } from './client';
export {
    AnyGatewayError,
    DetailsFor,
    GatewayError,
    GatewayErrorCode,
    GatewayErrorDetailsMap,
    isGatewayError,
    type ExceededLimitDetails,
    type InsufficientAmountDetails,
    type QuoteAmountTooLowDetails,
    type InsufficientSwapAmountDetails,
    type NoRouteDetails,
    type SimulationFailedDetails,
    type UnableToCoverFeesDetails,
} from './error/gateway-error';
export * from './generated-client';
export { BitcoinSigner, ExecuteQuoteStep, ExecuteQuoteStepType, GatewayQuoteParams, GetQuoteParams } from './types';
export {
    formatBtc,
    getChainConfig,
    getInnerQuote,
    parseBtc,
    ScureBitcoinSigner,
    supportedChainsMapping,
    type InnerQuote,
} from './utils';
