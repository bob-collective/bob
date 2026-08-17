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

// A custom/proxy `message` getter on `cause` could throw while we read it here, which must
// never happen during ExecuteQuoteError construction — fall back to the generic message instead.
function getExecuteQuoteErrorMessage(cause: unknown): string {
    try {
        if (cause instanceof Error && typeof cause.message === 'string') {
            return cause.message || EXECUTE_QUOTE_ERROR_MESSAGE;
        }
    } catch {
        // fall through to the generic message below
    }

    return EXECUTE_QUOTE_ERROR_MESSAGE;
}

/**
 * Thrown by {@link GatewayApiClient.executeQuote} for signing/approval/send failures after
 * order creation. `message` forwards `cause.message` when available so translated/user-facing
 * messages (e.g. the tokenSwap insufficient-funds message) surface at the top level; the
 * original error is always kept as `cause`.
 */
export class ExecuteQuoteError extends Error {
    readonly orderId: string;

    constructor(orderId: string, cause: unknown) {
        super(getExecuteQuoteErrorMessage(cause), { cause });
        this.name = 'ExecuteQuoteError';
        this.orderId = orderId;
    }
}
