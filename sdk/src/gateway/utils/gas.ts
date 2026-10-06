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

    if (options.checkBalance !== undefined && typeof options.checkBalance !== 'boolean') {
        throw new Error('gasOptions.checkBalance must be a boolean');
    }

    const { gasPrice, maxFeePerGas, maxPriorityFeePerGas } = options;
    if (gasPrice !== undefined && (maxFeePerGas !== undefined || maxPriorityFeePerGas !== undefined)) {
        throw new Error('gasOptions.gasPrice cannot be combined with maxFeePerGas or maxPriorityFeePerGas');
    }
    // A zero cap can never be included, so the send would sit pending after the order exists.
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

/** The cap the wallet multiplies the gas limit by in its balance check, or `undefined` when the wallet picks it. */
export function feeCap(options: ExecuteQuoteGasOptions | undefined): bigint | undefined {
    return options?.maxFeePerGas ?? options?.gasPrice;
}

const FEE_HISTORY_BLOCKS = 10;

/**
 * EIP-1559 fees for `gasOptions`, from the chain's own fee history:
 * - `maxPriorityFeePerGas`: the median of each recent block's median tip. `eth_maxPriorityFeePerGas` can run
 *   far below what blocks include (0.00002 gwei against a 0.078 gwei median on Ethereum, 2026-10-06), and
 *   blocks that included nothing report a zero tip, so those are left out.
 * - `maxFeePerGas`: twice the next block's base fee plus that tip. The base fee rises at most 12.5% a block,
 *   so doubling it keeps the transaction includable through about six full blocks (the rule ethers uses).
 *
 * Use the same result to size a native-balance reserve and as `gasOptions`, so the wallet's check runs
 * against the fee the reserve was sized with.
 */
export async function estimateGatewayFees(
    publicClient: Pick<PublicClient<Transport>, 'getFeeHistory'>
): Promise<{ maxFeePerGas: bigint; maxPriorityFeePerGas: bigint }> {
    const history = await publicClient.getFeeHistory({ blockCount: FEE_HISTORY_BLOCKS, rewardPercentiles: [50] });
    const nextBaseFee = history.baseFeePerGas.at(-1) ?? 0n;
    const tips = (history.reward ?? [])
        .map(([tip]) => tip ?? 0n)
        .filter((tip) => tip > 0n)
        .sort((a, b) => (a < b ? -1 : a > b ? 1 : 0));
    const maxPriorityFeePerGas = tips[Math.floor(tips.length / 2)] ?? 0n;
    const maxFeePerGas = nextBaseFee * 2n + maxPriorityFeePerGas;

    if (maxFeePerGas === 0n) {
        throw new Error('Fee history reported no base fee and no tips; pass gasPrice for this chain instead');
    }

    return { maxFeePerGas, maxPriorityFeePerGas };
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
