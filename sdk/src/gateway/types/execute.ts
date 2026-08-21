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

function getExecuteQuoteErrorMessage(cause: unknown): string {
    return cause instanceof Error && typeof cause.message === 'string' && cause.message
        ? cause.message
        : EXECUTE_QUOTE_ERROR_MESSAGE;
}

type ExecuteQuoteErrorOptions = { message: string } | { cause: unknown; message?: string };

/** Thrown by {@link GatewayApiClient.executeQuote} for post-order-creation failures. */
export class ExecuteQuoteError extends Error {
    readonly orderId: string;

    constructor(orderId: string, options: ExecuteQuoteErrorOptions) {
        const cause = 'cause' in options ? options.cause : undefined;
        const message = options.message ?? getExecuteQuoteErrorMessage(cause);

        super(message, 'cause' in options ? { cause } : undefined);
        this.name = 'ExecuteQuoteError';
        this.orderId = orderId;
    }
}
