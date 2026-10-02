import { type Account, type Address, type Hex, type PublicClient, type Transport } from 'viem';
import type { ExecuteQuoteGasOptions, GasLimitOption } from '../types';

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

/**
 * Rejects `gasOptions` that would otherwise fail only after the order exists. Checks types as well as
 * values, since a JS caller can pass a `number` where a `bigint` is expected.
 */
export function assertValidGasOptions(options: ExecuteQuoteGasOptions | undefined): void {
    if (!options) return;

    const { gasLimit } = options;
    if (
        gasLimit !== undefined &&
        gasLimit !== 'wallet' &&
        typeof gasLimit !== 'function' &&
        !(typeof gasLimit === 'bigint' && gasLimit > 0n)
    ) {
        throw new Error("gasOptions.gasLimit must be a positive bigint, a function, or 'wallet'");
    }

    for (const field of FEE_FIELDS) {
        const value = options[field];
        if (value !== undefined && !(typeof value === 'bigint' && value >= 0n)) {
            throw new Error(`gasOptions.${field} must be a non-negative bigint`);
        }
    }

    const { gasPrice, maxFeePerGas, maxPriorityFeePerGas } = options;
    if (gasPrice !== undefined && (maxFeePerGas !== undefined || maxPriorityFeePerGas !== undefined)) {
        throw new Error('gasOptions.gasPrice cannot be combined with maxFeePerGas or maxPriorityFeePerGas');
    }
    if (maxFeePerGas !== undefined && maxPriorityFeePerGas !== undefined && maxPriorityFeePerGas > maxFeePerGas) {
        throw new Error('gasOptions.maxPriorityFeePerGas cannot exceed maxFeePerGas');
    }
}

/** The fee fields the caller set, to spread into a viem write. Empty when none were. */
export function feeOverrides(options: ExecuteQuoteGasOptions | undefined) {
    if (options?.gasPrice !== undefined) return { gasPrice: options.gasPrice };

    return {
        ...(options?.maxFeePerGas !== undefined && { maxFeePerGas: options.maxFeePerGas }),
        ...(options?.maxPriorityFeePerGas !== undefined && { maxPriorityFeePerGas: options.maxPriorityFeePerGas }),
    };
}

/**
 * Gas limit for an offramp / tokenSwap send under `option`, or `undefined` to leave the limit to the
 * wallet. A failed `eth_estimateGas` also returns `undefined`, so the send behaves exactly as it does
 * without a limit — never introducing a new failure mode.
 */
export async function resolveGasLimit(
    publicClient: PublicClient<Transport>,
    account: Account,
    tx: { to: Address; data: Hex; value: bigint },
    option: GasLimitOption = applyGasBuffer
): Promise<{ estimate?: bigint; limit: bigint } | undefined> {
    if (option === 'wallet') return undefined;
    if (typeof option === 'bigint') return { limit: option };

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

    const limit = option(estimate);
    if (typeof limit !== 'bigint' || limit <= 0n) {
        throw new Error('gasOptions.gasLimit function must return a positive bigint');
    }

    return { estimate, limit };
}
