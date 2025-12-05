import { Address, Hex } from 'viem';
import { GatewayOrderType } from './order';
import { BaseExecuteQuoteParams } from './quote';
import { OnrampQuote } from './onramp';
import { OfframpQuote } from './offramp';
import { GatewayTokensInfo } from './token';

interface LayerZeroMessagePathway {
    srcEid: number;
    dstEid: number;
    sender: LayerZeroMessageSender;
    receiver: LayerZeroMessageReceiver;
    id: string;
    nonce: number;
}

interface LayerZeroMessageSender {
    address: string;
    id: string;
    name: string;
    chain: string;
}

interface LayerZeroMessageReceiver {
    address: string;
    id: string;
    name: string;
    chain: string;
}

interface LayerZeroMessageSource {
    status: string;
    tx: LayerZeroMessageTx;
}

export interface LayerZeroLzCompose {
    status: string;
}

interface LayerZeroMessageDestination {
    tx: LayerZeroMessageTx;
    status: string;
    lzCompose: LayerZeroLzCompose;
}

interface LayerZeroMessageTx {
    txHash: string;
    blockHash: string;
    blockNumber: string;
    blockTimestamp: number;
    from: string;
    payload: string;
    readinessTimestamp: number;
}

export type LayerZeroMessageWallet = {
    pathway: LayerZeroMessagePathway;
    source: LayerZeroMessageSource;
    destination: LayerZeroMessageDestination;
};

export type LayerZeroMessagesWalletResponse = {
    data: Array<LayerZeroMessageWallet>;
};

export type LayerZeroSendParam = {
    dstEid: number;
    to: Hex;
    amountLD: bigint;
    minAmountLD: bigint;
    extraOptions: Hex;
    composeMsg: Hex;
    oftCmd: Hex;
};

export type LayerZeroChainInfo = {
    name: string; // viem chain name
    eid?: string;
    oftAddress: string;
    nativeChainId?: number;
};

export type LayerZeroDeploymentsMetadataResponse = {
    [chainKey: string]: {
        deployments?: [
            {
                version: number;
                eid: string;
            },
        ];
        chainKey: string;
        chainDetails?: {
            nativeChainId: number;
        };
    };
};

export type LayerZeroTokenDeploymentsResponse = {
    [chainKey: string]: {
        address: string;
    };
};

// Types for EVM to EVM swaps with LayerZero

export type EVMToEVMWithLayerZeroOrderStatus =
    | 'source-pending'
    | 'source-confirmed'
    | 'destination-pending'
    | 'destination-confirmed'
    | 'source-failed'
    | 'destination-failed'
    | 'unknown';

export interface EVMToEVMWithLayerZeroOrder {
    amount: bigint;
    timestamp: number;
    status: EVMToEVMWithLayerZeroOrderStatus;
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

export type EVMToEVMWithLayerZeroFeeBreakdown = {
    nativeFee: bigint;
    lzTokenFee: bigint;
    // gasFee: bigint;
};

export interface EVMToEVMWithLayerZeroQuote {
    sourceEid: number;
    destinationEid: number;
    oftAddress: Address;
    feeBreakdown: EVMToEVMWithLayerZeroFeeBreakdown;
}

// eslint-disable-next-line @typescript-eslint/no-empty-object-type
export type OnrampWithLayerZeroExecuteQuoteParams<T = {}> = BaseExecuteQuoteParams<T> & {
    type: GatewayOrderType.OnrampWithLayerZero;
    data: OnrampQuote & GatewayTokensInfo;
};

// eslint-disable-next-line @typescript-eslint/no-empty-object-type
export type OfframpWithLayerZeroExecuteQuoteParams<T = {}> = BaseExecuteQuoteParams<T> & {
    type: GatewayOrderType.OfframpWithLayerZero;
    data: OfframpQuote;
};

// eslint-disable-next-line @typescript-eslint/no-empty-object-type
export type EVMToEVMWithLayerZeroExecuteQuoteParams<T = {}> = BaseExecuteQuoteParams<T> & {
    type: GatewayOrderType.EVMToEVMWithLayerZero;
    data: EVMToEVMWithLayerZeroQuote;
};
