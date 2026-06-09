import nock from 'nock';
import { getAddress } from 'viem';
import { afterEach, describe, expect, it } from 'vitest';
import { GatewaySDK, instanceOfGatewayQuoteV2OneOf, instanceOfGatewayQuoteV2OneOf1 } from '../src/gateway';
import { ETHEREUM_USDT_ADDRESS } from '../src/gateway/client';
import { GatewayQuoteV2OneOf, GatewayQuoteV2OneOf1 } from '../src/gateway/generated-client';

const ETHEREUM_GATEWAY_BASE_URL = 'https://gateway-api-ethereum.gobob.xyz';

const ADDR_A = '0x1111111111111111111111111111111111111111';
const ADDR_B = '0x2222222222222222222222222222222222222222';
const BTC_SENDER = 'bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq';
const BTC_TOKEN = '0x0000000000000000000000000000000000000000';

const mockTokenAmount = (amount: string) => ({
    address: BTC_TOKEN,
    amount,
    chain: 'ethereum',
});

afterEach(() => {
    nock.cleanAll();
});

describe('Gateway Multiple Affiliates', () => {
    const sdk = new GatewaySDK({ basePath: ETHEREUM_GATEWAY_BASE_URL });
    const affiliates = [
        { address: ADDR_A, bps: 25 },
        { address: ADDR_B, bps: 50 },
    ] as const;

    it('resolves multiple affiliates on a bitcoin → ethereum onramp quote', async () => {
        const mockOnrampQuote: GatewayQuoteV2OneOf = {
            onramp: {
                dstChain: 'ethereum',
                dstToken: ETHEREUM_USDT_ADDRESS,
                executionFees: mockTokenAmount('100'),
                feeBreakdown: {
                    affiliateFee: mockTokenAmount('75'),
                    executionFee: mockTokenAmount('100'),
                    layerzeroFee: mockTokenAmount('0'),
                    protocolFee: mockTokenAmount('50'),
                    solverFee: mockTokenAmount('25'),
                },
                fees: mockTokenAmount('250'),
                inputAmount: mockTokenAmount('100000'),
                outputAmount: mockTokenAmount('99750'),
                recipient: ADDR_A,
                signedQuoteData: 'mock-signed-quote',
                slippage: '0',
                token: ETHEREUM_USDT_ADDRESS,
                affiliates: [
                    { address: ADDR_A, fee: mockTokenAmount('25') },
                    { address: ADDR_B, fee: mockTokenAmount('50') },
                ],
            },
        };

        const expectedWireAffiliates = `${ADDR_A}:25,${ADDR_B}:50`;
        nock(ETHEREUM_GATEWAY_BASE_URL)
            .get('/v3/get-quote')
            .query((q) => q.srcChain === 'bitcoin' && q.affiliates === expectedWireAffiliates)
            .reply(200, mockOnrampQuote);

        const quote = await sdk.getQuote({
            fromChain: 'bitcoin',
            fromToken: BTC_TOKEN,
            toChain: 'ethereum',
            toToken: ETHEREUM_USDT_ADDRESS,
            fromUserAddress: BTC_SENDER,
            toUserAddress: ADDR_A,
            amount: 100_000,
            affiliates,
        });

        expect(instanceOfGatewayQuoteV2OneOf(quote)).toBe(true);
        if (!instanceOfGatewayQuoteV2OneOf(quote)) {
            return;
        }

        const resolved = quote.onramp.affiliates;
        expect(resolved).toBeDefined();
        expect(resolved).toHaveLength(2);

        const onrampAddresses = resolved!.map((a) => getAddress(a.address));
        expect(onrampAddresses).toContain(getAddress(ADDR_A));
        expect(onrampAddresses).toContain(getAddress(ADDR_B));

        for (const affiliate of resolved!) {
            expect(BigInt(affiliate.fee.amount)).toBeGreaterThan(0n);
        }
    });

    it('resolves multiple affiliates on an ethereum → bitcoin offramp quote', async () => {
        const mockOfframpQuote: GatewayQuoteV2OneOf1 = {
            offramp: {
                feeBreakdown: {
                    affiliateFee: mockTokenAmount('75'),
                    fastestFeeRate: '10',
                    inclusionFee: mockTokenAmount('200'),
                    protocolFee: mockTokenAmount('50'),
                    solverFee: mockTokenAmount('25'),
                },
                inputAmount: mockTokenAmount('100000000'),
                outputAmount: mockTokenAmount('99650000'),
                recipient: BTC_SENDER,
                slippage: 0,
                srcChain: 'ethereum',
                tokenAddress: ETHEREUM_USDT_ADDRESS,
                totalFeeUsd: '0.35',
                txTo: '0x1234567890123456789012345678901234567890',
                affiliates: [
                    { address: ADDR_A, fee: mockTokenAmount('25') },
                    { address: ADDR_B, fee: mockTokenAmount('50') },
                ],
            },
        };

        nock(ETHEREUM_GATEWAY_BASE_URL)
            .get('/v3/get-quote')
            .query((q) => q.dstChain === 'bitcoin')
            .reply(200, mockOfframpQuote);

        const quote = await sdk.getQuote({
            fromChain: 'ethereum',
            fromToken: ETHEREUM_USDT_ADDRESS,
            toChain: 'bitcoin',
            toToken: BTC_TOKEN,
            fromUserAddress: ADDR_A,
            toUserAddress: BTC_SENDER,
            amount: 100_000_000,
            affiliates,
        });

        expect(instanceOfGatewayQuoteV2OneOf1(quote)).toBe(true);
        if (!instanceOfGatewayQuoteV2OneOf1(quote)) {
            return;
        }

        const resolved = quote.offramp.affiliates;
        expect(resolved).toBeDefined();
        expect(resolved).toHaveLength(2);

        const offrampAddresses = resolved!.map((a) => getAddress(a.address));
        expect(offrampAddresses).toContain(getAddress(ADDR_A));
        expect(offrampAddresses).toContain(getAddress(ADDR_B));

        for (const affiliate of resolved!) {
            expect(BigInt(affiliate.fee.amount)).toBeGreaterThan(0n);
        }
    });
});
