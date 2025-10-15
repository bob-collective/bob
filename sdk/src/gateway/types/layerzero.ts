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
