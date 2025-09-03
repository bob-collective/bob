import { createPublicClient, http, type PublicClient, type Chain, erc20Abi, Address, zeroAddress } from 'viem';
import { ADDRESS_LOOKUP, tokenToStrategyTypeMap } from './tokens';
import {
    type DefiLlamaPool,
    type EnrichedToken,
    type PointsIncentive,
    type StrategyAssetState,
    type Token,
} from './types';
import { aaveV2AtokenAbi, compoundV2CTokenAbi } from './abi';
import { bob, optimism, mainnet } from 'viem/chains';

const projectPointsIncentives: Map<string, PointsIncentive[]> = new Map([
    [
        'veda',
        [
            {
                id: 'veda',
                name: 'Veda Points',
            },
        ],
    ],
    [
        'bob',
        [
            {
                id: 'bob',
                name: 'Bob Spice',
            },
        ],
    ],
    [
        'segment',
        [
            {
                id: 'bob',
                name: 'Bob Spice',
            },
            {
                id: 'segment',
                name: 'Segment Finance Points',
            },
        ],
    ],
    [
        'ionic',
        [
            {
                id: 'bob',
                name: 'Bob Spice',
            },
        ],
    ],
    [
        'avalon',
        [
            {
                id: 'bob',
                name: 'Bob Spice',
            },
            {
                id: 'avalon',
                name: 'Avalon Points',
            },
        ],
    ],
]);

const berockPoints = {
    id: 'bedrock',
    name: 'Bedrock Diamonds',
};
const babylonPoints = {
    id: 'babylon',
    name: 'Babylon Points',
};
const solvPoints = {
    id: 'solv',
    name: 'Solv XP',
};
const opRewards = {
    id: 'op',
    name: 'OP',
};

const tokensPointsIncentives: Map<string, PointsIncentive[]> = new Map([
    [
        // uniBTC
        '0x236f8c0a61da474db21b693fb2ea7aab0c803894',
        [berockPoints, babylonPoints],
    ],
    [
        // Solv (xSolvBTC)
        '0xCC0966D8418d412c599A6421b760a847eB169A8c',
        [solvPoints, babylonPoints],
    ],
    [
        // Segment xSolvBTC
        '0x5ef2b8fbcc8aea2a9dbe2729f0acf33e073fa43e',
        [solvPoints, babylonPoints],
    ],
    [
        // Segment uniBTC
        '0x7848f0775eebabbf55cb74490ce6d3673e68773a',
        [berockPoints, babylonPoints],
    ],
    [
        // Avalon xSolvBTC
        '0x2e6500a7add9a788753a897e4e3477f651c612eb',
        [solvPoints, babylonPoints],
    ],
    [
        // hybridBTC
        '0x9998e05030aee3af9ad3df35a34f5c51e1628779',
        [opRewards],
    ],
]);

const tokenToDefiLlamaPoolIdMap = new Map<string, string>([
    [
        // hybridBTC
        '0x9998e05030aee3af9ad3df35a34f5c51e1628779',
        'e8bfea35-ff6d-48db-aa08-51599b363219',
    ],
    [
        // Segment xSolvBTC
        '0x5ef2b8fbcc8aea2a9dbe2729f0acf33e073fa43e',
        'a430e355-9a6d-4435-90e5-7e74c10f523c',
    ],
    [
        // Segment wBTC
        '0x6265c05158f672016b771d6fb7422823ed2cbcdd',
        '56eed6bb-80ac-42e3-a7fb-f93c0438c72b',
    ],
    [
        // Segment uniBTC
        '0x7848f0775eebabbf55cb74490ce6d3673e68773a',
        'a944439e-73ca-4de7-a48d-fe1e75e0b1f2',
    ],
    [
        // Segment tBTC
        '0xd30288ea9873f376016a0250433b7ea375676077',
        'ecf4821d-b550-4d5f-b92f-b12d9c279271',
    ],
    [
        // Avalon xSolvBTC
        '0x2e6500a7add9a788753a897e4e3477f651c612eb',
        '19c9b477-6ce9-4e59-897e-1b3ef76afa3a',
    ],
    [
        // Avalon WBTC
        '0xd6890176e8d912142ac489e8b5d8d93f8de74d60',
        '8eb8b8bc-718c-41cf-9cd5-1a1c3c6045c1',
    ],
    // Not enough liquidity to be listest on DefiLlama
    // [
    //     // Avalon tBTC
    //     '0x5e007ed35c7d89f5889eb6fd0cdcaa38059560ef',
    //     ''
    // ],
]);

