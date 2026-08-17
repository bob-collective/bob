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

const EXECUTE_QUOTE_ERROR_MESSAGE = 'Failed to execute Gateway quote after order creation';

// A custom/proxy `message` getter on `cause` could throw here, which must never happen
// during ExecuteQuoteError construction.
function getExecuteQuoteErrorMessage(cause: unknown): string {
    try {
        if (cause instanceof Error && typeof cause.message === 'string') {
            return cause.message || EXECUTE_QUOTE_ERROR_MESSAGE;
        }
    } catch {
        // fall through
    }

    return EXECUTE_QUOTE_ERROR_MESSAGE;
}

/** Thrown by {@link GatewayApiClient.executeQuote} for post-order-creation failures. */
export class ExecuteQuoteError extends Error {
    readonly orderId: string;

    constructor(orderId: string, cause: unknown) {
        super(getExecuteQuoteErrorMessage(cause), { cause });
        this.name = 'ExecuteQuoteError';
        this.orderId = orderId;
    }
}
