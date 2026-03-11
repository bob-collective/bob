import {
    Account,
    Address,
    ContractFunctionExecutionError,
    erc20Abi,
    Hex,
    isAddressEqual,
    maxUint256,
    PublicClient,
    Transport,
    Chain as ViemChain,
    WalletClient,
    zeroAddress,
} from 'viem';
import { strategyCaller, USDTApproveAbi } from './abi';
import { BitcoinSigner, GetQuoteParams, StrategyParams } from './types';
import {
    Configuration,
    V1Api,
    GatewayOrderInfo,
    GatewayQuote,
    instanceOfGatewayCreateOrderOneOf,
    instanceOfGatewayCreateOrderOneOf1,
    instanceOfGatewayQuoteOneOf,
    instanceOfGatewayQuoteOneOf1,
    instanceOfGatewayQuoteOneOf2,
    instanceOfRegisterTxOneOf,
    RouteInfo,
    instanceOfGatewayCreateOrderOneOf2,
    GatewayMaxSpendable,
} from './generated-client';
import { formatBtc } from './utils';

export const WBTC_OFT_ADDRESS = '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c';
export const ETHEREUM_USDT_ADDRESS = '0xdAC17F958D2ee523a2206206994597C13D831ec7';

/**
 * Base url for the mainnet Gateway API.
 * @default "https://gateway-api-mainnet.gobob.xyz"
 */
export const MAINNET_GATEWAY_BASE_URL = 'https://gateway-api-mainnet.gobob.xyz';

/** Default slippage tolerance in basis points (300 = 3%). */
export const DEFAULT_MAX_SLIPPAGE_BPS = '300';

/**
 * Base url for the staging Gateway API.
 * @default "https://gateway-api-staging.gobob.xyz"
 */