const tokenToSolvStrategyMap = new Map<string, string>([
    [
        // Solv BTC+
        '0x4ca70811e831db42072cba1f0d03496ef126faad',
        'BTC+',
    ],
    [
        // SolvBTC.JUP
        '0x6b062aa7f5fc52b530cb13967ae2e6bc0d8dd3e4',
        'Jupiter',
    ],
]);

export default class StrategyClient {
    private viemClient: PublicClient;

    constructor(chain: Chain, rpcUrl?: string) {
        this.viemClient = createPublicClient({
            transport: http(),
            chain: {
                ...chain,
                rpcUrls: {
                    default: {
                        http: [rpcUrl || chain.rpcUrls.default.http[0]],
                    },
                },
            },
        }) as PublicClient;
    }

    async getTokensIncentives(
        tokens: string[]
    ): Promise<Pick<EnrichedToken, 'apyBase' | 'apyReward' | 'rewardTokens' | 'points'>[]> {
        const [defillamaPoolMap, solvAPYs] = await Promise.all([this.getDefillamaPools(), this.getSolvAPYs()]);

        return tokens.map((token) => {
            const tokenAddress = token.toLowerCase();

            const strategyType = tokenToStrategyTypeMap.get(tokenAddress) ?? 'bob';
            const points = [
                ...(projectPointsIncentives.get(strategyType) ?? []),
                ...(tokensPointsIncentives.get(tokenAddress) ?? []),
            ];

            const defillamaPoolId = tokenToDefiLlamaPoolIdMap.get(tokenAddress) || '';
            const defillamaPool = defillamaPoolMap.get(defillamaPoolId);

            if (defillamaPool) {
                return {
                    // HACK: set HybridBTC APY to 2%
                    apyBase:
                        defillamaPoolId === 'e8bfea35-ff6d-48db-aa08-51599b363219' ? 2 : (defillamaPool?.apyBase ?? 0),
                    apyReward: defillamaPool?.apyReward ?? 0,
                    rewardTokens: this.resolveTokens(defillamaPool?.rewardTokens),
                    points,
                };
            }

            const solvStrategy = tokenToSolvStrategyMap.get(tokenAddress);

            if (solvStrategy) {
                return {
                    apyBase: solvAPYs[solvStrategy].apyBase,
                    apyReward: solvAPYs[solvStrategy].apyReward,
                    rewardTokens: this.resolveTokens(solvAPYs[solvStrategy].rewardTokens),
                    points,
                };
            }

            return {
                apyBase: 0,
                apyReward: 0,
                rewardTokens: [],
                points,
            };
        });
    }

    private async getDefillamaPools() {
        const res = await fetch('https://yields.llama.fi/pools');

        const defillamaPools: DefiLlamaPool[] = res.ok ? (await res.json()).data : [];
        const defillamaPoolMap = new Map<string, DefiLlamaPool>(
            defillamaPools.filter((pool) => pool.chain === 'Bob').map((pool) => [pool.pool, pool])
        );
        return defillamaPoolMap;
    }

    private resolveTokens(tokens: string[] | undefined | null): Token[] {
        if (!tokens) {
            return [];
        }

        return tokens
            .map(
                (addr) =>
                    ADDRESS_LOOKUP[bob.id][addr.toLowerCase()] ||
                    ADDRESS_LOOKUP[optimism.id][addr.toLowerCase()] ||
                    ADDRESS_LOOKUP[mainnet.id][addr.toLowerCase()]
            )
            .filter(Boolean);
    }

    async getStrategyAssetState(token: Token): Promise<StrategyAssetState> {
        const strategyType = tokenToStrategyTypeMap.get(token.address.toLowerCase()) ?? 'bob';

        switch (strategyType) {
            case 'bob': {
                return this.getTokenAssetState(token);
            }

            case 'segment': {
                return this.getCompoundV2StrategyAssetState(token);
            }

            case 'ionic': {
                return this.getCompoundV2StrategyAssetState(token);
            }

            case 'veda': {
                return this.getVedaStrategyAssetState(token);
            }

            case 'avalon': {
                return this.getAAVEV2StrategyAssetState(token);
            }

            default: {
                return {
                    address: 'usd',
                    totalUnderlying: 0n,
                };
            }
        }
    }

