import {
    type Account,
    type Address,
    ContractFunctionExecutionError,
    erc20Abi,
    Hex,
    isAddress,
    isAddressEqual,
    maxUint256,
    type PublicClient,
    type Transport,
    type Chain as ViemChain,
    type WalletClient,
    zeroAddress,
} from 'viem';
import { oftApprovalRequiredAbi, strategyCaller, USDTApproveAbi } from './abi';
import { GatewayError } from './error';
import {
    Configuration,
    type GatewayCreateOrderOneOf,
    type GatewayCreateOrderV2,
    type GatewayMaxSpendable,
    type GatewayOrderInfoV2,
    type GatewayQuoteV3,
    type GetOrdersV3Request,
    instanceOfGatewayCreateOrderOneOf,
    instanceOfGatewayCreateOrderOneOf1,
    instanceOfGatewayCreateOrderV2OneOf,
    instanceOfGatewayQuoteV2OneOf,
    instanceOfGatewayQuoteV2OneOf2,
    instanceOfGatewayQuoteV3OneOf,
    instanceOfRegisterTxOneOf,
    type PaginatedOrdersResponse,
    type RouteInfo,
    V3Api,
} from './generated-client';
import type { GatewayError as GatewayErrorInterface } from './generated-client/models/GatewayError';
import {
    type BitcoinSigner,
    type ExecuteQuoteStep,
    ExecuteQuoteStepType,
    type GetQuoteParams,
    type StrategyParams,
} from './types';
import { formatBtc } from './utils';

const RETRY_COUNT = 8; // Number of times to retry fetching transaction receipt after sending a transaction

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
 * Result of executing a gateway quote.
 *
 * `tx` is the transaction hash/id (Bitcoin txid for onramp, EVM hash for offramp/LayerZero).
 * It is absent only in the walletless onramp flow, where no `btcSigner` was provided.
 * In that case, use `order.onramp.address` and `order.onramp.orderId` to complete
 * the BTC payment externally.
 */
export type ExecuteQuoteResult =
    | { order: GatewayCreateOrderV2; tx: string }
    | { order: GatewayCreateOrderOneOf; tx?: undefined };

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
 *
 * @example
 * ```typescript
 * // With API key authentication
 * const client = new GatewayApiClient({ apiKey: '<32-char key>' });
 * ```
 */
/** Options for {@link GatewayApiClient}. */
export interface GatewaySDKOptions {
    /** Custom Gateway API base URL. Defaults to mainnet. */
    basePath?: string;
    /** API key for authenticated requests. Must be exactly 32 characters. */
    apiKey?: string;
}

export class GatewayApiClient {
    api: V3Api;

