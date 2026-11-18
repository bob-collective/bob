import nock from 'nock';
import { Account, PublicClient, Transport, Chain as ViemChain, WalletClient } from 'viem';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { GatewaySDK } from '../src/gateway';
import { MAINNET_GATEWAY_BASE_URL } from '../src/gateway/client';
import { GatewayQuoteV4OneOf, GatewayQuoteV4OneOf1 } from '../src/gateway/generated-client';

const BTC = '0x0000000000000000000000000000000000000000';
const BTC_SENDER = 'bc1qafk4yhqvj4wep57m62dgrmutldusqde8adh20d';
const SOLANA_RECIPIENT = '7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU';
const USDC_MINT = 'EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v';
const WBTC_OFT_ADDRESS = '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c';

const amount = (chain: string, address: string, value: string) => ({ chain, address, amount: value });

/** A BTC -> Solana USDC onramp quote, as `get-quote` returns it. */
const solanaOnrampQuote = {
    onramp: {
        dstChain: 'solana',
        dstToken: USDC_MINT,
        token: USDC_MINT,
        recipient: SOLANA_RECIPIENT,
        sender: BTC_SENDER,
        signedQuoteData: 'signed-quote-data',
        slippage: '300',
        inputAmount: amount('bitcoin', BTC, '100000'),
        outputAmount: amount('solana', USDC_MINT, '95000000'),
        fees: amount('bitcoin', BTC, '100'),
        executionFees: amount('bitcoin', BTC, '100'),
        feeBreakdown: {},
    },
};

function quoteToSolana(sdk: GatewaySDK) {
    return sdk.getQuote({
        fromChain: 'bitcoin',
        fromToken: BTC,
        fromUserAddress: BTC_SENDER,
        refundAddress: BTC_SENDER,
        toChain: 'solana',
        toToken: USDC_MINT,
        toUserAddress: SOLANA_RECIPIENT,
        amount: 100000,
    });
}

describe('onramp to Solana without EVM clients', () => {
    afterEach(() => {
        nock.cleanAll();
    });

    it('quotes and executes a BTC -> Solana onramp with only a Bitcoin signer', async () => {
        let quoteQuery: Record<string, string> = {};
        let orderBody: { onramp: { recipient: string; dstChain: string } } | undefined;
        nock(MAINNET_GATEWAY_BASE_URL)
            .get('/v4/get-quote')
            .query((query) => {
                quoteQuery = query as Record<string, string>;
                return true;
            })
            .reply(200, solanaOnrampQuote);
        nock(MAINNET_GATEWAY_BASE_URL)
            .post('/v4/create-order', (body) => {
                orderBody = body;
                return true;
            })
            .reply(201, { onramp: { order_id: 'onramp-solana-1', address: 'bc1qdeposit' } });
        nock(MAINNET_GATEWAY_BASE_URL)
            .patch('/v4/register-tx')
            .reply(200, { onramp: { txid: 'btc-txid' } });

        const sdk = new GatewaySDK();
        const quote = await quoteToSolana(sdk);
        const sendBitcoin = vi.fn().mockResolvedValue('0200000001');

        const result = await sdk.executeQuote({ quote, btcSigner: { sendBitcoin } });

        expect(quoteQuery).toMatchObject({ dstChain: 'solana', dstToken: USDC_MINT, recipient: SOLANA_RECIPIENT });
        expect(orderBody?.onramp).toMatchObject({ dstChain: 'solana', recipient: SOLANA_RECIPIENT });
        expect(sendBitcoin).toHaveBeenCalledWith(expect.objectContaining({ from: BTC_SENDER, to: 'bc1qdeposit' }));
        expect(result.tx).toBe('btc-txid');
    });

    it('returns the order of a walletless BTC -> Solana onramp', async () => {
        nock(MAINNET_GATEWAY_BASE_URL).get('/v4/get-quote').query(true).reply(200, solanaOnrampQuote);
        nock(MAINNET_GATEWAY_BASE_URL)
            .post('/v4/create-order')
            .reply(201, { onramp: { order_id: 'onramp-solana-2', address: 'bc1qdeposit' } });

        const sdk = new GatewaySDK();
        const result = await sdk.executeQuote({ quote: await quoteToSolana(sdk) });

        expect(result.tx).toBeUndefined();
        expect(result.order).toEqual({ onramp: expect.objectContaining({ orderId: 'onramp-solana-2' }) });
    });
});

describe('offramp and token swap still need EVM clients', () => {
    afterEach(() => {
        nock.cleanAll();
    });

    const walletClient = {
        account: { address: '0x1234567890123456789012345678901234567890' },
    } as unknown as WalletClient<Transport, ViemChain, Account>;

    const offrampQuote: GatewayQuoteV4OneOf = {
        offramp: {
            srcChain: 'bob',
            tokenAddress: WBTC_OFT_ADDRESS,
            txTo: '0x1234567890123456789012345678901234567890',
            recipient: BTC_SENDER,
            slippage: 300,
            inputAmount: amount('bob', WBTC_OFT_ADDRESS, '1000'),
            outputAmount: amount('bitcoin', BTC, '990'),
            totalFeeUsd: '1',
            feeBreakdown: {
                protocolFee: amount('bob', BTC, '1'),
                affiliateFee: amount('bob', BTC, '0'),
                inclusionFee: amount('bob', BTC, '1'),
                solverFee: amount('bob', BTC, '1'),
                fastestFeeRate: '6',
            },
        },
    };

    const tokenSwapQuote: GatewayQuoteV4OneOf1 = {
        tokenSwap: {
            srcChain: 'ethereum',
            dstChain: 'solana',
            estimatedTimeInSecs: 60,
            fees: amount('ethereum', BTC, '0'),
            inputAmount: amount('ethereum', '0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599', '100000'),
            outputAmount: amount('solana', USDC_MINT, '95000000'),
            recipient: SOLANA_RECIPIENT,
            slippage: 100,
            txTo: WBTC_OFT_ADDRESS,
        },
    };

    it.each([
        ['offramp', offrampQuote, {}, 'walletClient is required for offramp order'],
        ['offramp', offrampQuote, { walletClient }, 'publicClient is required for offramp order'],
        ['token swap', tokenSwapQuote, {}, 'walletClient is required for token swap order'],
        ['token swap', tokenSwapQuote, { walletClient }, 'publicClient is required for token swap order'],
    ])('refuses a %s with a missing client before creating the order', async (_, quote, clients, message) => {
        const scope = nock(MAINNET_GATEWAY_BASE_URL).post('/v4/create-order').reply(201, {});

        await expect(new GatewaySDK().executeQuote({ quote, ...clients })).rejects.toThrow(message);
        expect(scope.isDone()).toBe(false);
    });

    it('still accepts both clients for a token swap', async () => {
        // Only the guard is under test: the swap stops at the allowance read, after the guard passed.
        const readContract = vi.fn().mockRejectedValue(new Error('allowance read reached'));

        await expect(
            new GatewaySDK().executeQuote({
                quote: tokenSwapQuote,
                walletClient,
                publicClient: { readContract } as unknown as PublicClient<Transport>,
            })
        ).rejects.toThrow('allowance read reached');
    });
});
