import { AddressType, Network } from 'bitcoin-address-validation';
import * as bitcoin from 'bitcoinjs-lib';
import { AbiCoder, ethers, MaxUint256 } from 'ethers';
import {
    Account,
    Address,
    erc20Abi,
    Hex,
    isAddress,
    isAddressEqual,
    PublicClient,
    Transport,
    Chain as ViemChain,
    WalletClient,
} from 'viem';
import { bob, bobSepolia } from 'viem/chains';
import { EsploraClient } from '../esplora';
import { bigIntToFloatingNumber } from '../utils';
import { createBitcoinPsbt, getAddressInfo } from '../wallet';
import { claimDelayAbi, offrampCaller, strategyCaller } from './abi';
import StrategyClient from './strategy';
import { ADDRESS_LOOKUP, getTokenAddress, getTokenDecimals } from './tokens';
import {
    Chain,
    ChainId,
    chainIdMapping,
    convertOrderDetailsRawToOrderDetails,
    convertOrderDetailsToRaw,
    EnrichedToken,
    ExecuteQuoteParams,
    ExecuteStakeParam,
    GatewayCreateOrderRequestPayload,
    GatewayCreateOrderResponse,
    GatewayQuote,
    GatewayStartOrder,
    GatewayStrategy,
    GatewayStrategyContract,
    GatewayTokensInfo,
    GetQuoteParams,
    OfframpBumpFeeParams,
    OfframpCreateOrderParams,
    OfframpLiquidity,
    OfframpOrder,
    OfframpOrderStatus,
    OfframpQuote,
    OfframpRawOrder,
    OfframpUnlockFundsParams,
    OnchainOfframpOrderDetails,
    OnrampExecuteQuoteParams,
    OnrampOrder,
    OnrampOrderResponse,
    OrderStatus,
    StakeParams,
    StakeTransactionParams,
    Token,
} from './types';
import { parseOrderStatus, slugify, stripHexPrefix, toHexScriptPubKey, viemClient } from './utils';

/**
 * Base url for the mainnet Gateway API.
 * @default "https://gateway-api-mainnet.gobob.xyz"
 */
export const MAINNET_GATEWAY_BASE_URL = 'https://gateway-api-mainnet.gobob.xyz';

/**
 * Base url for the Signet Gateway API.
 * @default "https://gateway-api-testnet.gobob.xyz"
 */
export const SIGNET_GATEWAY_BASE_URL = 'https://gateway-api-signet.gobob.xyz';

/**
 * Duration (in seconds) an offramp order remains valid before it can be finalized on-chain.
 * This is calculated from the moment the quote is fetched.
 */
export const ORDER_DEADLINE_IN_SECONDS = 30 * 60; // 30 minutes

/**
 * Gateway REST HTTP API client
 */
export class GatewayApiClient {
    private chain: Chain.BOB | Chain.BOB_SEPOLIA;
    private baseUrl: string;
    private strategy: StrategyClient;
    private isSignet: boolean = false;

    /**
     * @constructor
     * @param chainName The chain name.
     */
    constructor(chainName: 'mainnet' | 'signet' | 'bob', options?: { rpcUrl?: string }) {
        switch (chainName) {
            case 'mainnet':
            case 'bob':
                this.chain = Chain.BOB;
                this.baseUrl = MAINNET_GATEWAY_BASE_URL;
                this.strategy = new StrategyClient(bob, options?.rpcUrl);
                break;
            case 'signet':
                this.chain = Chain.BOB_SEPOLIA;
                this.baseUrl = SIGNET_GATEWAY_BASE_URL;
                this.strategy = new StrategyClient(bobSepolia, options?.rpcUrl);
                this.isSignet = true;
                break;
            default:
                throw new Error('Invalid chain');
        }
    }

    private get chainId(): ChainId {
        return this.chain === Chain.BOB ? ChainId.BOB : ChainId.BOB_SEPOLIA;
    }

    /**
     * Returns all chains supported by the SDK.
     *
     * @returns {string[]} The array of chain names.
     */
    getChains(): string[] {
        return Object.values(Chain);
    }

    /**
     * Fetches quote for the given token and amount.
     *
     * @param {GetQuoteParams} params parameters for quote.
     * @returns quote details.
     *
     * @dev use as drop-in replacement. Type safety is not guaranteed. Instead we do runtime checks.
     */
    async getQuote(params: GetQuoteParams): Promise<{
        params: GetQuoteParams;
        onrampQuote?: GatewayQuote & GatewayTokensInfo;
        offrampQuote?: OfframpQuote;
    }> {
        // NOTE: fromChain must be specified if you do onramp
        if (params.fromChain?.toString().toLowerCase() === 'bitcoin') {
            // NOTE: toChain validation is performed inside `getOnrampQuote` method
            const onrampQuote = await this.getOnrampQuote(params);
            return { params, onrampQuote };
        } else if (params.toChain.toString().toLowerCase() === 'bitcoin') {
            if (!params.fromToken) {
                throw new Error('`fromToken` must be specified for offramp');
            }
            if (!params.amount) {
                throw new Error('`amount` must be specified for offramp');
            }
            const tokenAddress = getTokenAddress(this.chainId, params.fromToken.toLowerCase());
            const offrampQuote = await this.fetchOfframpQuote(tokenAddress, BigInt(params.amount || 0));

            return { params, offrampQuote };
        }

        throw new Error('Invalid quote arguments');
    }

