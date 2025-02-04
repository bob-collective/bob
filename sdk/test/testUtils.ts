import { defineChain } from 'viem';

export const bobMainnet = defineChain({
    id: 60808,
    name: 'BOB',
    nativeCurrency: {
        decimals: 18,
        name: 'Ether',
        symbol: 'ETH',
    },
    rpcUrls: {
        default: {
            http: ['https://rpc.gobob.xyz'],
            webSocket: ['wss://rpc.gobob.xyz'],
        },
    },
    blockExplorers: {
        default: { name: 'Explorer', url: 'https://explorer.gobob.xyz' },
    },
    contracts: {
        multicall3: {
            address: '0xcA11bde05977b3631167028862bE2a173976CA11',
            blockCreated: 5882,
        },
    },
});

export const bobTestnet = defineChain({
    id: 808813,
    name: 'BOB Testnet',
    nativeCurrency: {
        decimals: 18,
        name: 'Ether',
        symbol: 'ETH',
    },
    rpcUrls: {
        default: {
            http: ['https://bob-sepolia.rpc.gobob.xyz/'],
            webSocket: ['wss://bob-sepolia.rpc.gobob.xyz'],
        },
    },
    blockExplorers: {
        default: { name: 'Explorer', url: 'https://bob-sepolia.explorer.gobob.xyz/' },
    },
});
