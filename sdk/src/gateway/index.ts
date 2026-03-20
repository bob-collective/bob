export {
    GatewayError,
    type ExceededLimitDetails,
    type InsufficientAmountDetails,
    type InsufficientFundsDetails,
    type InsufficientSwapAmountDetails,
    type NoRouteDetails,
    type SimulationFailedDetails,
    type UnableToCoverFeesDetails,
} from './error/GatewayError';
export { OkxWalletAdapter } from './adapters/okx-wallet';
export { ReownWalletAdapter } from './adapters/reown';
export { ExecuteQuoteResult, GatewayApiClient as GatewaySDK } from './client';
export * from './generated-client';
export { GatewayQuoteParams, GetQuoteParams } from './types';
export { formatBtc, parseBtc } from './utils';
