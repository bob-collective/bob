import { Address, Hex } from 'viem';
import { Optional } from './utils';

/**
 * One affiliate-fee split: an EVM recipient and the basis-point cut they
 * receive from the swap output. 1 bps = 0.01%, so `bps: 50` is 0.50%.
 */
export interface Affiliate {
    /** EVM address that receives the affiliate fee. */
    address: Address;
    /** Basis points to route to `address`. Must be > 0. */
    bps: number;
}

/**
 * Designed to be compatible with the Swing SDK.
 * https://developers.swing.xyz/reference/sdk/get-a-quote
 */
export interface GatewayQuoteParams {
    /** @description Source chain slug or ID */
    fromChain: string;
    /** @description Destination chain slug or ID */
    toChain: string;
    /** @description Token address on source chain (e.g. '0x0000000000000000000000000000000000000000'). Use getRoutes() to find supported token addresses. */
    fromToken: string;
    /** @description Token address on destination chain (e.g. '0x0000000000000000000000000000000000000000'). Use getRoutes() to find supported token addresses. */
    toToken: string;
    /** @description Wallet address on source chain */
    fromUserAddress: string;
    /** @description Wallet address on destination chain */
    toUserAddress: string;
    /** @description Amount of tokens to send from the source chain */
    amount: number | string | bigint; // NOTE: modified from Swing
    /** @description Maximum slippage percentage in bps */
    maxSlippage?: number;
    /**
     * Affiliate fee recipients. One or more `{ address, bps }` pairs; each
     * `bps` must be > 0. The total cap is enforced by the gateway — routes
     * that don't support affiliate fees return
     * `AFFILIATE_FEES_NOT_SUPPORTED_FOR_ROUTE`.
     */
    affiliates?: Affiliate[];

    // NOTE: the following are new fields added by us
    /** @description Amount of ETH to get to pay for fees */
    gasRefill?: bigint;
    /** @description Strategy address target */
    strategyAddress?: string;
    /** @description Cross chain message - strategy data */
    strategyMessage?: Hex;
    /** @description Owner address */
    ownerAddress: string;
}

export type GetQuoteParams = Optional<GatewayQuoteParams, 'fromUserAddress'>;
