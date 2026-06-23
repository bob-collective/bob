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
 */
export function applyGasBuffer(estimate: bigint): bigint {
    const multiplied = (estimate * GAS_BUFFER_NUM) / GAS_BUFFER_DEN;
    const fixed = estimate + GAS_BUFFER_FIXED;
    return multiplied > fixed ? multiplied : fixed;
}

/**
 * Buffered gas limit for local-key sends, or `undefined` to fall back to viem's
 * default estimation. If `eth_estimateGas` reverts we return `undefined` so the
 * send behaves exactly as it does today — never introducing a new failure mode.
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
