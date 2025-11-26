import { Address } from 'viem';
import { EsploraClient } from '../../esplora';
import { GatewayOrderType, OrderDetails, OrderDetailsRaw } from './order';
import { GatewayTokensInfo, Token } from './token';
import { BaseExecuteQuoteParams } from './quote';

export interface OnrampFeeBreakdownRaw {
    /** @dev Total fees in satoshis */
    overallFeeSats: number;
    /** @dev Protocol-specific fee */
    protocolFeeSats: number;
    /** @dev Affiliate-related fee */
    affiliateFeeSats: number;
    /** @dev Fee for gas costs on BOB */
    executionFeeWei: string;
    /** @dev L1 data fee */
    l1DataFeeWei: string;
}

export type OnrampFeeBreakdown = Omit<OnrampFeeBreakdownRaw, 'executionFeeWei' | 'l1DataFeeWei'> & {
    /** @dev Fee for gas costs on BOB */
    executionFeeWei: bigint;
    /** @dev L1 data fee */
    l1DataFeeWei: bigint;
};

export type OnrampQuote = {
    /** @description The gateway address */
    gatewayAddress: Address;
    /** @description The base token address (e.g. wBTC or tBTC) */
    baseTokenAddress: Address;
    /** @description The minimum amount of Bitcoin to send */
    dustThreshold: number;
    /** @description The satoshi input amount */
    satoshis: number;
    /** @description The satoshi output amount (amount user receives after fees) */
    outputSatoshis: number;
    /** @description The fee paid in satoshis (includes gas refill, l1 data fee and estimated prove tx fee) */
    fee: number;
    /** @description The Bitcoin address to send BTC */
    bitcoinAddress: string;
    /** @description The number of confirmations required to confirm the Bitcoin tx */
    txProofDifficultyFactor: number;
    /** @description The optional strategy address */
    strategyAddress?: Address;
    /** @description V4 order details */
    orderDetails: OrderDetails;
    /** @dev Detailed fee breakdown */
    feeBreakdown: OnrampFeeBreakdown;
};

interface TokenReceived {
    amount: string;
    tokenAddress: Address;
}

export interface OnrampOrderResponse {
    /** @description The gateway address */
    gatewayAddress: Address;
    /** @description The base token address (e.g. wBTC or tBTC) */
    baseTokenAddress: Address;
    /** @description The Bitcoin txid */
    txid: string;
    /** @description True when the order was executed on BOB */
    status: boolean;
    /** @description When the order was created */
    timestamp: number;
    // TODO: return converted fee
    /** @description The converted satoshi amount */
    tokens: string;
    /** @description The satoshi output amount */
    satoshis: number;
    /** @description The fee paid in satoshis (includes gas refill) */
    fee: number;
    /** @description The number of confirmations required to confirm the Bitcoin tx */
    txProofDifficultyFactor: number;
    /** @description The optional strategy address */
    strategyAddress: Address | null;
    /** @description The gas refill in satoshis */
    satsToConvertToEth: number;
    /** @description The amount of ETH received */
    outputEthAmount: string | null;
    /** @description The output token (from strategies) */
    outputTokenAddress: Address | null;
    /** @description The output amount (from strategies) */
    outputTokenAmount: string | null;
    /** @description The tx hash on the EVM chain */
    txHash: string | null;
    /** @description V4 order details */
    orderDetails: OrderDetailsRaw | null;
    /** @description layerzero dst eid if the order being routed through layerzero */
    layerzeroDstEid: number | null;
    /** @description ERC20 tokens received by the user for gateway order */
    tokensReceived: TokenReceived[] | null;
    /** @description Indicates the outcome of the strategy execution */
    strategyFailed: boolean | null;
}

export type OnrampOrderStatusData = {
    confirmations: number;
};

export type OnrampOrderStatus =
    | {
          confirmed: false;
          pending?: never;
          success?: never;
          data: OnrampOrderStatusData;
      }
    | {
          confirmed?: never;
          pending: true;
          success?: never;
          data: OnrampOrderStatusData;
      }
    | {
          confirmed?: never;
          pending?: never;
          success: boolean;
          data: OnrampOrderStatusData;
      };

/** Order given by the Gateway API once the bitcoin tx is submitted */
export type OnrampOrder = Omit<
    OnrampOrderResponse,
    'satsToConvertToEth' | 'orderDetails' | 'tokensReceived' | 'outputTokenAddress' | 'outputTokenAmount'
> & {
    /** @description The gas refill in satoshis */
    gasRefill: number;
    /** @description V4 order details */
    orderDetails?: OrderDetails;
} & {
    /** @deprecated please use getTokens() instead as gateway v4 can handle multiple tokens */
    /** @description Get the actual token address received */
    getTokenAddress(): string | undefined;
    /** @deprecated please use getTokens() instead as gateway v4 can handle multiple tokens */
    /** @description Get the actual token received */
    getToken(): Token | undefined;
    /** @deprecated please use getTokens() instead as gateway v4 can handle multiple tokens */
    /** @description Get the actual amount received of the token */
    getTokenAmount(): string | number | null;
    /** @description Get the number of confirmations */
    getConfirmations(esploraClient: EsploraClient, latestHeight?: number): Promise<number>;
    /** @description Get the actual order status */
    getStatus(esploraClient: EsploraClient, latestHeight?: number): Promise<OnrampOrderStatus>;
    /** @description Get all the output tokens */
    getOutputTokens(): { amount: string; token: Token | undefined }[];
    /** @description Get all the tokens */
    getTokens(): { amount: string | number; token: Token | undefined }[];
} & GatewayTokensInfo;

// eslint-disable-next-line @typescript-eslint/no-empty-object-type
export type OnrampExecuteQuoteParams<T = {}> = BaseExecuteQuoteParams<T> & {
    type: GatewayOrderType.Onramp;
    data: OnrampQuote & GatewayTokensInfo;
};
