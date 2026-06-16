import { base58 } from '@scure/base';
import { Account, Address, bytesToHex, Hex, isAddress, isAddressEqual, sha256, zeroAddress } from 'viem';

export function isTronChain(chain: string) {
    return chain.toLowerCase() === 'tron';
}

export function decodeTronBase58Address(address: string): Hex {
    if (address.startsWith('0x')) {
        return address as Hex;
    }

    const decoded = base58.decode(address);
    const payload = decoded.slice(0, -4);
    const checksum = decoded.slice(-4);
    const expectedChecksum = sha256(sha256(payload, 'bytes'), 'bytes').slice(0, 4);

    if (
        decoded.length !== 25 ||
        payload.length !== 21 ||
        payload[0] !== 0x41 ||
        !checksum.every((byte, index) => byte === expectedChecksum[index])
    ) {
        throw new Error(`Invalid Tron address: ${address}`);
    }

    return bytesToHex(payload);
}

const TRON_ZERO_ADDRESS = '0x410000000000000000000000000000000000000000';

export function toPublicClientAddress(address: string, isTron: boolean): Address {
    return (isTron ? decodeTronBase58Address(address) : address) as Address;
}

export function toPublicClientAccount(account: Account, isTron: boolean): Account {
    if (!isTron) {
        return account;
    }

    return { ...account, address: toPublicClientAddress(account.address, true) };
}

export function isNativeTokenAddress(address: string, isTron: boolean) {
    if (address === zeroAddress) {
        return true;
    }

    if (isTron) {
        return decodeTronBase58Address(address).toLowerCase() === TRON_ZERO_ADDRESS;
    }

    return isAddress(address) && isAddressEqual(address, zeroAddress);
}
