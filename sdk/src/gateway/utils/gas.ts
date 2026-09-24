import { type Account, type Address, type Hex, type PublicClient, type Transport } from 'viem';

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
 * Exported because `executeQuote` sends this limit to the wallet, whose pre-flight
 * check is `value + gasLimit * maxFeePerGas <= balance`. A caller reserving native
 * balance for a max-value send must therefore size the reserve from this exact
 * function — deriving the formula independently lets the two drift apart and
 * reproduces the insufficient-funds failure the limit exists to prevent.
 */
export function applyGasBuffer(estimate: bigint): bigint {
    const multiplied = (estimate * GAS_BUFFER_NUM) / GAS_BUFFER_DEN;
    const fixed = estimate + GAS_BUFFER_FIXED;
    return multiplied > fixed ? multiplied : fixed;
}

/**
 * Chains whose adapter does not speak plain EVM JSON-RPC for gas. The Tron adapter's
 * `publicClient` is not required to implement `estimateGas`, and its `sendTransaction`
 * takes no `gas` field (see README — required surface), so estimating there would both
 * break that contract and be discarded.
 */
const CHAINS_WITHOUT_GAS_ESTIMATION = new Set(['tron']);

/** Whether the source chain's client adapter can answer `eth_estimateGas`. */
export function supportsGasEstimation(srcChain: string): boolean {
    return !CHAINS_WITHOUT_GAS_ESTIMATION.has(srcChain.toLowerCase());
}

/**
 * Buffered gas limit for offramp / tokenSwap sends, or `undefined` to fall back to the
 * caller's own estimation. If `eth_estimateGas` reverts we return `undefined` so the send
 * behaves exactly as it does without a limit — never introducing a new failure mode.
 */
export async function estimateGasWithBuffer(
    publicClient: PublicClient<Transport>,
    account: Account,
    tx: { to: Address; data: Hex; value: bigint }
): Promise<bigint | undefined> {
    try {
        const estimate = await publicClient.estimateGas({
            account,
            to: tx.to,
            data: tx.data,
            value: tx.value,
        });
        return applyGasBuffer(estimate);
    } catch {
        return undefined;
    }
}
