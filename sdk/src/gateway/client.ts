import { AddressType, Network } from 'bitcoin-address-validation';
import * as bitcoin from 'bitcoinjs-lib';
import {
    Account,
    Address,
    erc20Abi,
    isAddress,
    PublicClient,
    Transport,
    Chain as ViemChain,
    WalletClient,
    zeroAddress,
    maxUint256,
    Hash,
    Hex,
} from 'viem';
import { bob, bobSepolia } from 'viem/chains';
import { EsploraClient } from '../esplora';
import { bigIntToFloatingNumber } from '../utils';
import { createBitcoinPsbt, getAddressInfo } from '../wallet';
import { claimDelayAbi, offrampCaller, strategyCaller } from './abi';
import StrategyClient from './strategy';
import { ADDRESS_LOOKUP, getTokenAddress, getTokenDecimals } from './tokens';
import {
    BitcoinSigner,
    BumpFeeParams,
    EnrichedToken,
    ExecuteQuoteParams,
    GatewayCreateOrderRequestPayload,
    GatewayCreateOrderResponse,
    GatewayQuote,
    GatewayStartOrder,
    GatewayStrategy,
    GatewayStrategyContract,
    GatewayTokensInfo,
    GetQuoteParams,
    OfframpCreateOrderParams,
    OfframpLiquidity,
    OfframpOrder,
    OfframpOrderStatus,
    OfframpQuote,
    OfframpRawOrder,
    OnchainOfframpOrderDetails,
    OnrampExecuteQuoteParams,
    OnrampOrder,
    OrderStatus,
    StrategyParams,
    Token,
    UnlockOrderParams,
} from './types';
import {
    parseOrderStatus,
    slugify,
    stripHexPrefix,
    toHexScriptPubKey,
    viemClient,
    convertOrderDetailsRawToOrderDetails,
    convertOrderDetailsToRaw,
    formatBtc,
} from './utils';

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
 *   toChain: bob.id,
 *   toToken: 'WBTC',
 *   amount: 100000000, // 1 BTC in satoshis
 *   toUserAddress: '0x...'
 * });
 * ```
 */
export class GatewayApiClient {
    private chain: ViemChain;
    private strategy: StrategyClient;
    private isSignet: boolean = false;

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
    constructor(chainId: number, options?: { rpcUrl?: string }) {
        switch (chainId) {
            case bob.id:
                this.chain = bob;
                this.strategy = new StrategyClient(bob, options?.rpcUrl);
                break;
            case bobSepolia.id:
                this.chain = bobSepolia;
                this.strategy = new StrategyClient(bobSepolia, options?.rpcUrl);
                this.isSignet = true;
                break;
            default:
                throw new Error('Invalid chain');
        }
    }

    private get baseUrl(): string {
        return this.chain.id === bob.id ? MAINNET_GATEWAY_BASE_URL : SIGNET_GATEWAY_BASE_URL;
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
            const offrampQuote = await this.getOfframpQuote(params);

            return { params, offrampQuote };
        }

        throw new Error('Invalid quote arguments');
    }

    /**
     * Get an onramp quote for converting Bitcoin to wrapped BTC on BOB.
     *
     * @param params Quote parameters - see {@link GetQuoteParams}
     * @returns Promise resolving to onramp quote with token info
     * @throws {Error} If invalid output chain or parameters
     */
    async getOnrampQuote(params: GetQuoteParams): Promise<GatewayQuote & GatewayTokensInfo> {
        const isMainnet =
            params.toChain === bob.id ||
            (typeof params.toChain === 'string' && params.toChain.toLowerCase() === bob.name.toLowerCase());
        const isTestnet =
            params.toChain === bobSepolia.id ||
            (typeof params.toChain === 'string' && params.toChain.toLowerCase() === bobSepolia.name.toLowerCase());

        if (
            (!isMainnet && !isTestnet) ||
            (isMainnet && this.chain.id !== bob.id) ||
            (isTestnet && this.chain.id !== bobSepolia.id)
        ) {
            throw new Error('Invalid output chain');
        }

        const toToken = params.toToken.toLowerCase();
        const outputTokenAddress = getTokenAddress(this.chainId, toToken);
        const strategyAddress = params.strategyAddress?.startsWith('0x') ? params.strategyAddress : undefined;

        const url = new URL(`${this.baseUrl}/v4/quote/${outputTokenAddress}`);
        url.searchParams.append('userAddress', `${params.toUserAddress}`);
        if (strategyAddress) url.searchParams.append('strategy', strategyAddress);
        if (params.amount) url.searchParams.append('satoshis', `${params.amount}`);
        if (params.gasRefill) url.searchParams.append('ethAmountToReceive', `${params.gasRefill}`);
        if (params.message) url.searchParams.append('strategyExtraData', `${params.message}`);

        const response = await this.safeFetch(
            url,
            {
                headers: {
                    'Content-Type': 'application/json',
                    Accept: 'application/json',
                },
            },
            'Failed to get onramp liquidity'
        );

        if (!response.ok) {
            const errorData = await response.json().catch(() => null);
            const errorMessage = errorData?.message || 'Failed to get onramp liquidity';
            throw new Error(errorMessage);
        }

        const jsonResponse = await response.json();

        if (!jsonResponse.orderDetails) {
            const errorData = jsonResponse.errorData;
            const apiMessage = errorData?.message;
            const errorMessage = apiMessage || 'Failed to get onramp quote';
            throw new Error(errorMessage);
        }

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
     * Get an offramp quote for converting wrapped BTC on BOB to Bitcoin.
     *
     * @param params Quote parameters - see {@link GetQuoteParams}
     * @returns Promise resolving to offramp quote details
     * @throws {Error} If fromToken is not specified
     */
    async getOfframpQuote(params: GetQuoteParams) {
        if (!params.fromToken) {
            throw new Error('`fromToken` must be specified for offramp');
        }

        const tokenAddress = getTokenAddress(this.chainId, params.fromToken.toLowerCase());
        const quote = await this.fetchOfframpQuote(
            tokenAddress,
            BigInt(params.amount || 0),
            params.fromUserAddress as Address
        );

        return quote;
    }

    /**
     * Fetches the offramp registry contract address.
     *
     * @returns Promise resolving to the registry contract address
     */
    async fetchOfframpRegistryAddress(): Promise<Address> {
        const response = await this.safeFetch(
            `${this.baseUrl}/offramp-registry-address`,
            undefined,
            'Failed to fetch offramp registry contract address'
        );
        if (!response.ok) {
            const errorData = await response.json().catch(() => null);
            const apiMessage = errorData?.message;
            const errorMessage = apiMessage || 'Failed to fetch offramp registry contract';
            throw new Error(errorMessage);
        }
        return response.text() as Promise<Address>;
    }

    /**
     * Fetches available offramp liquidity for a specific token.
     *
     * @param token Token symbol or address
     * @returns Promise resolving to liquidity information
     * @throws {Error} If API request fails
     */
    async fetchOfframpLiquidity(token: string): Promise<OfframpLiquidity> {
        const tokenAddress = getTokenAddress(this.chainId, token.toLowerCase());

        const response = await this.safeFetch(
            `${this.baseUrl}/offramp-liquidity/${tokenAddress}`,
            undefined,
            'Failed to get offramp liquidity'
        );

        if (!response.ok) {
            const errorData = await response.json().catch(() => null);
            const errorMessage = errorData?.message || 'Failed to get offramp liquidity';
            throw new Error(errorMessage);
        }

        const rawLiquidity = await response.json();

        return {
            token: rawLiquidity.tokenAddress as Address,
            maxOrderAmount: BigInt(rawLiquidity.maxOrderAmount),
            totalOfframpLiquidity: BigInt(rawLiquidity.totalOfframpLiquidity),
        };
    }

    /**
     * Fetches an offramp quote for converting tokens to Bitcoin.
     *
     * @param token ERC20 token address
     * @param amountInToken Amount in token's smallest unit
     * @returns Promise resolving to offramp quote with fee breakdown
     * @throws {Error} If API request fails
     */
    async fetchOfframpQuote(token: Address, amountInToken: bigint, userAddress: Address): Promise<OfframpQuote> {
        const queryParams = new URLSearchParams({
            amountInWrappedToken: amountInToken.toString(),
            token,
            userAddress: userAddress,
        });

        const response = await this.safeFetch(
            `${this.baseUrl}/offramp-quote?${queryParams}`,
            undefined,
            'Failed to get offramp quote'
        );

        if (!response.ok) {
            const errorData = await response.json().catch(() => null);
            const apiMessage = errorData?.message;
            const errorMessage = apiMessage || `Failed to get offramp quote`;
            throw new Error(`${errorMessage} | queryParams: ${queryParams}`);
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
     * Prepares parameters for creating an offramp order on-chain.
     *
     * @param quote Offramp quote from {@link fetchOfframpQuote}
     * @param params Quote parameters - see {@link GetQuoteParams}
     * @returns Contract call parameters for creating the order
     * @throws {Error} If Bitcoin address type is unsupported
     */
    async createOfframpOrder(quote: OfframpQuote, params: GetQuoteParams): Promise<OfframpCreateOrderParams> {
        // get btc script pub key
        let bitcoinNetwork = bitcoin.networks.regtest;
        if (this.chain.id == bob.id) {
            bitcoinNetwork = bitcoin.networks.bitcoin;
        } else if (this.chain.id == bobSepolia.id) {
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
                    creationDeadline: BigInt(quote.deadline),
                    outputScript: receiverAddress as Hex,
                    token: quote.token,
                    owner: params.fromUserAddress as Address,
                },
            ],
        };
    }

    /**
     * Bumps fees for an active offramp order that requires higher fees.
     *
     * @param params Parameters including orderId and wallet clients - see {@link BumpFeeParams} & {@link EvmWalletClientParams}
     * @returns Promise resolving to transaction hash
     * @throws {Error} If order is not active or fees don't need bumping
     */
    async bumpFeeForOfframpOrder({
        orderId,
        walletClient,
        publicClient,
    }: BumpFeeParams & EvmWalletClientParams): Promise<Hash> {
        // check order status via viem should be Active/Accepted
        const orderDetails: OnchainOfframpOrderDetails = await this.fetchOfframpOrder(orderId);

        if (orderDetails.status !== 'Active') {
            throw new Error(`Offramp order needs to be Active for bumping fees`);
        }

        const [shouldFeesBeBumped, newFeeSat, error] = await this.getBumpFeeRequirement(
            orderDetails.token,
            orderDetails.satAmountLocked,
            orderDetails.satFeesMax,
            orderDetails.owner
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

        const { request } = await publicClient.simulateContract({
            address: offrampRegistryAddress,
            abi: offrampCaller,
            functionName: 'bumpFeeOfExistingOrder',
            args: [orderId, BigInt(newFeeSat)],
            account: walletClient.account,
        });

        const transactionHash = await walletClient.writeContract(request);
        await publicClient.waitForTransactionReceipt({ hash: transactionHash });

        return transactionHash;
    }

    /**
     * Unlocks funds from an unprocessed offramp order after the claim delay.
     *
     * @param params Parameters including orderId, receiver, and wallet clients - see {@link UnlockOrderParams} & {@link EvmWalletClientParams}
     * @returns Promise resolving to transaction hash
     * @throws {Error} If order cannot be unlocked yet or is already processed
     */
    async unlockOfframpOrder({
        orderId,
        receiver,
        walletClient,
        publicClient,
    }: UnlockOrderParams & EvmWalletClientParams): Promise<Hash> {
        // check order status via viem should be Active/Accepted
        const orderDetails: OnchainOfframpOrderDetails = await this.fetchOfframpOrder(orderId);

        // Processed and refunded order can't be unlocked
        if (orderDetails.status == 'Processed' || orderDetails.status == 'Refunded') {
            throw new Error(`Offramp order already processed / refunded`);
        }

        const offrampRegistryAddress: Address = await this.fetchOfframpRegistryAddress();

        // Active order can be unlocked and Accepted order can be unlocked after delay
        if (
            !(await this.canOrderBeUnlocked(orderDetails.status, orderDetails.orderTimestamp, offrampRegistryAddress))
        ) {
            throw new Error(`Offramp order is still within the 7-day claim delay and cannot be unlocked yet.`);
        }

        const { request } = await publicClient.simulateContract({
            address: offrampRegistryAddress,
            abi: offrampCaller,
            functionName: 'refundOrder',
            args: [orderId, receiver],
            account: walletClient.account,
        });

        const transactionHash = await walletClient.writeContract(request);
        await publicClient.waitForTransactionReceipt({ hash: transactionHash });

        return transactionHash;
    }

    /**
     * Retrieves all offramp orders for a specific user address.
     *
     * @param userAddress The user's EVM address
     * @returns Promise resolving to array of offramp orders with status info
     */
    async getOfframpOrders(userAddress: Address): Promise<OfframpOrder[]> {
        const response = await this.safeFetch(
            `${this.baseUrl}/offramp-orders/${userAddress}`,
            undefined,
            'Failed to fetch offramp orders'
        );
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
     * Determines if an offramp order requires a fee bump based on current market rates.
     *
     * @param token Token address
     * @param satAmountLocked Amount locked in satoshis
     * @param satFeesMax Current maximum fee in satoshis
     * @returns Promise resolving to [shouldBump, newFeeSat, error?]
     * @throws {Error} If quote fetch fails
     */
    private async getBumpFeeRequirement(
        token: Address,
        satAmountLocked: bigint,
        satFeesMax: bigint,
        userAddress: Address
    ): Promise<[boolean, bigint, string?]> {
        const decimals = getTokenDecimals(token);
        if (decimals === undefined) {
            throw new Error('Tokens with less than 8 decimals are not supported');
        }

        const amountInToken = satAmountLocked * BigInt(10 ** (decimals - 8));

        try {
            const offrampQuote = await this.fetchOfframpQuote(token, amountInToken, userAddress);
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
        if (status === 'Active' && Math.floor(Date.now() / 1000) - orderTimestamp >= 60) {
            return true;
        }
        if (status !== 'Accepted') {
            return false;
        }
        // check if Accepted order has passed claim delay
        const nowInSec = Math.floor(Date.now() / 1000);
        const publicClient = viemClient(this.chain);
        const claimDelay = await publicClient.readContract({
            address: offrampRegistryAddress,
            abi: claimDelayAbi,
            functionName: 'CLAIM_DELAY',
        });
        return orderTimestamp + Number(claimDelay) <= nowInSec;
    }

    /**
     * Fetches on-chain details for a specific offramp order.
     *
     * @param orderId The order ID
     * @returns Promise resolving to on-chain order details
     */
    private async fetchOfframpOrder(orderId: bigint): Promise<OnchainOfframpOrderDetails> {
        const offrampRegistryAddress: Address = await this.fetchOfframpRegistryAddress();
        const publicClient = viemClient(this.chain);

        const order = await publicClient.readContract({
            address: offrampRegistryAddress,
            abi: offrampCaller,
            functionName: 'getOrderDetails',
            args: [orderId],
        });

        return {
            orderId,
            token: order.token as Address,
            satAmountLocked: order.satAmountLocked,
            satFeesMax: order.satFeesMax,
            owner: order.owner as Address,
            solverOwner: order.solverOwner !== (zeroAddress as Address) ? (order.solverOwner as Address) : null,
            solverRecipient:
                order.solverRecipient !== (zeroAddress as Address) ? (order.solverRecipient as Address) : null,
            outputScript: order.outputScript,
            status: parseOrderStatus(Number(order.status)) as OnchainOfframpOrderDetails['status'],
            orderTimestamp: Number(order.timestamp),
        };
    }

    /**
     * Starts an onramp order to reserve liquidity (step 1 of 2).
     *
     * Creates a Bitcoin PSBT for the user to sign and broadcast.
     * Must be followed by {@link finalizeOnrampOrder} to complete the process.
     *
     * @param gatewayQuote Quote from {@link getOnrampQuote}
     * @param params Quote parameters - see {@link GetQuoteParams}
     * @returns Promise resolving to order details with PSBT
     * @throws {Error} If user address is invalid or PSBT creation fails
     */
    async startOnrampOrder(gatewayQuote: GatewayQuote, params: GetQuoteParams): Promise<GatewayStartOrder> {
        if (!params.toUserAddress || !isAddress(params.toUserAddress)) {
            throw new Error('Invalid user address');
        }

        const request: GatewayCreateOrderRequestPayload = {
            gatewayAddress: gatewayQuote.gatewayAddress,
            strategyAddress: gatewayQuote.strategyAddress,
            userAddress: params.toUserAddress,
            gatewayExtraData: undefined,
            strategyExtraData: params.message,
            satoshis: gatewayQuote.satoshis,
            campaignId: params.campaignId,
            orderDetails: convertOrderDetailsToRaw(gatewayQuote.orderDetails),
        };

        const response = await this.safeFetch(
            `${this.baseUrl}/v4/order`,
            {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    Accept: 'application/json',
                },
                body: JSON.stringify(request),
            },
            'Failed to create order'
        );

        if (!response.ok) {
            const errorData = await response.json().catch(() => null);
            const apiMessage = errorData?.message;
            const errorMessage = apiMessage || 'Failed to create order';
            throw new Error(errorMessage);
        }

        const data: GatewayCreateOrderResponse = await response.json();

        let psbtBase64: string = '';
        if (
            params.fromUserAddress &&
            typeof params.fromChain === 'string' &&
            params.fromChain.toLowerCase() === 'bitcoin'
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
     * Executes a complete quote (onramp or offramp) in a single transaction.
     *
     * For onramp: creates order, signs Bitcoin transaction, and finalizes
     * For offramp: approves tokens if needed and creates on-chain order
     *
     * @param params Parameters including quote and wallet clients - see {@link ExecuteQuoteParams} & {@link AllWalletClientParams}
     * @returns Promise resolving to transaction hash
     * @throws {Error} If required signers are missing or transaction fails
     */
    async executeQuote({
        quote: executeQuoteParams,
        walletClient,
        publicClient,
        btcSigner,
    }: { quote: ExecuteQuoteParams } & AllWalletClientParams): Promise<string> {
        const isOnrampQuoteParams = (args: ExecuteQuoteParams): args is OnrampExecuteQuoteParams => {
            return args.params.fromChain?.toString().toLowerCase() === 'bitcoin';
        };

        if (isOnrampQuoteParams(executeQuoteParams)) {
            const { onrampQuote, params } = executeQuoteParams;
            const quote = onrampQuote!;

            const esploraClient = new EsploraClient(this.chain.id === bob.id ? Network.mainnet : Network.signet);

            // TODO: refactor to construct the PSBT instead since it may fund from other inputs
            const availableBtcBalance = await esploraClient.getBalance(quote.bitcoinAddress);
            if (availableBtcBalance.total < BigInt(quote.satoshis)) {
                throw new Error(
                    `Insufficient BTC balance in address ${quote.bitcoinAddress}. Required: ${formatBtc(BigInt(quote.satoshis))}`
                );
            }

            const { uuid, psbtBase64, bitcoinAddress, satoshis, opReturnHash } = await this.startOnrampOrder(
                quote,
                params
            );

            if (!btcSigner) {
                throw new Error(`btcSigner is required for onramp order`);
            }

            let bitcoinTxHex: string;

            if (btcSigner.sendBitcoin) {
                bitcoinTxHex = await btcSigner.sendBitcoin({
                    from: params.fromUserAddress!,
                    to: bitcoinAddress,
                    value: formatBtc(BigInt(satoshis)),
                    opReturn: opReturnHash,
                    isSignet: this.isSignet,
                });
            } else if (btcSigner.signAllInputs) {
                bitcoinTxHex = await btcSigner.signAllInputs(psbtBase64!);
            } else {
                throw new Error('btcSigner must implement either sendBitcoin or signAllInputs method');
            }

            if (!bitcoinTxHex) throw new Error('Failed to get signed transaction');

            const txId = await this.finalizeOnrampOrder(uuid, bitcoinTxHex);

            return txId;
        } else {
            const { params, offrampQuote } = executeQuoteParams;
            const quote = offrampQuote!;

            const tokenAddress = getTokenAddress(this.chainId, params.fromToken.toLowerCase());
            const [offrampOrder, offrampRegistryAddress] = await Promise.all([
                this.createOfframpOrder(quote, params),
                this.fetchOfframpRegistryAddress(),
            ]);

            const accountAddress = walletClient.account?.address ?? (params.fromUserAddress as Address);

            // Check ETH balance for gas fees
            const ethBalance = await publicClient.getBalance({
                address: accountAddress,
            });

            // Estimate gas for both potential transactions
            const [approvalGasEstimate, createOrderGasEstimate] = await Promise.all([
                publicClient.estimateContractGas({
                    address: tokenAddress,
                    abi: erc20Abi,
                    functionName: 'approve',
                    args: [offrampRegistryAddress, maxUint256],
                    account: walletClient.account,
                }),
                publicClient.estimateContractGas({
                    address: offrampRegistryAddress,
                    abi: offrampOrder.offrampABI,
                    functionName: offrampOrder.offrampFunctionName,
                    args: offrampOrder.offrampArgs,
                    account: walletClient.account,
                }),
            ]);

            const feeValues = await publicClient.estimateFeesPerGas();
            const gasPrice = await publicClient.getGasPrice();
            const fee = feeValues.maxFeePerGas ?? gasPrice;
            const approvalGasCost = approvalGasEstimate * fee;
            const createOrderGasCost = createOrderGasEstimate * fee;

            const [allowance, decimals] = await publicClient.multicall({
                allowFailure: false,
                contracts: [
                    {
                        address: tokenAddress,
                        abi: erc20Abi,
                        functionName: 'allowance',
                        args: [params.fromUserAddress as Address, offrampRegistryAddress],
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
            const requiredAmount = BigInt(quote.amountLockInSat) * multiplier;
            const needsApproval = requiredAmount > allowance;

            // Calculate total gas cost needed
            const totalGasCost = needsApproval ? approvalGasCost + createOrderGasCost : createOrderGasCost;

            if (ethBalance < totalGasCost) {
                throw new Error(
                    `Insufficient ETH balance for gas fees. Required: ${totalGasCost} wei, Available: ${ethBalance} wei`
                );
            }

            if (needsApproval) {
                const { request } = await publicClient.simulateContract({
                    account: walletClient.account,
                    address: tokenAddress,
                    abi: erc20Abi,
                    functionName: 'approve',
                    args: [offrampRegistryAddress, maxUint256],
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
            account: params.sender,
        });

        const transactionHash = await walletClient.writeContract(request);

        await publicClient?.waitForTransactionReceipt({ hash: transactionHash });

        return transactionHash;
    }

    /**
     * Finalizes an onramp order by submitting the Bitcoin transaction (step 2 of 2).
     *
     * The transaction is validated and broadcast to the Bitcoin network automatically.
     * This completes the onramp process started with {@link startOnrampOrder}.
     *
     * @param uuid Order UUID from {@link startOnrampOrder}
     * @param bitcoinTxOrId Signed Bitcoin transaction hex or transaction ID
     * @returns Promise resolving to Bitcoin transaction ID
     * @throws {Error} If order update fails
     */
    async finalizeOnrampOrder(uuid: string, bitcoinTxOrId: string): Promise<string> {
        bitcoinTxOrId = stripHexPrefix(bitcoinTxOrId);

        let bitcoinTxHex: string;
        if (bitcoinTxOrId.length === 64) {
            const esploraClient = new EsploraClient(this.chain.id === bob.id ? Network.mainnet : Network.signet);
            bitcoinTxHex = await esploraClient.getTransactionHex(bitcoinTxOrId);
        } else {
            bitcoinTxHex = bitcoinTxOrId;
        }

        const response = await this.safeFetch(
            `${this.baseUrl}/order/${uuid}`,
            {
                method: 'PATCH',
                headers: {
                    'Content-Type': 'application/json',
                    Accept: 'application/json',
                },
                body: JSON.stringify({ bitcoinTx: bitcoinTxHex }),
            },
            'Failed to update order'
        );

        if (!response.ok) {
            const errorData = await response.json().catch(() => null);
            const apiMessage = errorData?.message;
            const errorMessage = apiMessage || `Failed to update order`;
            throw new Error(errorMessage);
        }

        return response.json();
    }

    /**
     * Retrieves all onramp orders for a specific user address.
     *
     * @param userAddress The user's EVM address
     * @returns Promise resolving to array of onramp orders with helper methods
     */
    async getOnrampOrders(userAddress: Address): Promise<OnrampOrder[]> {
        const chainId = this.chainId;
        const response = await this.safeFetch(
            `${this.baseUrl}/orders/${userAddress}`,
            undefined,
            'Failed to fetch onramp orders'
        );
        const orders = await response.json();
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

            const orderDetails = order.orderDetails
                ? convertOrderDetailsRawToOrderDetails(order.orderDetails)
                : undefined;

            return {
                ...order,
                orderDetails,
                gasRefill: order.satsToConvertToEth,
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
     * Retrieves all available strategies from the Gateway API.
     *
     * @deprecated Moving away from hardcoded strategies
     * @returns Promise resolving to array of strategy contracts
     */
    async getStrategies(): Promise<GatewayStrategyContract[]> {
        const response = await this.safeFetch(
            `${this.baseUrl}/strategies`,
            undefined,
            'Failed to fetch gateway strategies'
        );

        const chainName = this.chain.name;
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
                address: strategy.strategyAddress as Address,
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
     * Retrieves all supported token addresses from the Gateway API.
     *
     * @param includeStrategies Whether to include strategy output tokens
     * @returns Promise resolving to array of token addresses
     */
    async getTokenAddresses(includeStrategies: boolean = true): Promise<Address[]> {
        const response = await this.safeFetch(
            `${this.baseUrl}/tokens?includeStrategies=${includeStrategies}`,
            undefined,
            'Failed to fetch supported token addresses'
        );
        if (!response.ok) {
            const errorData = await response.json().catch(() => null);
            const apiMessage = errorData?.message;
            const errorMessage = apiMessage || 'Failed to fetch supported token addresses';
            throw new Error(errorMessage);
        }
        return response.json();
    }

    /**
     * Retrieves all supported tokens with detailed information.
     *
     * @param includeStrategies Whether to include strategy output tokens
     * @returns Promise resolving to array of token details
     */
    async getTokens(includeStrategies: boolean = true): Promise<Token[]> {
        // https://github.com/ethereum-optimism/ecosystem/blob/c6faa01455f9e846f31c0343a0be4c03cbeb2a6d/packages/op-app/src/hooks/useOPTokens.ts#L10
        const tokens = await this.getTokenAddresses(includeStrategies);
        return tokens.map((token) => ADDRESS_LOOKUP[this.chainId][token]).filter((token) => token !== undefined);
    }

    // TODO: should get price from the gateway API
    private async getPrices(): Promise<Map<string, number>> {
        const response = await this.safeFetch(
            'https://fusion-api.gobob.xyz/pricefeed',
            undefined,
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
     * @param includeStrategies Whether to include strategy output tokens
     * @returns Promise resolving to array of enriched token data
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
     * Retrieves all orders (both onramp and offramp) for a specific user address.
     *
     * @param userAddress The user's EVM address
     * @returns Promise resolving to array of typed orders
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

    private async safeFetch(url: URL | string, init: RequestInit | undefined, errorMessage: string) {
        const defaultInit = {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
                Accept: 'application/json',
            },
        };

        try {
            // NOTE: await inside try/catch to handle runtime error
            const response = await fetch(url, init || defaultInit);
            return response;
        } catch (err) {
            this.handleFetchError(err, errorMessage);
        }
    }

    private handleFetchError(err: unknown, message: string): never {
        if (err instanceof TypeError && err.message === 'Failed to fetch') {
            throw new Error(message);
        }
        throw err;
    }
}
