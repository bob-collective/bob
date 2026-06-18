import { createBase58check } from '@scure/base';
import { bytesToHex, Hex, sha256 } from 'viem';

const TRON_ADDRESS_BYTE_LENGTH = 21;
const TRON_ADDRESS_PREFIX = 0x41;

const tronBase58Check = createBase58check((data) => sha256(data, 'bytes'));

function decodeTronAddress(address: string): Uint8Array | null {
    try {
        const decoded = tronBase58Check.decode(address.trim());

        return decoded.length === TRON_ADDRESS_BYTE_LENGTH && decoded[0] === TRON_ADDRESS_PREFIX ? decoded : null;
    } catch {
        return null;
    }
}

export function isValidTronAddress(address: string): boolean {
    return decodeTronAddress(address) !== null;
}

export function tronAddressToHex(address: string): Hex | null {
    const decoded = decodeTronAddress(address);
    if (!decoded) return null;
    return bytesToHex(decoded.slice(1));
}
