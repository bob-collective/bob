import { createPublicClient, http, type PublicClient, type Chain, erc20Abi, Address } from 'viem';
import { tokenToStrategyTypeMap } from './tokens';
import type { StrategyAssetState, Token } from './types';
import { aaveV2AtokenAbi, compoundV2CTokenAbi } from './abi';

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
