import { Address } from 'viem';
import { GatewayOrderType } from './order';
import { BaseExecuteQuoteParams } from './quote';

export type CrossChainOrderStatus =
    | 'source-pending'
    | 'source-confirmed'
    | 'destination-pending'
    | 'destination-confirmed'
    | 'source-failed'
    | 'destination-failed'
    | 'unknown';

export interface CrossChainOrder {
    amount: bigint;
    timestamp: number;
    status: CrossChainOrderStatus;
    source: {
        eid: number;
        txHash: string;
        token: Address;
    };
    destination: {
        eid: number;
        txHash: string;
        token: Address;
    };
}

// eslint-disable-next-line @typescript-eslint/no-empty-object-type
export type CrossChainSwapQuoteParams<T = {}> = BaseExecuteQuoteParams<T> & {
    type: GatewayOrderType.CrossChainSwap;
    data: CrossChainSwapQuote;
};

export type CrossChainFeeBreakdown = {
    nativeFee: bigint;
    lzTokenFee: bigint;
};

export interface CrossChainSwapQuote {
    sourceEid: number;
    destinationEid: number;
    oftAddress: Address;
    feeBreakdown: CrossChainFeeBreakdown;
}
