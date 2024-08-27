export type Token = {
    name: string,
    symbol: string,
    bob: string,
    bobSepolia: string
}

const TOKENS: { [key: string]: Token } = {
    "tBTC": {
        name: "tBTC v2",
        symbol: "tBTC",
        bob: "0xBBa2eF945D523C4e2608C9E1214C2Cc64D4fc2e2",
        bobSepolia: "0x6744bAbDf02DCF578EA173A9F0637771A9e1c4d0",
    },
    "WBTC": {
        name: "Wrapped BTC",
        symbol: "WBTC",
        bob: "0x03C7054BCB39f7b2e5B2c7AcB37583e32D70Cfa3",
        bobSepolia: "0xe51e40e15e6e1496a0981f90Ca1D632545bdB519",
    },
    "sbtBTC": {
        name: "sb tBTC v2",
        symbol: "sbtBTC",
        bob: "0x2925dF9Eb2092B53B06A06353A7249aF3a8B139e",
        bobSepolia: "",
    },
    "sbWBTC": {
        name: "sb Wrapped BTC",
        symbol: "sbWBTC",
        bob: "0x5c46D274ed8AbCAe2964B63c0360ad3Ccc384dAa",
        bobSepolia: "",
    },
    "seTBTC": {
        name: "Segment TBTC",
        symbol: "seTBTC",
        bob: "0xD30288EA9873f376016A0250433b7eA375676077",
        bobSepolia: "",
    },
    "seWBTC": {
        name: "Segment WBTC",
        symbol: "seWBTC",
        bob: "0x6265C05158f672016B771D6Fb7422823ed2CbcDd",
        bobSepolia: "",
    },
    "stmtBTC": {
        name: "Staked mtBTC",
        symbol: "stmtBTC",
        bob: "",
        bobSepolia: "0xc4229678b65e2D9384FDf96F2E5D512d6eeC0C77",
    }
};

/** @description Tokens supported on BOB and BOB Sepolia */
export const SYMBOL_LOOKUP: { [key: string]: Token } = {};
export const ADDRESS_LOOKUP: { [address: string]: Token } = {};

for (const key in TOKENS) {
    const token = TOKENS[key];

    const lowerBob = token.bob.toLowerCase();
    const lowerBobSepolia = token.bobSepolia.toLowerCase();

    const lowercasedToken: Token = {
        name: token.name,
        symbol: token.symbol,
        bob: lowerBob,
        bobSepolia: lowerBobSepolia,
    };

    SYMBOL_LOOKUP[key.toLowerCase()] = lowercasedToken;
    ADDRESS_LOOKUP[lowerBob] = lowercasedToken;
    ADDRESS_LOOKUP[lowerBobSepolia] = lowercasedToken;
}