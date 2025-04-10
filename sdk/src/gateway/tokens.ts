import { ChainId, Token } from './types';

// TODO: re-write to use superchain tokenlist
const bobTokens = [
    {
        name: 'tBTC v2',
        symbol: 'tBTC',
        decimals: 18,
        tokens: {
            bob: {
                address: '0xBBa2eF945D523C4e2608C9E1214C2Cc64D4fc2e2',
            },
        },
        logoURI: 'https://ethereum-optimism.github.io/data/tBTC/logo.svg',
    },
    {
        name: 'Wrapped BTC',
        symbol: 'WBTC',
        decimals: 8,
        tokens: {
            bob: {
                address: '0x03C7054BCB39f7b2e5B2c7AcB37583e32D70Cfa3',
            },
        },
        logoURI: 'https://ethereum-optimism.github.io/data/WBTC/logo.svg',
    },
    {
        name: 'Solv BTC',
        symbol: 'SolvBTC',
        decimals: 18,
        tokens: {
            bob: {
                address: '0x541FD749419CA806a8bc7da8ac23D346f2dF8B77',
            },
        },
        logoURI: 'https://raw.githubusercontent.com/bob-collective/bob/master/assets/solvBTC.svg',
    },
    {
        name: 'xSolvBTC',
        symbol: 'xSolvBTC',
        decimals: 18,
        tokens: {
            bob: {
                address: '0xCC0966D8418d412c599A6421b760a847eB169A8c',
            },
        },
        logoURI: 'https://raw.githubusercontent.com/bob-collective/bob/master/assets/xSolvBTC.svg',
    },
    {
        name: 'uniBTC',
        symbol: 'uniBTC',
        decimals: 8,
        tokens: {
            bob: {
                address: '0x236f8c0a61dA474dB21B693fB2ea7AAB0c803894',
            },
        },
        logoURI: 'https://raw.githubusercontent.com/bob-collective/bob/master/assets/uniBTC.svg',
    },
];

const bobSepoliaTokens = [
    {
        name: 'Staked mtBTC',
        symbol: 'stmtBTC',
        decimals: 18,
        tokens: {
            'bob-sepolia': {
                address: '0xc4229678b65e2D9384FDf96F2E5D512d6eeC0C77',
            },
        },
        logoURI: 'https://ethereum-optimism.github.io/data/tBTC/logo.svg',
    },
    {
        name: 'tBTC v2',
        symbol: 'tBTC',
        decimals: 18,
        tokens: {
            'bob-sepolia': {
                address: '0x6744bAbDf02DCF578EA173A9F0637771A9e1c4d0',
            },
        },
        logoURI: 'https://ethereum-optimism.github.io/data/tBTC/logo.svg',
    },
    {
        name: 'Wrapped BTC',
        symbol: 'WBTC',
        decimals: 8,
        tokens: {
            'bob-sepolia': {
                address: '0xe51e40e15e6e1496a0981f90Ca1D632545bdB519',
            },
        },
        logoURI: 'https://ethereum-optimism.github.io/data/WBTC/logo.svg',
    },
];

const shoebillTokens = [
    {
        name: 'sb tBTC v2',
        symbol: 'sbtBTC',
        decimals: 8,
        nobridge: true,
        tokens: {
            bob: {
                address: '0x2925dF9Eb2092B53B06A06353A7249aF3a8B139e',
            },
        },
        logoURI: 'https://raw.githubusercontent.com/bob-collective/bob/master/assets/shoebill.svg',
    },
    {
        name: 'sb Wrapped BTC',
        symbol: 'sbWBTC',
        decimals: 8,
        nobridge: true,
        tokens: {
            bob: {
                address: '0x5c46D274ed8AbCAe2964B63c0360ad3Ccc384dAa',
            },
        },
        logoURI: 'https://raw.githubusercontent.com/bob-collective/bob/master/assets/shoebill.svg',
    },
];

const segmentTokens = [
    {
        name: 'Segment TBTC',
        symbol: 'seTBTC',
        decimals: 8,
        nobridge: true,
        tokens: {
            bob: {
                address: '0xD30288EA9873f376016A0250433b7eA375676077',
            },
        },
        logoURI: 'https://raw.githubusercontent.com/bob-collective/bob/master/assets/segment.svg',
    },
    {
        name: 'Segment WBTC',
        symbol: 'seWBTC',
        decimals: 8,
        nobridge: true,
        tokens: {
            bob: {
                address: '0x6265C05158f672016B771D6Fb7422823ed2CbcDd',
            },
        },
        logoURI: 'https://raw.githubusercontent.com/bob-collective/bob/master/assets/segment.svg',
    },
    {
        name: 'Segment SOLVBTCBBN',
        symbol: 'seSOLVBTCBBN',
        decimals: 8,
        nobridge: true,
        tokens: {
            bob: {
                address: '0x5EF2B8fbCc8aea2A9Dbe2729F0acf33E073Fa43e',
            },
        },
        logoURI: 'https://raw.githubusercontent.com/bob-collective/bob/master/assets/segment.svg',
    },
    {
        name: 'Segment UNIBTC',
        symbol: 'seUNIBTC',
        decimals: 8,
        nobridge: true,
        tokens: {
            bob: {
                address: '0x7848F0775EebaBbF55cB74490ce6D3673E68773A',
            },
        },
        logoURI: 'https://raw.githubusercontent.com/bob-collective/bob/master/assets/segment.svg',
    },
];

