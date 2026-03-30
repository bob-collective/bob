export { OkxWalletAdapter } from './adapters/okx-wallet';
export { ReownWalletAdapter } from './adapters/reown';
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
    type InsufficientFundsDetails,
    type InsufficientSwapAmountDetails,
    type NoRouteDetails,
    type SimulationFailedDetails,
    type UnableToCoverFeesDetails
} from './error/gateway-error';
export * from './generated-client';
export { BitcoinSigner, GatewayQuoteParams, GetQuoteParams } from './types';
export {
    formatBtc,
    getChainConfig,
    getInnerQuote,
    parseBtc,
    ScureBitcoinSigner,
    supportedChainsMapping,
    type InnerQuote
} from './utils';

