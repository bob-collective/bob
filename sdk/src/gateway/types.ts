import type { EsploraClient } from '../esplora';
import { Address } from 'viem';
import { offrampCaller, strategyCaller } from './abi';

type ChainSlug = string | number;
type TokenSymbol = string;

export type EvmAddress = string;

export enum Chain {
    // NOTE: we also support Bitcoin testnet
    BITCOIN = 'bitcoin',
    BOB = 'bob',
    BOB_SEPOLIA = 'bob-sepolia',
}

export enum ChainId {
    BOB = 60808,
    BOB_SEPOLIA = 808813,
    OPTIMISM = 10,
}

/**
 * Parameters required to construct a staking transaction.
 */
export type BuildStakeParams = {
    /** @description The address of the staking strategy contract */
    strategyAddress: Address;
    /** @description The token address being staked */
    token: Address;
    /** @description The sender's wallet address (must be an EVM address) */
    sender: Address;
    /** @description The receiver's wallet address (must be an EVM address) */
    receiver: Address;
    /** @description The amount of tokens to stake (in smallest unit, e.g., wei for ERC-20 tokens) */
    amount: bigint;
    /** @description Minimum acceptable output amount after slippage */
    amountOutMin: bigint;
};

/**
 * Parameters needed to execute a staking transaction on an EVM-based chain.
 *
 * ⚠️ **Important**: The token must be approved before calling this transaction.
 */
export type StakeTransactionParams = {
    /** @description The address of the staking strategy contract */
    strategyAddress: Address;
    /** @description The ABI used to interact with the staking contract */
    strategyABI: typeof strategyCaller;
    /** @description The name of the function being called on the contract */
    strategyFunctionName: string;
    /** @description Arguments required for the staking contract call */
    strategyArgs: [Address, bigint, Address, { amountOutMin: bigint }];
    /** @description The wallet address executing the transaction */
    account: Address;
    /** @description  Arguments required for the token approval transaction */
    erc20ApproveArgs: [Address, bigint];
};

/**
 * Designed to be compatible with the Superchain token list.
 * https://github.com/ethereum-optimism/ethereum-optimism.github.io
 */
export interface Token {
    chainId: number;
    address: string;
    name: string;
    symbol: string;
    decimals: number;
    logoURI: string;
}

export interface PointsIncentive {
    id: string;
    name: string;
}

export interface EnrichedToken extends Token {
    tvl: number;
    apyBase: number;
    apyReward: number;
    rewardTokens: Token[];
    points: PointsIncentive[];
}

/**
 * Designed to be compatible with the Swing SDK.
 * https://developers.swing.xyz/reference/sdk/get-a-quote
 */
export interface GatewayQuoteParams {
    /** @description Source chain slug or ID */
    fromChain: ChainSlug;
    /** @description Destination chain slug or ID */
    toChain: ChainSlug;
    /** @description Token symbol or address on source chain */
    fromToken: TokenSymbol;
    /** @description Token symbol or address on destination chain */
    toToken: TokenSymbol;
    /** @description Wallet address on source chain */
    fromUserAddress: string;
    /** @description Wallet address on destination chain */
    toUserAddress: string;
    /** @description Amount of tokens to send from the source chain */
    amount: number | string; // NOTE: modified from Swing

    /** @description Maximum slippage percentage between 0.01 and 0.03 (Default: 0.03) */
    maxSlippage?: number;

    /** @description Unique affiliate ID for tracking */
    affiliateId?: string;
    /** @description Optionally filter the type of routes returned */
    type?: 'swap' | 'deposit' | 'withdraw' | 'claim';
    /** @description The percentage of fee charged by partners in Basis Points (BPS) units. This will override the default fee rate configured via platform. 1 BPS = 0.01%. The maximum value is 1000 (which equals 10%). The minimum value is 1 (which equals 0.01%). */
    fee?: number;

    feeRate?: number;

