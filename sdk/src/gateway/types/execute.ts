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
}

/**
 * The transaction `executeQuote` is about to ask the wallet to sign, reported once the gas estimate is in
 * and before the wallet opens.
 *
 * Exists so a caller can size a native-token balance reserve from the real transaction rather than from an
 * assumed route cost: `data` gives the calldata an OP-stack L1 data fee is charged on, and `estimatedGas`
 * the units the route actually needs. Both are unknowable before the order exists, and the order is only
 * created inside this call.
 */
export interface PreparedTransaction {
    orderId: string;
    /** Source chain as the Gateway names it, e.g. `ethereum`, `base`. */
    chain: string;
    to: string;
    data: string;
    value: bigint;
    /** `eth_estimateGas` on this transaction. Absent when the chain has no estimation, or it reverted. */
    estimatedGas?: bigint;
    /** The limit attached to the send, `applyGasBuffer(estimatedGas)`. Absent whenever `estimatedGas` is. */
    gasLimit?: bigint;
}

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
