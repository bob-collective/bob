import {
    Account,
    Address,
    erc20Abi,
    Hex,
    maxUint256,
    PublicClient,
    Transport,
    Chain as ViemChain,
    WalletClient,
} from 'viem';
import { bob, bobSepolia } from 'viem/chains';
import { bigIntToFloatingNumber } from '../utils';
import { strategyCaller } from './abi';
import { BaseClient } from './base-client';
import StrategyClient from './strategy';
import { BitcoinSigner, EnrichedToken, GetQuoteParams, StrategyParams } from './types';

import {
    Configuration,
    DefaultApi,
    GatewayOrderInfo,
    GatewayQuote,
    instanceOfGatewayCreateOrderOneOf,
    instanceOfGatewayQuoteOneOf,
    instanceOfGatewayQuoteOneOf1,
    instanceOfGatewayQuoteOneOf2,
    instanceOfRegisterTxOneOf,
    RouteInfo,
} from './generated-client';
import { getTokenDetails } from './tokens';

export const WBTC_OFT_ADDRESS = '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c';

/**
 * Base url for the mainnet Gateway API.
 * @default "https://gateway-api-mainnet.gobob.xyz"
 */
export const MAINNET_GATEWAY_BASE_URL = 'https://gateway-api-staging.gobob.xyz';

/**
 * Base url for the Signet Gateway API.
 * @default "https://gateway-api-testnet.gobob.xyz"
 */
export const SIGNET_GATEWAY_BASE_URL = 'https://gateway-api-signet.gobob.xyz';

interface EvmWalletClientParams {
    /**
     * The wallet client used to interact with the EVM chain.
     */
    walletClient: WalletClient<Transport, ViemChain, Account>;
    /**
     * The public client used to read data from the EVM chain.
     */
    publicClient: PublicClient<Transport>;
}

export interface AllWalletClientParams extends EvmWalletClientParams {
    /**
     * Bitcoin signer used to sign the transaction inputs.
     */
    btcSigner?: BitcoinSigner;
}

/**
 * Gateway REST HTTP API client.
 *
 * Provides methods for:
 * - Getting quotes for onramp (Bitcoin to BOB) and offramp (BOB to Bitcoin) transactions
 * - Creating and managing onramp and offramp orders
 * - Executing strategies for token swaps and staking
 * - Managing token information and balances
 *
 * Supports both mainnet (BOB) and testnet (BOB Sepolia) networks.
 *
 * @example
 * ```typescript
 * const client = new GatewayApiClient(bob.id);
 * const quote = await client.getQuote({
 *   fromChain: 'bitcoin',
 *   toChain: 'bob',
 *   toToken: 'WBTC',
 *   amount: 100000000, // 1 BTC in satoshis
 *   toUserAddress: '0x...'
 * });
 * ```
 */
export class GatewayApiClient extends BaseClient {
    private chain: ViemChain;
    private strategy: StrategyClient;
    // private isSignet: boolean = false;

    api: DefaultApi;

    /**
     * Creates a new Gateway API client instance.
     *
     * @param chainId The chain ID to connect to. Supported values:
     *   - `60808` for BOB mainnet
     *   - `808813` for BOB Sepolia testnet
     * @param options Optional configuration
     * @param options.rpcUrl Custom RPC URL for the chain (optional)
     *
     * @throws {Error} If an unsupported chainId is provided
     *
     * @example
     * ```typescript
     * // Mainnet client
     * const mainnetClient = new GatewayApiClient(60808);
     *
     * // Testnet client with custom RPC
     * const testnetClient = new GatewayApiClient(808813, {
     *   rpcUrl: 'https://my-custom-rpc.com'
     * });
     * ```
     */
    // TODO: remove constructor, set the config from `getQuote`
    constructor(chainId: number, options?: { rpcUrl?: string }) {
        super();
        switch (chainId) {
            case bob.id:
                this.chain = bob;
                this.strategy = new StrategyClient(bob, options?.rpcUrl);
                this.api = new DefaultApi(
                    new Configuration({
                        basePath: MAINNET_GATEWAY_BASE_URL,
                    })
                ); // TODO: set URL
                break;
            case bobSepolia.id:
                this.chain = bobSepolia;
                this.strategy = new StrategyClient(bobSepolia, options?.rpcUrl);
                // this.isSignet = true;
                this.api = new DefaultApi(
                    new Configuration({
                        basePath: SIGNET_GATEWAY_BASE_URL,
                    })
                ); // TODO: set URL
                break;
            default:
                throw new Error('Invalid chain');
        }
    }

