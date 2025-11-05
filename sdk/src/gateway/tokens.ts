import { Token } from './types';
import { Address, isAddress } from 'viem';
import { bob, bobSepolia, mainnet, optimism } from 'viem/chains';
import tokenList from '../../../token-list.json';

// Storage slot mapping for tokens that need specific slot information
const STORAGE_SLOTS_MAP: { [address: string]: { allowanceSlot: bigint; balanceSlot: bigint } } = {
    '0x0555e30da8f98308edb960aa94c0db47230d2b9c': {
        // WBTC (OFT)
        allowanceSlot: 6n,
        balanceSlot: 5n,
    },
    '0x2260fac5e5542a773aa44fbcfedf7c193bc2c599': {
        // WBTC (Ethereum)
        allowanceSlot: 2n,
        balanceSlot: 0n,
    },
    '0xadce1ab74c8e64c155953a8bde37cbb06cf7086d': {
        // BTC (Bob Sepolia)
        allowanceSlot: 1n,
        balanceSlot: 0n,
    },
};

const bobTokens = tokenList.tokens
    .filter((token) => token.chainId === 60808)
    .filter((token) => token.address !== '0x03C7054BCB39f7b2e5B2c7AcB37583e32D70Cfa3') // Exclude the old WBTC
    .map((token) => {
        const address = token.address.toLowerCase();
        const slots = STORAGE_SLOTS_MAP[address];

        return {
            name: token.name,
            address: token.address,
            symbol: token.symbol,
            decimals: token.decimals,
            chainId: token.chainId,
            logoURI: token.logoURI,
            ...(slots
                ? {
                      allowanceSlot: slots.allowanceSlot,
                      balanceSlot: slots.balanceSlot,
                  }
                : {}),
        };
    });

const bobSepoliaTokens = [
    {
        name: 'BTC',
        address: '0xAdCE1AB74C8e64c155953A8BdE37cBB06Cf7086D',
        symbol: 'BTC',
        decimals: 18,
        chainId: 808813,
        logoURI: 'https://raw.githubusercontent.com/bob-collective/bob/master/assets/btc.svg',
        allowanceSlot: 1n,
        balanceSlot: 0n,
    },
    {
        name: 'Staked mtBTC',
        address: '0xc4229678b65e2D9384FDf96F2E5D512d6eeC0C77',
        symbol: 'stmtBTC',
        decimals: 18,
        chainId: 808813,
        logoURI: 'https://raw.githubusercontent.com/bob-collective/bob/master/assets/tbtc.svg',
    },
    {
        name: 'tBTC v2',
        address: '0x6744bAbDf02DCF578EA173A9F0637771A9e1c4d0',
        symbol: 'tBTC',
        decimals: 18,
        chainId: 808813,
        logoURI: 'https://raw.githubusercontent.com/bob-collective/bob/master/assets/tbtc.svg',
    },
    {
        name: 'Wrapped BTC',
        address: '0xe51e40e15e6e1496a0981f90Ca1D632545bdB519',
        symbol: 'WBTC',
        decimals: 8,
        chainId: 808813,
        logoURI: 'https://raw.githubusercontent.com/bob-collective/bob/master/assets/wbtc.svg',
    },
    {
        name: 'Bob BTC',
        address: '0x4496ebE7C8666a8103713EE6e0c08cA0cD25b888',
        symbol: 'bobBTC',
        decimals: 8,
        chainId: 808813,
        logoURI: 'https://raw.githubusercontent.com/bob-collective/bob/master/assets/bobBTC.svg',
    },
];

const optimismTokens = [
    {
        name: 'Optimism',
        address: '0x4200000000000000000000000000000000000042',
        symbol: 'OP',
        decimals: 18,
        chainId: 10,
        logoURI: 'https://optimistic.etherscan.io/token/images/optimism_32.png',
    },
];

const ethereumTokens = [
    {
        name: 'Solv',
        address: '0x04830a96a23ea718faa695a5aae74695aae3a23f',
        symbol: 'SOLV',
        decimals: 18,
        chainId: 1,
        logoURI: 'https://raw.githubusercontent.com/bob-collective/bob/master/assets/SOLV.svg',
    },
    {
        name: 'Wrapped BTC',
        address: '0x2260fac5e5542a773aa44fbcfedf7c193bc2c599',
        symbol: 'WBTC',
        decimals: 8,
        chainId: 1,
        logoURI: 'https://raw.githubusercontent.com/bob-collective/bob/master/assets/wbtc.svg',
        allowanceSlot: 2n,
        balanceSlot: 0n,
    },
];

