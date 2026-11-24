import * as bitcoin from 'bitcoinjs-lib';
import { formatUnits, parseUnits, Chain as ViemChain } from 'viem';
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

export function toHexScriptPubKey(userAddress: string, network: bitcoin.Network): string {
    const address = bitcoin.address.toOutputScript(userAddress, network);
    const buffer = Buffer.concat([Buffer.from([address.length]), address]);
    return '0x' + buffer.toString('hex'); // Convert buffer to hex string
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

const chainIdToChainConfigMapping = /* @__PURE__ */ Object.values(supportedChainsMapping).reduce(
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
