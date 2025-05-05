import { ethers, AbiCoder } from 'ethers';
import {
    GatewayQuoteParams,
    GatewayQuote,
    Chain,
    ChainId,
    Token,
    GatewayStrategyContract,
    GatewayCreateOrderRequest,
    GatewayCreateOrderResponse,
    GatewayOrder,
    GatewayOrderResponse,
    GatewayStartOrder,
    GatewayStrategy,
    EvmAddress,
    GatewayTokensInfo,
    OrderStatus,
    StakeTransactionParams,
    BuildStakeParams,
    OfframpOrderDetails,
    OfframpQuote,
    OfframpCreateOrderParams,
    OfframpBumpFeeParams,
    OnchainOfframpOrderDetails,
    OfframpUnlockFundsParams,
    OfframpRawOrder,
    OfframpOrderStatus,
    EnrichedToken,
    OfframpLiquidity,
} from './types';
import { createBitcoinPsbt, getAddressInfo } from '../wallet';
import { SYMBOL_LOOKUP, ADDRESS_LOOKUP, getTokenDecimals } from './tokens';
import { AddressType, Network } from 'bitcoin-address-validation';
import { EsploraClient } from '../esplora';
import { claimDelayAbi, offrampCaller, strategyCaller } from './abi';
import { isAddress, Address, isAddressEqual, createPublicClient, http, PublicClient } from 'viem';
import * as bitcoin from 'bitcoinjs-lib';
import { bob, bobSepolia } from 'viem/chains';
import StrategyClient from './strategy';
import { bigIntToFloatingNumber } from '../utils';

type Optional<T, K extends keyof T> = Omit<T, K> & Partial<T>;

/**
 * Base url for the mainnet Gateway API.
 * @default "https://gateway-api-mainnet.gobob.xyz"
 */
export const MAINNET_GATEWAY_BASE_URL = 'https://gateway-api-mainnet.gobob.xyz';

/**
 * Base url for the testnet Gateway API.
 * @default "https://gateway-api-testnet.gobob.xyz"
 */
