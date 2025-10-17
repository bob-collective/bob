/** @dev Internal */
export interface DefiLlamaPool {
    pool: string;
    chain: string;
    project: string;
    tvlUsd: number;
    apy: number | null;
    apyBase: number | null;
    apyReward: number | null;
    underlyingTokens: null | string[];
    rewardTokens: null | string[];
}
