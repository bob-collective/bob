import { Address } from 'viem';

/**
 * Designed to be compatible with the Superchain token list.
 * https://github.com/ethereum-optimism/ethereum-optimism.github.io
 */
export interface Token {
    chainId: number;
    address: Address;
    name: string;
    symbol: string;
    decimals: number;
    logoURI: string;
}

export interface PointsIncentive {
    id: string;
    name: string;
}

export interface EnrichedToken extends Token {
    tvl: number;
    apyBase: number;
    apyReward: number;
    rewardTokens: Token[];
    points: PointsIncentive[];
}

export type GatewayTokensInfo = {
    /** @description The base token (e.g. wBTC or tBTC) */
    baseToken: Token;
    /** @description The output token (e.g. uniBTC or xSolvBTC) */
    outputToken?: Token;
};