    private get chainId(): number {
        return this.chain.id;
    }

    /**
     * Fetches a quote for token swaps between Bitcoin and BOB.
     *
     * Automatically determines whether to create an onramp or offramp quote based on the
     * fromChain and toChain parameters:
     * - If fromChain is 'bitcoin', creates an onramp quote (Bitcoin → BOB tokens)
     * - If toChain is 'bitcoin', creates an offramp quote (BOB tokens → Bitcoin)
     *
     * @param params Quote parameters - see {@link GetQuoteParams}
     * @returns Promise resolving to quote details with either onrampQuote or offrampQuote populated
     * @throws {Error} If neither onramp nor offramp conditions are met
     */
    async getQuote(params: GetQuoteParams, initOverrides?: RequestInit): Promise<GatewayQuote> {
        return this.api.getQuote(
            {
                srcChain: params.fromChain.toString(), // TODO: don't use number
                dstChain: params.toChain.toString(), // TODO: don't use number
                sender: params.fromUserAddress?.toString() || '',
                recipient: params.toUserAddress.toString(),
                srcToken: params.fromToken.toString(),
                dstToken: params.toToken.toString(),
                amount: params.amount.toString(),
                slippage: params.maxSlippage?.toString() || '0', // TODO
                gasRefill: params.gasRefill?.toString(),
            },
            initOverrides
        );
    }