    // NOTE: the following are new fields added by us
    /** @description Amount of satoshis to swap for ETH */
    gasRefill?: number;
    /** @description Wallet public key on source chain */
    fromUserPublicKey?: string;
    /** @description Strategy address */
    strategyAddress?: string;
    /** @description Campaign id for tracking */
    campaignId?: string;
    /** @description Users bitcoin Address */
    bitcoinUserAddress?: string;
}

/**
 * IntegrationType
 * @enum {string}
 */
type GatewayIntegrationType = 'bridge' | 'dex' | 'staking' | 'lending';

interface GatewayIntegration {
    type: GatewayIntegrationType;
    /** @example pell-network-wbtc */
    slug: string;
    /** @example Pell Network (wBTC) */
    name: string;
    /** Format: uri */
    logo: string;
    monetization: boolean;
}

type GatewayStrategyType = 'deposit' | 'withdraw' | 'claim' | 'router' | 'bridge';

interface GatewayToken {
    /** @example ETH */
    symbol: string;
    /** @example 0x000000000000000 */
    address: string;
    /** @example https://raw.githubusercontent.com/bob-collective/assets/master/blockchains/ethereum/assets/0x0000000000000000000000000000000000000000/logo.png */
    logo: string;
    /** @example 18 */
    decimals: number;
    /** @example ethereum */
    chain: string;
}

type GatewayChainType = 'evm' | 'ibc' | 'solana' | 'multiversx' | 'bitcoin' | 'ton' | 'tron';

interface GatewayChain {
    id: string;
    chainId: number;
    /** @example ethereum */
    slug: string;
    /** @example Ethereum */
    name: string;
    /** @example https://raw.githubusercontent.com/bob-collective/assets/master/blockchains/ethereum/info/logo.png */
    logo: string;
    type: GatewayChainType;
    /** @description Single chain swapping is supported for this chain. */
    singleChainSwap: boolean;
    /** @description Single chain staking is supported for this chain. */
    singleChainStaking: boolean;
    nativeToken?: GatewayToken;
    /**
     * @description URL template to transaction details.
     * @example https://etherscan.io/tx/{txHash}
     */
    txExplorer?: string;
    /**
     * @description URL template to token details.
     * @example https://etherscan.io/tokens/{address}
     */
    tokenExplorer?: string;
    /**
     * @description URL template to RPC endpoint.
     * @example https://eth-mainnet.g.alchemy.com/v2/xxx
     */
    rpcUrl?: string;
}

/**
 * Designed to be compatible with the Swing SDK.
 * https://developers.swing.xyz/reference/sdk/staking/contracts
 */
export interface GatewayStrategyContract {
    id: string;
    type: GatewayStrategyType;
    /**
     * @description Contract address
     * @example 0x...
     */
    address: string;
    /** @example deposit */
    method: string;
    /** @example bob */
    chain: GatewayChain;
    /** @example segment */
    integration: GatewayIntegration;

    inputToken: GatewayToken;
    /** @example seWBTC */
    outputToken: GatewayToken | null;
}

export type GatewayQuote = {
    /** @description The gateway address */
    gatewayAddress: EvmAddress;
    /** @description The base token address (e.g. wBTC or tBTC) */
    baseTokenAddress: EvmAddress;
    /** @description The minimum amount of Bitcoin to send */
    dustThreshold: number;
    /** @description The satoshi output amount */
    satoshis: number;
    /** @description The fee paid in satoshis (includes gas refill) */
    fee: number;
    /** @description The Bitcoin address to send BTC */
    bitcoinAddress: string;
    /** @description The number of confirmations required to confirm the Bitcoin tx */
    txProofDifficultyFactor: number;
    /** @description The optional strategy address */
    strategyAddress?: EvmAddress;
};

/** @dev Internal */
export type GatewayCreateOrderRequest = {
    gatewayAddress: EvmAddress;
    strategyAddress?: EvmAddress;
    satsToConvertToEth: number;
    userAddress: EvmAddress;
    gatewayExtraData?: string;
    strategyExtraData?: string;
    satoshis: number;
    campaignId?: string;
};

/** @dev Internal */
export type OffRampGatewayCreateQuoteResponse = {
    amountToLock: string;
    minimumFeesToPay: string;
    gateway: EvmAddress;
};

