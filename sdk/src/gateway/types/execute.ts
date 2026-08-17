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
 * Error thrown by {@link GatewayApiClient.executeQuote} once order creation has succeeded.
 * Carries the Gateway `orderId` so failures during signing, approval, or send can be
 * cross-referenced against the order record, without altering the original error's
 * identity, message, or stack.
 */
export interface ExecuteQuoteError extends Error {
    orderId?: string;
}
