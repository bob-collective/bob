import { Hex } from 'viem';
import { Optional } from './utils';

/**
 * Designed to be compatible with the Swing SDK.
 * https://developers.swing.xyz/reference/sdk/get-a-quote
 */
export interface GatewayQuoteParams {
    /** @description Source chain slug or ID */
    fromChain: string;
    /** @description Destination chain slug or ID */
    toChain: string;
    /** @description Token symbol or address on source chain */
    fromToken: string;
    /** @description Token symbol or address on destination chain */
    toToken: string;
    /** @description Wallet address on source chain */
    fromUserAddress: string;
    /** @description Wallet address on destination chain */
    toUserAddress: string;
    /** @description Amount of tokens to send from the source chain */
    amount: number | string | bigint; // NOTE: modified from Swing
    /** @description Maximum slippage percentage in bps */
    maxSlippage?: number;
    /** @description Unique affiliate ID for tracking */
    affiliateId?: string;

    // NOTE: the following are new fields added by us
    /** @description Amount of ETH to get to pay for fees */
    gasRefill?: bigint;
    /** @description Wallet public key on source chain */
    fromUserPublicKey?: string;
    /** @description Strategy address target */
    strategyAddress?: string;
    /** @description Cross chain message - strategy data */
    strategyMessage?: Hex;
}

export type GetQuoteParams = Optional<GatewayQuoteParams, 'fromUserAddress'>;
