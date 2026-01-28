import { Address } from 'viem';

/**
 * Parameters required to construct a legacy strategy call.
 */
export type StrategyParams = {
    /** @description The address of the staking strategy contract */
    strategyAddress: Address;
    /** @description The token address being staked */
    token: Address;
    /** @description The receiver's wallet address (must be an EVM address) */
    receiver: Address;
    /** @description The amount of tokens to stake (in smallest unit, e.g., wei for ERC-20 tokens) */
    amount: bigint;
    /** @description Minimum acceptable output amount after slippage */
    amountOutMin: bigint;
};
