import { Hex } from 'viem';

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

export interface LayerZeroQuoteParamsExt {
    /** @description temporary field for chain ID */
    l0ChainId?: number | null;
    /** @description Buffer in BPS to account for Bitcoin to BOB finality delay (30 mins+) when using the L0 Strategy */
    l0OriginFinalityBuffer?: number | bigint;
    /** @description Buffer in BPS to account for BOB to destination finality delay (a few minutes) when using the L0 Strategy */
    l0DestinationFinalityBuffer?: number | bigint;
}
