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

/**
 * Text of a caught value, for the message of the error wrapping it.
 *
 * viem's `message` appends request arguments, docs URL and version to `shortMessage`, while `details`
 * carries the wallet's or node's own text (e.g. `InternalRpcError`'s `shortMessage` is generic). Read
 * structurally rather than via `instanceof BaseError`: a consumer on another viem version holds another class.
 */
function describeCause(cause: unknown): string | undefined {
    if (typeof cause === 'string') return cause || undefined;
    if (typeof cause !== 'object' || cause === null) return undefined;

    // A caught value can carry a throwing getter, which would replace the error being reported.
    try {
        const { shortMessage, details, message } = cause as {
            shortMessage?: unknown;
            details?: unknown;
            message?: unknown;
        };

        if (typeof shortMessage === 'string' && shortMessage) {
            return typeof details === 'string' && details && !shortMessage.includes(details)
                ? `${shortMessage} (${details})`
                : shortMessage;
        }

        return typeof message === 'string' && message ? message : undefined;
    } catch {
        return undefined;
    }
}

export type ExecuteQuoteErrorOptions = ErrorOptions & {
    /** The step whose operation failed. Absent outside one: an allowance read, or transaction registration. */
    step?: ExecuteQuoteStepType;
    /** Set once the transaction was broadcast, when waiting for its receipt is what failed. */
    txHash?: string;
};

/**
 * Thrown by {@link GatewayApiClient.executeQuote} after order creation; `cause`, when present, is the exact caught value.
 * Without an explicit `message`, the message is the cause's own text.
 */
export class ExecuteQuoteError extends Error {
    readonly orderId: string;

    readonly name = 'ExecuteQuoteError';

    readonly step?: ExecuteQuoteStepType;

    readonly txHash?: string;

    constructor(orderId: string, message?: string, { step, txHash, ...options }: ExecuteQuoteErrorOptions = {}) {
        super(message || describeCause(options.cause) || EXECUTE_QUOTE_ERROR_MESSAGE, options);
        this.orderId = orderId;
        this.step = step;
        this.txHash = txHash;
    }
}
