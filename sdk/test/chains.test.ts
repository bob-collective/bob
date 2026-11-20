import { describe, expect, it } from 'vitest';
import { assertAllowanceHolderSpender } from '../src/gateway/allowance-holder';
import { ETHEREUM_USDT_ADDRESS } from '../src/gateway/client';
import { getChainConfig, supportedChainsMapping, supportsGasEstimation } from '../src/gateway/utils';

// Pre-refactor values of the chain tables, copied verbatim from master before #1244 item 5.
// Every derived table must keep producing exactly these values.
const ALLOWANCE_HOLDER_0X_CANONICAL = '0x0000000000001fF3684f28c67538d4D072C22734';
const ALLOWANCE_HOLDER_SELF_DEPLOYED = '0x8fd545b348e84deb145f0179a00c671f0b9519c3';
const ALLOWANCE_HOLDER_TRON = '0x07a39ae4c49dee86e892450b20881f32cd5d500d';
const ALLOWANCE_HOLDER_TRON_BASE58 = 'TAfbit1ENsRmtZbPQfYU3srURpfYuWYS7K';

const OLD_ALLOWANCE_HOLDER_BY_CHAIN: Record<string, string> = {
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
    hyperliquid: ALLOWANCE_HOLDER_0X_CANONICAL,
    robinhood: ALLOWANCE_HOLDER_0X_CANONICAL,
};

/** Mapping key, viem chain id and viem chain name, in the original key order. */
const OLD_SUPPORTED_CHAINS: [string, number, string][] = [
    ['bob', 60808, 'BOB'],
    ['ethereum', 1, 'Ethereum'],
    ['sonic', 146, 'Sonic'],
    ['bsc', 56, 'BNB Smart Chain'],
    ['unichain', 130, 'Unichain'],
    ['bera', 80094, 'Berachain'],
    ['sei', 1329, 'Sei Network'],
    ['avalanche', 43114, 'Avalanche'],
    ['base', 8453, 'Base'],
    ['soneium', 1868, 'Soneium Mainnet'],
    ['optimism', 10, 'OP Mainnet'],
    ['arbitrum', 42161, 'Arbitrum One'],
    ['hyperevm', 999, 'HyperEVM'],
    ['hyperliquid', 999, 'HyperEVM'],
    ['robinhood', 4663, 'Robinhood Chain'],
];

const OLD_CHAINS_WITHOUT_GAS_ESTIMATION = ['tron'];

const OLD_ETHEREUM_USDT_ADDRESS = '0xdAC17F958D2ee523a2206206994597C13D831ec7';

/** Every name any table mentions, plus one no table knows. */
const ALL_NAMES = [
    ...new Set([
        ...Object.keys(OLD_ALLOWANCE_HOLDER_BY_CHAIN),
        ...OLD_SUPPORTED_CHAINS.map(([key]) => key),
        ...OLD_CHAINS_WITHOUT_GAS_ESTIMATION,
        'bitcoin',
        'unknown-chain',
    ]),
];

describe('chain tables keep their pre-refactor values', () => {
    it('supportedChainsMapping has the same keys, order and viem chains', () => {
        expect(Object.entries(supportedChainsMapping).map(([key, chain]) => [key, chain.id, chain.name])).toStrictEqual(
            OLD_SUPPORTED_CHAINS
        );
    });

    it('getChainConfig resolves every key, viem name and id as before', () => {
        for (const [key, id, name] of OLD_SUPPORTED_CHAINS) {
            expect(getChainConfig(key).id).toBe(id);
            expect(getChainConfig(key.toUpperCase()).id).toBe(id);
            expect(getChainConfig(name).id).toBe(id);
            expect(getChainConfig(id).id).toBe(id);
        }
        for (const name of ['tron', 'plasma', 'polygon', 'bitcoin']) {
            expect(() => getChainConfig(name)).toThrow(
                `Chain id for "${name}" not found. Allowed values: ${OLD_SUPPORTED_CHAINS.map(([key]) => key).join(', ')}`
            );
        }
    });

    it('assertAllowanceHolderSpender accepts exactly the old spender per chain', () => {
        const other = '0x1111111111111111111111111111111111111111';
        for (const name of ALL_NAMES) {
            const expected = OLD_ALLOWANCE_HOLDER_BY_CHAIN[name];
            for (const chain of [name, name.toUpperCase()]) {
                if (expected) {
                    expect(() => assertAllowanceHolderSpender(chain, expected, 'order')).not.toThrow();
                    expect(() => assertAllowanceHolderSpender(chain, other, 'order')).toThrow(
                        `expected the AllowanceHolder at ${expected}`
                    );
                } else {
                    expect(() => assertAllowanceHolderSpender(chain, other, 'order')).toThrow(
                        `no known AllowanceHolder for chain "${chain}"`
                    );
                }
            }
        }
        expect(() => assertAllowanceHolderSpender('tron', ALLOWANCE_HOLDER_TRON_BASE58, 'order')).not.toThrow();
    });

    it('supportsGasEstimation is false only for the old set', () => {
        for (const name of ALL_NAMES) {
            const expected = !OLD_CHAINS_WITHOUT_GAS_ESTIMATION.includes(name);
            expect(supportsGasEstimation(name)).toBe(expected);
            expect(supportsGasEstimation(name.toUpperCase())).toBe(expected);
        }
    });

    it('ETHEREUM_USDT_ADDRESS is unchanged', () => {
        expect(ETHEREUM_USDT_ADDRESS).toBe(OLD_ETHEREUM_USDT_ADDRESS);
    });
});
