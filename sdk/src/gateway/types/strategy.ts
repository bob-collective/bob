import { Address } from 'viem';

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
    address: Address;
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

/**
 * Parameters required to construct a legacy strategy call.
 */
export type StrategyParams = {
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