    /**
     * Creates a new Gateway API client instance.
     *
     * @param options - {@link GatewaySDKOptions}
     *
     * @example
     * ```typescript
     * // Mainnet client
     * const mainnetClient = new GatewayApiClient();
     *
     * // Staging client
     * const stagingClient = new GatewayApiClient({ basePath: 'https://gateway-api-staging.gobob.xyz' });
     *
     * // With API key
     * const authenticatedClient = new GatewayApiClient({ apiKey: '<32-char key>' });
     * ```
     */
    constructor(options: GatewaySDKOptions = {}) {
        const { basePath, apiKey } = options;
        if (apiKey && apiKey.length !== 32) {
            throw new Error('apiKey must be exactly 32 characters');
        }

        this.api = new V3Api(
            new Configuration({
                basePath: basePath || MAINNET_GATEWAY_BASE_URL,
                headers: apiKey ? { Authorization: `Bearer ${apiKey}` } : undefined,
                middleware: [
                    {
                        async post(context) {
                            if (context.response && (context.response.status < 200 || context.response.status >= 300)) {
                                let body: GatewayErrorInterface;
                                try {
                                    body = await context.response.json();
                                } catch (parseError) {
                                    const err = GatewayError.fromText(
                                        `Non-JSON response: HTTP ${context.response.status} ${context.response.statusText}`
                                    );
                                    err.cause = parseError;
                                    throw err;
                                }

                                throw GatewayError.fromResponse(body);
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
    async getQuote(params: GetQuoteParams, initOverrides?: RequestInit): Promise<GatewayQuoteV3> {
        return this.api.getQuoteV3(
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
                ownerAddress: params.ownerAddress?.toString(),
                gasRefill: params.gasRefill?.toString(),
                affiliates: params.affiliates?.map((a) => `${a.address}:${a.bps}`).join(','),
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
     * @returns Promise resolving to the full order and optional transaction hash
     * @throws {Error} If required signers are missing or transaction fails
     */
    async executeQuote(
        {
            quote,
            walletClient,
            publicClient,
            btcSigner,
            callback,
        }: {
            quote: GatewayQuoteV3;
            callback?: (step: ExecuteQuoteStep) => void;
        } & AllWalletClientParams,
        initOverrides?: RequestInit
    ): Promise<ExecuteQuoteResult> {
        if (instanceOfGatewayQuoteV2OneOf(quote)) {
            const order = await this.api.createOrderV3({
                gatewayQuoteV3: { onramp: quote.onramp },
            });

            if (!instanceOfGatewayCreateOrderOneOf(order)) {
                throw new Error('Invalid order type returned from API');
            }

            if (!btcSigner) {
                // Walletless flow: return the order for the user to complete payment externally
                return { order };
            }

            let bitcoinTxHex: string;
            if (btcSigner.sendBitcoin) {
                callback?.({ step: 1, type: ExecuteQuoteStepType.SignBitcoinTransaction, totalSteps: 1 });
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
                callback?.({ step: 1, type: ExecuteQuoteStepType.SignBitcoinTransaction, totalSteps: 1 });
                bitcoinTxHex = await btcSigner.signAllInputs(order.onramp.psbtHex);
            } else {
                throw new Error('btcSigner must implement either sendBitcoin or signAllInputs method');
            }

            // bitcoinTxOrId = stripHexPrefix(bitcoinTxOrId);
            if (!bitcoinTxHex) throw new Error('Failed to get signed transaction');

            const tx = await this.api.registerTxV3(
                {
                    registerTxV2: {
                        onramp: {
                            orderId: order.onramp.orderId,
                            bitcoinTxHex: bitcoinTxHex,
                        },
                    },
                },
                initOverrides
            );

            if (typeof tx === 'string') {
                return { order, tx };
            }

            if (!instanceOfRegisterTxOneOf(tx)) {
                throw new Error('Invalid registerTx response type');
            }

            return { order, tx: tx.onramp.txid };
        } else if (instanceOfGatewayQuoteV3OneOf(quote)) {
            if (!walletClient.account) {
                throw new Error(`walletClient is required for offramp order`);
            }
            const accountAddress = walletClient.account.address;
            const tokenAddress = quote.offramp.tokenAddress as Address;
            const requiredAmount = BigInt(quote.offramp.inputAmount.amount);

            const order = await this.api.createOrderV3({
                gatewayQuoteV3: { offramp: quote.offramp },
            });

            if (!instanceOfGatewayCreateOrderOneOf1(order)) {
                throw new Error('Invalid order type returned from API');
            }

            const spenderAddress = order.offramp.tx.to as Address;

            let allowance = maxUint256;
            let requiresApproval = false;

            // Some receiver contracts (eg certain OFTs) do not require approvals, we check that here
            if (isAddress(tokenAddress) && !isAddressEqual(tokenAddress, zeroAddress)) {
                requiresApproval = await publicClient
                    .readContract({
                        address: spenderAddress,
                        abi: oftApprovalRequiredAbi,
                        functionName: 'approvalRequired',
                    })
                    .then((required) => Boolean(required))
                    .catch(() => true);

                if (requiresApproval) {
                    // If the OFT requires approval, we check the allowance already set
                    [allowance] = await publicClient.multicall({
                        allowFailure: false,
                        contracts: [
                            {
                                address: tokenAddress,
                                abi: erc20Abi,
                                functionName: 'allowance',
                                args: [accountAddress, spenderAddress],
                            },
                        ],
                    });
                }
            }

            const needsApproval =
                requiresApproval && requiredAmount > allowance && isAddress(tokenAddress) && !isAddressEqual(tokenAddress, zeroAddress);
            // Only for USDT on Ethereum, we need to reset the allowance to 0 before approving again because of a quirk in their implementation
            const needsReset = needsApproval && isAddress(tokenAddress) && isAddressEqual(tokenAddress, ETHEREUM_USDT_ADDRESS) && allowance !== 0n;

            const totalSteps = needsReset ? 3 : needsApproval ? 2 : 1;

            if (needsApproval) {
                // To change the USDT approval, first set the allowance to 0 (approve(_spender, 0))
                // to avoid the ERC20 race condition:
                // https://github.com/ethereum/EIPs/issues/20#issuecomment-263524729
                if (needsReset) {
                    const { request: resetRequest } = await publicClient.simulateContract({
                        account: walletClient.account,
                        address: tokenAddress,
                        abi: USDTApproveAbi,
                        functionName: 'approve',
                        args: [spenderAddress, 0n],
                    });
                    callback?.({ step: 1, type: ExecuteQuoteStepType.ResetApproval, totalSteps });
                    const resetTxHash = await walletClient.writeContract(resetRequest);
                    await publicClient.waitForTransactionReceipt({ hash: resetTxHash, retryCount: RETRY_COUNT });
                }

                const { request } = await publicClient.simulateContract({
                    account: walletClient.account,
                    address: tokenAddress,
                    abi: isAddressEqual(tokenAddress, ETHEREUM_USDT_ADDRESS) ? USDTApproveAbi : erc20Abi,
                    functionName: 'approve',
                    args: [spenderAddress, requiredAmount],
                });

                callback?.({ step: needsReset ? 2 : 1, type: ExecuteQuoteStepType.Approve, totalSteps });
                const approveTxHash = await walletClient.writeContract(request);
                await publicClient.waitForTransactionReceipt({ hash: approveTxHash, retryCount: RETRY_COUNT });
            }

            callback?.({ step: totalSteps, type: ExecuteQuoteStepType.SendTransaction, totalSteps });
            const transactionHash = await walletClient.sendTransaction({
                account: walletClient.account,
                data: order.offramp.tx.data as Hex,
                to: spenderAddress,
                value: BigInt(order.offramp.tx.value || 0),
            });

            await publicClient?.waitForTransactionReceipt({ hash: transactionHash, retryCount: RETRY_COUNT });

            try {
                await this.api.registerTxV3(
                    {
                        registerTxV2: {
                            offramp: {
                                orderId: order.offramp.orderId,
                                evmTxhash: transactionHash,
                            },
                        },
                    },
                    initOverrides
                );
            } catch {
                // Best-effort: the on-chain tx already succeeded, so return the result
                // even if registration fails. The order can be reconciled later.
            }

            return { order, tx: transactionHash };
        } else if (instanceOfGatewayQuoteV2OneOf2(quote)) {
            const tokenAddress = quote.tokenSwap.inputAmount.address as Address;
            const requiredAmount = BigInt(quote.tokenSwap.inputAmount.amount);
            const accountAddress = walletClient.account.address;
            const receiver = quote.tokenSwap.txTo as Address;

            // Pre-check allowance before createOrder so we can compute totalSteps upfront.
            // receiver is available from the quote (unlike offramp where spender comes from the order).
            let preCheckAllowance = 0n;

            // Some receiver contracts (eg certain OFTs) do not require approvals, we check that here
            const requiresApproval = await publicClient
                .readContract({
                    address: receiver,
                    abi: oftApprovalRequiredAbi,
                    functionName: 'approvalRequired',
                })
                .then((required) => Boolean(required))
                .catch(() => true);

            if (requiresApproval) {
                // If the OFT requires approval, we check the allowance already set
                preCheckAllowance = await publicClient.readContract({
                    account: walletClient.account,
                    address: tokenAddress,
                    abi: erc20Abi,
                    functionName: 'allowance',
                    args: [accountAddress, receiver],
                });
            }

            const needsApproval = requiresApproval && preCheckAllowance < requiredAmount;

            // Only for USDT on Ethereum, we need to reset the allowance to 0 before approving again because of a quirk in their implementation
            const needsReset =
                needsApproval && isAddressEqual(tokenAddress, ETHEREUM_USDT_ADDRESS) && preCheckAllowance !== 0n;

            const totalSteps = needsReset ? 3 : needsApproval ? 2 : 1;

            const order = await this.api.createOrderV3({
                gatewayQuoteV3: { tokenSwap: quote.tokenSwap },
            });

            if (!instanceOfGatewayCreateOrderV2OneOf(order)) {
                throw new Error('Invalid order type returned from API');
            }

            if (needsApproval) {
                // ERC20 token
                try {
                    // To change the USDT approval, first set the allowance to 0 (approve(_spender, 0))
                    // to avoid the ERC20 race condition:
                    // https://github.com/ethereum/EIPs/issues/20#issuecomment-263524729
                    if (needsReset) {
                        const { request: resetRequest } = await publicClient.simulateContract({
                            account: walletClient.account,
                            address: tokenAddress,
                            abi: USDTApproveAbi,
                            functionName: 'approve',
                            args: [receiver, 0n],
                        });
                        callback?.({ step: 1, type: ExecuteQuoteStepType.ResetApproval, totalSteps });
                        const resetTxHash = await walletClient.writeContract(resetRequest);
                        await publicClient.waitForTransactionReceipt({ hash: resetTxHash, retryCount: RETRY_COUNT });
                    }

                    const { request } = await publicClient.simulateContract({
                        account: walletClient.account,
                        address: tokenAddress,
                        abi: isAddressEqual(tokenAddress, ETHEREUM_USDT_ADDRESS) ? USDTApproveAbi : erc20Abi,
                        functionName: 'approve',
                        args: [receiver, requiredAmount],
                    });

                    callback?.({ step: needsReset ? 2 : 1, type: ExecuteQuoteStepType.Approve, totalSteps });
                    const txHash = await walletClient.writeContract(request);

                    await publicClient.waitForTransactionReceipt({ hash: txHash, retryCount: RETRY_COUNT });
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

            callback?.({ step: totalSteps, type: ExecuteQuoteStepType.SendTransaction, totalSteps });
            const transactionHash = await walletClient.sendTransaction({
                account: walletClient.account,
                data: order.tokenSwap.tx.data as Hex,
                to: order.tokenSwap.tx.to as Address,
                value: BigInt(order.tokenSwap.tx.value || 0),
            });

            await publicClient.waitForTransactionReceipt({ hash: transactionHash, retryCount: RETRY_COUNT });

            try {
                await this.api.registerTxV3(
                    {
                        registerTxV2: {
                            tokenSwap: {
                                evmTxhash: transactionHash,
                                orderId: order.tokenSwap.orderId,
                            },
                        },
                    },
                    initOverrides
                );
            } catch {
                // Best-effort: the on-chain tx already succeeded, so return the result
                // even if registration fails. The order can be reconciled later.
            }

            return { order, tx: transactionHash };
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
                args: [params.strategyAddress, BigInt(params.amount)],
            });

            const approveTxHash = await walletClient.writeContract(request);

            await publicClient.waitForTransactionReceipt({ hash: approveTxHash, retryCount: RETRY_COUNT });
        }

        const { request } = await publicClient.simulateContract({
            address: params.strategyAddress,
            abi: strategyCaller,
            functionName: 'handleGatewayMessageWithSlippageArgs', // TODO: encode args
            args: [params.token, params.amount, params.receiver, { amountOutMin: params.amountOutMin }],
            account: walletClient.account,
        });

        const transactionHash = await walletClient.writeContract(request);

        await publicClient?.waitForTransactionReceipt({ hash: transactionHash, retryCount: RETRY_COUNT });

        return transactionHash;
    }

    /**
     * Retrieves the maximum spendable amount for a given Bitcoin address
     *
     * @param address The user's Bitcoin address
     * @param initOverrides Optional request initialization overrides
     * @returns Promise resolving to the maximum spendable amount
     */
    async getMaxSpendable(address: string, initOverrides?: RequestInit): Promise<GatewayMaxSpendable> {
        return this.api.getMaxSpendableV2({ address }, initOverrides);
    }

    /**
     * Retrieves all orders (onramp and offramp) for a specific user address.
     *
     * @param userAddress The user's EVM address
     * @param initOverrides Optional request initialization overrides
     * @returns Promise resolving to array of typed orders
     */
    async getOrders(
        requestParameters: GetOrdersV3Request,
        initOverrides?: RequestInit
    ): Promise<PaginatedOrdersResponse> {
        return this.api.getOrdersV2(requestParameters, initOverrides);
    }

    /**
     * Retrieves a specific order by its ID (txId/txHash).
     *
     * @param id The order ID (txId/txHash)
     * @param initOverrides Optional request initialization overrides
     * @returns Promise resolving to the order information
     */
    async getOrder(id: string, initOverrides?: RequestInit): Promise<GatewayOrderInfoV2> {
        return this.api.getOrderV2({ id }, initOverrides);
    }

    /**
     * Retrieves all supported routes.
     *
     * @returns Promise resolving to array of supported routes
     */
    async getRoutes(initOverrides?: RequestInit): Promise<Array<RouteInfo>> {
        return this.api.getRoutesV3(initOverrides);
    }
}
