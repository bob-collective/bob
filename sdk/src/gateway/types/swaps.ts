import { Address, Hex } from 'viem';
import { BaseExecuteQuoteParams } from './quote';
import { GatewayOrderType } from './order';
import { GatewayTokensInfo } from './token';

type ActionType = 'swap-action' | 'evm-calldata-tx ';

type SwapDirection = 'exact-amount-in' | 'exact-amount-out';

export interface ActionsParams {
    actionType: ActionType;
    sender: Address;
    srcChainId: number;
    srcToken: Address;
    dstChainId: number;
    dstToken: Address;
    slippage: number; // Required range: 0 <= x <= 10000
    amount: string; // swap-action
    swapDirection?: SwapDirection; // swap-action
    recipient?: Address; // swap-action
    to?: Address; // The EVM address of the target contract (20 bytes hex)  evm-calldata-tx
    data?: Hex; // evm-calldata-tx
    value?: string; //Value to send with transaction evm-calldata-tx
    erc20Amount?: string; //The amount of Erc20 being transferred evm-calldata-tx
    erc20Spender?: Address; // The Erc20 spender requiring approval for the transaction call Optional: Will default to the to address evm-calldata-tx
    bridgeIds?: BridgeProtocolId[]; // enum<string>[]
    refundTo?: Address;
}

export interface ActionsResponse {
    tx: {
        to: Address;
        data: Hex;
        value: string;
        chainId: number;
    };
    txId: Hex;
    vmId: VmId;
    amountIn: {
        address: Address;
        decimals: number;
        symbol: string;
        name: string;
        chainId: number;
        isNative: boolean;
        amount: string;
    };
    amountInMax: {
        address: Address;
        decimals: number;
        symbol: string;
        name: string;
        chainId: number;
        isNative: boolean;
        amount: string;
    };
    amountOut: {
        address: Address;
        decimals: number;
        symbol: string;
        name: string;
        chainId: number;
        isNative: boolean;
        amount: string;
    };
    amountOutMin: {
        address: Address;
        decimals: number;
        symbol: string;
        name: string;
        chainId: number;
        isNative: boolean;
        amount: string;
    };
    protocolFee: {
        address: Address;
        decimals: number;
        symbol: string;
        name: string;
        chainId: number;
        isNative: boolean;
        amount: string;
    };
    applicationFee: {
        address: Address;
        decimals: number;
        symbol: string;
        name: string;
        chainId: number;
        isNative: boolean;
        amount: string;
    };
    bridgeFee: {
        address: Address;
        decimals: number;
        symbol: string;
        name: string;
        chainId: number;
        isNative: boolean;
        amount: string;
    };
    bridgeIds: BridgeProtocolId[];
    bridgeRoute: [
        {
            srcChainId: number;
            dstChainId: number;
            srcBridgeToken: Address;
            dstBridgeToken: Address;
            bridgeId: BridgeProtocolId;
        },
    ];
    exchangeRate: number;
    estimatedTxTime: number;
    estimatedPriceImpact: number;
    requiresTokenApproval: boolean;
    allRoutes: [
        {
            tx: {
                to: Address;
                data: Hex;
                value: string;
                chainId: number;
            };
            txId: Hex;
            vmId: VmId;
            amountIn: {
                address: Address;
                decimals: number;
                symbol: string;
                name: string;
                chainId: number;
                isNative: boolean;
                amount: string;
            };
            amountInMax: {
                address: Address;
                decimals: number;
                symbol: string;
                name: string;
                chainId: number;
                isNative: boolean;
                amount: string;
            };
            amountOut: {
                address: Address;
                decimals: number;
                symbol: string;
                name: string;
                chainId: number;
                isNative: boolean;
                amount: string;
            };
            amountOutMin: {
                address: Address;
                decimals: number;
                symbol: string;
                name: string;
                chainId: number;
                isNative: boolean;
                amount: string;
            };
            protocolFee: {
                address: Address;
                decimals: number;
                symbol: string;
                name: string;
                chainId: number;
                isNative: boolean;
                amount: string;
            };
            applicationFee: {
                address: Address;
                decimals: number;
                symbol: string;
                name: string;
                chainId: number;
                isNative: boolean;
                amount: string;
            };
            bridgeFee: {
                address: Address;
                decimals: number;
                symbol: string;
                name: string;
                chainId: number;
                isNative: boolean;
                amount: string;
            };
            bridgeIds: BridgeProtocolId[];
            bridgeRoute: [
                {
                    srcChainId: number;
                    dstChainId: number;
                    srcBridgeToken: string;
                    dstBridgeToken: string;
                    bridgeId: BridgeProtocolId;
                },
            ];
            exchangeRate: number;
            estimatedTxTime: number;
            estimatedPriceImpact: number;
        },
    ];
}

