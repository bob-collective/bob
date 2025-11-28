import { Address } from 'viem';
import { offrampCallerV2 } from '../abi';
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
    /** @dev Amount the user will receive after deducting offramp and affiliate fee (in satoshis) */
    amountReceiveInSat: number;
    /** @dev Deadline for order creation (unix timestamp) */
    deadline: number;
    /** @dev Token address used for payment */
    token: Address;
    /** @dev Detailed fee breakdown */
    feeBreakdown: OfframpFeeBreakdown;
    /** @dev The address of the affiliate who will receive the affiliate fee */
    affiliateFeeRecipient: Address;
}

/** @dev Offramp available liquidity details */
export interface OfframpLiquidity {
    /** @dev Address of the token accepted for offramp payments */
    tokenAddress: Address;
    /** @dev Maximum sats amount that a *single* order can be served with for a specific user address */
    maxOrderAmountInSats: bigint;
    /** @dev Total available offramp liquidity across all solver addresses */
    totalOfframpLiquidityInSats: bigint;
    /** @dev Details about the minimum offramp quote, including fee rate */
    minimumOfframpQuote: MinimumOfframpQuote;
}

/** @dev Minimum offramp quote information */
export interface MinimumOfframpQuote {
    /** @dev Minimum sats amount that can be processed for a single offramp order */
    minimumAmountInSats: bigint;
    /** @dev Fee rate used to calculate the minimum offramp quote */
    calculatedForFeeRate: bigint;
}

/** @dev Onramp available liquidity details */
export interface OnrampLiquidity {
    /** @dev Address of the token accepted for onramp payments */
    tokenAddress: Address;
    /** @dev Maximum sats amount that a *single* order can be served with for a specific user address */
    maxOrderAmountInSats: bigint;
    /** @dev Total available onramp liquidity across all gateway addresses */
    totalOnrampLiquidityInSats: bigint;
    /** @dev Minimum sats amount that can be processed for an onramp order */
    minSatsAmount: bigint;
}

/** @dev Params used for createOrder call on the off-ramp contract */
export type OfframpCreateOrderParams = {
    quote: OfframpQuote;
    feeBreakdown: OfframpFeeBreakdown;
    offrampABI: typeof offrampCallerV2;
    offrampFunctionName: 'createOrderV2';
    offrampArgs: [
        {
            /** @dev Amount of sats to lock in the order */
            satAmountToLock: bigint;
            /** @dev Max sats to be paid as fees */
            satSolverFeeMax: bigint;
            /** @dev Amount of fee to be paid to affiliate */
            satAffiliateFee: bigint;
            /** @dev The address of the affiliate who will receive the affiliate fee */
            affiliateFeeRecipient: Address;
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
    /** @dev The amount of fees to pay in sat to the solver */
    satSolverFeeMax: bigint;
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
    /** @dev The affiliate fee (in satoshis) given part of the order. */
    satAffiliateFee: bigint;
    /** @dev The address of the affiliate who will receive the affiliate fee. */
    affiliateFeeRecipient: Address;
    /** @dev The version number of the Offramp Registry contract used for this order. */
    offrampRegistryVersion: number;
    /** @dev The additional fee amount (in satoshis) required to bump the transaction to get accepted, or `null` if not applicable. */
    bumpFeeAmountInSats: bigint | null;
    /** @dev The address of the user who placed the order. */
    userAddress: Address;
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
    offrampRegistryAddress: string;
    satAffiliateFee: string;
    affiliateFeeRecipient: string;
    offrampRegistryVersion: string;
    bumpFeeAmountInSats: string | null;
    userAddress: string;
}

// eslint-disable-next-line @typescript-eslint/no-empty-object-type
export type OfframpExecuteQuoteParams<T = {}> = BaseExecuteQuoteParams<T> & {
    type: GatewayOrderType.Offramp;
    data: OfframpQuote;
};

export interface BumpFeeParams {
    orderId: bigint;
    offrampRegistryAddress: Address;
}

export interface UnlockOrderParams {
    orderId: bigint;
    receiver: Address;
    offrampRegistryAddress: Address;
}