    private async getTokenAssetState(token: Token): Promise<StrategyAssetState> {
        const outputTokenAddress = token.address as Address;

        const [totalSupply] = await this.viemClient.multicall({
            allowFailure: false,
            contracts: [
                {
                    address: outputTokenAddress,
                    abi: erc20Abi,
                    functionName: 'totalSupply',
                    args: [],
                },
            ],
        });

        return {
            address: outputTokenAddress,
            totalUnderlying: totalSupply!,
        };
    }

    private async getCompoundV2StrategyAssetState(token: Token): Promise<StrategyAssetState> {
        const outputTokenAddress = token.address as Address;

        const [totalSupply, exchangeRate, underlyingAddress] = await this.viemClient.multicall({
            allowFailure: false,
            contracts: [
                {
                    address: outputTokenAddress,
                    abi: erc20Abi,
                    functionName: 'totalSupply',
                    args: [],
                },
                {
                    address: outputTokenAddress,
                    abi: compoundV2CTokenAbi,
                    functionName: 'exchangeRateCurrent',
                    args: [],
                },
                {
                    address: outputTokenAddress,
                    abi: compoundV2CTokenAbi,
                    functionName: 'underlying',
                    args: [],
                },
            ],
        });

        return {
            address: underlyingAddress ?? zeroAddress,
            totalUnderlying: exchangeRate && totalSupply ? (exchangeRate * totalSupply) / 10n ** 18n : 0n,
        };
    }

    private async getAAVEV2StrategyAssetState(token: Token): Promise<StrategyAssetState> {
        const outputTokenAddress = token.address as Address;

        const [totalSupply, underlyingAddress] = await this.viemClient.multicall({
            allowFailure: false,
            contracts: [
                {
                    address: outputTokenAddress,
                    abi: erc20Abi,
                    functionName: 'totalSupply',
                    args: [],
                },
                {
                    address: outputTokenAddress,
                    abi: aaveV2AtokenAbi,
                    functionName: 'UNDERLYING_ASSET_ADDRESS',
                    args: [],
                },
            ],
        });

        return {
            address: underlyingAddress!,
            totalUnderlying: totalSupply!,
        };
    }

    private async getVedaStrategyAssetState(token: Token): Promise<StrategyAssetState> {
        // NOTE: remove 1 week so we are sure to pick up at least one historical data point
        const timestamp = Math.floor(Date.now() / 1000) - 7 * 24 * 3600;

        const res = await fetch(`https://api.sevenseas.capital/hourlyData/bob/${token.address}/${timestamp}/latest`);

        if (!res.ok) {
            return {
                address: 'usd',
                totalUnderlying: 0n,
            };
        }

        const body = await res.json();

        if (!body.Response?.[0]) {
            return {
                address: 'usd',
                totalUnderlying: 0n,
            };
        }

        const { tvl } = body.Response[0];

        return {
            address: 'usd',
            totalUnderlying: BigInt(Math.floor(tvl)),
        };
    }

    private async getSolvAPYs() {
        try {
            const query = `
                query SolvAPYs {
                    btcPlusStats(stageNo: 1) {
                        baseApy
                        rewardApy
                        tvl
                    }
                    lsts {
                        details {
                            protocol
                            apy
                            estApy
                            tvlUsd
                        }
                    }
                }
            `
                .split('\n')
                .join(' ');

            const res = await fetch('https://sft-api.com/graphql', {
                headers: {
                    'content-type': 'application/json',
                    Authorization: 'solv',
                },
                body: `{
                    "operationName": "SolvAPYs",
                    "variables": {},
                    "query": "${query}"
                }`,
                method: 'POST',
            });

            const data = await res.json();

            const apys = {
                'BTC+': {
                    apyBase: Number(data.data.btcPlusStats.baseApy),
                    apyReward: Number(data.data.btcPlusStats.rewardApy),
                    rewardTokens: ['0x04830a96a23ea718faa695a5aae74695aae3a23f'],
                },
            };

            data.data.lsts.details.forEach((pool) => {
                apys[pool.protocol] = {
                    apyBase: Number(pool.apy),
                    apyReward: 0,
                };
            });

            return apys;
        } catch (err) {
            console.error('Failed to fetch APY data from Solv', err);
            return {};
        }
    }
}
