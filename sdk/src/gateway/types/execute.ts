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
    /**
     * The Gateway order id, once the order has been created server-side.
     * Absent only for callback invocations that occur before order creation.
     */
    orderId?: string;
}

/**
 * Error thrown by {@link GatewayApiClient.executeQuote} when a signing, approval, or send
 * operation fails after order creation has succeeded. Carries the Gateway `orderId` so the
 * failure can be cross-referenced against the order record. The original error is preserved
 * as `cause` rather than folded into `message`, since a hostile/custom `message` getter on
 * the original error could itself throw during construction.
 */
export class ExecuteQuoteError extends Error {
    readonly orderId: string;

    constructor(orderId: string, cause: unknown) {
        super('Failed to execute Gateway quote after order creation', { cause });
        this.name = 'ExecuteQuoteError';
        this.orderId = orderId;
    }
}
