import { base58 } from '@scure/base';

const PUBLIC_KEY_LENGTH = 32;
const SIGNATURE_LENGTH = 64;
const VERSION_PREFIX_MASK = 0x80;

export function isSolanaChain(chain: string): boolean {
    return chain.toLowerCase() === 'solana';
}

export function isValidSolanaAddress(address: string): boolean {
    try {
        return base58.decode(address).length === PUBLIC_KEY_LENGTH;
    } catch {
        return false;
    }
}

/** What `executeQuote` checks of a Solana transaction before its wallet signs it. */
export interface SolanaTransactionHeader {
    /** Base58 public key of the fee payer, the first account key. */
    feePayer: string;
    numRequiredSignatures: number;
}

/** Read a compact-u16 ("short_vec" length): 7 bits per byte, low bits first, at most 3 bytes. */
function readCompactU16(bytes: Uint8Array, offset: number): [value: number, next: number] {
    let value = 0;
    for (let i = 0; i < 3; i++) {
        const byte = bytes[offset + i];
        if (byte === undefined) throw new Error('Solana transaction is truncated');
        value |= (byte & 0x7f) << (7 * i);
        if ((byte & 0x80) === 0) return [value, offset + i + 1];
    }
    throw new Error('Solana transaction has an invalid length prefix');
}

/**
 * Read the header of a serialized `VersionedTransaction` (legacy or v0): its fee payer and how many signatures it
 * requires. Throws on bytes that are not one.
 */
export function readSolanaTransactionHeader(bytes: Uint8Array): SolanaTransactionHeader {
    const [signatureCount, signaturesStart] = readCompactU16(bytes, 0);
    let offset = signaturesStart + signatureCount * SIGNATURE_LENGTH;

    const prefix = bytes[offset];
    if (prefix === undefined) throw new Error('Solana transaction is truncated');
    if (prefix & VERSION_PREFIX_MASK) {
        const version = prefix & ~VERSION_PREFIX_MASK;
        if (version !== 0) throw new Error(`Unsupported Solana transaction version ${version}`);
        offset++;
    }

    // Message header: required signatures, read-only signed accounts, read-only unsigned accounts.
    const numRequiredSignatures = bytes[offset];
    if (numRequiredSignatures === undefined || bytes[offset + 2] === undefined) {
        throw new Error('Solana transaction is truncated');
    }
    if (signatureCount !== numRequiredSignatures) {
        throw new Error(
            `Solana transaction has ${signatureCount} signature slots for ${numRequiredSignatures} required signatures`
        );
    }

    const [accountCount, accountsStart] = readCompactU16(bytes, offset + 3);
    if (accountCount === 0) throw new Error('Solana transaction has no account keys');
    if (accountsStart + PUBLIC_KEY_LENGTH > bytes.length) throw new Error('Solana transaction is truncated');

    return {
        feePayer: base58.encode(bytes.subarray(accountsStart, accountsStart + PUBLIC_KEY_LENGTH)),
        numRequiredSignatures,
    };
}
