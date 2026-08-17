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
 * Thrown by {@link GatewayApiClient.executeQuote} for signing/approval/send failures after
 * order creation. Original error is kept as `cause`, not folded into `message` — a hostile
 * `message` getter on the original could otherwise throw during construction.
 */
export class ExecuteQuoteError extends Error {
    readonly orderId: string;

    constructor(orderId: string, cause: unknown) {
        super('Failed to execute Gateway quote after order creation', { cause });
        this.name = 'ExecuteQuoteError';
        this.orderId = orderId;
    }
}
