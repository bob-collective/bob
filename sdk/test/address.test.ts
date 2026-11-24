import * as legacy from 'bitcoin-address-validation';
import { describe, expect, it } from 'vitest';
import { AddressInfoOptions, AddressType, getAddressInfo, Network, validate } from '../src/wallet/address';

type Case = {
    name: string;
    address: string;
    options?: AddressInfoOptions;
    /** Expected `[network, type, bech32]`, or `null` when the address is rejected. */
    expected: [Network, AddressType, boolean] | null;
};

const { mainnet, testnet, regtest, signet } = Network;
const { p2pkh, p2sh, p2wpkh, p2wsh, p2tr } = AddressType;

// One key (03b366c6…abfe1) encoded as every address type on every network.
const VALID: Case[] = [
    { name: 'mainnet p2pkh', address: '1LFYz3pJ8uQK865Ed72wGUBHpYbgVJSvjv', expected: [mainnet, p2pkh, false] },
    { name: 'mainnet p2sh', address: '3DFVKuT9Ft4rWpysAZ1bHpg55EBy1HVPcr', expected: [mainnet, p2sh, false] },
    {
        name: 'mainnet p2wpkh',
        address: 'bc1q6v4wjumj7ze9f8fxlfqwx0cxn887sm24c5eqp3',
        expected: [mainnet, p2wpkh, true],
    },
    {
        name: 'mainnet p2wsh',
        address: 'bc1qlll5ckgy86v053vg30x3utme90q8hj3a3hfqtfu9ttzwp4yvkwwqul6e70',
        expected: [mainnet, p2wsh, true],
    },
    {
        name: 'mainnet p2tr',
        address: 'bc1pnu9gqu78szpmmmnw8kdcz6j2nsmcunrewwhx4kemtd7ffgt3523q2eenay',
        expected: [mainnet, p2tr, true],
    },
    { name: 'testnet p2pkh', address: 'mzmWH6uGwvqZuCYrLg1K6PPcgYCPTvztCt', expected: [testnet, p2pkh, false] },
    { name: 'testnet p2sh', address: '2N4ohPePAsLaCiccQqgdTumfLHaQ8ngHacV', expected: [testnet, p2sh, false] },
    {
        name: 'testnet p2wpkh',
        address: 'tb1q6v4wjumj7ze9f8fxlfqwx0cxn887sm24jjzn6z',
        expected: [testnet, p2wpkh, true],
    },
    {
        name: 'testnet p2wsh',
        address: 'tb1qlll5ckgy86v053vg30x3utme90q8hj3a3hfqtfu9ttzwp4yvkwwqthvkyq',
        expected: [testnet, p2wsh, true],
    },
    {
        name: 'testnet p2tr',
        address: 'tb1pnu9gqu78szpmmmnw8kdcz6j2nsmcunrewwhx4kemtd7ffgt3523qa30u8t',
        expected: [testnet, p2tr, true],
    },
    {
        name: 'regtest p2wpkh',
        address: 'bcrt1q6v4wjumj7ze9f8fxlfqwx0cxn887sm24smm7dt',
        expected: [regtest, p2wpkh, true],
    },
    {
        name: 'regtest p2wsh',
        address: 'bcrt1qlll5ckgy86v053vg30x3utme90q8hj3a3hfqtfu9ttzwp4yvkwwqxwxs36',
        expected: [regtest, p2wsh, true],
    },
    {
        name: 'regtest p2tr',
        address: 'bcrt1pnu9gqu78szpmmmnw8kdcz6j2nsmcunrewwhx4kemtd7ffgt3523qsg96j3',
        expected: [regtest, p2tr, true],
    },
    {
        name: 'upper-case mainnet p2wpkh',
        address: 'BC1Q6V4WJUMJ7ZE9F8FXLFQWX0CXN887SM24C5EQP3',
        expected: [mainnet, p2wpkh, true],
    },
];

