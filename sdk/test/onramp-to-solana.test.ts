import nock from 'nock';
import { afterEach, expect, it, vi } from 'vitest';
import { GatewaySDK } from '../src/gateway';
import { MAINNET_GATEWAY_BASE_URL } from '../src/gateway/client';

const BTC = '0x0000000000000000000000000000000000000000';
const BTC_SENDER = 'bc1qafk4yhqvj4wep57m62dgrmutldusqde8adh20d';
const SOLANA_RECIPIENT = '7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU';
const USDC_MINT = 'EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v';

const amount = (chain: string, address: string, value: string) => ({ chain, address, amount: value });

afterEach(() => {
    nock.cleanAll();
});

it('onramps BTC to a Solana address with only a Bitcoin signer', async () => {
    let quoteQuery: Record<string, string> = {};
    let orderBody: { onramp: { recipient: string; dstChain: string } } | undefined;
    nock(MAINNET_GATEWAY_BASE_URL)
        .get('/v4/get-quote')
        .query((query) => {
            quoteQuery = query as Record<string, string>;
            return true;
        })
        .reply(200, {
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
        });
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
    const quote = await sdk.getQuote({
        fromChain: 'bitcoin',
        fromToken: BTC,
        fromUserAddress: BTC_SENDER,
        refundAddress: BTC_SENDER,
        toChain: 'solana',
        toToken: USDC_MINT,
        toUserAddress: SOLANA_RECIPIENT,
        amount: 100000,
    });
    const sendBitcoin = vi.fn().mockResolvedValue('0200000001');

    // No walletClient / publicClient: the Solana side is only the recipient address.
    const result = await sdk.executeQuote({ quote, btcSigner: { sendBitcoin } });

    expect(quoteQuery).toMatchObject({ dstChain: 'solana', dstToken: USDC_MINT, recipient: SOLANA_RECIPIENT });
    expect(orderBody?.onramp).toMatchObject({ dstChain: 'solana', recipient: SOLANA_RECIPIENT });
    expect(sendBitcoin).toHaveBeenCalledWith(expect.objectContaining({ from: BTC_SENDER, to: 'bc1qdeposit' }));
    expect(result.tx).toBe('btc-txid');
});
