import nock from 'nock';
import { afterEach, expect, it, vi } from 'vitest';
import type { Account, Transport, Chain as ViemChain, WalletClient } from 'viem';
import { GatewaySDK } from '../src/gateway';
import { MAINNET_GATEWAY_BASE_URL } from '../src/gateway/client';
import type { GatewayQuoteV4OneOf, GatewayQuoteV4OneOf1 } from '../src/gateway/generated-client';

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

const EVM_SENDER = '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF';
const offrampQuote = {
    offramp: {
        srcChain: 'bob',
        tokenAddress: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c',
        inputAmount: amount('bob', '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c', '1000'),
    },
} as unknown as GatewayQuoteV4OneOf;
const tokenSwapQuote = {
    tokenSwap: {
        srcChain: 'ethereum',
        txTo: '0x1234567890123456789012345678901234567890',
        inputAmount: amount('ethereum', '0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599', '100000'),
    },
} as unknown as GatewayQuoteV4OneOf1;
const walletClient = {
    account: { address: EVM_SENDER },
} as unknown as WalletClient<Transport, ViemChain, Account>;

it.each([
    ['offramp', offrampQuote, {}],
    ['offramp', offrampQuote, { walletClient }],
    ['token swap', tokenSwapQuote, {}],
    ['token swap', tokenSwapQuote, { walletClient }],
])('rejects a %s without the EVM clients before creating the order', async (kind, quote, clients) => {
    const createOrder = nock(MAINNET_GATEWAY_BASE_URL).post('/v4/create-order').reply(201, {});

    await expect(new GatewaySDK().executeQuote({ quote, ...clients })).rejects.toThrow(
        `walletClient and publicClient are required for ${kind} order`
    );
    expect(createOrder.isDone()).toBe(false);
});