// Base58 regtest addresses share testnet's version bytes, and signet shares testnet's encoding.
const CAST: Case[] = [
    {
        name: 'regtest p2pkh (testnet bytes) cast to regtest',
        address: 'mzmWH6uGwvqZuCYrLg1K6PPcgYCPTvztCt',
        options: { castTestnetTo: regtest },
        expected: [regtest, p2pkh, false],
    },
    {
        name: 'regtest p2sh (testnet bytes) cast to regtest',
        address: '2N4ohPePAsLaCiccQqgdTumfLHaQ8ngHacV',
        options: { castTestnetTo: regtest },
        expected: [regtest, p2sh, false],
    },
    {
        name: 'signet p2wpkh',
        address: 'tb1q6v4wjumj7ze9f8fxlfqwx0cxn887sm24jjzn6z',
        options: { castTestnetTo: signet },
        expected: [signet, p2wpkh, true],
    },
    {
        name: 'signet p2tr',
        address: 'tb1pnu9gqu78szpmmmnw8kdcz6j2nsmcunrewwhx4kemtd7ffgt3523qa30u8t',
        options: { castTestnetTo: signet },
        expected: [signet, p2tr, true],
    },
    {
        name: 'signet p2pkh',
        address: 'mzmWH6uGwvqZuCYrLg1K6PPcgYCPTvztCt',
        options: { castTestnetTo: signet },
        expected: [signet, p2pkh, false],
    },
    {
        name: 'regtest bech32 cast to signet',
        address: 'bcrt1q6v4wjumj7ze9f8fxlfqwx0cxn887sm24smm7dt',
        options: { castTestnetTo: signet },
        expected: [signet, p2wpkh, true],
    },
    {
        name: 'mainnet p2pkh cast to signet',
        address: '1LFYz3pJ8uQK865Ed72wGUBHpYbgVJSvjv',
        options: { castTestnetTo: signet },
        expected: null,
    },
    {
        name: 'mainnet p2tr cast to regtest',
        address: 'bc1pnu9gqu78szpmmmnw8kdcz6j2nsmcunrewwhx4kemtd7ffgt3523q2eenay',
        options: { castTestnetTo: regtest },
        expected: null,
    },
];

const INVALID: Case[] = [
    { name: 'empty', address: '', expected: null },
    { name: 'garbage', address: 'not an address', expected: null },
    { name: 'EVM address', address: '0x675Ca18A04027fd50C88CcD03939E0e5C97b795f', expected: null },
    { name: 'base58 bad checksum', address: '1LFYz3pJ8uQK865Ed72wGUBHpYbgVJSvjw', expected: null },
    { name: 'base58 truncated', address: '1LFYz3pJ8uQK865Ed72wGUBHpYbgVJSvj', expected: null },
    { name: 'base58 forbidden character 0', address: '1LFYz3pJ8uQK865Ed72wGUBHpYbgVJSvj0', expected: null },
    { name: 'base58 trailing space', address: '1LFYz3pJ8uQK865Ed72wGUBHpYbgVJSvjv ', expected: null },
    { name: 'base58 unknown version (litecoin)', address: 'LUxXFcwXFPpRZdMv4aYu6bDwPdC2skrsxd', expected: null },
    { name: 'bech32 bad checksum', address: 'bc1q6v4wjumj7ze9f8fxlfqwx0cxn887sm24c5eqp4', expected: null },
    { name: 'bech32 mixed case', address: 'bc1Q6v4wjumj7ze9f8fxlfqwx0cxn887sm24c5eqp3', expected: null },
    {
        name: 'bech32 testnet checksum on mainnet',
        address: 'bc1q6v4wjumj7ze9f8fxlfqwx0cxn887sm24jjzn6z',
        expected: null,
    },
    { name: 'bech32 unknown prefix', address: 'ltc1qqyqszqgpqyqszqgpqyqszqgpqyqszqgpq4933p', expected: null },
    {
        name: 'taproot with bech32 checksum',
        address: 'bc1pqvpsxqcrqvpsxqcrqvpsxqcrqvpsxqcrqvpsxqcrqvpsxqcrqvpse2hjr3',
        expected: null,
    },
    {
        name: 'v0 with bech32m checksum',
        address: 'bc1qqvpsxqcrqvpsxqcrqvpsxqcrqvpsxqcrqvpsxqcrqvpsxqcrqvpsnahmmd',
        expected: null,
    },
    {
        name: 'witness version 17',
        address: 'bc13qvpsxqcrqvpsxqcrqvpsxqcrqvpsxqcrqvpsxqcrqvpsxqcrqvps97dska',
        expected: null,
    },
    { name: 'bech32 excess padding', address: 'bc1qqyqszqgpqyqszqgpqyqszqgpqyqszqgppmrr6zf', expected: null },
    { name: 'bech32 too short', address: 'bc1qq', expected: null },
    { name: 'bech32 over 90 characters', address: `bc1q${'q'.repeat(90)}`, expected: null },
];

