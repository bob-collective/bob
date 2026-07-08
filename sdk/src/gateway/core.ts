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
// Import from specific util modules (not the './utils' barrel) so this lean entry does
// not pull in the bitcoin-signer helper (and its scure signing deps). Consumers that need
// the ScureBitcoinSigner helper import it from the package root / full gateway barrel.
export { formatBtc, getChainConfig, parseBtc, supportedChainsMapping } from './utils/common';
export { getInnerQuote, type InnerQuote } from './utils/quote';
