type Token = {
    name: string,
    bob: string,
    bob_sepolia: string
}

export const TOKENS: { [key: string]: Token } = {
    "tBTC": {
        name: "tBTC v2",
        bob: "0xBBa2eF945D523C4e2608C9E1214C2Cc64D4fc2e2",
        bob_sepolia: "0x6744bAbDf02DCF578EA173A9F0637771A9e1c4d0",
    },
    "WBTC": {
        name: "Wrapped BTC",
        bob: "0x03C7054BCB39f7b2e5B2c7AcB37583e32D70Cfa3",
        bob_sepolia: "0xe51e40e15e6e1496a0981f90Ca1D632545bdB519",
    },
    "sbtBTC": {
        name: "sb tBTC v2",
        bob: "0x2925dF9Eb2092B53B06A06353A7249aF3a8B139e",
        bob_sepolia: "",
    },
    "sbWBTC": {
        name: "sb Wrapped BTC",
        bob: "0x5c46D274ed8AbCAe2964B63c0360ad3Ccc384dAa",
        bob_sepolia: "",
    },
    "seTBTC": {
        name: "Segment TBTC",
        bob: "0xD30288EA9873f376016A0250433b7eA375676077",
        bob_sepolia: "",
    },
    "seWBTC": {
        name: "Segment WBTC",
        bob: "0x6265C05158f672016B771D6Fb7422823ed2CbcDd",
        bob_sepolia: "",
    }
}