export const STAGING_GATEWAY_BASE_URL = 'https://gateway-api-staging.gobob.xyz';

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
 * const client = new GatewayApiClient();
 * const quote = await client.getQuote({
 *   fromChain: 'bitcoin',
 *   toChain: 'bob',
 *   toToken: 'WBTC',
 *   amount: 100000000, // 1 BTC in satoshis
 *   toUserAddress: '0x...'
 * });
 * ```
 */
export class GatewayApiClient {
    api: V1Api;

    /**
     * Creates a new Gateway API client instance.
     *
     * @example
     * ```typescript
     * // Mainnet client
     * const mainnetClient = new GatewayApiClient();
     * ```
     */
    constructor(basePath?: string) {
        this.api = new V1Api(
            new Configuration({
                basePath: basePath || MAINNET_GATEWAY_BASE_URL,
                middleware: [
                    {
                        async post(context) {
                            if (context.response && (context.response.status < 200 || context.response.status >= 300)) {
                                let body: Record<string, string>;
                                try {
                                    body = await context.response.json();
                                } catch {
                                    throw new Error(context.response.statusText);
                                }

                                throw new Error(body.error || body.message || JSON.stringify(body));
                            }

                            return context.response;
                        },
                    },
                ],
            })
        );
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
                // eslint-disable-next-line @typescript-eslint/no-explicit-any
                sender: params.fromUserAddress ? params.fromUserAddress?.toString() : (null as any),
                // eslint-disable-next-line @typescript-eslint/no-explicit-any
                recipient: params.toUserAddress ? params.toUserAddress.toString() : (null as any),
                srcToken: params.fromToken.toString(),
                dstToken: params.toToken.toString(),
                amount: params.amount.toString(),
                slippage: params.maxSlippage?.toString() || DEFAULT_MAX_SLIPPAGE_BPS,
                gasRefill: params.gasRefill?.toString(),
                affiliateId: params.affiliateId,
                strategyTarget: params.strategyAddress,
                strategyMessage: params.strategyMessage,
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
            const order = await this.api.createOrder({ gatewayQuote: { onramp: quote.onramp } });

            if (!instanceOfGatewayCreateOrderOneOf(order)) {
                throw new Error('Invalid order type returned from API');
            }

            if (!btcSigner) {
                // Walletless flow: return the order ID for the user to complete payment externally
                return order.onramp.orderId;
            }

            let bitcoinTxHex: string;
            if (btcSigner.sendBitcoin) {
                bitcoinTxHex = await btcSigner.sendBitcoin({
                    from: quote.onramp.sender,
                    to: order.onramp.address,
                    value: formatBtc(BigInt(quote.onramp.inputAmount.amount)),
                    opReturn: order.onramp.opReturnData || undefined,
                    // isSignet: this.isSignet,
                });
            } else if (btcSigner.signAllInputs) {
                if (!order.onramp.psbtHex) {
                    throw new Error('PSBT not available: sender address is required when using signAllInputs');
                }
                bitcoinTxHex = await btcSigner.signAllInputs(order.onramp.psbtHex);
            } else {
                throw new Error('btcSigner must implement either sendBitcoin or signAllInputs method');
            }

            // bitcoinTxOrId = stripHexPrefix(bitcoinTxOrId);
            if (!bitcoinTxHex) throw new Error('Failed to get signed transaction');

            const tx = await this.api.registerTx(
                {
                    registerTx: {
                        onramp: {
                            orderId: order.onramp.orderId,
                            bitcoinTxHex: bitcoinTxHex,
                        },
                    },
                },
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
            const accountAddress = walletClient.account.address;
            const tokenAddress = quote.offramp.tokenAddress as Address;

            const order = await this.api.createOrder({ gatewayQuote: { offramp: quote.offramp } });

            if (!instanceOfGatewayCreateOrderOneOf1(order)) {
                throw new Error('Invalid order type returned from API');
            }

            const spenderAddress = order.offramp.tx.to as Address;

            // Check ETH balance and estimate gas for both potential transactions
            const allowance = !isAddressEqual(tokenAddress, zeroAddress)
                ? (
                      await publicClient.multicall({
                          allowFailure: false,
                          contracts: [
                              {
                                  address: tokenAddress,
                                  abi: erc20Abi,
                                  functionName: 'allowance',
                                  args: [accountAddress, spenderAddress],
                              },
                          ],
                      })
                  )[0]
                : maxUint256;

            const requiredAmount = BigInt(quote.offramp.inputAmount.amount);
            const needsApproval = requiredAmount > allowance;

            if (needsApproval && !isAddressEqual(tokenAddress, zeroAddress)) {
                if (isAddressEqual(tokenAddress, WBTC_OFT_ADDRESS)) {
                    if (quote.offramp.srcChain === 'bob') {
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
                } else {
                    // To change the USDT approval, first set the allowance to 0 (approve(_spender, 0))
                    // to avoid the ERC20 race condition:
                    // https://github.com/ethereum/EIPs/issues/20#issuecomment-263524729
                    if (isAddressEqual(tokenAddress, ETHEREUM_USDT_ADDRESS) && allowance !== 0n) {
                        const { request: resetRequest } = await publicClient.simulateContract({
                            account: walletClient.account,
                            address: tokenAddress,
                            abi: USDTApproveAbi,
                            functionName: 'approve',
                            args: [spenderAddress, 0n],
                        });
                        const resetTxHash = await walletClient.writeContract(resetRequest);
                        await publicClient.waitForTransactionReceipt({ hash: resetTxHash });
                    }

                    const { request } = await publicClient.simulateContract({
                        account: walletClient.account,
                        address: tokenAddress,
                        abi: isAddressEqual(tokenAddress, ETHEREUM_USDT_ADDRESS) ? USDTApproveAbi : erc20Abi,
                        functionName: 'approve',
                        args: [spenderAddress, maxUint256],
                    });

                    const approveTxHash = await walletClient.writeContract(request);
                    await publicClient.waitForTransactionReceipt({ hash: approveTxHash });
                }
            }

            const transactionHash = await walletClient.sendTransaction({
                account: walletClient.account,
                data: order.offramp.tx.data as Hex,
                to: spenderAddress,
                value: BigInt(order.offramp.tx.value || 0),
            });

            await publicClient?.waitForTransactionReceipt({ hash: transactionHash });

            await this.api.registerTx(
                {
                    registerTx: {
                        offramp: {
                            orderId: order.offramp.orderId,
                            evmTxhash: transactionHash,
                        },
                    },
                },
                initOverrides
            );

            return transactionHash;
        } else if (instanceOfGatewayQuoteOneOf2(quote)) {
            const tokenAddress = quote.layerZero.inputAmount.address as Address;
            const requiredAmount = BigInt(quote.layerZero.inputAmount.amount);
            const accountAddress = walletClient.account.address;
            const receiver = quote.layerZero.tx.to as Address;

            const order = await this.api.createOrder({ gatewayQuote: { layerZero: quote.layerZero } });

            if (!instanceOfGatewayCreateOrderOneOf2(order)) {
                throw new Error('Invalid order type returned from API');
            }

            if (!isAddressEqual(tokenAddress, WBTC_OFT_ADDRESS)) {
                // ERC20 token
                try {
                    const allowance = await publicClient.readContract({
                        account: walletClient.account,
                        address: tokenAddress,
                        abi: erc20Abi,
                        functionName: 'allowance',
                        args: [accountAddress, receiver],
                    });

                    if (allowance < requiredAmount) {
                        const { request } = await publicClient.simulateContract({
                            account: walletClient.account,
                            address: tokenAddress,
                            abi: erc20Abi,
                            functionName: 'approve',
                            args: [receiver, maxUint256],
                        });

                        const txHash = await walletClient.writeContract(request);

                        await publicClient.waitForTransactionReceipt({ hash: txHash });
                    }
                } catch (error) {
                    if (error instanceof ContractFunctionExecutionError) {
                        // https://github.com/wevm/viem/blob/3aa882692d2c4af3f5e9cc152099e07cde28e551/src/actions/public/simulateContract.test.ts#L711
                        // throw new error
                        throw new Error(
                            'Insufficient native funds for source and destination gas fees, please add more native funds to your account',
                            { cause: error }
                        );
                    }

                    throw error;
                }
            }

            // execute send call
            const transactionHash = await walletClient.sendTransaction({
                account: walletClient.account,
                data: quote.layerZero.tx.data as Hex,
                to: receiver,
                value: BigInt(quote.layerZero.tx.value || 0),
            });

            await publicClient.waitForTransactionReceipt({ hash: transactionHash });

            await this.api.registerTx(
                {
                    registerTx: {
                        layerZero: {
                            evmTxhash: transactionHash,
                            orderId: order.layerZero.orderId,
                        },
                    },
                },
                initOverrides
            );

            return transactionHash;
        }

        throw new Error('Invalid quote type');
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
    async _executeQuote(
        { quote, walletClient, publicClient, btcSigner }: { quote: GatewayQuote } & AllWalletClientParams,
        initOverrides?: RequestInit
    ): Promise<{ tx: string; orderId?: string; address?: string }> {
        if (instanceOfGatewayQuoteOneOf(quote)) {
            const order = await this.api.createOrder({ gatewayQuote: { onramp: quote.onramp } });

            if (!instanceOfGatewayCreateOrderOneOf(order)) {
                throw new Error('Invalid order type returned from API');
            }

            if (!btcSigner) {
                // Walletless flow: return order details for the user to complete payment externally
                return {
                    tx: '',
                    orderId: order.onramp.orderId,
                    address: order.onramp.address,
                };
            }

            let bitcoinTxHex: string;
            if (btcSigner.sendBitcoin) {
                bitcoinTxHex = await btcSigner.sendBitcoin({
                    from: quote.onramp.sender,
                    to: order.onramp.address,
                    value: formatBtc(BigInt(quote.onramp.inputAmount.amount)),
                    opReturn: order.onramp.opReturnData || undefined,
                    // isSignet: this.isSignet,
                });
            } else if (btcSigner.signAllInputs) {
                if (!order.onramp.psbtHex) {
                    throw new Error('PSBT not available: sender address is required when using signAllInputs');
                }
                bitcoinTxHex = await btcSigner.signAllInputs(order.onramp.psbtHex);
            } else {
                throw new Error('btcSigner must implement either sendBitcoin or signAllInputs method');
            }

            // bitcoinTxOrId = stripHexPrefix(bitcoinTxOrId);
            if (!bitcoinTxHex) throw new Error('Failed to get signed transaction');

            const tx = await this.api.registerTx(
                {
                    registerTx: {
                        onramp: {
                            orderId: order.onramp.orderId,
                            bitcoinTxHex: bitcoinTxHex,
                        },
                    },
                },
                initOverrides
            );

            if (typeof tx === 'string') {
                return { tx: tx, orderId: order.onramp.orderId };
            }

            if (!instanceOfRegisterTxOneOf(tx)) {
                throw new Error('Invalid registerTx response type');
            }

            return { tx: tx.onramp.txid, orderId: order.onramp.orderId };
        } else if (instanceOfGatewayQuoteOneOf1(quote)) {
            if (!walletClient.account) {
                throw new Error(`walletClient is required for offramp order`);
            }
            const accountAddress = walletClient.account.address;
            const tokenAddress = quote.offramp.tokenAddress as Address;

            const order = await this.api.createOrder({ gatewayQuote: { offramp: quote.offramp } });

            if (!instanceOfGatewayCreateOrderOneOf1(order)) {
                throw new Error('Invalid order type returned from API');
            }

            const spenderAddress = order.offramp.tx.to as Address;

            // Check ETH balance and estimate gas for both potential transactions
            const allowance = !isAddressEqual(tokenAddress, zeroAddress)
                ? (
                      await publicClient.multicall({
                          allowFailure: false,
                          contracts: [
                              {
                                  address: tokenAddress,
                                  abi: erc20Abi,
                                  functionName: 'allowance',
                                  args: [accountAddress, spenderAddress],
                              },
                          ],
                      })
                  )[0]
                : maxUint256;

            const requiredAmount = BigInt(quote.offramp.inputAmount.amount);
            const needsApproval = requiredAmount > allowance;

            if (needsApproval && !isAddressEqual(tokenAddress, zeroAddress)) {
                if (isAddressEqual(tokenAddress, WBTC_OFT_ADDRESS)) {
                    if (quote.offramp.srcChain === 'bob') {
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
                } else {
                    // To change the USDT approval, first set the allowance to 0 (approve(_spender, 0))
                    // to avoid the ERC20 race condition:
                    // https://github.com/ethereum/EIPs/issues/20#issuecomment-263524729
                    if (isAddressEqual(tokenAddress, ETHEREUM_USDT_ADDRESS) && allowance !== 0n) {
                        const { request: resetRequest } = await publicClient.simulateContract({
                            account: walletClient.account,
                            address: tokenAddress,
                            abi: USDTApproveAbi,
                            functionName: 'approve',
                            args: [spenderAddress, 0n],
                        });
                        const resetTxHash = await walletClient.writeContract(resetRequest);
                        await publicClient.waitForTransactionReceipt({ hash: resetTxHash });
                    }

                    const { request } = await publicClient.simulateContract({
                        account: walletClient.account,
                        address: tokenAddress,
                        abi: isAddressEqual(tokenAddress, ETHEREUM_USDT_ADDRESS) ? USDTApproveAbi : erc20Abi,
                        functionName: 'approve',
                        args: [spenderAddress, maxUint256],
                    });

                    const approveTxHash = await walletClient.writeContract(request);
                    await publicClient.waitForTransactionReceipt({ hash: approveTxHash });
                }
            }

            const transactionHash = await walletClient.sendTransaction({
                account: walletClient.account,
                data: order.offramp.tx.data as Hex,
                to: spenderAddress,
                value: BigInt(order.offramp.tx.value || 0),
            });

            await publicClient?.waitForTransactionReceipt({ hash: transactionHash });

            await this.api.registerTx(
                {
                    registerTx: {
                        offramp: {
                            orderId: order.offramp.orderId,
                            evmTxhash: transactionHash,
                        },
                    },
                },
                initOverrides
            );

            return { tx: transactionHash, orderId: order.offramp.orderId };
        } else if (instanceOfGatewayQuoteOneOf2(quote)) {
            const tokenAddress = quote.layerZero.inputAmount.address as Address;
            const requiredAmount = BigInt(quote.layerZero.inputAmount.amount);
            const accountAddress = walletClient.account.address;
            const receiver = quote.layerZero.tx.to as Address;

            if (!isAddressEqual(tokenAddress, WBTC_OFT_ADDRESS)) {
                // ERC20 token
                try {
                    const allowance = await publicClient.readContract({
                        account: walletClient.account,
                        address: tokenAddress,
                        abi: erc20Abi,
                        functionName: 'allowance',
                        args: [accountAddress, receiver],
                    });

                    if (allowance < requiredAmount) {
                        const { request } = await publicClient.simulateContract({
                            account: walletClient.account,
                            address: tokenAddress,
                            abi: erc20Abi,
                            functionName: 'approve',
                            args: [receiver, maxUint256],
                        });

                        const txHash = await walletClient.writeContract(request);

                        await publicClient.waitForTransactionReceipt({ hash: txHash });
                    }
                } catch (error) {
                    if (error instanceof ContractFunctionExecutionError) {
                        // https://github.com/wevm/viem/blob/3aa882692d2c4af3f5e9cc152099e07cde28e551/src/actions/public/simulateContract.test.ts#L711
                        // throw new error
                        throw new Error(
                            'Insufficient native funds for source and destination gas fees, please add more native funds to your account',
                            { cause: error }
                        );
                    }

                    throw error;
                }
            }

            // execute send call
            const transactionHash = await walletClient.sendTransaction({
                account: walletClient.account,
                data: quote.layerZero.tx.data as Hex,
                to: receiver,
                value: BigInt(quote.layerZero.tx.value || 0),
            });

            await publicClient.waitForTransactionReceipt({ hash: transactionHash });

            return { tx: transactionHash };
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

    /**
     * Retrieves the maximum spendable amount for a given Bitcoin address
     *
     * @param address The user's Bitcoin address
     * @returns Promise resolving to the maximum spendable amount
     */
    async getMaxSpendable(address: string, initOverrides?: RequestInit): Promise<GatewayMaxSpendable> {
        return this.api.getMaxSpendable({ address }, initOverrides);
    }

    /**
     * Retrieves all orders (onramp and offramp) for a specific user address.
     *
     * @param userAddress The user's EVM address
     * @returns Promise resolving to array of typed orders
     */
    async getOrders(userAddress: Address, initOverrides?: RequestInit): Promise<Array<GatewayOrderInfo>> {
        return this.api.getOrders({ userAddress }, initOverrides);
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