export interface GatewayOrderResponse {
    /** @description The gateway address */
    gatewayAddress: EvmAddress;
    /** @description The base token address (e.g. wBTC or tBTC) */
    baseTokenAddress: EvmAddress;
    /** @description The Bitcoin txid */
    txid: string;
    /** @description True when the order was executed on BOB */
    status: boolean;
    /** @description When the order was created */
    timestamp: number;
    // TODO: return converted fee
    /** @description The converted satoshi amount */
    tokens: string;
    /** @description The satoshi output amount */
    satoshis: number;
    /** @description The fee paid in satoshis (includes gas refill) */
    fee: number;
    /** @description The number of confirmations required to confirm the Bitcoin tx */
    txProofDifficultyFactor: number;
    /** @description The optional strategy address */
    strategyAddress?: EvmAddress;
    /** @description The gas refill in satoshis */
    satsToConvertToEth: number;
    /** @description The amount of ETH received */
    outputEthAmount?: string;
    /** @description The output token (from strategies) */
    outputTokenAddress?: EvmAddress;
    /** @description The output amount (from strategies) */
    outputTokenAmount?: string;
    /** @description The tx hash on the EVM chain */
    txHash?: string;
}

export type OrderStatusData = {
    confirmations: number;
};

export type OrderStatus =
    | {
          confirmed: false;
          pending?: never;
          success?: never;
          data: OrderStatusData;
      }
    | {
          confirmed?: never;
          pending: true;
          success?: never;
          data: OrderStatusData;
      }
    | {
          confirmed?: never;
          pending?: never;
          success: boolean;
          data: OrderStatusData;
      };

/** Order given by the Gateway API once the bitcoin tx is submitted */
export type GatewayOrder = Omit<
    GatewayOrderResponse & {
        /** @description The gas refill in satoshis */
        gasRefill: number;
    },
    'satsToConvertToEth'
> & {
    /** @description Get the actual token address received */
    getTokenAddress(): string | undefined;
    /** @description Get the actual token received */
    getToken(): Token | undefined;
    /** @description Get the actual amount received of the token */
    getTokenAmount(): string | number | undefined;
    /** @description Get the number of confirmations */
    getConfirmations(esploraClient: EsploraClient, latestHeight?: number): Promise<number>;
    /** @description Get the actual order status */
    getStatus(esploraClient: EsploraClient, latestHeight?: number): Promise<OrderStatus>;
};

export type GatewayTokensInfo = {
    /** @description The base token (e.g. wBTC or tBTC) */
    baseToken: Token;
    /** @description The output token (e.g. uniBTC or xSolvBTC) */
    outputToken?: Token;
};

/** @dev Internal */
export type GatewayCreateOrderResponse = {
    uuid: string;
    opReturnHash: string;
};

/** @dev The success type on create order */
export type GatewayStartOrder = GatewayCreateOrderResponse & {
    bitcoinAddress: string;
    satoshis: number;
    psbtBase64?: string;
};

export type OfframpOrderStatus = 'Active' | 'Accepted' | 'Processed' | 'Refunded';

/** @dev Offramp order quote returned by the quoting logic */
export interface OfframpQuote {
    /** @dev Amount to lock in satoshis */
    amountLockInSat: bigint;
    /** @dev Maximum fee paid in satoshis */
    feesInSat: bigint;
    /** @dev Deadline for order creation (unix timestamp) */
    deadline: bigint;
    /** @dev Address of the off-ramp registry handling the order */
    registryAddress: Address;
    /** @dev Fee rate used for calculating satoshi fee */
    feeRate: bigint;
    /** @dev Token address used for payment */
    token: Address;
}

/** @dev Offramp Available Liquidity */
export interface OfframpLiquidity {
    /** @dev Token address used for payment */
    token: Address;
    /** @dev Max token amount a *single* order can be served with (in token decimals) */
    maxOrderAmount: bigint;
    /** @dev Total liquidity across all solver addresses (in token decimals) */
    totalOfframpLiquidity: bigint;
}