type Status = 'success' | 'pending' | 'requires refund' | 'refunded' | 'failed';

export interface StatusParams {
    txHash: Hex;
    txId: Hex;
    chainId: number;
}

export interface StatusResponse {
    status: Status;
    sender: Address;
    srcChainId: number;
    dstChainId: number;
    srcTxHash: Hex;
    dstTxHash: Hex;
    bridgeDetails: {
        isBridge: boolean;
        bridgeTime: number;
        txPath: [
            {
                chainId: number;
                txHash: Hex;
                timestamp: `${number}`;
                nextBridge: string;
            },
        ];
    };
    txId: Hex;
    actionRequest: {
        actionType: ActionType;
        sender: Address;
        srcChainId: number;
        srcToken: Address;
        dstChainId: number;
        dstToken: Address;
        slippage: number;
        amount: string;
        swapDirection: SwapDirection;
        recipient: Address;
        to: Address;
        data: Hex;
        value: string;
        erc20Amount: string;
        erc20Spender: string;
        bridgeIds: BridgeProtocolId[];
        refundTo: Address;
    };
    actionResponse: {
        tx: {
            to: Address;
            data: Hex;
            value: string;
            chainId: number;
        };
        txId: Hex;
        vmId: VmId;
        amountIn: {
            address: Address;
            decimals: number;
            symbol: string;
            name: string;
            chainId: number;
            isNative: boolean;
            amount: string;
        };
        amountInMax: {
            address: Address;
            decimals: number;
            symbol: string;
            name: string;
            chainId: number;
            isNative: boolean;
            amount: string;
        };
        amountOut: {
            address: Address;
            decimals: number;
            symbol: string;
            name: string;
            chainId: number;
            isNative: boolean;
            amount: string;
        };
        amountOutMin: {
            address: Address;
            decimals: number;
            symbol: string;
            name: string;
            chainId: number;
            isNative: boolean;
            amount: string;
        };
        protocolFee: {
            address: Address;
            decimals: number;
            symbol: string;
            name: string;
            chainId: number;
            isNative: boolean;
            amount: string;
        };
        applicationFee: {
            address: Address;
            decimals: number;
            symbol: string;
            name: string;
            chainId: number;
            isNative: boolean;
            amount: string;
        };
        bridgeFee: {
            address: Address;
            decimals: number;
            symbol: string;
            name: string;
            chainId: number;
            isNative: boolean;
            amount: string;
        };
        bridgeIds: BridgeProtocolId[];
        bridgeRoute: [
            {
                srcChainId: number;
                dstChainId: number;
                srcBridgeToken: string;
                dstBridgeToken: string;
                bridgeId: BridgeProtocolId;
            },
        ];
        exchangeRate: number;
        estimatedTxTime: number;
        estimatedPriceImpact: number;
        requiresTokenApproval: true;
        allRoutes: [
            {
                tx: {
                    to: Address;
                    data: Hex;
                    value: string;
                    chainId: number;
                };
                txId: Hex;
                vmId: VmId;
                amountIn: {
                    address: Address;
                    decimals: number;
                    symbol: string;
                    name: string;
                    chainId: number;
                    isNative: boolean;
                    amount: string;
                };
                amountInMax: {
                    address: Address;
                    decimals: number;
                    symbol: string;
                    name: string;
                    chainId: number;
                    isNative: boolean;
                    amount: string;
                };
                amountOut: {
                    address: Address;
                    decimals: number;
                    symbol: string;
                    name: string;
                    chainId: number;
                    isNative: boolean;
                    amount: string;
                };
                amountOutMin: {
                    address: Address;
                    decimals: number;
                    symbol: string;
                    name: string;
                    chainId: number;
                    isNative: boolean;
                    amount: string;
                };
                protocolFee: {
                    address: Address;
                    decimals: number;
                    symbol: string;
                    name: string;
                    chainId: number;
                    isNative: boolean;
                    amount: string;
                };
                applicationFee: {
                    address: Address;
                    decimals: number;
                    symbol: string;
                    name: string;
                    chainId: number;
                    isNative: boolean;
                    amount: string;
                };
                bridgeFee: {
                    address: Address;
                    decimals: number;
                    symbol: string;
                    name: string;
                    chainId: number;
                    isNative: boolean;
                    amount: string;
                };
                bridgeIds: BridgeProtocolId[];
                bridgeRoute: [
                    {
                        srcChainId: number;
                        dstChainId: number;
                        srcBridgeToken: string;
                        dstBridgeToken: string;
                        bridgeId: BridgeProtocolId;
                    },
                ];
                exchangeRate: number;
                estimatedTxTime: number;
                estimatedPriceImpact: number;
            },
        ];
    };
    org: {
        appId: 'app_number';
        affiliateId: 'affiliate_number';
    };
    usdValue: number;
    srcTx: {
        toAddress: Hex;
        txHash: Hex;
        chainId: number;
        value: '1000000000000000000n';
        timestamp: `${number}`;
        paymentToken: {
            name: 'Ethereum';
            symbol: 'ETH';
            decimals: 18;
            amount: '1000000000000000000n';
            address: Address;
        };
    };
    dstTx: {
        toAddress: Hex;
        txHash: Hex;
        chainId: number;
        value: '1000000000000000000n';
        timestamp: `${number}`;
        paymentToken: {
            name: 'Ethereum';
            symbol: 'ETH';
            decimals: 18;
            amount: '1000000000000000000n';
            address: Address;
        };
    };
}