const TOKENS: Array<{
    name: string;
    address: string;
    symbol: string;
    decimals: number;
    chainId: number;
    logoURI: string;
    allowanceSlot?: bigint; // optional
    balanceSlot?: bigint; // optional
}> = [...bobTokens, ...bobSepoliaTokens, ...optimismTokens, ...ethereumTokens];

/** @description Tokens supported on BOB and BOB Sepolia */
export const SYMBOL_LOOKUP: { [key in number]: { [key in string]: Token } } = {};
export const ADDRESS_LOOKUP: { [key in number]: { [key in string]: Token } } = {};

SYMBOL_LOOKUP[bob.id] = {};
SYMBOL_LOOKUP[bobSepolia.id] = {};
SYMBOL_LOOKUP[optimism.id] = {};
SYMBOL_LOOKUP[mainnet.id] = {};

ADDRESS_LOOKUP[bob.id] = {};
ADDRESS_LOOKUP[bobSepolia.id] = {};
ADDRESS_LOOKUP[optimism.id] = {};
ADDRESS_LOOKUP[mainnet.id] = {};

function addToken(address: string, token: (typeof TOKENS)[number]) {
    const lowerAddress = address.toLowerCase();
    const lowerToken: Token = {
        chainId: token.chainId,
        address: lowerAddress as Address,
        name: token.name,
        symbol: token.symbol,
        decimals: token.decimals,
        logoURI: token.logoURI,
    };

    SYMBOL_LOOKUP[token.chainId][lowerToken.symbol.toLowerCase()] = lowerToken;
    ADDRESS_LOOKUP[token.chainId][lowerAddress] = lowerToken;
}

for (const token of TOKENS) {
    addToken(token.address, token);
}

export const tokenToStrategyTypeMap = new Map([
    ...bobTokens.map((token) => [token.address.toLowerCase(), 'bob'] as const),
    ...bobSepoliaTokens.map((token) => [token.address.toLowerCase(), 'bob'] as const),
    ...[
        '0xD30288EA9873f376016A0250433b7eA375676077',
        '0x6265C05158f672016B771D6Fb7422823ed2CbcDd',
        '0x5EF2B8fbCc8aea2A9Dbe2729F0acf33E073Fa43e',
        '0x7848F0775EebaBbF55cB74490ce6D3673E68773A',
    ].map((address) => [address.toLowerCase(), 'segment'] as const),
    ...['0x68e0e4d875FDe34fc4698f40ccca0Db5b67e3693', '0xEBc8a7EE7f1D6534eBF45Bd5311203BF0A17493c'].map(
        (address) => [address.toLowerCase(), 'ionic'] as const
    ),
    ...['0x9998e05030Aee3Af9AD3df35A34F5C51e1628779'].map((address) => [address.toLowerCase(), 'veda'] as const),
    ...[
        '0x5E007Ed35c7d89f5889eb6FD0cdCAa38059560ef',
        '0xd6890176e8d912142AC489e8B5D8D93F8dE74D60',
        '0x2E6500A7Add9a788753a897e4e3477f651c612eb',
    ].map((address) => [address.toLowerCase(), 'avalon'] as const),
]);

export function getTokenAddress(chainId: number, token: string): Address {
    if (isAddress(token)) {
        return token;
    } else if (SYMBOL_LOOKUP[chainId][token]) {
        return SYMBOL_LOOKUP[chainId][token].address;
    } else {
        throw new Error('Unknown output token');
    }
}
export function getTokenSlots(tokenAddress: Address): { allowanceSlot: bigint; balanceSlot: bigint } {
    const lowerAddress = tokenAddress.toLowerCase();

    // Look up the token in the master TOKENS array
    const slots = STORAGE_SLOTS_MAP[lowerAddress];
    if (!slots) {
        throw new Error(`Slots not found for address ${tokenAddress}`);
    }

    // Check if slots are defined
    if (slots.allowanceSlot === undefined || slots.balanceSlot === undefined) {
        throw new Error(`Slots not defined at address ${tokenAddress}`);
    }

    return slots;
}
