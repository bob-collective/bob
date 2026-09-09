import { type Address, isAddress, isAddressEqual } from 'viem';
import { ExecuteQuoteError } from './types';
import { isValidTronAddress, tronAddressToHex } from './utils';

/** 0x Protocol's canonical AllowanceHolder singleton, pre-deployed on most supported EVM chains. */
const ALLOWANCE_HOLDER_0X_CANONICAL: Address = '0x0000000000001fF3684f28c67538d4D072C22734';

/** Gateway self-deployed AllowanceHolder, for chains without 0x's vanity deployment. */
const ALLOWANCE_HOLDER_SELF_DEPLOYED: Address = '0x8fd545b348e84deb145f0179a00c671f0b9519c3';

/** Gateway self-deployed AllowanceHolder on Tron (`TAfbit1ENsRmtZbPQfYU3srURpfYuWYS7K`). */
const ALLOWANCE_HOLDER_TRON: Address = '0x07a39ae4c49dee86e892450b20881f32cd5d500d';

const ALLOWANCE_HOLDER_BY_CHAIN: Record<string, Address> = {
    bob: ALLOWANCE_HOLDER_SELF_DEPLOYED,
    tron: ALLOWANCE_HOLDER_TRON,
    ethereum: ALLOWANCE_HOLDER_0X_CANONICAL,
    base: ALLOWANCE_HOLDER_0X_CANONICAL,
    bsc: ALLOWANCE_HOLDER_0X_CANONICAL,
    arbitrum: ALLOWANCE_HOLDER_0X_CANONICAL,
    avalanche: ALLOWANCE_HOLDER_0X_CANONICAL,
    unichain: ALLOWANCE_HOLDER_0X_CANONICAL,
    plasma: ALLOWANCE_HOLDER_0X_CANONICAL,
    polygon: ALLOWANCE_HOLDER_0X_CANONICAL,
    hyperevm: ALLOWANCE_HOLDER_0X_CANONICAL,
    // Staging names the same chain (id 999) `hyperliquid`; accept both spellings.
    hyperliquid: ALLOWANCE_HOLDER_0X_CANONICAL,
};

/** Normalize a spender the API returned, converting Tron base58 to its hex form. */
function toHexAddress(spender: string): Address | null {
    const hex = isValidTronAddress(spender) ? tronAddressToHex(spender) : spender;
    return hex && isAddress(hex, { strict: false }) ? (hex as Address) : null;
}

/**
 * Throw unless `spender` is the AllowanceHolder the Gateway backend uses for `chain`.
 *
 * The SDK grants an unbounded (`maxUint256`) allowance, so the spender is checked against a
 * hardcoded table rather than trusted from the API response: a wrong or hostile `tx.to` would
 * otherwise put the user's whole token balance at risk instead of one order's worth.
 */
export function assertAllowanceHolderSpender(chain: string, spender: string, orderId: string): void {
    const expected = ALLOWANCE_HOLDER_BY_CHAIN[chain.toLowerCase()];
    if (!expected) {
        throw new ExecuteQuoteError(
            orderId,
            `Refusing to approve ${spender}: no known AllowanceHolder for chain "${chain}". ` +
                `Known chains: ${Object.keys(ALLOWANCE_HOLDER_BY_CHAIN).join(', ')}.`
        );
    }

    const actual = toHexAddress(spender);
    if (!actual || !isAddressEqual(actual, expected)) {
        throw new ExecuteQuoteError(
            orderId,
            `Refusing to approve ${spender} on chain "${chain}": expected the AllowanceHolder at ${expected}.`
        );
    }
}
