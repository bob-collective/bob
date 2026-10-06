export enum ExecuteQuoteStepType {
    SignBitcoinTransaction = 'sign_bitcoin_transaction',
    ResetApproval = 'reset_approval',
    Approve = 'approve',
    SendTransaction = 'send_transaction',
}

export interface ExecuteQuoteStep {
    step: number;
    type: ExecuteQuoteStepType;
    totalSteps: number;
    /** Absent for callback invocations before order creation. */
    orderId?: string;
    /**
     * `SendTransaction` step on EVM sources only: the gas limit attached to the send and, when the limit was
     * derived from one, the `eth_estimateGas` result. Absent when the wallet picks the limit.
     */
    gas?: { estimate?: bigint; limit: bigint };
}

/**
 * Gas limit for the order transaction (the offramp / tokenSwap send):
 * - `bigint`: attached as-is; `eth_estimateGas` is skipped.
 * - function: receives the `eth_estimateGas` result and returns the limit. The default is `applyGasBuffer`.
 * - `'wallet'`: no limit is attached, so the wallet estimates its own.
 */
export type GasLimitOption = bigint | ((estimate: bigint) => bigint) | 'wallet';

/**
 * A fee cap needs its tip: viem fills a missing tip from the node only after the order exists, and fails there
 * when that tip exceeds the cap.
 */
type Eip1559FeeOptions =
    | { maxFeePerGas: bigint; maxPriorityFeePerGas: bigint; gasPrice?: never }
    | { maxFeePerGas?: never; maxPriorityFeePerGas?: bigint; gasPrice?: never };

type LegacyFeeOptions = {
    gasPrice?: bigint;
    maxFeePerGas?: never;
    maxPriorityFeePerGas?: never;
};

/**
 * Gas settings for the EVM transactions `executeQuote` sends. Fees apply to every one of them (allowance
 * reset, approve, order send); `gasLimit` applies to the order send only. Ignored on onramps, which send
 * no EVM transaction, and on Tron sources, whose adapter takes no gas fields.
 */
export type ExecuteQuoteGasOptions = {
    gasLimit?: GasLimitOption;
    /**
     * Before the wallet prompt, throw {@link InsufficientGasFundsError} when the native balance cannot cover
     * `value + gasLimit × fee cap`. Skipped without a fee cap or a resolved limit. Off by default: a sponsored
     * or smart account can pay gas from somewhere this balance does not show.
     */
    checkBalance?: boolean;
} & (Eip1559FeeOptions | LegacyFeeOptions);

/** Thrown by {@link GatewayApiClient.executeQuote} after order creation; `cause`, when present, is the exact caught value. */
export class ExecuteQuoteError extends Error {
    readonly orderId: string;

    readonly name: string = 'ExecuteQuoteError';

    constructor(
        orderId: string,
        message = 'Failed to execute Gateway quote after order creation',
        options?: ErrorOptions
    ) {
        super(message, options);
        this.orderId = orderId;
    }
}

/** Thrown under `gasOptions.checkBalance`. `balance - gasCost` is the largest `value` that would have passed. */
export class InsufficientGasFundsError extends ExecuteQuoteError {
    readonly name: string = 'InsufficientGasFundsError';

    readonly balance: bigint;

    readonly value: bigint;

    /** `gasLimit × fee cap` of the send that was refused. */
    readonly gasCost: bigint;

    constructor(orderId: string, { balance, value, gasCost }: { balance: bigint; value: bigint; gasCost: bigint }) {
        super(orderId, 'Insufficient native balance for the transaction value and its gas');
        this.balance = balance;
        this.value = value;
        this.gasCost = gasCost;
    }
}
