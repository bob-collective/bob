import { type Address, isAddress, isAddressEqual } from 'viem';
import { ALLOWANCE_HOLDER_BY_CHAIN } from './chains';
import { ExecuteQuoteError } from './types';
import { isValidTronAddress, tronAddressToHex } from './utils';

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
