import { parseGwei, type Account, type Chain, type PublicClient, type Transport } from 'viem';
import { supportsGasEstimation } from './gas';

/**
 * Minimum EIP-1559 tip per source chain (bob#1214).
 *
 * viem takes the tip from `eth_maxPriorityFeePerGas`, which common Ethereum RPCs return as 0. A
 * zero-tip transaction stalls whenever the base fee rises above its cap, and can then be mined after
 * the order's creation deadline: it reverts with `Order creation deadline has passed` and still costs
 * gas. Only Ethereum gets a floor; L2 tips are orders of magnitude lower, so it would overpay there.
 */
const MIN_PRIORITY_FEE_PER_GAS: Record<string, bigint> = {
    ethereum: parseGwei('0.1'),
};

/** Base-fee headroom in `maxFeePerGas`. viem's default of 1.2x stalls a send after a modest base-fee rise. */
const BASE_FEE_MULTIPLIER = 2;

export interface SendFees {
    maxFeePerGas: bigint;
    maxPriorityFeePerGas: bigint;
}

/**
 * EIP-1559 fees for a Gateway write (approval, allowance reset or order transaction), or
 * `undefined` to leave fees to the wallet as before. Fees are only set when all of these hold:
 *
 * - The account is a local key. JSON-RPC (browser) wallets choose their own fees; explicit fields
 *   would override them, possibly with a zero tip.
 * - The chain takes EVM fee fields (not Tron) and supports EIP-1559, and estimation succeeds.
 * - `value + gas * maxFeePerGas` fits the native balance, so the higher cap never rejects a write
 *   that fit under the wallet's own. An unknown gas limit or balance counts as not fitting.
 */
export async function estimateSendFees(
    publicClient: PublicClient<Transport>,
    srcChain: string,
    account: Account | undefined,
    tx: { value: bigint; gas: bigint | undefined }
): Promise<SendFees | undefined> {
    if (account?.type !== 'local' || !takesFeeFields(srcChain) || tx.gas === undefined) return undefined;
    try {
        const [estimate, balance] = await Promise.all([
            publicClient.estimateFeesPerGas({ chain: withBaseFeeMultiplier(publicClient.chain), type: 'eip1559' }),
            publicClient.getBalance({ address: account.address }),
        ]);
        const floor = MIN_PRIORITY_FEE_PER_GAS[srcChain.toLowerCase()] ?? 0n;
        const tip = estimate.maxPriorityFeePerGas > floor ? estimate.maxPriorityFeePerGas : floor;
        // viem's maxFeePerGas is baseFee * multiplier + tip: keep the base-fee part, swap in the floored tip.
        const baseFeePart = estimate.maxFeePerGas - estimate.maxPriorityFeePerGas;
        const fees = { maxFeePerGas: baseFeePart + tip, maxPriorityFeePerGas: tip };
        return tx.value + tx.gas * fees.maxFeePerGas > balance ? undefined : fees;
    } catch {
        return undefined;
    }
}

/**
 * A simulated write `request` (approval or allowance reset) with {@link estimateSendFees} applied,
 * or unchanged when no fees apply. Its gas is estimated for the balance check. The returned cast
 * keeps the request's own type: viem types simulated requests as possibly legacy, but fees are only
 * returned for EIP-1559 chains.
 */
export async function withSendFees<T extends object>(
    publicClient: PublicClient<Transport>,
    srcChain: string,
    account: Account | undefined,
    request: T
): Promise<T> {
    if (account?.type !== 'local' || !takesFeeFields(srcChain)) return request;
    // viem's simulateContract request type doesn't line up with estimateContractGas's parameters,
    // though the object is exactly what estimateContractGas takes.
    const gas = await publicClient
        .estimateContractGas(request as unknown as Parameters<typeof publicClient.estimateContractGas>[0])
        .catch(() => undefined);
    const value = (request as { value?: bigint }).value ?? 0n;
    const fees = await estimateSendFees(publicClient, srcChain, account, { value, gas });
    return fees ? ({ ...request, ...fees } as T) : request;
}

/** The Tron adapter takes neither a gas limit nor fee fields, so it shares the gas-limit exclusion. */
function takesFeeFields(srcChain: string): boolean {
    return supportsGasEstimation(srcChain);
}

/**
 * The client's chain with `fees.baseFeeMultiplier` raised. viem's estimateFeesPerGas only reads
 * `chain.fees` here, so an absent chain still works as a bare `{ fees }` object.
 */
function withBaseFeeMultiplier(chain: Chain | undefined): Chain {
    return { ...chain, fees: { ...chain?.fees, baseFeeMultiplier: BASE_FEE_MULTIPLIER } } as Chain;
}