enum VmId {
    Evm = 'evm',
    Solana = 'solana',
    AltVm = 'alt-vm',
    Hypercore = 'hypercore',
}

export interface Chain {
    chainId: number;
    name: string;
    vmId: VmId;
}

export enum BridgeProtocolId {
    Optimism = 'optimism',
    Arbitrum = 'arbitrum',
    Oft = 'oft',
    YieldOft = 'yield-oft',
    Hyperlane = 'hyperlane',
    Ghost = 'ghost',
    Mayan = 'mayan',
    Across = 'across',
    Relay = 'relay',
    AltVm = 'alt-vm',
}

type Period = 'day' | 'week' | 'month' | 'allTime';

export interface TransactionParams {
    appIds: string; //Comma-separated list of your app IDs. By default, a request to this endpoint will return transactions across all appIds in your org. Each API key has a unique app ID. Multiple app IDs can exist within a single org ID.
    walletAddress: Address;
    chainIds: string; // Comma-separated list of chain IDs to filter by Example: "1,42161,10"
    page: number;
    limit: number;
    period: Period;
    startDate: `${number}`;
    endDate: `${number}`;
}

interface Transaction {
    status: 'success';
    sender: Address;
    srcChainId: number;
    dstChainId: 42161;
    srcTxHash: Hex;
    dstTxHash: Hex;
    bridgeDetails: {
        isBridge: true;
        bridgeTime: 420;
        txPath: [
            {
                chainId: number;
                txHash: Hex;
                timestamp: `${number}`;
                nextBridge: string;
            },
        ];
    };
    txId: Hex;
    actionRequest: {
        actionType: ActionType;
        sender: string;
        srcChainId: number;
        srcToken: string;
        dstChainId: number;
        dstToken: string;
        slippage: number;
        amount: string;
        swapDirection: SwapDirection;
        recipient: string;
        to: string;
        data: string;
        value: string;
        erc20Amount: string;
        erc20Spender: string;
        bridgeIds: BridgeProtocolId[];
        refundTo: string;
    };
    actionResponse: {
        tx: {
            to: string;
            data: string;
            value: string;
            chainId: number;
        };
        txId: string;
        vmId: VmId;
        amountIn: {
            address: Address;
            decimals: number;
            symbol: string;
            name: string;
            chainId: number;
            isNative: true;
            amount: string;
        };
        amountInMax: {
            address: Address;
            decimals: number;
            symbol: string;
            name: string;
            chainId: number;
            isNative: true;
            amount: string;
        };
        amountOut: {
            address: Address;
            decimals: number;
            symbol: string;
            name: string;
            chainId: number;
            isNative: true;
            amount: string;
        };
        amountOutMin: {
            address: Address;
            decimals: number;
            symbol: string;
            name: string;
            chainId: number;
            isNative: true;
            amount: string;
        };
        protocolFee: {
            address: Address;
            decimals: number;
            symbol: string;
            name: string;
            chainId: number;
            isNative: true;
            amount: string;
        };
        applicationFee: {
            address: Address;
            decimals: number;
            symbol: string;
            name: string;
            chainId: number;
            isNative: true;
            amount: string;
        };
        bridgeFee: {
            address: Address;
            decimals: number;
            symbol: string;
            name: string;
            chainId: number;
            isNative: true;
            amount: string;
        };
        bridgeIds: BridgeProtocolId[];
        bridgeRoute: [
            {
                srcChainId: number;
                dstChainId: number;
                srcBridgeToken: string;
                dstBridgeToken: string;
                bridgeId: BridgeProtocolId;
            },
        ];
        exchangeRate: number;
        estimatedTxTime: number;
        estimatedPriceImpact: number;
        requiresTokenApproval: true;
        allRoutes: [
            {
                tx: {
                    to: string;
                    data: string;
                    value: string;
                    chainId: number;
                };
                txId: string;
                vmId: VmId;
                amountIn: {
                    address: Address;
                    decimals: number;
                    symbol: string;
                    name: string;
                    chainId: number;
                    isNative: true;
                    amount: string;
                };
                amountInMax: {
                    address: Address;
                    decimals: number;
                    symbol: string;
                    name: string;
                    chainId: number;
                    isNative: true;
                    amount: string;
                };
                amountOut: {
                    address: Address;
                    decimals: number;
                    symbol: string;
                    name: string;
                    chainId: number;
                    isNative: true;
                    amount: string;
                };
                amountOutMin: {
                    address: Address;
                    decimals: number;
                    symbol: string;
                    name: string;
                    chainId: number;
                    isNative: true;
                    amount: string;
                };
                protocolFee: {
                    address: Address;
                    decimals: number;
                    symbol: string;
                    name: string;
                    chainId: number;
                    isNative: true;
                    amount: string;
                };
                applicationFee: {
                    address: Address;
                    decimals: number;
                    symbol: string;
                    name: string;
                    chainId: number;
                    isNative: true;
                    amount: string;
                };
                bridgeFee: {
                    address: Address;
                    decimals: number;
                    symbol: string;
                    name: string;
                    chainId: number;
                    isNative: true;
                    amount: string;
                };
                bridgeIds: BridgeProtocolId[];
                bridgeRoute: [
                    {
                        srcChainId: number;
                        dstChainId: number;
                        srcBridgeToken: string;
                        dstBridgeToken: string;
                        bridgeId: BridgeProtocolId;
                    },
                ];
                exchangeRate: number;
                estimatedTxTime: number;
                estimatedPriceImpact: number;
            },
        ];
    };
    org: {
        appId: 'app_123';
        affiliateId: 'affiliate_123';
    };
    usdValue: number;
    srcTx: {
        toAddress: Address;
        txHash: Hex;
        chainId: number;
        value: '1000000000000000000n';
        timestamp: `${number}`;
        paymentToken: {
            name: 'Ethereum';
            symbol: 'ETH';
            decimals: 18;
            amount: '1000000000000000000n';
            address: Address;
        };
    };
    dstTx: {
        toAddress: Address;
        txHash: Hex;
        chainId: number;
        value: '1000000000000000000n';
        timestamp: `${number}`;
        paymentToken: {
            name: 'Ethereum';
            symbol: 'ETH';
            decimals: 18;
            amount: '1000000000000000000n';
            address: Address;
        };
    };
}

