import { createPublicClient, http, type PublicClient, type Chain, erc20Abi, Address } from 'viem';
import { tokenToStrategyTypeMap } from './tokens';
import type { EnrichedToken, PointsIncentive, StrategyAssetState, Token } from './types';
import { aaveV2AtokenAbi, compoundV2CTokenAbi } from './abi';

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
        'none',
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

const tokensPointsIncentives: Map<string, PointsIncentive[]> = new Map([
    [
        '0x236f8c0a61da474db21b693fb2ea7aab0c803894',
        [
            {
                id: 'bedrock',
                name: 'Bedrock Diamonds',
            },
            {
                id: 'babylon',
                name: 'Babylon Points',
            },
        ],
    ],
    [
        // Solv (xSolvBTC)
        '0x0bef2a8b771e37763c1ce02a88f404c6b2573843',
        [
            {
                id: 'solv',
                name: 'Solv XP',
            },
            {
                id: 'babylon',
                name: 'Babylon Points',
            },
        ],
    ],
    [
        // Segment xSolvBTC
        '0x5ef2b8fbcc8aea2a9dbe2729f0acf33e073fa43e',
        [
            {
                id: 'solv',
                name: 'Solv XP',
            },
            {
                id: 'babylon',
                name: 'Babylon Points',
            },
        ],
    ],
    [
        // Segment uniBTC
        '0x7848f0775eebabbf55cb74490ce6d3673e68773a',
        [
            {
                id: 'bedrock',
                name: 'Bedrock Diamonds',
            },
            {
                id: 'babylon',
                name: 'Babylon Points',
            },
        ],
    ],
    [
        // Avalon xSolvBTC
        '0x2e6500a7add9a788753a897e4e3477f651c612eb',
        [
            {
                id: 'solv',
                name: 'Solv XP',
            },
            {
                id: 'babylon',
                name: 'Babylon Points',
            },
        ],
    ],
    [
        // hybridBTC
        '0x9998e05030aee3af9ad3df35a34f5c51e1628779',
        [
            {
                id: 'op',
                name: 'OP',
            },
        ],
    ],
]);

const rewardTokens = new Map<string, Token[]>([
    [
        // hybridBTC
        '0x9998e05030aee3af9ad3df35a34f5c51e1628779',
        [
            {
                chainId: 10,
                address: '0x4200000000000000000000000000000000000042',
                name: 'Optimism',
                symbol: 'OP',
                decimals: 18,
                logoURI: 'https://ethereum-optimism.github.io/data/OP/logo.png',
            },
        ],
    ],
]);

export default class StrategyClient {
    private viemClient: PublicClient;

    constructor(chain: Chain, rpcUrl?: string) {
        // @ts-expect-error: prevent TS2589
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
    ): Promise<Pick<EnrichedToken, 'baseApy' | 'rewardApy' | 'rewardTokens' | 'points'>[]> {
        return tokens.map((token) => {
            const tokenAddress = token.toLowerCase();

            const strategyType = tokenToStrategyTypeMap.get(tokenAddress) ?? 'none';

            return {
                baseApy: 0,
                rewardApy: 0,
                rewardTokens: rewardTokens.get(tokenAddress) ?? [],
                points: [
                    ...(projectPointsIncentives.get(strategyType) ?? []),
                    ...(tokensPointsIncentives.get(tokenAddress) ?? []),
                ],
            };
        });
    }

    async getStrategyAssetState(token: Token): Promise<StrategyAssetState> {
        const strategyType = tokenToStrategyTypeMap.get(token.address.toLowerCase()) ?? 'none';

        switch (strategyType) {
            case 'none': {
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

        const [{ result: totalSupply }] = await this.viemClient.multicall({
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
            totalUnderlying: totalSupply,
        };
    }

    private async getCompoundV2StrategyAssetState(token: Token): Promise<StrategyAssetState> {
        const outputTokenAddress = token.address as Address;

        const [{ result: totalSupply }, { result: exchangeRate }, { result: underlyingAddress }] =
            await this.viemClient.multicall({
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
            address: underlyingAddress,
            totalUnderlying: (exchangeRate * totalSupply) / 10n ** 18n,
        };
    }

    private async getAAVEV2StrategyAssetState(token: Token): Promise<StrategyAssetState> {
        const outputTokenAddress = token.address as Address;

        const [{ result: totalSupply }, { result: underlyingAddress }] = await this.viemClient.multicall({
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
            address: underlyingAddress,
            totalUnderlying: totalSupply,
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
}
