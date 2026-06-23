import { createBase58check } from '@scure/base';
import { bytesToHex, getAddress, hexToBytes, isAddress, isHex, sha256, type Address, type Hex } from 'viem';
import { privateKeyToAccount } from 'viem/accounts';

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

export function hexToTronAddress(hexAddress: Address | string): string {
  const ethAddr = getAddress(hexAddress);
  const payload = new Uint8Array(TRON_ADDRESS_BYTE_LENGTH);
  payload[0] = TRON_ADDRESS_PREFIX;
  payload.set(hexToBytes(ethAddr), 1);
  return tronBase58Check.encode(payload);
}

/** Normalize a hex private key (with or without 0x, trimmed). */
export function normalizePrivateKey(key: string): Hex {
  const trimmed = key.trim();
  if (!isHex(trimmed) && !/^[0-9a-fA-F]{64}$/.test(trimmed)) {
    throw new Error('Tron private key must be a 64-character hex string (optionally 0x-prefixed), as exported from TronLink.');
  }
  return (trimmed.startsWith('0x') ? trimmed : `0x${trimmed}`) as Hex;
}

export function deriveTronAddress(privateKey: string): string {
  const account = privateKeyToAccount(normalizePrivateKey(privateKey));
  return hexToTronAddress(account.address);
}

/** Map chain + token address to a token-index key (base58 addresses are case-sensitive). */
export function tokenAddressKey(chain: string, address: string): string {
  if (chain === 'tron') return `${chain}:${address}`;
  return `${chain}:${address.toLowerCase()}`;
}

/** Convert a contract address from hex or base58 to base58 for Tron RPC. */
export function toTronContractAddress(address: string): string {
  if (isValidTronAddress(address)) return address;
  if (isAddress(address, { strict: false })) return hexToTronAddress(address);
  throw new Error(`Invalid Tron contract address: ${address}`);
}
