import { Address } from 'viem';
import { offrampCaller } from '../abi';
import { BaseExecuteQuoteParams } from './quote';
import { GatewayOrderType } from './order';

/** @dev Internal */
export type OfframpGatewayCreateQuoteResponse = {
    amountToLock: string;
    minimumFeesToPay: string;
    gateway: Address;
};

export type OfframpOrderStatus = 'Active' | 'Accepted' | 'Processed' | 'Refunded';

/** @dev Detailed breakdown of fees associated with an offramp quote */
export interface OfframpFeeBreakdown {
    /** @dev Total fees in satoshis */
    overallFeeSats: number;
    /** @dev Fee for transaction inclusion */
    inclusionFeeSats: number;
    /** @dev Protocol-specific fee */
    protocolFeeSats: number;
    /** @dev Affiliate-related fee */
    affiliateFeeSats: number;
    /** @dev Fastest available fee rate (e.g., sat/vB) */
    fastestFeeRate: number;
    /** @dev Fastest gas fees on the source chain */
    gasFee?: bigint;
}

/** @dev Offramp order quote returned by the quoting logic */
export interface OfframpQuote {
    /** @dev Amount to lock in satoshis */
    amountLockInSat: number;
    /** @dev Amount the user will receive after deducting fees (in satoshis) */
    amountReceiveInSat: number;
    /** @dev Deadline for order creation (unix timestamp) */
    deadline: number;
    /** @dev Address of the off-ramp registry handling the order */
    registryAddress: Address;
    /** @dev Token address used for payment */
    token: Address;
    /** @dev Detailed fee breakdown */
    feeBreakdown: OfframpFeeBreakdown;
}

/** @dev Offramp Available Liquidity */
export interface OfframpLiquidity {
    /** @dev Token address used for payment */
    token: Address;
    /** @dev Max token amount a *single* order can be served with (in token decimals) */
    maxOrderAmount: bigint;
    /** @dev Total liquidity across all solver addresses (in token decimals) */
    totalOfframpLiquidity: bigint;
}

/** @dev Offramp Available Liquidity */
export interface OfframpLiquidityV2 {
    /** @dev Token address used for payment */
    token: Address;
    /** @dev Max sats amount a *single* order can be served with for a given user address */
    maxOrderAmountInSats: bigint;
    /** @dev Total liquidity across all solver addresses */
    totalOfframpLiquidityInSats: bigint;
    /** @dev Minimum offramp quote liquidity details */
    minimumOfframpQuote: MinimumOfframpQuote;
}

export interface MinimumOfframpQuote {
    /** @dev Minimum sats amount a *single* order can be served */
    minimumAmountInSats: bigint;
    /** @dev Minimum offramp quote calculated for fee rate */
    calculatedForFeeRate: bigint;
}

export interface OnrampLiquidity {
    /** @dev Token address used for payment */
    tokenAddress: Address;
    /** @dev Max sats amount a *single* order can be served with for a given user address */
    maxOrderAmountInSats: bigint;
    /** @dev Total liquidity across all gateway addresses */
    totalOnrampLiquidityInSats: bigint;
    /** @dev Minimum sats onramp quote liquidity details */
    minSatsAmount: bigint;
}

/** @dev Params used for createOrder call on the off-ramp contract */
export type OfframpCreateOrderParams = {
    quote: OfframpQuote;
    feeBreakdown: OfframpFeeBreakdown;
    offrampABI: typeof offrampCaller;
    offrampFunctionName: 'createOrder';
    offrampArgs: [
        {
            /** @dev Amount of sats to lock in the order */
            satAmountToLock: bigint;
            /** @dev Max sats to be paid as fees */
            satFeesMax: bigint;
            /** @dev Timestamp by which the order must be created */
            creationDeadline: bigint;
            /** @dev Output script for Bitcoin settlement */
            outputScript: `0x${string}`;
            /** @dev Token to use for payment */
            token: Address;
            /** @dev EVM address of the user who can unlock the order or bump its fee */
            owner: Address;
        },
    ];
};

/** @dev Final state of an offramp order used in UI/backend */
export type OfframpOrder = {
    /** @dev Unique identifier for the off-ramp order */
    orderId: bigint;
    /** @dev The token address */
    token: Address;
    /** @dev The amount of sats that is locked */
    satAmountLocked: bigint;
    /** @dev The amount of fees to pay in sat */
    satFeesMax: bigint;
    /** @dev The current status of the order */
    status: OfframpOrderStatus;
    /** @dev The timestamp when the order was created or updated */
    orderTimestamp: number;
    /** @dev The user submit order transaction hash on the EVM chain */
    submitOrderEvmTx: string | null;
    /** @dev The user refunded order transaction hash on the EVM chain */
    refundedEvmTx: string | null;
    /** @dev The transaction ID on the Bitcoin network */
    btcTx: string | null;
    /** @dev Indicates whether the fees for this order should be bumped based on current network conditions */
    shouldFeesBeBumped: boolean;
    /** @dev Indicates whether the user can unlock this order (typically if it's still active) */
    canOrderBeUnlocked: boolean;
    /** @dev The offramp registry address the order is related to */
    offrampRegistryAddress: Address;
};

/** @dev Internal, On-chain fetched state of an active/processed/refunded order */
export type OnchainOfframpOrderDetails = {
    /** @dev Unique identifier for the off-ramp order */
    orderId: bigint;
    /** @dev The token address used for payment */
    token: Address;
    /** @dev Amount locked in sats */
    satAmountLocked: bigint;
    /** @dev Max sats for fee */
    satFeesMax: bigint;
    /** @dev Address that created the order */
    owner: Address;
    /** @dev Optional solver owner address */
    solverOwner: Address | null;
    /** @dev Optional solver recipient address */
    solverRecipient: Address | null;
    /** @dev Output script for Bitcoin tx */
    outputScript: string;
    /** @dev Order status */
    status: OfframpOrderStatus;
    /** @dev When the order was created (unix timestamp) */
    orderTimestamp: number;
};

/** @dev Internal */
export interface OfframpRawOrder {
    orderId: string;
    token: string;
    satAmountLocked: string;
    satFeesMax: string;
    status: string;
    orderTimestamp: string;
    btcTx: string | null;
    submitOrderEvmTx: string | null;
    shouldFeesBeBumped: boolean;
    refundedEvmTx: string | null;
}

// eslint-disable-next-line @typescript-eslint/no-empty-object-type
export type OfframpExecuteQuoteParams<T = {}> = BaseExecuteQuoteParams<T> & {
    type: GatewayOrderType.Offramp;
    data: OfframpQuote;
};

export interface BumpFeeParams {
    orderId: bigint;
}

export interface UnlockOrderParams {
    orderId: bigint;
    receiver: Address;
}