    /**
     * Get a quote from the Gateway API for swapping or staking BTC.
     *
     * @param params The parameters for the quote.
     */
    async getOnrampQuote(params: GetQuoteParams): Promise<GatewayQuote & GatewayTokensInfo> {
        const isMainnet =
            params.toChain === ChainId.BOB ||
            (typeof params.toChain === 'string' && params.toChain.toLowerCase() === Chain.BOB);
        const isTestnet =
            params.toChain === ChainId.BOB_SEPOLIA ||
            (typeof params.toChain === 'string' && params.toChain.toLowerCase() === Chain.BOB_SEPOLIA);

        if (
            (!isMainnet && !isTestnet) ||
            (isMainnet && this.chain !== Chain.BOB) ||
            (isTestnet && this.chain !== Chain.BOB_SEPOLIA)
        ) {
            throw new Error('Invalid output chain');
        }

        const toToken = params.toToken.toLowerCase();
        const outputTokenAddress = getTokenAddress(this.chainId, toToken);
        const strategyAddress = params.strategyAddress?.startsWith('0x') ? params.strategyAddress : undefined;

        const url = new URL(`${this.baseUrl}/quote/v4/${outputTokenAddress}`);
        if (strategyAddress) url.searchParams.append('strategy', strategyAddress);
        if (params.amount) url.searchParams.append('satoshis', `${params.amount}`);
        if (params.gasRefill) url.searchParams.append('ethAmountToReceive', `${params.gasRefill}`);

        const response = await fetch(url, {
            headers: {
                'Content-Type': 'application/json',
                Accept: 'application/json',
            },
        });

        const jsonResponse = await response.json();

        const quote: GatewayQuote = {
            ...jsonResponse,
            orderDetails: convertOrderDetailsRawToOrderDetails(jsonResponse.orderDetails),
        };

        return {
            ...quote,
            baseToken: ADDRESS_LOOKUP[this.chainId][quote.baseTokenAddress],
            outputToken: quote.strategyAddress ? ADDRESS_LOOKUP[this.chainId][outputTokenAddress] : undefined,
        };
    }

    /**
     * Fetches the offramp registry address.
     * This address is used to interact with the offramp registry contract.
     *
     * @returns The offramp registry address.
     */
    async fetchOfframpRegistryAddress(): Promise<Address> {
        const response = await this.fetchGet(`${this.baseUrl}/offramp-registry-address`);
        return response.text() as Promise<Address>;
    }

    /**
     * Fetches an offramp liquidity for the given token.
     *
     * @param token ERC20 token address.
     * @returns Offramp liquidity details.
     */
    async fetchOfframpLiquidity(token: string): Promise<OfframpLiquidity> {
        const tokenAddress = getTokenAddress(this.chainId, token.toLowerCase());

        const response = await fetch(`${this.baseUrl}/offramp-liquidity/${tokenAddress}`, {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
                Accept: 'application/json',
            },
        });

        if (!response.ok) {
            const errorData = await response.json().catch(() => null);
            const errorMessage = errorData?.message || 'Failed to get offramp liquidity';
            throw new Error(`Offramp liquidity API Error: ${errorMessage}`);
        }

        const rawLiquidity = await response.json();