    /**
     * Executes a complete quote (onramp or offramp) in a single transaction.
     *
     * For onramp: creates order, signs Bitcoin transaction, and finalizes
     * For offramp: approves tokens if needed and creates on-chain order
     *
     * @param params Parameters including quote and wallet clients - see {@link GatewayQuote} & {@link AllWalletClientParams}
     * @returns Promise resolving to transaction hash
     * @throws {Error} If required signers are missing or transaction fails
     */
    async executeQuote(
        { quote, walletClient, publicClient, btcSigner }: { quote: GatewayQuote } & AllWalletClientParams,
        initOverrides?: RequestInit
    ): Promise<string> {
        if (instanceOfGatewayQuoteOneOf(quote)) {
            if (!btcSigner) {
                throw new Error(`btcSigner is required for onramp order`);
            }

            const order = await this.api.createOrder({ gatewayQuote: { onramp :quote.onramp }});

            if (!instanceOfGatewayCreateOrderOneOf(order)) {
                throw new Error('Invalid order type returned from API');
            }

            let bitcoinTxHex: string;
            if (btcSigner.sendBitcoin) {
                // TODO: add this
                // bitcoinTxHex = await btcSigner.sendBitcoin({
                //     from: params.fromUserAddress!,
                //     to: bitcoinAddress,
                //     value: formatBtc(BigInt(satoshis)),
                //     opReturn: opReturnHash,
                //     isSignet: this.isSignet,
                // });
                bitcoinTxHex = '';
            } else if (btcSigner.signAllInputs) {
                bitcoinTxHex = await btcSigner.signAllInputs(order.onramp.psbt);
            } else {
                throw new Error('btcSigner must implement either sendBitcoin or signAllInputs method');
            }

            // bitcoinTxOrId = stripHexPrefix(bitcoinTxOrId);
            if (!bitcoinTxHex) throw new Error('Failed to get signed transaction');

            const tx = await this.api.registerTx(
                { registerTx: { onramp: {
                    orderId: order.onramp.orderId,
                     bitcoinTx: bitcoinTxHex
                }} },
                initOverrides
            );

            if (typeof tx === 'string') {
                return tx;
            }

            if (!instanceOfRegisterTxOneOf(tx)) {
                throw new Error('Invalid registerTx response type');
            }

            return tx.onramp.txid;
        } else if (instanceOfGatewayQuoteOneOf1(quote)) {
            if (!walletClient.account) {
                throw new Error(`walletClient is required for offramp order`);
            }
            // const accountAddress = walletClient.account?.address ?? (params.fromUserAddress as Address);
            const accountAddress = walletClient.account.address;
            const tokenAddress = WBTC_OFT_ADDRESS; // TODO: get from API

            const spenderAddress = quote.offramp.tx.to as Address;

            // if (
            //     getAddress(quote.data.amountInMax.address) ===
            //     getAddress('0xdAC17F958D2ee523a2206206994597C13D831ec7')
            // ) {
            //     // USDT has a non-standard approve function that doesn't return bool
            //     // First reset allowance to 0, then set to desired amount
            //     const usdtAbi = [
            //         {
            //             constant: false,
            //             inputs: [
            //                 { name: '_spender', type: 'address' },
            //                 { name: '_value', type: 'uint256' },
            //             ],
            //             name: 'approve',
            //             outputs: [],
            //             type: 'function',
            //         },
            //     ] as const;

            //     // Reset to 0 first
            //     const { request: resetRequest } = await publicClient.simulateContract({
            //         account: walletClient.account,
            //         address: quote.data.amountInMax.address,
            //         abi: usdtAbi,
            //         functionName: 'approve',
            //         args: [quote.data.tx.to, 0n],
            //     });
            //     const resetTxHash = await walletClient.writeContract(resetRequest);
            //     await publicClient.waitForTransactionReceipt({ hash: resetTxHash });

            //     // Set to max
            //     const { request } = await publicClient.simulateContract({
            //         account: walletClient.account,
            //         address: quote.data.amountInMax.address,
            //         abi: usdtAbi,
            //         functionName: 'approve',
            //         args: [quote.data.tx.to, maxUint256],
            //     });

            //     const txHash = await walletClient.writeContract(request);
            //     await publicClient.waitForTransactionReceipt({ hash: txHash });
            // }

            // Check ETH balance and estimate gas for both potential transactions
            const [allowance, decimals] = await publicClient.multicall({
                allowFailure: false,
                contracts: [
                    {
                        address: tokenAddress,
                        abi: erc20Abi,
                        functionName: 'allowance',
                        args: [accountAddress, spenderAddress],
                    },
                    {
                        address: tokenAddress,
                        abi: erc20Abi,
                        functionName: 'decimals',
                    },
                ],
            });

            if (decimals < 8) {
                throw new Error('Tokens with less than 8 decimals are not supported');
            }
            const multiplier = 10n ** BigInt(decimals - 8);
            const requiredAmount = BigInt(quote.offramp.inputAmount.amount) * multiplier;
            const needsApproval = requiredAmount > allowance;

            if (needsApproval) {
                const { request } = await publicClient.simulateContract({
                    account: walletClient.account,
                    address: tokenAddress,
                    abi: erc20Abi,
                    functionName: 'approve',
                    args: [spenderAddress, maxUint256],
                });

                const approveTxHash = await walletClient.writeContract(request);

                await publicClient.waitForTransactionReceipt({ hash: approveTxHash });
            }

            const transactionHash = await walletClient.sendTransaction({
                account: walletClient.account,
                data: quote.offramp.tx.data as Hex,
                to: spenderAddress,
                value: BigInt(quote.offramp.tx.value || 0),
            });

            await publicClient?.waitForTransactionReceipt({ hash: transactionHash });

            return transactionHash;
        } else if (instanceOfGatewayQuoteOneOf2(quote)) {
            // TODO: implement
        }

        throw new Error('Invalid quote type');
    }