// Behaviour of bitcoin-address-validation@3.0.0 that is arguably wrong, kept for parity.
const QUIRKS: Case[] = [
    {
        name: 'upper-case taproot is rejected',
        address: 'BC1PNU9GQU78SZPMMMNW8KDCZ6J2NSMCUNREWWHX4KEMTD7FFGT3523Q2EENAY',
        expected: null,
    },
    {
        name: 'witness v2 with a bech32 (not bech32m) checksum is accepted as p2wsh',
        address: 'bc1zqvpsxqcrqvpsxqcrqvpsxqcrqvpsxqcrqvpsxqcrqvpsxqcrqvps3hwad6',
        expected: [mainnet, p2wsh, true],
    },
    {
        name: 'witness v2 with a bech32m checksum is rejected',
        address: 'bc1zqvpsxqcrqvpsxqcrqvpsxqcrqvpsxqcrqvpsxqcrqvpsxqcrqvpsyt73gc',
        expected: null,
    },
    {
        name: 'witness v1 with a 20-byte program is p2wpkh',
        address: 'bc1pqyqszqgpqyqszqgpqyqszqgpqyqszqgp6tcjpc',
        expected: [mainnet, p2wpkh, true],
    },
    {
        name: 'witness v0 with a 25-byte program is p2wsh',
        address: 'bc1qqgpqyqszqgpqyqszqgpqyqszqgpqyqszqgpqyqszn4rkqh',
        expected: [mainnet, p2wsh, true],
    },
    { name: 'witness v0 with an empty program is p2wsh', address: 'bc1q9zpgru', expected: [mainnet, p2wsh, true] },
    { name: 'witness v16 with a 2-byte program is p2wsh', address: 'tb1sqqqqk4seeg', expected: [testnet, p2wsh, true] },
    {
        name: 'a trailing newline in a base58 address is ignored',
        address: '1LFYz3pJ8uQK865Ed72wGUBHpYbgVJSvjv\n',
        expected: [mainnet, p2pkh, false],
    },
    {
        name: 'line breaks inside a base58 address are ignored',
        address: '3DFVKuT9Ft4r\r\nWpysAZ1bHpg55EBy1HVPcr\u2028',
        expected: [mainnet, p2sh, false],
    },
    {
        name: 'leading 1s are counted on the first line only',
        address: '1\n1LFYz3pJ8uQK865Ed72wGUBHpYbgVJSvjv',
        expected: [mainnet, p2pkh, false],
    },
];

const CASES = [...VALID, ...CAST, ...INVALID, ...QUIRKS];

describe('getAddressInfo', () => {
    it.each(CASES)('$name', ({ address, options, expected }) => {
        if (expected === null) {
            expect(() => getAddressInfo(address, options)).toThrow();
            return;
        }
        const [network, type, bech32] = expected;
        expect(getAddressInfo(address, options)).toEqual({ network, type, bech32, address });
    });

    it('names the reason for a mainnet address cast to another network', () => {
        expect(() => getAddressInfo('1LFYz3pJ8uQK865Ed72wGUBHpYbgVJSvjv', { castTestnetTo: signet })).toThrow(
            'Cannot cast mainnet to non-mainnet'
        );
    });

    it('throws "Invalid address" for an invalid address', () => {
        expect(() => getAddressInfo('not an address')).toThrow('Invalid address');
    });
});

describe('validate', () => {
    it.each(CASES)('$name', ({ address, options, expected }) => {
        expect(validate(address, undefined, options)).toBe(expected !== null);
        for (const network of Object.values(Network)) {
            expect(validate(address, network, options)).toBe(expected?.[0] === network);
        }
    });
});

describe('parity with bitcoin-address-validation@3.0.0', () => {
    const outcome = (fn: () => unknown) => {
        try {
            return { ok: fn() };
        } catch {
            return { threw: true };
        }
    };

    it.each(CASES)('$name', ({ address, options }) => {
        expect(outcome(() => getAddressInfo(address, options))).toEqual(
            outcome(() => legacy.getAddressInfo(address, options as never))
        );
        for (const network of [undefined, ...Object.values(Network)]) {
            expect(validate(address, network, options)).toBe(
                legacy.validate(address, network as legacy.Network | undefined, options as never)
            );
        }
    });
});
