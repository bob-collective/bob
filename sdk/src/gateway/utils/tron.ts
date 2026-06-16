import { base58 } from '@scure/base';
import { TronWeb } from 'tronweb';
import { Account, Address, bytesToHex, getAddress, Hex, isAddress, isAddressEqual, sha256, zeroAddress } from 'viem';

export function isTronChain(chain: string) {
    return chain.toLowerCase() === 'tron';
}

export function decodeTronBase58Address(address: string): Hex {
  if (address.startsWith('T') && TronWeb.isAddress(address)) {
    const tronHex = TronWeb.address.toHex(address);

    return `0x${tronHex.slice(2)}`;
  }

  return getAddress(address);
}

const TRON_ZERO_ADDRESS = '0x410000000000000000000000000000000000000000';

export function toPublicClientAddress(address: string, isTron: boolean): Address {
    return (isTron ? decodeTronBase58Address(address) : getAddress(address)) as Address;
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
