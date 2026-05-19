import { getAddress } from 'viem';
import { describe, expect, it } from 'vitest';
import { GatewaySDK, instanceOfGatewayQuoteV2OneOf, instanceOfGatewayQuoteV2OneOf1 } from '../src/gateway';
import { ETHEREUM_USDT_ADDRESS } from '../src/gateway/client';

const ETHEREUM_GATEWAY_BASE_URL = 'https://gateway-api-ethereum.gobob.xyz';

const ADDR_A = '0x001Da04C0B6cA5fEcDCD50E071966e4096A31177';
const ADDR_B = '0x70997970C51812dc3A010C7d01b50e0d17dc79C8';
const BTC_SENDER = 'bc1q6tgkjx4pgc5qda52fsgeuvjrhml5nuawwplejq';
const BTC_TOKEN = '0x0000000000000000000000000000000000000000';

describe('Gateway Multiple Affiliates', () => {
    const sdk = new GatewaySDK(ETHEREUM_GATEWAY_BASE_URL);
    const affiliateIds = `${ADDR_A}:25,${ADDR_B}:50`;

    it('resolves multiple affiliates on a live bitcoin → ethereum onramp quote', { timeout: 30_000 }, async () => {
        const quote = await sdk.getQuote({
            fromChain: 'bitcoin',
            fromToken: BTC_TOKEN,
            toChain: 'ethereum',
            toToken: ETHEREUM_USDT_ADDRESS,
            fromUserAddress: BTC_SENDER,
            toUserAddress: ADDR_A,
            amount: 100_000, // 0.001 BTC (within live rate limits)
            affiliateIds,
        });

        expect(instanceOfGatewayQuoteV2OneOf(quote)).toBe(true);
        if (!instanceOfGatewayQuoteV2OneOf(quote)) {
            return;
        }

        const { affiliates } = quote.onramp;
        expect(affiliates).toBeDefined();
        expect(affiliates).toHaveLength(2);

        const addresses = affiliates!.map((a) => getAddress(a.address));
        expect(addresses).toContain(getAddress(ADDR_A));
        expect(addresses).toContain(getAddress(ADDR_B));

        for (const affiliate of affiliates!) {
            expect(BigInt(affiliate.fee.amount)).toBeGreaterThan(0n);
        }
    });

    it('resolves multiple affiliates on a live ethereum → bitcoin offramp quote', { timeout: 30_000 }, async () => {
        const quote = await sdk.getQuote({
            fromChain: 'ethereum',
            fromToken: ETHEREUM_USDT_ADDRESS,
            toChain: 'bitcoin',
            toToken: BTC_TOKEN,
            fromUserAddress: ADDR_A,
            toUserAddress: BTC_SENDER,
            amount: 100_000_000, // 100 USDT (6 decimals)
            affiliateIds,
        });

        expect(instanceOfGatewayQuoteV2OneOf1(quote)).toBe(true);
        if (!instanceOfGatewayQuoteV2OneOf1(quote)) {
            return;
        }

        const { affiliates } = quote.offramp;
        expect(affiliates).toBeDefined();
        expect(affiliates).toHaveLength(2);

        const addresses = affiliates!.map((a) => getAddress(a.address));
        expect(addresses).toContain(getAddress(ADDR_A));
        expect(addresses).toContain(getAddress(ADDR_B));

        for (const affiliate of affiliates!) {
            expect(BigInt(affiliate.fee.amount)).toBeGreaterThan(0n);
        }
    });
});
