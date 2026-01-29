import * as bitcoin from 'bitcoinjs-lib';
import { Address, createPublicClient, formatUnits, Hex, http, parseUnits, Chain as ViemChain } from 'viem';
import {
    avalanche,
    base,
    berachain,
    bob,
    bsc,
    mainnet,
    optimism,
    sei,
    soneium,
    sonic,
    unichain,
    arbitrum,
} from 'viem/chains';

export function toHexScriptPubKey(userAddress: string, network: bitcoin.Network): string {
    const address = bitcoin.address.toOutputScript(userAddress, network);
    const buffer = Buffer.concat([Buffer.from([address.length]), address]);
    return '0x' + buffer.toString('hex'); // Convert buffer to hex string
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
} as const;

const chainIdToChainConfigMapping = Object.values(supportedChainsMapping).reduce(
    (acc, chain) => {
        acc[chain.id] = chain;
        return acc;
    },
    {} as Record<ViemChain['id'], ViemChain>
);

function getChainIdByName(chainName: string) {
    const chain = Object.values(supportedChainsMapping).find(
        (chain) => chain.name.toLowerCase() === chainName.toLowerCase()
    );

    if (!chain) {
        throw new Error(
            `Chain id for "${chainName}" not found. Allowed values ${Object.values(supportedChainsMapping).map((chain) => chain.name)}`
        );
    }
    return chain.id;
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
