import { parseGwei, type Chain, type PublicClient, type Transport } from 'viem';
import { supportsGasEstimation } from './gas';

/**
 * Minimum EIP-1559 tip per source chain for Gateway sends (bob#1214).
 *
 * viem takes the tip from `eth_maxPriorityFeePerGas`, which common Ethereum RPCs return as 0.
 * A zero-tip transaction waits whenever the base fee rises above its cap and can then be mined
 * after the order's creation deadline, reverting with `Order creation deadline has passed` while
 * still costing gas. Only Ethereum gets a floor: L2 tips are orders of magnitude lower, so the
 * same floor would overpay there.
 */
const MIN_PRIORITY_FEE_PER_GAS: Record<string, bigint> = {
    ethereum: parseGwei('0.1'),
};

/** Base-fee headroom for `maxFeePerGas`: viem's default 1.2x strands a send after a modest base-fee rise. */
const BASE_FEE_MULTIPLIER = 2;

export interface SendFees {
    maxFeePerGas: bigint;
    maxPriorityFeePerGas: bigint;
}

/**
 * EIP-1559 fees for Gateway sends and their approvals, or `undefined` to fall back to the
 * wallet's own fee estimation. Estimation failures (legacy chains, RPC errors) and adapters
 * that take no fee fields return `undefined`, so a send behaves exactly as it does without
 * explicit fees — never introducing a new failure mode.
 */
export async function estimateSendFees(
    publicClient: PublicClient<Transport>,
    srcChain: string
): Promise<SendFees | undefined> {
    if (!supportsGasEstimation(srcChain)) return undefined;
    try {
        const chain = {
            ...publicClient.chain,
            fees: { ...publicClient.chain?.fees, baseFeeMultiplier: BASE_FEE_MULTIPLIER },
        } as Chain;
        const estimate = await publicClient.estimateFeesPerGas({ chain, type: 'eip1559' });
        const floor = MIN_PRIORITY_FEE_PER_GAS[srcChain.toLowerCase()] ?? 0n;
        const tip = estimate.maxPriorityFeePerGas > floor ? estimate.maxPriorityFeePerGas : floor;
        return {
            maxPriorityFeePerGas: tip,
            maxFeePerGas: estimate.maxFeePerGas - estimate.maxPriorityFeePerGas + tip,
        };
    } catch {
        return undefined;
    }
}

/**
 * A simulated write `request` with {@link estimateSendFees} applied, or unchanged when no fees
 * can be estimated. The cast keeps the request's own type: viem types simulated requests as
 * possibly legacy, but fees are only returned for EIP-1559 chains.
 */
export async function withSendFees<T extends object>(
    publicClient: PublicClient<Transport>,
    srcChain: string,
    request: T
): Promise<T> {
    const fees = await estimateSendFees(publicClient, srcChain);
    return fees ? ({ ...request, ...fees } as T) : request;
}