/** @dev Params used for createOrder call on the off-ramp contract */
export type OfframpCreateOrderParams = {
    quote: OfframpQuote;
    offrampABI: typeof offrampCaller;
    offrampFunctionName: 'createOrder';
    offrampArgs: [
        {
            /** @dev Amount of sats to lock in the order */
            satAmountToLock: bigint;
            /** @dev Max sats to be paid as fees */
            satFeesMax: bigint;
            /** @dev Timestamp by which the order must be created */
            orderCreationDeadline: bigint;
            /** @dev Output script for Bitcoin settlement */
            outputScript: string;
            /** @dev Token to use for payment */
            token: Address;
            /** @dev EVM address of the user who can unlock the order or bump its fee */
            orderOwner: Address;
        },
    ];
};

/** @dev Params used to bump fee for an existing order */
export type OfframpBumpFeeParams = {
    offrampABI: typeof offrampCaller;
    offrampRegistryAddress: Address;
    offrampFunctionName: 'bumpFeeOfExistingOrder';
    /**
     * @param orderId The order ID to bump fee for
     * @param newFeeSat The new fee amount in satoshis
     */
    offrampArgs: [orderId: bigint, newFeeSat: bigint];
};

/** @dev Params used to unlock funds after order completion or refund */
export type OfframpUnlockFundsParams = {
    offrampABI: typeof offrampCaller;
    offrampRegistryAddress: Address;
    offrampFunctionName: 'unlockFunds';
    /**
     * @param orderId The order ID to bump fee for
     * @param receiver address to send unlocked funds to
     */
    offrampArgs: [orderId: bigint, receiver: Address];
};

/** @dev Final state of an offramp order used in UI/backend */
export type OfframpOrderDetails = {
    /** @dev Unique identifier for the off-ramp order */
    orderId: bigint;
    /** @dev The token address */
    token: Address;
    /** @dev The amount of sats that is locked */
    satAmountLocked: bigint;
    /** @dev The amount of fees to pay in sat */
    satFeesMax: bigint;
    /** @dev The current status of the order */
    status: OfframpOrderStatus;
    /** @dev The timestamp when the order was created or updated */
    orderTimestamp: bigint;
    /** @dev The user submit order transaction hash on the EVM chain */
    submitOrderEvmTx: string;
    /** @dev The transaction ID on the Bitcoin network */
    btcTx: string;
    /** @dev Indicates whether the fees for this order should be bumped based on current network conditions */
    shouldFeesBeBumped: boolean;
    /** @dev Indicates whether the user can unlock this order (typically if it's still active) */
    canOrderBeUnlocked: boolean;
};

/** @dev Internal, On-chain fetched state of an active/processed/refunded order */
export type OnchainOfframpOrderDetails = {
    /** @dev Unique identifier for the off-ramp order */
    orderId: bigint;
    /** @dev The token address used for payment */
    token: Address;
    /** @dev Amount locked in sats */
    satAmountLocked: bigint;
    /** @dev Max sats for fee */
    satFeesMax: bigint;
    /** @dev Address that created the order */
    sender: Address;
    /** @dev Optional receiver address for order payout */
    receiver: Address | null;
    /** @dev Optional owner address of the order */
    owner: Address | null;
    /** @dev Output script for Bitcoin tx */
    outputScript: string;
    /** @dev Order status */
    status: OfframpOrderStatus;
    /** @dev When the order was created (unix timestamp) */
    orderTimestamp: bigint;
};

/** @dev Internal */
export interface OfframpRawOrder {
    orderId: string;
    token: string;
    satAmountLocked: string;
    satFeesMax: string;
    status: string;
    orderTimestamp: string;
    btcTx: string | null;
    submitOrderEvmTx: string | null;
    shouldFeesBeBumped: boolean;
}

/** @dev Internal */
export interface GatewayStrategy {
    strategyAddress: string;
    strategyName: string;
    strategyType: 'staking' | 'lending';
    projectName: string;
    projectLogo?: string;
    inputTokenAddress: string;
    outputTokenAddress?: string;
}

/** @dev Internal */

export interface StrategyAssetState {
    address: Address | 'usd';
    totalUnderlying: bigint;
}

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
