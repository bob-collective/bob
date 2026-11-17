import { base64 } from '@scure/base';
import type { GatewayTxData } from './generated-client';
import {
    ExecuteQuoteError,
    type ExecuteQuoteStep,
    ExecuteQuoteStepType,
    type SolanaConnection,
    type SolanaSignatureStatus,
    type SolanaWallet,
} from './types';
import { isValidSolanaAddress, readSolanaTransactionHeader, type SolanaTransactionHeader } from './utils';

/** About the lifetime of the transaction's blockhash: past it, the transaction can no longer land. */
const CONFIRMATION_TIMEOUT_MS = 90_000;
const CONFIRMATION_POLL_INTERVAL_MS = 1_000;

export function isSolanaWallet(value: unknown): value is SolanaWallet {
    return (
        typeof value === 'object' &&
        value !== null &&
        typeof (value as SolanaWallet).signAndSendTransaction === 'function'
    );
}

export function isSolanaConnection(value: unknown): value is SolanaConnection {
    return (
        typeof value === 'object' &&
        value !== null &&
        typeof (value as SolanaConnection).getSignatureStatuses === 'function'
    );
}

export interface SolanaClients {
    wallet: SolanaWallet;
    connection: SolanaConnection;
}

/** The clients of an order from Solana, checked before the order is created so a wrong one leaves no order behind. */
export function solanaClients(walletClient: unknown, publicClient: unknown): SolanaClients {
    if (!isSolanaWallet(walletClient)) {
        throw new Error('walletClient must be a SolanaWallet for an order from Solana');
    }
    if (!isValidSolanaAddress(walletClient.address)) {
        throw new Error('SolanaWallet address is not a valid Solana address');
    }
    if (!isSolanaConnection(publicClient)) {
        throw new Error('publicClient must be a SolanaConnection for an order from Solana');
    }

    return { wallet: walletClient, connection: publicClient };
}

/** Text of a failed transaction's `err`, e.g. `{"InstructionError":[0,{"Custom":1}]}`. */
function describeTransactionError(err: unknown): string | undefined {
    try {
        return typeof err === 'string' ? err : JSON.stringify(err);
    } catch {
        return undefined;
    }
}

export interface SolanaConfirmationOptions {
    timeoutMs?: number;
    pollIntervalMs?: number;
}

/**
 * Poll the signature until the cluster confirms it. A failed transaction throws `reverted`; one not confirmed by the
 * deadline throws `unconfirmed`. RPC errors are retried until the deadline, as the transaction is already out.
 */
export async function waitForSolanaConfirmation(
    orderId: string,
    connection: SolanaConnection,
    signature: string,
    {
        timeoutMs = CONFIRMATION_TIMEOUT_MS,
        pollIntervalMs = CONFIRMATION_POLL_INTERVAL_MS,
    }: SolanaConfirmationOptions = {}
): Promise<void> {
    const step = ExecuteQuoteStepType.SendTransaction;
    const deadline = Date.now() + timeoutMs;
    let lastError: unknown;

    for (;;) {
        let status: SolanaSignatureStatus | null | undefined;
        try {
            status = (await connection.getSignatureStatuses([signature])).value[0];
            lastError = undefined;
        } catch (error) {
            lastError = error;
        }

        if (status && status.err !== null && status.err !== undefined) {
            const details = describeTransactionError(status.err);
            throw new ExecuteQuoteError(
                orderId,
                details ? `Transaction reverted: ${details}` : 'Transaction reverted',
                {
                    step,
                    txHash: signature,
                    txStatus: 'reverted',
                }
            );
        }

        if (status?.confirmationStatus === 'confirmed' || status?.confirmationStatus === 'finalized') return;

        if (Date.now() >= deadline) {
            throw new ExecuteQuoteError(orderId, 'Transaction not confirmed in time', {
                cause: lastError,
                step,
                txHash: signature,
                txStatus: 'unconfirmed',
            });
        }

        await new Promise((resolve) => setTimeout(resolve, pollIntervalMs));
    }
}

/**
 * Have the wallet sign and send the transaction Gateway built for an order from Solana, and wait until it is
 * confirmed. Solana has no token approvals, so this is the order's only step.
 */
export async function sendSolanaOrderTransaction(
    orderId: string,
    tx: GatewayTxData,
    { wallet, connection }: SolanaClients,
    callback?: (step: ExecuteQuoteStep) => void
): Promise<string> {
    if (tx.type !== 'solana') {
        throw new ExecuteQuoteError(
            orderId,
            `Gateway returned a ${String(tx.type)} transaction for an order from Solana`
        );
    }

    let transaction: Uint8Array;
    let header: SolanaTransactionHeader;
    try {
        transaction = base64.decode(tx.transaction);
        header = readSolanaTransactionHeader(transaction);
    } catch (error) {
        throw new ExecuteQuoteError(orderId, 'Gateway returned a malformed Solana transaction', { cause: error });
    }

    // Only the wallet signs. A transaction needing another signer would have to come partially signed by Gateway.
    if (header.numRequiredSignatures !== 1) {
        throw new ExecuteQuoteError(
            orderId,
            `Solana transaction requires ${header.numRequiredSignatures} signatures; only the wallet's is supported`
        );
    }
    if (header.feePayer !== wallet.address) {
        throw new ExecuteQuoteError(orderId, 'Solana transaction fee payer is not the wallet address');
    }

    const step = ExecuteQuoteStepType.SendTransaction;
    callback?.({ step: 1, type: step, totalSteps: 1, orderId });

    let signature: string;
    try {
        signature = await wallet.signAndSendTransaction(transaction);
    } catch (error) {
        throw new ExecuteQuoteError(orderId, undefined, { cause: error, step });
    }

    await waitForSolanaConfirmation(orderId, connection, signature);

    return signature;
}