export const TESTNET_GATEWAY_BASE_URL = 'https://gateway-api-testnet.gobob.xyz';

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
    constructor(chainName: 'mainnet' | 'testnet' | 'signet' | 'bob', options?: { rpcUrl?: string }) {
        switch (chainName) {
            case 'mainnet':
            case Chain.BOB:
                this.chain = Chain.BOB;
                this.baseUrl = MAINNET_GATEWAY_BASE_URL;
                this.strategy = new StrategyClient(bob, options?.rpcUrl);
                break;
            case 'testnet':
                this.chain = Chain.BOB_SEPOLIA;
                this.baseUrl = TESTNET_GATEWAY_BASE_URL;
                this.strategy = new StrategyClient(bobSepolia, options?.rpcUrl);
                break;
            case 'signet':
                this.chain = Chain.BOB_SEPOLIA; // Same chain as testnet
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
     * Get a quote from the Gateway API for swapping or staking BTC.
     *
     * @param params The parameters for the quote.
     */
    async getQuote(
        params: Optional<GatewayQuoteParams, 'amount' | 'fromChain' | 'fromToken' | 'fromUserAddress' | 'toUserAddress'>
    ): Promise<GatewayQuote & GatewayTokensInfo> {
        const isMainnet =
            params.toChain === ChainId.BOB ||
            (typeof params.toChain === 'string' && params.toChain.toLowerCase() === Chain.BOB);
        const isTestnet =
            params.toChain === ChainId.BOB_SEPOLIA ||
            (typeof params.toChain === 'string' && params.toChain.toLowerCase() === Chain.BOB_SEPOLIA);

        const isInvalidNetwork = !isMainnet && !isTestnet;
        const isMismatchMainnet = isMainnet && this.chain !== Chain.BOB;
        const isMismatchTestnet = isTestnet && this.chain !== Chain.BOB_SEPOLIA;

        // TODO: switch URL if `toChain` is different chain?
        if (isInvalidNetwork || isMismatchMainnet || isMismatchTestnet) {
            throw new Error('Invalid output chain');
        }

        let outputTokenAddress = '';
        let strategyAddress: string | undefined;

        const toToken = params.toToken.toLowerCase();
        if (params.strategyAddress?.startsWith('0x')) {
            strategyAddress = params.strategyAddress;
        }

        const chainId = this.chainId;
        if (toToken.startsWith('0x')) {
            outputTokenAddress = toToken;
        } else if (isMainnet && SYMBOL_LOOKUP[chainId][toToken]) {
            outputTokenAddress = SYMBOL_LOOKUP[chainId][toToken].address;
        } else if (isTestnet && SYMBOL_LOOKUP[chainId][toToken]) {
            outputTokenAddress = SYMBOL_LOOKUP[chainId][toToken].address;
        } else {
            throw new Error('Unknown output token');
        }

        const url = new URL(`${this.baseUrl}/quote/${outputTokenAddress}`);
        if (strategyAddress) {
            url.searchParams.append('strategy', `${strategyAddress}`);
        }
        const atomicAmount = params.amount;
        if (atomicAmount) {
            url.searchParams.append('satoshis', `${atomicAmount}`);
        }

        const response = await fetch(url, {
            headers: {
                'Content-Type': 'application/json',
                Accept: 'application/json',
            },
        });

        const quote: GatewayQuote = await response.json();
        return {
            ...quote,
            fee: quote.fee + (params.gasRefill || 0),
            baseToken: ADDRESS_LOOKUP[chainId][quote.baseTokenAddress],
            outputToken: quote.strategyAddress ? ADDRESS_LOOKUP[chainId][outputTokenAddress] : undefined,
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
        return JSON.parse(await response.text()) as Address;
    }

    /**
     * Fetches an offramp liquidity for the given token.
     *
     * @param token ERC20 token address.
     * @returns Offramp liquidity details.
     */
    async fetchOfframpLiquidity(token: string): Promise<OfframpLiquidity> {
        let outputTokenAddress = '';
        const toToken = token.toLowerCase();

        if (isAddress(toToken)) {
            outputTokenAddress = toToken;
        } else if (SYMBOL_LOOKUP[this.chainId][toToken]) {
            outputTokenAddress = SYMBOL_LOOKUP[this.chainId][toToken].address;
        } else {
            throw new Error('Unknown output token');
        }

        const response = await fetch(`${this.baseUrl}/offramp-liquidity/${outputTokenAddress}`, {
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
     * @param token ERC20 token address.
     * @param amountInToken Amount specified in token decimals.
     * @returns Offramp quote details.
     */
    async fetchOfframpQuote(token: string, amountInToken: bigint): Promise<OfframpQuote> {
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
            throw new Error(`Offramp API Error: ${errorMessage}`);
        }

        const rawQuote = await response.json();
        const currentUnixTimeInSec = Math.floor(Date.now() / 1000);
        const deadline = currentUnixTimeInSec + ORDER_DEADLINE_IN_SECONDS;

        return {
            amountLockInSat: BigInt(rawQuote.amountLockInSat.toString()),
            feesInSat: BigInt(rawQuote.feesInSat.toString()),
            feeRate: BigInt(rawQuote.feeRate.toString()),
            deadline: BigInt(deadline.toString()),
            registryAddress: rawQuote.registryAddress as Address,
            token: token as Address,
        };
    }

    /**
     * Prepares calldata to create an offramp order for BTC.
     *
     * @param params Offramp order params, excluding auto-filled fields.
     * @returns Parameters for onchain `createOrder` call.
     */
    async createOfframpOrder(
        params: Optional<
            GatewayQuoteParams,
            | 'toChain'
            | 'toToken'
            | 'toUserAddress'
            | 'affiliateId'
            | 'type'
            | 'fee'
            | 'feeRate'
            | 'gasRefill'
            | 'strategyAddress'
            | 'campaignId'
            | 'fromToken'
            | 'fromUserAddress'
            | 'maxSlippage'
            | 'fromChain'
        >
    ): Promise<OfframpCreateOrderParams> {
        const isMainnet =
            this.chainId === ChainId.BOB || (typeof this.chainId === 'string' && this.chainId === Chain.BOB);

        const isTestnet =
            this.chainId === ChainId.BOB_SEPOLIA ||
            (typeof this.chainId === 'string' && this.chainId === Chain.BOB_SEPOLIA);

        let outputTokenAddress = '';
        const toToken = params.toToken.toLowerCase();

        if (toToken.startsWith('0x')) {
            outputTokenAddress = toToken;
        } else if (isMainnet && SYMBOL_LOOKUP[this.chainId][toToken]) {
            outputTokenAddress = SYMBOL_LOOKUP[this.chainId][toToken].address;
        } else if (isTestnet && SYMBOL_LOOKUP[this.chainId][toToken]) {
            outputTokenAddress = SYMBOL_LOOKUP[this.chainId][toToken].address;
        } else {
            throw new Error('Unknown output token');
        }

        const offrampQuote: OfframpQuote = await this.fetchOfframpQuote(outputTokenAddress, BigInt(params.amount));

        // get btc script pub key
        let bitcoinNetwork = bitcoin.networks.regtest;
        if (this.chain == Chain.BOB) {
            bitcoinNetwork = bitcoin.networks.bitcoin;
        } else if (this.chain == Chain.BOB_SEPOLIA) {
            bitcoinNetwork = bitcoin.networks.testnet;
        }

        if (getAddressInfo(params.bitcoinUserAddress, this.isSignet).type === AddressType.p2tr) {
            throw new Error('Only following bitcoin address types are supported P2PKH, P2WPKH, P2SH or P2WSH.');
        }
        const receiverAddress = toHexScriptPubKey(params.bitcoinUserAddress, bitcoinNetwork);

        return {
            quote: offrampQuote,
            offrampABI: offrampCaller['createOrder'],
            offrampFunctionName: 'createOrder' as const,
            offrampArgs: [
                {
                    satAmountToLock: offrampQuote.amountLockInSat,
                    satFeesMax: offrampQuote.feesInSat,
                    orderCreationDeadline: offrampQuote.deadline,
                    outputScript: receiverAddress,
                    token: offrampQuote.token,
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
            offrampABI: offrampCaller['bumpFeeOfExistingOrder'],
            offrampRegistryAddress: offrampRegistryAddress,
            offrampFunctionName: 'bumpFeeOfExistingOrder' as const,
            offrampArgs: [orderId, newFeeSat],
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
            offrampABI: offrampCaller['unlockFunds'],
            offrampRegistryAddress: offrampRegistryAddress,
            offrampFunctionName: 'unlockFunds',
            offrampArgs: [orderId, receiver],
        };
    }

    /**
     * Returns all offramp orders for this account.
     *
     * @param userAddress The user's EVM address.
     * @returns {Promise<OfframpOrderDetails[]>} The array of account orders.
     */
    async getOfframpOrders(userAddress: EvmAddress): Promise<OfframpOrderDetails[]> {
        const response = await this.fetchGet(`${this.baseUrl}/offramp-orders/${userAddress}`);
        const rawOrders: OfframpRawOrder[] = await response.json();
        const offrampRegistryAddress: Address = await this.fetchOfframpRegistryAddress();

        return Promise.all(
            rawOrders.map(async (order) => {
                const status = order.status as OfframpOrderStatus;
                const canOrderBeUnlocked = await this.canOrderBeUnlocked(
                    status,
                    BigInt(order.orderTimestamp),
                    offrampRegistryAddress
                );

                return {
                    ...order,
                    status,
                    token: order.token as Address,
                    orderId: BigInt(order.orderId.toString()),
                    satAmountLocked: BigInt(order.satAmountLocked.toString()),
                    satFeesMax: BigInt(order.satFeesMax.toString()),
                    orderTimestamp: BigInt(order.orderTimestamp.toString()),
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
            return [false, 0n, 'Tokens with less than 8 decimals are not supported'];
        }

        const amountInToken = satAmountLocked * BigInt(10 ** (decimals - 8));

        let offrampQuote: OfframpQuote;
        try {
            offrampQuote = await this.fetchOfframpQuote(token.toString(), amountInToken);
        } catch (err) {
            // Return false and 0n with an error message if fetching the quote fails
            return [false, 0n, `Error fetching offramp quote: ${err.message || err}`];
        }

        const shouldBump = satFeesMax < offrampQuote.feesInSat;
        return [shouldBump, offrampQuote.feesInSat];
    }

    async canOrderBeUnlocked(
        status: OfframpOrderStatus,
        orderTimestamp: bigint,
        offrampRegistryAddress: Address
    ): Promise<boolean> {
        if (status === 'Active') {
            return true;
        }
        if (status !== 'Accepted') {
            return false;
        }
        // check if Accepted order has passed claim delay
        const nowInSec = BigInt(Math.floor(Date.now() / 1000));
        const publicClient = viemClient(this.chain) as PublicClient;
        const claimDelay: bigint = await publicClient.readContract({
            address: offrampRegistryAddress,
            abi: claimDelayAbi,
            functionName: 'CLAIM_DELAY',
        });
        return orderTimestamp + claimDelay <= nowInSec;
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
            abi: [offrampCaller['getOfframpOrder']],
            functionName: 'getOfframpOrder',
            args: [orderId],
        });

        return {
            orderId,
            token: order.token as Address,
            satAmountLocked: BigInt(order.satAmountLocked),
            satFeesMax: BigInt(order.satFeesMax),
            sender: order.sender as Address,
            receiver: order.receiver !== (ethers.ZeroAddress as Address) ? (order.receiver as Address) : null,
            owner: order.owner !== (ethers.ZeroAddress as Address) ? (order.owner as Address) : null,
            outputScript: order.outputScript,
            status: parseOrderStatus(Number(order.status)) as OnchainOfframpOrderDetails['status'],
            orderTimestamp: BigInt(order.timestamp),
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
    async startOrder(
        gatewayQuote: GatewayQuote,
        params: Optional<GatewayQuoteParams, 'toToken' | 'amount'>
    ): Promise<GatewayStartOrder> {
        if (!params.toUserAddress.startsWith('0x') || !ethers.isAddress(params.toUserAddress)) {
            throw new Error('Invalid user address');
        }

        const abiCoder = new AbiCoder();
        const request: GatewayCreateOrderRequest = {
            gatewayAddress: gatewayQuote.gatewayAddress,
            strategyAddress: gatewayQuote.strategyAddress,
            satsToConvertToEth: params.gasRefill || 0,
            userAddress: params.toUserAddress,
            gatewayExtraData: undefined,
            // TODO: update strategy data
            strategyExtraData: abiCoder.encode(['uint256'], [0]),
            satoshis: gatewayQuote.satoshis,
            campaignId: params.campaignId,
        };

        const response = await fetch(`${this.baseUrl}/order`, {
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
        // NOTE: could remove this check but good for sanity
        if (data.opReturnHash != calculateOpReturnHash(request)) {
            throw new Error('Invalid OP_RETURN hash');
        }

        let psbtBase64: string;
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
        }

        return {
            uuid: data.uuid,
            opReturnHash: data.opReturnHash,
            bitcoinAddress: gatewayQuote.bitcoinAddress,
            satoshis: gatewayQuote.satoshis,
            psbtBase64,
        };
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
            const esploraClient = new EsploraClient(this.chain === Chain.BOB ? Network.mainnet : Network.testnet);
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

        return await response.json();
    }

    /**
     * Returns all pending and completed orders for this account.
     *
     * @param userAddress The user's EVM address.
     * @returns {Promise<GatewayOrder[]>} The array of account orders.
     */
    async getOrders(userAddress: EvmAddress): Promise<(GatewayOrder & GatewayTokensInfo)[]> {
        const chainId = this.chainId;
        const response = await this.fetchGet(`${this.baseUrl}/orders/${userAddress}`);
        const orders: GatewayOrderResponse[] = await response.json();
        return orders.map((order) => {
            function getFinal<L, R>(base?: L, output?: R) {
                return order.status
                    ? order.strategyAddress
                        ? output
                            ? output // success
                            : base // failed
                        : base // success
                    : order.strategyAddress // pending
                      ? output
                      : base;
            }
            const getTokenAddress = (): string | undefined => {
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
                outputToken: ADDRESS_LOOKUP[chainId][order.outputTokenAddress],
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
     * @returns {Promise<EvmAddress[]>} The array of token addresses.
     */
    async getTokenAddresses(includeStrategies: boolean = true): Promise<EvmAddress[]> {
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
     * @param {BuildStakeParams} stakeParams - The parameters required for staking.
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
     * const params: BuildStakeParams = {
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
     *     args: result.erc20ApproveArgs,
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
    async buildStake(stakeParams: BuildStakeParams): Promise<StakeTransactionParams> {
        const strategies = await this.getStrategies();

        // Convert addresses to lowercase and check if strategy exists
        const strategy = strategies.find((s) => isAddressEqual(s.address as Address, stakeParams.strategyAddress));
        if (!strategy) {
            throw new Error(`Strategy with address ${stakeParams.strategyAddress} not found.`);
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
            erc20ApproveArgs: [stakeParams.strategyAddress, stakeParams.amount],
            account: stakeParams.sender,
        };
    }

    private async fetchGet(url: string) {
        return await fetch(url, {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
                Accept: 'application/json',
            },
        });
    }
}

/**
 * Should compute the same OP_RETURN hash as the Gateway API and smart contracts.
 * This is used for data integrity checking.
 */
function calculateOpReturnHash(req: GatewayCreateOrderRequest) {
    const abiCoder = new AbiCoder();
    return ethers.keccak256(
        abiCoder.encode(
            ['address', 'address', 'uint256', 'address', 'bytes', 'bytes'],
            [
                req.gatewayAddress,
                req.strategyAddress || ethers.ZeroAddress,
                req.satsToConvertToEth,
                req.userAddress,
                req.gatewayExtraData || '0x',
                req.strategyExtraData || '0x',
            ]
        )
    );
}

export function toHexScriptPubKey(userAddress: string, network: bitcoin.Network): string {
    const address = bitcoin.address.toOutputScript(userAddress, network);
    const buffer = Buffer.concat([Buffer.from([address.length]), address]);
    return '0x' + buffer.toString('hex'); // Convert buffer to hex string
}

function isHexPrefixed(str: string): boolean {
    return str.slice(0, 2) === '0x';
}

function stripHexPrefix(str: string): string {
    return isHexPrefixed(str) ? str.slice(2) : str;
}

function slugify(str: string): string {
    return str
        .toLowerCase()
        .replace(/ /g, '-')
        .replace(/[^\w-]+/g, '');
}

function parseOrderStatus(value: number): OfframpOrderStatus {
    switch (value) {
        case 0:
            return 'Active';
        case 1:
            return 'Accepted';
        case 2:
            return 'Processed';
        case 3:
            return 'Refunded';
        default:
            throw new Error(`Invalid order status: ${value}`);
    }
}

export function viemClient(chain) {
    const chainIs = chain === Chain.BOB ? bob : bobSepolia;
    return createPublicClient({ chain: chainIs, transport: http() });
}