const avalonTokens = [
    {
        name: 'Avalon TBTC',
        symbol: 'aBOBTBTC',
        decimals: 18,
        nobridge: true,
        tokens: {
            bob: {
                address: '0x5E007Ed35c7d89f5889eb6FD0cdCAa38059560ef',
            },
        },
        logoURI: 'https://app.avalonfinance.xyz/icons/tokens/avalon.svg',
    },
    {
        name: 'Avalon WBTC',
        symbol: 'aBOBWBTC',
        decimals: 8,
        nobridge: true,
        tokens: {
            bob: {
                address: '0xd6890176e8d912142AC489e8B5D8D93F8dE74D60',
            },
        },
        logoURI: 'https://app.avalonfinance.xyz/icons/tokens/avalon.svg',
    },
    {
        name: 'Avalon SOLVBTCBBN',
        symbol: 'aBOBSOLVBTCBBN',
        decimals: 18,
        nobridge: true,
        tokens: {
            bob: {
                address: '0x2E6500A7Add9a788753a897e4e3477f651c612eb',
            },
        },
        logoURI: 'https://app.avalonfinance.xyz/icons/tokens/avalon.svg',
    },
];

const ionicTokens = [
    {
        name: 'Ionic TBTC',
        symbol: 'iontBTC',
        decimals: 18,
        nobridge: true,
        tokens: {
            bob: {
                address: '0x68e0e4d875FDe34fc4698f40ccca0Db5b67e3693',
            },
        },
        logoURI:
            'https://doc.ionic.money/~gitbook/image?url=https://1954749119-files.gitbook.io/~/files/v0/b/gitbook-x-prod.appspot.com/o/spaces%252F4L4EczL4rCp3jyJhAxHG%252Fuploads%252FPy57T6N8vDpy1jGqxElR%252FTwitter%2520Avatar%2520green.png?alt%3Dmedia%26token%3D1808baa5-0996-41d1-ad6d-fe09c7188cdc&width=768&dpr=4&quality=100&sign=14802451&sv=2',
    },
    {
        name: 'Ionic Wrapped BTC',
        symbol: 'ionWBTC',
        decimals: 8,
        nobridge: true,
        tokens: {
            bob: {
                address: '0xEBc8a7EE7f1D6534eBF45Bd5311203BF0A17493c',
            },
        },
        logoURI:
            'https://doc.ionic.money/~gitbook/image?url=https://1954749119-files.gitbook.io/~/files/v0/b/gitbook-x-prod.appspot.com/o/spaces%252F4L4EczL4rCp3jyJhAxHG%252Fuploads%252FPy57T6N8vDpy1jGqxElR%252FTwitter%2520Avatar%2520green.png?alt%3Dmedia%26token%3D1808baa5-0996-41d1-ad6d-fe09c7188cdc&width=768&dpr=4&quality=100&sign=14802451&sv=2',
    },
];

const vedaTokens = [
    {
        name: 'Hybrid Bitcoin',
        symbol: 'HybridBTC.pendle',
        decimals: 8,
        tokens: {
            bob: {
                address: '0x9998e05030Aee3Af9AD3df35A34F5C51e1628779',
            },
        },
        logoURI: 'https://raw.githubusercontent.com/bob-collective/bob/master/assets/hybridBTC-pendle.svg',
    },
];

const TOKENS: Array<{
    name: string;
    symbol: string;
    decimals: number;
    nobridge?: boolean;
    tokens: {
        bob?: {
            address: string;
        };
        'bob-sepolia'?: {
            address: string;
        };
    };
    logoURI: string;
}> = [
    ...bobTokens,
    ...bobSepoliaTokens,
    ...shoebillTokens,
    ...segmentTokens,
    ...avalonTokens,
    ...ionicTokens,
    ...vedaTokens,
];

/** @description Tokens supported on BOB and BOB Sepolia */
export const SYMBOL_LOOKUP: { [key in number]: { [key in string]: Token } } = {};
export const ADDRESS_LOOKUP: { [key in number]: { [key in string]: Token } } = {};

SYMBOL_LOOKUP[ChainId.BOB] = {};
SYMBOL_LOOKUP[ChainId.BOB_SEPOLIA] = {};

ADDRESS_LOOKUP[ChainId.BOB] = {};
ADDRESS_LOOKUP[ChainId.BOB_SEPOLIA] = {};

function addToken(address: string, token: (typeof TOKENS)[number], chainId: ChainId) {
    const lowerAddress = address.toLowerCase();
    const lowerToken: Token = {
        chainId: ChainId.BOB,
        address: lowerAddress,
        name: token.name,
        symbol: token.symbol,
        decimals: token.decimals,
        logoURI: token.logoURI,
    };

    SYMBOL_LOOKUP[chainId][lowerToken.symbol.toLowerCase()] = lowerToken;
    ADDRESS_LOOKUP[chainId][lowerAddress] = lowerToken;
}

for (const token of TOKENS) {
    if (token.tokens.bob) {
        addToken(token.tokens.bob.address, token, ChainId.BOB);
    }

    if (token.tokens['bob-sepolia']) {
        addToken(token.tokens['bob-sepolia'].address, token, ChainId.BOB_SEPOLIA);
    }
}

export const tokenToStrategyTypeMap = new Map([
    ...bobTokens.map((token) => [token.tokens['bob'].address.toLowerCase(), 'none'] as const),
    ...bobSepoliaTokens.map((token) => [token.tokens['bob-sepolia'].address.toLowerCase(), 'none'] as const),
    ...segmentTokens.map((token) => [token.tokens['bob'].address.toLowerCase(), 'segment'] as const),
    ...ionicTokens.map((token) => [token.tokens['bob'].address.toLowerCase(), 'ionic'] as const),
    ...vedaTokens.map((token) => [token.tokens['bob'].address.toLowerCase(), 'veda'] as const),
    ...avalonTokens.map((token) => [token.tokens['bob'].address.toLowerCase(), 'avalon'] as const),
]);