        return {
            token: rawLiquidity.tokenAddress as Address,
            maxOrderAmount: BigInt(rawLiquidity.maxOrderAmount),
            totalOfframpLiquidity: BigInt(rawLiquidity.totalOfframpLiquidity),
        };
    }

    /**
     * Fetches an offramp quote for the given token and amount.
     *
     * @param {Address} token ERC20 token address.
     * @param amountInToken Amount specified in token decimals.
     * @returns Offramp quote details.
     */
    async fetchOfframpQuote(token: Address, amountInToken: bigint): Promise<OfframpQuote> {
        const queryParams = new URLSearchParams({
            amountInWrappedToken: amountInToken.toString(),
            token,
        });

        const response = await fetch(`${this.baseUrl}/offramp-quote?${queryParams}`, {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
                Accept: 'application/json',
            },
        });

        if (!response.ok) {
            const errorData = await response.json();
            const errorMessage = errorData?.message || 'Failed to get offramp quote';
            throw new Error(`Offramp API Error: ${errorMessage} ${queryParams}`);
        }

        const rawQuote: OfframpQuote = await response.json();
        const currentUnixTimeInSec = Math.floor(Date.now() / 1000);
        const deadline = currentUnixTimeInSec + ORDER_DEADLINE_IN_SECONDS;

        return {
            amountLockInSat: rawQuote.amountLockInSat,
            registryAddress: rawQuote.registryAddress as Address,
            deadline: deadline,
            token: token as Address,
            feeBreakdown: {
                overallFeeSats: rawQuote.feeBreakdown.overallFeeSats,
                inclusionFeeSats: rawQuote.feeBreakdown.inclusionFeeSats,
                protocolFeeSats: rawQuote.feeBreakdown.protocolFeeSats,
                affiliateFeeSats: rawQuote.feeBreakdown.affiliateFeeSats,
                fastestFeeRate: rawQuote.feeBreakdown.fastestFeeRate,
            },
        };
    }

    /**
     * Prepares calldata to create an offramp order for BTC.
     *
     * @param params Offramp order params, excluding auto-filled fields.
     * @returns Parameters for onchain `createOrder` call.
     */
    async createOfframpOrder(quote: OfframpQuote, params: GetQuoteParams): Promise<OfframpCreateOrderParams> {
        // get btc script pub key
        let bitcoinNetwork = bitcoin.networks.regtest;
        if (this.chain == Chain.BOB) {
            bitcoinNetwork = bitcoin.networks.bitcoin;
        } else if (this.chain == Chain.BOB_SEPOLIA) {
            bitcoinNetwork = bitcoin.networks.testnet;
        }

        if (!params.toUserAddress || getAddressInfo(params.toUserAddress, this.isSignet).type === AddressType.p2tr) {
            throw new Error('Only following bitcoin address types are supported P2PKH, P2WPKH, P2SH or P2WSH.');
        }
        const receiverAddress = toHexScriptPubKey(params.toUserAddress, bitcoinNetwork);

        return {
            quote,
            offrampABI: offrampCaller,
            feeBreakdown: quote.feeBreakdown,
            offrampFunctionName: 'createOrder' as const,
            offrampArgs: [
                {
                    satAmountToLock: BigInt(quote.amountLockInSat),
                    satFeesMax: BigInt(quote.feeBreakdown.overallFeeSats),
                    orderCreationDeadline: BigInt(quote.deadline),
                    outputScript: receiverAddress as `0x${string}`,
                    token: quote.token,
                    orderOwner: params.fromUserAddress as Address,
                },
            ],
        };
    }

    /**
     * Prepares calldata to bump fees for an active offramp order.
     *
     * @param orderId The ID of the existing order.
     * @returns Parameters for onchain `bumpFeeOfExistingOrder` call.
     */
    async bumpFeeForOfframpOrder(orderId: bigint): Promise<OfframpBumpFeeParams> {
        // check order status via viem should be Active/Accepted
        const orderDetails: OnchainOfframpOrderDetails = await this.fetchOfframpOrder(orderId);

        if (orderDetails.status !== 'Active') {
            throw new Error(`Offramp order needs to be Active for bumping fees`);
        }

        const [shouldFeesBeBumped, newFeeSat, error] = await this.getBumpFeeRequirement(
            orderDetails.token,
            orderDetails.satAmountLocked,
            orderDetails.satFeesMax
        );

        if (error) {
            throw new Error(`Unable to calculate a new quote for the order. reason: (${error.toString()}).`);
        }

        if (!shouldFeesBeBumped) {
            throw new Error(
                `Current fees (${orderDetails.satFeesMax.toString()} sat) are sufficient to satisfy the order, as the new required fees (${newFeeSat.toString()} sat) are lower or equal.`
            );
        }

        const offrampRegistryAddress: Address = await this.fetchOfframpRegistryAddress();

        return {
            offrampABI: offrampCaller,
            offrampRegistryAddress: offrampRegistryAddress,
            offrampFunctionName: 'bumpFeeOfExistingOrder' as const,
            offrampArgs: [orderId, BigInt(newFeeSat)],
        };
    }

    /**
     * Prepares calldata to unlock funds for an unprocessed offramp order.
     *
     * @param orderId The ID of the order to unlock.
     * @param receiver The address to receive the funds.
     * @returns Parameters for onchain `unlockFunds` call.
     */
    async unlockOfframpOrder(orderId: bigint, receiver: Address): Promise<OfframpUnlockFundsParams> {
        // check order status via viem should be Active/Accepted
        const orderDetails: OnchainOfframpOrderDetails = await this.fetchOfframpOrder(orderId);

        // Processed and refunded order can't be unlocked
        if (orderDetails.status == 'Processed' || orderDetails.status == 'Refunded') {
            throw new Error(`Offramp order already processed/refunded`);
        }

        const offrampRegistryAddress: Address = await this.fetchOfframpRegistryAddress();

        // Active order can be unlocked and Accepted order can be unlocked after delay
        if (
            !(await this.canOrderBeUnlocked(orderDetails.status, orderDetails.orderTimestamp, offrampRegistryAddress))
        ) {
            throw new Error(`Offramp order is still within the 7-day claim delay and cannot be unlocked yet.`);
        }

        return {
            offrampABI: offrampCaller,
            offrampRegistryAddress: offrampRegistryAddress,
            offrampFunctionName: 'unlockFunds',
            offrampArgs: [orderId, receiver],
        };
    }

    /**
     * Returns all offramp orders for this account.
     *
     * @param userAddress The user's EVM address.
     * @returns {Promise<OfframpOrder[]>} The array of account orders.
     */
    async getOfframpOrders(userAddress: Address): Promise<OfframpOrder[]> {
        const response = await this.fetchGet(`${this.baseUrl}/offramp-orders/${userAddress}`);
        const rawOrders: OfframpRawOrder[] = await response.json();
        const offrampRegistryAddress: Address = await this.fetchOfframpRegistryAddress();

        return Promise.all(
            rawOrders.map(async (order) => {
                const status = order.status as OfframpOrderStatus;
                const canOrderBeUnlocked = await this.canOrderBeUnlocked(
                    status,
                    Number(order.orderTimestamp),
                    offrampRegistryAddress
                );

                return {
                    ...order,
                    status,
                    token: order.token as Address,
                    orderId: BigInt(order.orderId.toString()),
                    satAmountLocked: BigInt(order.satAmountLocked.toString()),
                    satFeesMax: BigInt(order.satFeesMax.toString()),
                    orderTimestamp: Number(order.orderTimestamp),
                    shouldFeesBeBumped: order.shouldFeesBeBumped,
                    canOrderBeUnlocked,
                };
            })
        );
    }

    /**
     * Checks whether the current fee cap (`satFeesMax`) is sufficient for processing
     * the offramp order. If the current fee is too low compared to the latest quote,
     * a fee bump is required.
     */
    private async getBumpFeeRequirement(
        token: Address,
        satAmountLocked: bigint,
        satFeesMax: bigint
    ): Promise<[boolean, bigint, string?]> {
        const decimals = getTokenDecimals(token);
        if (decimals === undefined) {
            throw new Error('Tokens with less than 8 decimals are not supported');
        }

        const amountInToken = satAmountLocked * BigInt(10 ** (decimals - 8));

        try {
            const offrampQuote = await this.fetchOfframpQuote(token, amountInToken);
            const shouldBump = satFeesMax < offrampQuote.feeBreakdown.overallFeeSats;
            return [shouldBump, BigInt(offrampQuote.feeBreakdown.overallFeeSats)];
        } catch (err) {
            // Return false and 0n with an error message if fetching the quote fails
            throw new Error(`Error fetching offramp quote: ${err.message || err}`);
        }
    }

    async canOrderBeUnlocked(
        status: OfframpOrderStatus,
        orderTimestamp: number,
        offrampRegistryAddress: Address
    ): Promise<boolean> {
        if (status === 'Active') {
            return true;
        }
        if (status !== 'Accepted') {
            return false;
        }
        // check if Accepted order has passed claim delay
        const nowInSec = Math.floor(Date.now() / 1000);
        const publicClient = viemClient(this.chain) as PublicClient;
        const claimDelay = await publicClient.readContract({
            address: offrampRegistryAddress,
            abi: claimDelayAbi,
            functionName: 'CLAIM_DELAY',
        });
        return orderTimestamp + Number(claimDelay) <= nowInSec;
    }

    /**
     * Fetches onchain details for a specific offramp order.
     *
     * @param orderId The ID of the order to fetch.
     * @returns Onchain order details.
     */
    private async fetchOfframpOrder(orderId: bigint): Promise<OnchainOfframpOrderDetails> {
        const offrampRegistryAddress: Address = await this.fetchOfframpRegistryAddress();
        const publicClient = viemClient(this.chain) as PublicClient;

        const order = await publicClient.readContract({
            address: offrampRegistryAddress,
            abi: offrampCaller,
            functionName: 'getOfframpOrder',
            args: [orderId],
        });

        return {
            orderId,
            token: order.token as Address,
            satAmountLocked: order.satAmountLocked,
            satFeesMax: order.satFeesMax,
            sender: order.sender as Address,
            receiver: order.receiver !== (ethers.ZeroAddress as Address) ? (order.receiver as Address) : null,
            owner: order.owner !== (ethers.ZeroAddress as Address) ? (order.owner as Address) : null,
            outputScript: order.outputScript,
            status: parseOrderStatus(Number(order.status)) as OnchainOfframpOrderDetails['status'],
            orderTimestamp: Number(order.timestamp),
        };
    }

    // TODO: add error handling
    /**
     * Start an order via the Gateway API to reserve liquidity. This is step 1 of 2, see the {@link finalizeOrder} method.
     *
     * @param gatewayQuote The quote given by the {@link getQuote} method.
     * @param params The parameters for the quote, same as before.
     * @returns {Promise<GatewayStartOrder>} The success object.
     */
    async startOrder(gatewayQuote: GatewayQuote, params: GetQuoteParams): Promise<GatewayStartOrder> {
        if (!params.toUserAddress || !isAddress(params.toUserAddress)) {
            throw new Error('Invalid user address');
        }

        const abiCoder = new AbiCoder();
        const request: GatewayCreateOrderRequestPayload = {
            gatewayAddress: gatewayQuote.gatewayAddress,
            strategyAddress: gatewayQuote.strategyAddress,
            satsToConvertToEth: params.gasRefill || 0,
            userAddress: params.toUserAddress,
            gatewayExtraData: undefined,
            // TODO: update strategy data
            strategyExtraData: abiCoder.encode(['uint256'], [0]) as Hex,
            satoshis: gatewayQuote.satoshis,
            campaignId: params.campaignId,
            orderDetails: convertOrderDetailsToRaw(gatewayQuote.orderDetails),
        };

        const response = await fetch(`${this.baseUrl}/order/v4`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                Accept: 'application/json',
            },
            body: JSON.stringify(request),
        });

        if (!response.ok) {
            throw new Error('Failed to create order');
        }

        const data: GatewayCreateOrderResponse = await response.json();

        let psbtBase64: string = '';
        if (
            params.fromUserAddress &&
            typeof params.fromChain === 'string' &&
            params.fromChain.toLowerCase() === Chain.BITCOIN
        ) {
            psbtBase64 = await createBitcoinPsbt(
                params.fromUserAddress,
                gatewayQuote.bitcoinAddress,
                gatewayQuote.satoshis,
                params.fromUserPublicKey,
                data.opReturnHash,
                params.feeRate,
                gatewayQuote.txProofDifficultyFactor,
                this.isSignet
            );

            return {
                uuid: data.uuid,
                opReturnHash: data.opReturnHash,
                bitcoinAddress: gatewayQuote.bitcoinAddress,
                satoshis: gatewayQuote.satoshis,
                psbtBase64,
            };
        }

        throw new Error('Failed to create bitcoin psbt due to an unexpected error.');
    }

    /**
     * Execute an order via the Gateway API.
     * This method invokes the {@link startOrder} and the {@link finalizeOrder} methods.
     *
     * @param {ExecuteQuoteParams} executeQuoteParams - The params to initiate gateway or evm transaction.
     * @param options - Configuration object containing client instances and optional signer.
     * @param {WalletClient<Transport, ViemChain, Account>} options.walletClient - The wallet client for interacting with the blockchain.
     * @param {PublicClient<Transport>} options.publicClient - The public client for reading blockchain data.
     * @param {{ signAllInputs: (psbtBase64: string) => Promise<string> }} options.btcSigner - Optional Bitcoin signer for signing transaction inputs.
     * @async
     * @returns {Promise<GatewayStartOrder>} The success object.
     */
    async executeQuote(
        executeQuoteParams: ExecuteQuoteParams,
        {
            walletClient,
            publicClient,
            btcSigner,
        }: {
            walletClient: WalletClient<Transport, ViemChain, Account>;
            publicClient: PublicClient<Transport>;
            btcSigner?: { signAllInputs: (psbtBase64: string) => Promise<string> };
        }
    ): Promise<string> {
        const isOnrampQuoteParams = (args: ExecuteQuoteParams): args is OnrampExecuteQuoteParams => {
            return args.params.fromChain?.toString().toLowerCase() === 'bitcoin';
        };

        if (isOnrampQuoteParams(executeQuoteParams)) {
            const { onrampQuote, params } = executeQuoteParams;
            const quote = onrampQuote!;

            const localParams = { ...params };
            if (typeof localParams.toChain === 'number') {
                localParams.toChain = chainIdMapping[localParams.toChain];
            }
            const { uuid, psbtBase64 } = await this.startOrder(quote, localParams);

            if (!btcSigner) {
                throw new Error(`btcSigner is required for onramp order`);
            }

            const bitcoinTxHex = await btcSigner?.signAllInputs(psbtBase64!);

            if (!bitcoinTxHex) throw new Error('no psbt');

            const txId = await this.finalizeOrder(uuid, bitcoinTxHex);

            return txId;
        } else {
            const { params, offrampQuote } = executeQuoteParams;
            const quote = offrampQuote!;

            if (typeof params.fromChain === 'number') {
                params.fromChain = chainIdMapping[params.fromChain];
            }
            const tokenAddress = getTokenAddress(this.chainId, params.fromToken.toLowerCase());
            const [offrampOrder, offrampRegistryAddress] = await Promise.all([
                this.createOfframpOrder(quote, params),
                this.fetchOfframpRegistryAddress(),
            ]);

            const allowance = await publicClient.readContract({
                address: tokenAddress,
                abi: erc20Abi,
                functionName: 'allowance',
                args: [params.fromUserAddress as Address, offrampRegistryAddress],
            });

            // TODO: is this sat or token amount?
            if (BigInt(quote.amountLockInSat) > allowance) {
                const { request } = await publicClient.simulateContract({
                    account: walletClient.account,
                    address: tokenAddress,
                    abi: erc20Abi,
                    functionName: 'approve',
                    args: [offrampRegistryAddress, MaxUint256],
                });

                const approveTxHash = await walletClient.writeContract(request);

                await publicClient.waitForTransactionReceipt({ hash: approveTxHash });
            }

            const { request } = await publicClient.simulateContract({
                account: walletClient.account,
                address: offrampRegistryAddress,
                abi: offrampOrder.offrampABI,
                functionName: offrampOrder.offrampFunctionName,
                args: offrampOrder.offrampArgs,
            });

            const transactionHash = await walletClient.writeContract(request);

            await publicClient?.waitForTransactionReceipt({ hash: transactionHash });

            return transactionHash;
        }
    }

    async executeStake(
        params: ExecuteStakeParam,
        walletClient: WalletClient<Transport, ViemChain, Account>,
        publicClient: PublicClient<Transport>
    ) {
        const result = await this.buildStake(params);

        const allowance = await publicClient.readContract({
            address: params.token,
            abi: erc20Abi,
            functionName: 'allowance',
            args: [walletClient.account.address as Address, result.strategyAddress],
        });

        if (BigInt(params.amount) > allowance) {
            const { request } = await publicClient.simulateContract({
                account: walletClient.account,
                address: params.token,
                abi: erc20Abi,
                functionName: 'approve',
                args: [result.strategyAddress, MaxUint256],
            });

            const approveTxHash = await walletClient.writeContract(request);

            await publicClient.waitForTransactionReceipt({ hash: approveTxHash });
        }

        const { request } = await publicClient.simulateContract({
            address: result.strategyAddress, // Ensure correct type
            abi: result.strategyABI,
            functionName: result.strategyFunctionName,
            args: result.strategyArgs,
            account: result.address,
        });

        const transactionHash = await walletClient.writeContract(request);

        await publicClient?.waitForTransactionReceipt({ hash: transactionHash });

        return transactionHash;
    }

    /**
     * Finalize an order via the Gateway API by providing the Bitcoin transaction. The tx will
     * be validated for correctness and forwarded to the mempool so there is no need to separately
     * broadcast the transaction. This is step 2 of 2, see the {@link startOrder} method.
     *
     * @param uuid The id given by the {@link startOrder} method.
     * @param bitcoinTxOrId The hex encoded Bitcoin transaction or txid.
     * @returns {Promise<string>} The Bitcoin txid.
     */
    async finalizeOrder(uuid: string, bitcoinTxOrId: string): Promise<string> {
        bitcoinTxOrId = stripHexPrefix(bitcoinTxOrId);

        let bitcoinTxHex: string;
        if (bitcoinTxOrId.length === 64) {
            const esploraClient = new EsploraClient(this.chain === Chain.BOB ? Network.mainnet : Network.signet);
            bitcoinTxHex = await esploraClient.getTransactionHex(bitcoinTxOrId);
        } else {
            bitcoinTxHex = bitcoinTxOrId;
        }

        const response = await fetch(`${this.baseUrl}/order/${uuid}`, {
            method: 'PATCH',
            headers: {
                'Content-Type': 'application/json',
                Accept: 'application/json',
            },
            body: JSON.stringify({ bitcoinTx: bitcoinTxHex }),
        });

        if (!response.ok) {
            throw new Error('Failed to update order');
        }

        return response.json();
    }

    /**
     * Returns all pending and completed orders for this account.
     *
     * @param userAddress The user's EVM address.
     * @returns {Promise<OnrampOrder[]>} The array of account orders.
     */
    async getOnrampOrders(userAddress: Address): Promise<OnrampOrder[]> {
        const chainId = this.chainId;
        const response = await this.fetchGet(`${this.baseUrl}/orders/${userAddress}`);
        const orders: OnrampOrderResponse[] = await response.json();
        return orders.map((order) => {
            function getFinal<L, R>(base?: L, output?: R): NonNullable<L | R> {
                return order.status
                    ? order.strategyAddress
                        ? output
                            ? output // success
                            : (base as NonNullable<typeof base>) // failed
                        : (base as NonNullable<typeof base>) // success
                    : order.strategyAddress // pending
                      ? (output as NonNullable<typeof output>)
                      : (base as NonNullable<typeof base>);
            }
            const getTokenAddress = (): string => {
                return getFinal(order.baseTokenAddress, order.outputTokenAddress);
            };
            const getToken = (): Token | undefined => {
                return ADDRESS_LOOKUP[chainId][getTokenAddress()];
            };
            const getConfirmations = async (esploraClient: EsploraClient, latestHeight?: number) => {
                const txStatus = await esploraClient.getTransactionStatus(order.txid);
                if (!latestHeight) {
                    latestHeight = await esploraClient.getLatestHeight();
                }
                return txStatus.confirmed ? latestHeight - txStatus.block_height! + 1 : 0;
            };
            return {
                gasRefill: order.satsToConvertToEth,
                ...order,
                baseToken: ADDRESS_LOOKUP[chainId][order.baseTokenAddress],
                outputToken: ADDRESS_LOOKUP[chainId][order.outputTokenAddress!],
                getTokenAddress,
                getToken,
                getTokenAmount() {
                    let amount = order.satoshis - order.fee;
                    const token = getToken();
                    if (token && !order.outputTokenAmount) {
                        amount *= Math.pow(10, token.decimals - 8);
                    }
                    return getFinal(amount, order.outputTokenAmount);
                },
                getConfirmations,
                async getStatus(esploraClient: EsploraClient, latestHeight?: number): Promise<OrderStatus> {
                    const confirmations = await getConfirmations(esploraClient, latestHeight);
                    const hasEnoughConfirmations = confirmations >= order.txProofDifficultyFactor;
                    const data = { confirmations };
                    return !hasEnoughConfirmations
                        ? { confirmed: false, data }
                        : order.status
                          ? order.strategyAddress
                              ? order.outputTokenAddress
                                  ? { success: true, data }
                                  : { success: false, data }
                              : { success: true, data }
                          : { pending: true, data };
                },
            };
        });
    }

    /**
     * Returns all strategies supported by the Gateway API.
     *
     * @returns {Promise<GatewayStrategyContract[]>} The array of strategies.
     */
    async getStrategies(): Promise<GatewayStrategyContract[]> {
        const response = await this.fetchGet(`${this.baseUrl}/strategies`);

        const chainName = this.chain.toString();
        const chainId = this.chainId;

        const strategies: GatewayStrategy[] = await response.json();
        return strategies.map((strategy) => {
            const strategySlug = slugify(strategy.strategyName);
            const inputToken = ADDRESS_LOOKUP[chainId][strategy.inputTokenAddress];
            const outputToken = strategy.outputTokenAddress
                ? ADDRESS_LOOKUP[chainId][strategy.outputTokenAddress]
                : undefined;
            return {
                id: strategySlug,
                type: 'deposit',
                address: strategy.strategyAddress,
                method: '',
                chain: {
                    id: '', // TODO
                    chainId: chainId,
                    slug: chainName,
                    name: chainName,
                    logo: '', // TODO
                    type: 'evm',
                    singleChainSwap: true,
                    singleChainStaking: true,
                },
                integration: {
                    type: strategy.strategyType,
                    slug: strategySlug,
                    name: strategy.strategyName,
                    logo: strategy.projectLogo || outputToken?.logoURI || '',
                    monetization: false,
                },
                inputToken: {
                    symbol: inputToken.symbol,
                    address: inputToken.address,
                    logo: inputToken.logoURI,
                    decimals: inputToken.decimals,
                    chain: chainName,
                },
                outputToken: outputToken
                    ? {
                          symbol: outputToken.symbol,
                          address: outputToken.address,
                          logo: outputToken.logoURI,
                          decimals: outputToken.decimals,
                          chain: chainName,
                      }
                    : null,
            };
        });
    }

    /**
     * Returns all tokens supported by the Gateway API.
     *
     * @param includeStrategies Also include output tokens via strategies (e.g. staking or lending).
     * @returns {Promise<Address[]>} The array of token addresses.
     */
    async getTokenAddresses(includeStrategies: boolean = true): Promise<Address[]> {
        const response = await this.fetchGet(`${this.baseUrl}/tokens?includeStrategies=${includeStrategies}`);
        return response.json();
    }

    /**
     * Same as {@link getTokenAddresses} but with additional info.
     *
     * @param includeStrategies Also include output tokens via strategies (e.g. staking or lending).
     * @returns {Promise<Token[]>} The array of tokens.
     */
    async getTokens(includeStrategies: boolean = true): Promise<Token[]> {
        // https://github.com/ethereum-optimism/ecosystem/blob/c6faa01455f9e846f31c0343a0be4c03cbeb2a6d/packages/op-app/src/hooks/useOPTokens.ts#L10
        const tokens = await this.getTokenAddresses(includeStrategies);
        return tokens.map((token) => ADDRESS_LOOKUP[this.chainId][token]).filter((token) => token !== undefined);
    }

    // TODO: should get price from the gateway API
    private async getPrices(): Promise<Map<string, number>> {
        const response = await this.fetchGet('https://fusion-api.gobob.xyz/pricefeed');

        if (!response.ok) {
            console.error('Failed to fetch prices from Fusion API');
            return new Map();
        }

        const list = await response.json();

        return new Map(list.map((x) => [x.token_address.toLowerCase(), Number(x.price)]));
    }

    /**
     * Same as {@link getTokens} but with additional info, like tvl.
     *
     * @param includeStrategies Also include output tokens via strategies (e.g. staking or lending).
     * @returns {Promise<EnrichedToken[]>} The array of tokens.
     */
    async getEnrichedTokens(includeStrategies: boolean = true): Promise<EnrichedToken[]> {
        const [tokens, prices] = await Promise.all([this.getTokenAddresses(includeStrategies), this.getPrices()]);

        const tokensIncentives = await this.strategy.getTokensIncentives(tokens);

        return Promise.all(
            tokens.map(async (address, i) => {
                const token = ADDRESS_LOOKUP[this.chainId][address];
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

                const underlyingToken = ADDRESS_LOOKUP[this.chainId][underlyingAddress.toLowerCase()];

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
     * Builds the parameters required to stake ERC-20 tokens using the specified strategy.
     *
     * @param {StakeParams} stakeParams - The parameters required for staking.
     * @returns {Promise<StakeTransactionParams>} The constructed staking parameters.
     * @throws {Error} If the strategy or token does not match, or if any address is invalid.
     *
     * @note Tokens must be approved first before calling the staking function.
     * @example
     * ```ts
     * // Check configs: https://viem.sh/docs/contract/writeContract.html#usage
     * import { account, publicClient, walletClient } from './config';
     * import { erc20Abi } from 'viem';
     *
     * // Define staking parameters
     * const params: StakeParams = {
     *     strategyAddress: '0x06cEA150E651236499319d78f92791f0FAe6FE67' as Address,
     *     token: '0x6744babdf02dcf578ea173a9f0637771a9e1c4d0' as Address,
     *     sender: '0x5e46D220eC8B01f55B70Dbb503c697f6E231eb65' as Address, // Sender must hold the input token
     *     receiver: '0x5e46D220eC8B01f55B70Dbb503c697f6E231eb65' as Address,
     *     amount: 100n,
     *     amountOutMin: 0n,
     * };
     *
     * // Call SDK method to build stake parameters
     * const result = await gatewaySDK.buildStake(params);
     *
     * // Approve ERC-20 token to be spent
     * const { request: approveRequest } = await publicClient.simulateContract({
     *     address: params.token, // Ensure correct type
     *     abi: erc20Abi,
     *     functionName: 'approve',
     *     args: [result.strategyAddress, MaxUint256],
     *     account: result.account, // Ensure correct type
     * });
     * await walletClient.writeContract(approveRequest);
     *
     * // Call strategy contract
     * const { request: stakeRequest } = await publicClient.simulateContract({
     *     address: result.strategyAddress, // Ensure correct type
     *     abi: result.strategyABI,
     *     functionName: result.strategyFunctionName,
     *     args: result.strategyArgs,
     *     account: result.account, // Ensure correct type
     * });
     * await walletClient.writeContract(stakeRequest);
     * ```
     */
    async buildStake(stakeParams: StakeParams): Promise<StakeTransactionParams> {
        const strategies = await this.getStrategies();

        const strategy = strategies.find((s) => isAddressEqual(s.address as Address, stakeParams.strategyAddress));
        if (!strategy) {
            throw new Error(
                `Strategy with address ${stakeParams.strategyAddress} not found. ${JSON.stringify(strategies)}`
            );
        }

        if (!isAddressEqual(strategy.inputToken.address as Address, stakeParams.token)) {
            throw new Error(
                `Provided token ${stakeParams.token} does not match strategy's input token ${strategy.inputToken.address}.`
            );
        }

        if (![stakeParams.sender, stakeParams.receiver, stakeParams.token].every((address) => isAddress(address))) {
            throw new Error(`Invalid EVM address detected.`);
        }

        return {
            strategyAddress: stakeParams.strategyAddress,
            strategyABI: strategyCaller,
            strategyFunctionName: 'handleGatewayMessageWithSlippageArgs',
            strategyArgs: [
                stakeParams.token,
                stakeParams.amount,
                stakeParams.receiver,
                { amountOutMin: stakeParams.amountOutMin },
            ],
            address: stakeParams.sender,
        };
    }

    /**
     * Returns all orders (onramp and offramp) for this account.
     *
     * @param userAddress The user's EVM address.
     * @returns {Promise<Array<OnrampOrder | OfframpOrder>>} The array of account orders.
     */
    async getOrders(
        userAddress: Address
    ): Promise<Array<{ type: 'onramp'; order: OnrampOrder } | { type: 'offramp'; order: OfframpOrder }>> {
        const [onrampOrders, offrampOrders] = await Promise.all([
            this.getOnrampOrders(userAddress),
            this.getOfframpOrders(userAddress),
        ]);
        return [
            ...onrampOrders.map((order) => ({ type: 'onramp' as const, order })),
            ...offrampOrders.map((order) => ({ type: 'offramp' as const, order })),
        ];
    }

    private fetchGet(url: string) {
        return fetch(url, {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
                Accept: 'application/json',
            },
        });
    }
}
