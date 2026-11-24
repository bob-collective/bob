import { base58, bech32, bech32m } from '@scure/base';
import { sha256 } from 'viem';

// Drop-in replacement for the `bitcoin-address-validation` package (v3.0.0), built on @scure/base,
// which the SDK already ships. It keeps that package's accept/reject behaviour and error messages,
// quirks included; each quirk is marked "Quirk" below and pinned in test/address.test.ts.
//
// One intended divergence: base58-js (its base58 decoder) reads characters outside the base58
// alphabet, other than I, O, l and 0, as the digit -1 instead of rejecting them, so the old package
// accepts crafted strings such as '3JLL1f7EoHK4z3vrmoxmJvaGzMPWYdBZg!' or a trailing space. This
// code rejects every such string; also pinned in the test.

/** Bitcoin network an address belongs to. */
export enum Network {
    mainnet = 'mainnet',
    testnet = 'testnet',
    regtest = 'regtest',
    signet = 'signet',
}

/** Bitcoin address (output script) type. */
export enum AddressType {
    p2pkh = 'p2pkh',
    p2sh = 'p2sh',
    p2wpkh = 'p2wpkh',
    p2wsh = 'p2wsh',
    p2tr = 'p2tr',
}

export type AddressInfo = {
    bech32: boolean;
    network: Network;
    address: string;
    type: AddressType;
};

export type AddressInfoOptions = {
    /** Report testnet (and regtest) addresses as this network instead; mainnet addresses throw. */
    castTestnetTo?: Network.regtest | Network.signet;
};

const base58Versions: Record<number, { type: AddressType; network: Network }> = {
    0x00: { type: AddressType.p2pkh, network: Network.mainnet },
    0x6f: { type: AddressType.p2pkh, network: Network.testnet },
    0x05: { type: AddressType.p2sh, network: Network.mainnet },
    0xc4: { type: AddressType.p2sh, network: Network.testnet },
};

const bech32Networks: Record<string, Network> = {
    bc: Network.mainnet,
    tb: Network.testnet,
    bcrt: Network.regtest,
};

const invalid = () => new Error('Invalid address');

const normalize = (info: AddressInfo, options?: AddressInfoOptions): AddressInfo => {
    const toNetwork = options?.castTestnetTo;
    if (!toNetwork) {
        return info;
    }
    if (info.network === Network.mainnet) {
        throw new Error('Cannot cast mainnet to non-mainnet');
    }
    return { ...info, network: toNetwork };
};

const parseBech32 = (address: string, options?: AddressInfoOptions): AddressInfo => {
    let prefix: string;
    let words: number[];
    try {
        // Quirk: only lower-case taproot addresses are checked as bech32m, so an upper-case `BC1P…` is
        // rejected, and witness versions 2-16 are checked as bech32 instead of bech32m (BIP350).
        const coder =
            address.startsWith('bc1p') || address.startsWith('tb1p') || address.startsWith('bcrt1p') ? bech32m : bech32;
        ({ prefix, words } = coder.decode(address as `${string}1${string}`));
    } catch {
        throw invalid();
    }

    const network = bech32Networks[prefix];
    const witnessVersion = words[0];
    if (network === undefined || witnessVersion === undefined || witnessVersion > 16) {
        throw invalid();
    }

    let data: Uint8Array;
    try {
        data = bech32.fromWords(words.slice(1));
    } catch (error) {
        // Keep the messages of the `bech32` package the old code let through: "Excess padding" and
        // "Non-zero padding" (@scure/base appends the padding value to the latter).
        const message = error instanceof Error ? error.message : '';
        throw message.startsWith('Non-zero padding') ? new Error('Non-zero padding') : error;
    }

    // Quirk: the program length is not checked against the witness version, so any 20-byte program
    // is p2wpkh (even for version 1) and any other non-taproot program is p2wsh.
    const type = data.length === 20 ? AddressType.p2wpkh : witnessVersion === 1 ? AddressType.p2tr : AddressType.p2wsh;

    return normalize({ bech32: true, network, address, type }, options);
};

/**
 * Decodes base58 the way `base58-js` does: the zero bytes for leading `1`s are counted from the
 * first line that starts with `1`, and line terminators are skipped (Quirk: so `"<address>\n"`
 * decodes like `"<address>"`).
 */
const decodeBase58 = (address: string): Uint8Array => {
    const leadingZeros = address.match(/^1+/mu)?.[0].length ?? 0;
    const digits = base58.decode(address.replace(/[\n\r\u2028\u2029]/gu, ''));
    const firstNonZero = digits.findIndex((byte) => byte !== 0);
    const value = firstNonZero === -1 ? new Uint8Array() : digits.subarray(firstNonZero);
    const bytes = new Uint8Array(leadingZeros + value.length);
    bytes.set(value, leadingZeros);
    return bytes;
};

const parseBase58 = (address: string, options?: AddressInfoOptions): AddressInfo => {
    let decoded: Uint8Array;
    try {
        decoded = decodeBase58(address);
    } catch {
        throw invalid();
    }
    if (decoded.length !== 25) {
        throw invalid();
    }

    const body = decoded.subarray(0, 21);
    const checksum = sha256(sha256(body, 'bytes'), 'bytes');
    if (decoded.subarray(21).some((byte, i) => byte !== checksum[i])) {
        throw invalid();
    }

    const version = base58Versions[decoded[0]];
    if (!version) {
        throw invalid();
    }
    return normalize({ ...version, address, bech32: false }, options);
};

/**
 * Decodes a Bitcoin address into its network and type.
 *
 * @param address The address to decode.
 * @param options Optional `castTestnetTo` to report testnet addresses as regtest or signet.
 * @returns The address, its network and type, and whether it is bech32-encoded.
 * @throws If the address is invalid, or `castTestnetTo` is set for a mainnet address.
 */
export const getAddressInfo = (address: string, options?: AddressInfoOptions): AddressInfo => {
    const prefix = address.slice(0, 2).toLowerCase();
    if (prefix === 'bc' || prefix === 'tb') {
        return parseBech32(address, options);
    }
    return parseBase58(address, options);
};

/**
 * Checks whether a string is a valid Bitcoin address.
 *
 * @param address The address to check.
 * @param network Optional network the address must belong to.
 * @param options Optional `castTestnetTo` to treat testnet addresses as regtest or signet.
 * @returns `true` if the address is valid (and on `network`, if given).
 */
export const validate = (address: string, network?: Network, options?: AddressInfoOptions): boolean => {
    try {
        const info = getAddressInfo(address, options);
        return network ? network === info.network : true;
    } catch {
        return false;
    }
};
