import { type Account, type Address, type Hex, type PublicClient, type Transport } from 'viem';
import type { ExecuteQuoteGasOptions } from '../types';

const GAS_BUFFER_NUM = 12n;
const GAS_BUFFER_DEN = 10n;
const GAS_BUFFER_FIXED = 300_000n;

/**
 * Gas-limit buffer for offramp / tokenSwap sends (bob#1088).
 *
 * Takes the max of a 1.2x multiplier and a fixed 300k cushion, so small txs get
 * the fixed floor and large (aggregator) txs get the multiplier. Unused gas is
 * refunded, so a generous limit is nearly free while a tight one causes
 * out-of-gas failures on gas-heavy routes.
 *
 * Public so callers can size a balance reserve from it — see the README's gas-limit section.
 */
export function applyGasBuffer(estimate: bigint): bigint {
    const multiplied = (estimate * GAS_BUFFER_NUM) / GAS_BUFFER_DEN;
    const fixed = estimate + GAS_BUFFER_FIXED;
    return multiplied > fixed ? multiplied : fixed;
}

/**
 * The Tron adapter is not required to implement `estimateGas`, and its `sendTransaction`
 * takes no `gas` or fee fields (README — required surface), so setting them there breaks the contract.
 */
const CHAINS_WITHOUT_GAS_ESTIMATION = new Set(['tron']);

export function supportsGasEstimation(srcChain: string): boolean {
    return !CHAINS_WITHOUT_GAS_ESTIMATION.has(srcChain.toLowerCase());
}

const FEE_FIELDS = ['gasPrice', 'maxFeePerGas', 'maxPriorityFeePerGas'] as const;

/** Checks types as well as values: a JS caller can pass a `number` where a `bigint` is expected. */
export function assertValidGasOptions(options: ExecuteQuoteGasOptions | undefined): void {
    if (!options) return;

    if (options.gasLimit !== undefined && options.gasLimit !== 'wallet') {
        throw new Error("gasOptions.gasLimit must be 'wallet' when set");
    }

    for (const field of FEE_FIELDS) {
        const value = options[field];
        if (value !== undefined && !(typeof value === 'bigint' && value >= 0n)) {
            throw new Error(`gasOptions.${field} must be a non-negative bigint`);
        }
    }

    if (options.checkBalance !== undefined && typeof options.checkBalance !== 'boolean') {
        throw new Error('gasOptions.checkBalance must be a boolean');
    }

    const { gasPrice, maxFeePerGas, maxPriorityFeePerGas } = options;
    if (gasPrice !== undefined && (maxFeePerGas !== undefined || maxPriorityFeePerGas !== undefined)) {
        throw new Error('gasOptions.gasPrice cannot be combined with maxFeePerGas or maxPriorityFeePerGas');
    }
    if (gasPrice === 0n || maxFeePerGas === 0n) {
        throw new Error(`gasOptions.${gasPrice === 0n ? 'gasPrice' : 'maxFeePerGas'} must be positive`);
    }
    if (maxFeePerGas !== undefined && maxPriorityFeePerGas === undefined) {
        throw new Error('gasOptions.maxFeePerGas requires maxPriorityFeePerGas');
    }
    if (maxFeePerGas !== undefined && maxPriorityFeePerGas !== undefined && maxPriorityFeePerGas > maxFeePerGas) {
        throw new Error('gasOptions.maxPriorityFeePerGas cannot exceed maxFeePerGas');
    }
}

export function feeCap(options: ExecuteQuoteGasOptions | undefined): bigint | undefined {
    return options?.maxFeePerGas ?? options?.gasPrice;
}

export function feeOverrides(options: ExecuteQuoteGasOptions | undefined) {
    if (options?.gasPrice !== undefined) return { gasPrice: options.gasPrice };

    return {
        ...(options?.maxFeePerGas !== undefined && { maxFeePerGas: options.maxFeePerGas }),
        ...(options?.maxPriorityFeePerGas !== undefined && { maxPriorityFeePerGas: options.maxPriorityFeePerGas }),
    };
}

/** A failed `eth_estimateGas` returns `undefined`, leaving the limit to the wallet rather than failing the send. */
export async function resolveGasLimit(
    publicClient: PublicClient<Transport>,
    account: Account,
    tx: { to: Address; data: Hex; value: bigint },
    gasLimit: 'wallet' | undefined
): Promise<{ estimate: bigint; limit: bigint } | undefined> {
    if (gasLimit === 'wallet') return undefined;

    let estimate: bigint;
    try {
        estimate = await publicClient.estimateGas({
            account,
            to: tx.to,
            data: tx.data,
            value: tx.value,
        });
    } catch {
        return undefined;
    }

    return { estimate, limit: applyGasBuffer(estimate) };
}
