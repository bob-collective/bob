import { Address } from 'viem';
import { GatewayOrderType } from './order';
import { BaseExecuteQuoteParams } from './quote';

// Currently this order type just supports L0 WBTC swaps between EVM chains

export type EVMToEVMOrderStatus =
    | 'source-pending'
    | 'source-confirmed'
    | 'destination-pending'
    | 'destination-confirmed'
    | 'source-failed'
    | 'destination-failed'
    | 'unknown';

export interface EVMToEVMOrder {
    amount: bigint;
    timestamp: number;
    status: EVMToEVMOrderStatus;
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
export type EVMToEVMQuoteParams<T = {}> = BaseExecuteQuoteParams<T> & {
    type: GatewayOrderType.EVMToEVM;
    data: EVMToEVMSwapQuote;
};

export type EVMToEVMSwapFeeBreakdown = {
    nativeFee: bigint;
    lzTokenFee: bigint;
    // gasFee: bigint;
};

export interface EVMToEVMSwapQuote {
    sourceEid: number;
    destinationEid: number;
    oftAddress: Address;
    feeBreakdown: EVMToEVMSwapFeeBreakdown;
}
