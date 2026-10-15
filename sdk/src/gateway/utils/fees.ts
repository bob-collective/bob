import { parseGwei, type Account, type Chain, type LocalAccount, type PublicClient, type Transport } from 'viem';
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
 * - A write whose `value + gas * maxFeePerGas` would exceed the balance, or can't be checked
 *   because the gas limit or balance is unknown. The higher cap must never reject a write that
 *   fit under the wallet's own, lower one — including approvals paid from a near-empty balance.
 */
export async function estimateSendFees(
    publicClient: PublicClient<Transport>,
    srcChain: string,
    account: Account | undefined,
    tx: { value: bigint; gas: bigint | undefined }
): Promise<SendFees | undefined> {
    if (!setsFees(account, srcChain) || tx.gas === undefined) return undefined;
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
        const balance = await publicClient.getBalance({ address: account.address });
        return tx.value + tx.gas * fees.maxFeePerGas > balance ? undefined : fees;
    } catch {
        return undefined;
    }
}

function setsFees(account: Account | undefined, srcChain: string): account is LocalAccount {
    return account?.type === 'local' && supportsGasEstimation(srcChain);
}

/**
 * A simulated write `request` (approval or allowance reset) with {@link estimateSendFees}
 * applied, estimating its gas for the balance check, or unchanged when no fees apply. The cast
 * keeps the request's own type: viem types simulated requests as possibly legacy, but fees are
 * only returned for EIP-1559 chains.
 */
export async function withSendFees<T extends object>(
    publicClient: PublicClient<Transport>,
    srcChain: string,
    account: Account | undefined,
    request: T
): Promise<T> {
    if (!setsFees(account, srcChain)) return request;
    const gas = await publicClient
        .estimateContractGas(request as unknown as Parameters<typeof publicClient.estimateContractGas>[0])
        .catch(() => undefined);
    const value = (request as { value?: bigint }).value ?? 0n;
    const fees = await estimateSendFees(publicClient, srcChain, account, { value, gas });
    return fees ? ({ ...request, ...fees } as T) : request;
}
