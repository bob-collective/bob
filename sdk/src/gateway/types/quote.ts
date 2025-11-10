import { Address, Hex } from 'viem';
import { Optional } from './utils';
import { OnrampExecuteQuoteParams } from './onramp';
import { OfframpExecuteQuoteParams } from './offramp';
import { CrossChainSwapQuoteParams } from './crosschain-swap';

type ChainSlug = string | number;
type TokenSymbol = string;

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
    amount: number | string | bigint; // NOTE: modified from Swing

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
    /** @description Amount of ETH to get to pay for fees */
    gasRefill?: bigint;
    /** @description Wallet public key on source chain */
    fromUserPublicKey?: string;
    /** @description Strategy address */
    strategyAddress?: string;
    /** @description Campaign id for tracking */
    campaignId?: string;
    /** @description Affiliate-related fee */
    affiliateFeeSats?: bigint;
    /** @description The address of the affiliate who will receive the affiliate fee */
    affiliateFeeRecipient?: Address;

    /** @description Cross chain message - strategy data */
    message?: Hex;
    /** @description Gas limit for the cross chain message */
    destinationGasLimit?: number;
}

// eslint-disable-next-line @typescript-eslint/no-empty-object-type
export type GetQuoteParams<T = {}> = Optional<GatewayQuoteParams & T, 'fromUserAddress'>;

// eslint-disable-next-line @typescript-eslint/no-empty-object-type
export type BaseExecuteQuoteParams<T = {}> = {
    finalOutputSats: number;
    finalFeeSats: number;
    params: GetQuoteParams;
} & T;

// eslint-disable-next-line @typescript-eslint/no-empty-object-type
export type ExecuteQuoteParams<T = {}> =
    | OnrampExecuteQuoteParams<T>
    | OfframpExecuteQuoteParams<T>
    | CrossChainSwapQuoteParams<T>;
