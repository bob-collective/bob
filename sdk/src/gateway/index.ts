export { OkxWalletAdapter } from './adapters/okx-wallet';
export { ReownWalletAdapter } from './adapters/reown';
export { type ExecuteQuoteResult, GatewayApiClient as GatewaySDK } from './client';
export { ApiError, isApiError } from './error/api-error';
export {
    type AnyGatewayError,
    type DetailsFor,
    GatewayError,
    GatewayErrorCode,
    type GatewayErrorDetailsMap,
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
export {
    BitcoinBroadcastError,
    type BitcoinSigner,
    ExecuteQuoteError,
    type ExecuteQuoteErrorOptions,
    type ExecuteQuoteStep,
    ExecuteQuoteStepType,
    type ExecuteQuoteTxStatus,
    type GatewayQuoteParams,
    type GetQuoteParams,
} from './types';
export {
    applyGasBuffer,
    formatBtc,
    getChainConfig,
    getInnerQuote,
    parseBtc,
    ScureBitcoinSigner,
    supportedChainsMapping,
    type InnerQuote,
} from './utils';
