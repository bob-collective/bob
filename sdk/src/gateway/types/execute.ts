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
     * `SendTransaction` step on EVM sources only: the `eth_estimateGas` result and the limit attached to the
     * send. Absent when the wallet picks the limit.
     */
    gas?: { estimate: bigint; limit: bigint };
}

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
    /** `'wallet'` attaches no limit to the order send, so the wallet estimates its own. */
    gasLimit?: 'wallet';
} & (Eip1559FeeOptions | LegacyFeeOptions);

/** Thrown by {@link GatewayApiClient.executeQuote} after order creation; `cause`, when present, is the exact caught value. */
export class ExecuteQuoteError extends Error {
    readonly orderId: string;

    readonly name = 'ExecuteQuoteError';

    constructor(
        orderId: string,
        message = 'Failed to execute Gateway quote after order creation',
        options?: ErrorOptions
    ) {
        super(message, options);
        this.orderId = orderId;
    }
}