    /**
     * Executes a strategy directly for token swaps (e.g. WBTC → xSolvBTC).
     *
     * @deprecated Will be replaced by Multicall strategies in the future
     * @param params Strategy parameters and wallet clients - see {@link StrategyParams} & {@link EvmWalletClientParams}
     * @returns Promise resolving to transaction hash
     */
    async executeStrategy({ walletClient, publicClient, ...params }: StrategyParams & EvmWalletClientParams) {
        const allowance = await publicClient.readContract({
            address: params.token,
            abi: erc20Abi,
            functionName: 'allowance',
            args: [walletClient.account.address, params.strategyAddress],
        });

        if (BigInt(params.amount) > allowance) {
            const { request } = await publicClient.simulateContract({
                account: walletClient.account,
                address: params.token,
                abi: erc20Abi,
                functionName: 'approve',
                args: [params.strategyAddress, maxUint256],
            });

            const approveTxHash = await walletClient.writeContract(request);

            await publicClient.waitForTransactionReceipt({ hash: approveTxHash });
        }

        const { request } = await publicClient.simulateContract({
            address: params.strategyAddress,
            abi: strategyCaller,
            functionName: 'handleGatewayMessageWithSlippageArgs', // TODO: encode args
            args: [params.token, params.amount, params.receiver, { amountOutMin: params.amountOutMin }],
            account: walletClient.account,
        });

        const transactionHash = await walletClient.writeContract(request);

        await publicClient?.waitForTransactionReceipt({ hash: transactionHash });

        return transactionHash;
    }

    // TODO: should get price from the gateway API
    private async getPrices(initOverrides?: RequestInit): Promise<Map<string, number>> {
        const response = await this.safeFetch(
            'https://fusion-api.gobob.xyz/pricefeed',
            initOverrides,
            'Failed to fetch prices from Fusion API'
        );

        if (!response.ok) {
            console.error('Failed to fetch prices from Fusion API');
            return new Map();
        }

        const list = await response.json();

        return new Map(list.map((x) => [x.token_address.toLowerCase(), Number(x.price)]));
    }

    /**
     * Retrieves all supported tokens with enriched data including TVL and incentives.
     *
     * @returns Promise resolving to array of enriched token data
     */
    async getEnrichedTokens(initOverrides?: RequestInit): Promise<EnrichedToken[]> {
        const [routes, prices] = await Promise.all([this.getRoutes(initOverrides), this.getPrices(initOverrides)]);

        const tokensSet = routes.reduce((acc, route) => {
            if (route.srcChain === 'bob') acc.add(route.srcToken as Address);
            if (route.dstChain === 'bob') acc.add(route.dstToken as Address);
            return acc;
        }, new Set<Address>());
        const tokens = Array.from(tokensSet);

        const tokensIncentives = await this.strategy.getTokensIncentives(tokens);

        return Promise.all(
            tokens.map(async (address, i) => {
                const token = getTokenDetails(this.chainId, address);
                if (!token) {
                    throw new Error(`Token not found: ${address} on chain ${this.chainId}`);
                }
                const tokenIncentives = tokensIncentives[i];

                const { address: underlyingAddress, totalUnderlying } =
                    await this.strategy.getStrategyAssetState(token);

                if (underlyingAddress === 'usd') {
                    return {
                        ...token,
                        ...tokenIncentives,
                        tvl: Number(totalUnderlying),
                    };
                }

                const underlyingToken = getTokenDetails(this.chainId, underlyingAddress);

                if (!underlyingToken) {
                    return {
                        ...token,
                        ...tokenIncentives,
                        tvl: 0,
                    };
                }

                return {
                    ...token,
                    ...tokenIncentives,
                    tvl:
                        bigIntToFloatingNumber(totalUnderlying, underlyingToken.decimals) *
                        (prices.get(underlyingAddress.toLowerCase()) ?? 0),
                };
            })
        );
    }

    /**
     * Retrieves all orders (onramp and offramp) for a specific user address.
     *
     * @param userAddress The user's EVM address
     * @returns Promise resolving to array of typed orders
     */
    async getOrders(userAddress: Address, initOverrides?: RequestInit): Promise<Array<GatewayOrderInfo>> {
        return this.api.getOrders({ userAddress: userAddress.toString() }, initOverrides);
    }

    /**
     * Retrieves all supported routes.
     *
     * @returns Promise resolving to array of supported routes
     */
    async getRoutes(initOverrides?: RequestInit): Promise<Array<RouteInfo>> {
        return this.api.getRoutes(initOverrides);
    }
}
