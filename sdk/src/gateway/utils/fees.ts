import { parseGwei, type Account, type Chain, type PublicClient, type Transport } from 'viem';
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
 * EIP-1559 fees for Gateway sends and their approvals, or `undefined` to leave fees to the
 * wallet, exactly as without this function:
 *
 * - JSON-RPC (browser) accounts: the wallet picks its own fees, and explicit fields would
 *   override them — off Ethereum, possibly with a zero tip. The zero-tip problem is viem's
 *   default for local-key accounts.
 * - Adapters that take no fee fields (Tron), and estimation failures (legacy chains, RPC errors).
 * - An order `send` carrying native value whose `value + gas * maxFeePerGas` would exceed the
 *   balance (or can't be checked), so the higher cap never rejects a send sized against the
 *   wallet's own, lower one.
 */
export async function estimateSendFees(
    publicClient: PublicClient<Transport>,
    srcChain: string,
    account: Account | undefined,
    send?: { value: bigint; gas?: bigint }
): Promise<SendFees | undefined> {
    if (account?.type !== 'local' || !supportsGasEstimation(srcChain)) return undefined;
    try {
        const chain = {
            ...publicClient.chain,
            fees: { ...publicClient.chain?.fees, baseFeeMultiplier: BASE_FEE_MULTIPLIER },
        } as Chain;
        const estimate = await publicClient.estimateFeesPerGas({ chain, type: 'eip1559' });
        const floor = MIN_PRIORITY_FEE_PER_GAS[srcChain.toLowerCase()] ?? 0n;
        const tip = estimate.maxPriorityFeePerGas > floor ? estimate.maxPriorityFeePerGas : floor;
        const fees = {
            maxPriorityFeePerGas: tip,
            maxFeePerGas: estimate.maxFeePerGas - estimate.maxPriorityFeePerGas + tip,
        };
        if (send && send.value > 0n) {
            if (send.gas === undefined) return undefined;
            const balance = await publicClient.getBalance({ address: account.address });
            if (send.value + send.gas * fees.maxFeePerGas > balance) return undefined;
        }
        return fees;
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
    account: Account | undefined,
    request: T
): Promise<T> {
    const fees = await estimateSendFees(publicClient, srcChain, account);
    return fees ? ({ ...request, ...fees } as T) : request;
}
