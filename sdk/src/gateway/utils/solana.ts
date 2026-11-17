import { base58, base64 } from '@scure/base';
import { bytesToHex, type Hex } from 'viem';

const PUBLIC_KEY_LENGTH = 32;
const SIGNATURE_LENGTH = 64;
const VERSION_PREFIX_MASK = 0x80;

export function isSolanaChain(chain: string): boolean {
    return chain.toLowerCase() === 'solana';
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

/**
 * The `data` `executeQuote` passes to `walletClient.sendTransaction` for a Solana order: the wire bytes of the
 * unsigned `VersionedTransaction` Gateway built (sent as base64), as hex. Throws unless `signer` pays the
 * transaction's fee and is its only signer.
 */
export function solanaTransactionData(transaction: string, signer: string): Hex {
    let bytes: Uint8Array;
    let header: SolanaTransactionHeader;
    try {
        bytes = base64.decode(transaction);
        header = readSolanaTransactionHeader(bytes);
    } catch (error) {
        throw new Error('Gateway returned a malformed Solana transaction', { cause: error });
    }

    // Only the wallet signs. A transaction needing another signer would have to come partially signed by Gateway.
    if (header.numRequiredSignatures !== 1) {
        throw new Error(
            `Solana transaction requires ${header.numRequiredSignatures} signatures; only the wallet's is supported`
        );
    }
    if (header.feePayer !== signer) {
        throw new Error('Solana transaction fee payer is not the wallet address');
    }

    return bytesToHex(bytes);
}