export interface TransactionResponse {
    txs: Transaction[];
    pagination: {
        page: number;
        limit: number;
        total: number;
        totalPages: number;
    };
}

export interface PathsParams {
    srcChainId: number;
    srcToken: Address;
    dstChainId?: number;
    dstToken?: Address;
    excludeBridgeIds?: BridgeProtocolId[];
    excludeDexIds?: string[]; // DEX IDs to exclude from routing. DEX identifier depends on integration
}

interface Token {
    address: Address;
    decimals: number;
    symbol: string;
    name: string;
    chainId: number;
    isNative: boolean;
    minAmount: string | null;
    maxAmount: string | null;
}

interface Path {
    chainId: number;
    tokens: 'all' | Token[];
    supportsExactAmountIn: boolean;
    supportsExactAmountOut: boolean;
    amountLimits?: {
        minAmount: string | null;
        maxAmount: string | null;
    };
}

export interface PathsResponse {
    paths: Path[];
    timestamp: string;
}

export interface SwapsQuote {
    actionResponse: ActionsResponse;
    actionParams: ActionsParams;
}

// eslint-disable-next-line @typescript-eslint/no-empty-object-type
export type EVMToEVMWithSwapsExecuteQuoteParams<T = {}> = BaseExecuteQuoteParams<T> & {
    type: GatewayOrderType.EVMToEVMWithSwaps;
    data: SwapsQuote & GatewayTokensInfo;
};

// eslint-disable-next-line @typescript-eslint/no-empty-object-type
export type OnrampWithSwapsExecuteQuoteParams<T = {}> = BaseExecuteQuoteParams<T> & {
    type: GatewayOrderType.OnrampWithSwaps;
    data: SwapsQuote & GatewayTokensInfo;
};

// eslint-disable-next-line @typescript-eslint/no-empty-object-type
export type OfframpWithSwapsExecuteQuoteParams<T = {}> = BaseExecuteQuoteParams<T> & {
    type: GatewayOrderType.OfframpWithSwaps;
    data: SwapsQuote & GatewayTokensInfo;
};
