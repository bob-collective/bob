import { hex } from '@scure/base';
import { Address as BtcAddress, NETWORK, OutScript } from '@scure/btc-signer';
import { createPublicClient, formatUnits, http, parseUnits, Chain as ViemChain } from 'viem';
import {
    avalanche,
    base,
    berachain,
    bob,
    bsc,
    hyperEvm,
    mainnet,
    optimism,
    sei,
    soneium,
    sonic,
    unichain,
    arbitrum,
    robinhood,
} from 'viem/chains';

/**
 * Length-prefixed output script for `userAddress`, as a 0x-prefixed hex string.
 *
 * @param network - Address parameters of the network, e.g. `NETWORK` from `@scure/btc-signer` or a bitcoinjs-lib `networks` entry.
 */
export function toHexScriptPubKey(userAddress: string, network: typeof NETWORK): string {
    const script = OutScript.encode(BtcAddress(network).decode(userAddress));
    return '0x' + hex.encode(new Uint8Array([script.length, ...script]));
}

export function isHexPrefixed(str: string): boolean {
    return str.slice(0, 2) === '0x';
}

export function stripHexPrefix(str: string): string {
    return isHexPrefixed(str) ? str.slice(2) : str;
}

export function slugify(str: string): string {
    return str
        .toLowerCase()
        .replace(/ /g, '-')
        .replace(/[^\w-]+/g, '');
}

export function viemClient(chain: ViemChain) {
    return createPublicClient({ chain, transport: http() });
}

export function parseBtc(btc: string) {
    return parseUnits(btc, 8);
}

export function formatBtc(btc: bigint) {
    return formatUnits(btc, 8);
}

export const supportedChainsMapping = {
    bob,
    ethereum: mainnet,
    sonic,
    bsc,
    unichain,
    bera: berachain,
    sei,
    avalanche,
    base,
    soneium,
    optimism,
    arbitrum,
    hyperevm: hyperEvm,
    // `hyperliquid` is the staging gateway's name for the same chain (id 999);
    // register both aliases so the CLI resolves either until the API canonicalizes to `hyperevm`.
    hyperliquid: hyperEvm,
    robinhood,
} as const;

const chainIdToChainConfigMapping = Object.values(supportedChainsMapping).reduce(
    (acc, chain) => {
        acc[chain.id] = chain;
        return acc;
    },
    {} as Record<ViemChain['id'], ViemChain>
);

function getChainIdByName(chainName: string) {
    const needle = chainName.toLowerCase();

    // Check by mapping key first (e.g. "bsc", "bera", "optimism")
    for (const [key, chain] of Object.entries(supportedChainsMapping)) {
        if (key === needle || chain.name.toLowerCase() === needle) {
            return chain.id;
        }
    }

    throw new Error(
        `Chain id for "${chainName}" not found. Allowed values: ${Object.keys(supportedChainsMapping).join(', ')}`
    );
}

function getChainConfigById(chainId: number) {
    const config = chainIdToChainConfigMapping[chainId];
    if (!config) {
        throw new Error(
            `Chain id for "${chainId}" not found. Allowed values ${Object.values(supportedChainsMapping).map((chain) => chain.id)}`
        );
    }

    return config;
}

export function getChainConfig(fromChain: string | number) {
    if (typeof fromChain === 'string') {
        const chainId = getChainIdByName(fromChain);
        return getChainConfigById(chainId);
    }

    return getChainConfigById(fromChain);
}

// Viem chain names are used to identify chains
export function resolveChainId(chain: number): string {
    return getChainConfig(chain).name.toLowerCase();
}

export function resolveChainName(chain: number | string): string {
    if (typeof chain === 'number') {
        return resolveChainId(chain);
    }
    return chain.toLowerCase();
}

export function safeBigInt(value: string): bigint {
    try {
        return BigInt(value);
    } catch {
        throw new Error(`Invalid BigInt for value "${value}"`);
    }
}

export function safeNumber(value: string): number {
    const n = Number(value);
    if (Number.isNaN(n)) {
        throw new Error(`Invalid number for value  "${value}"`);
    }
    return n;
}
