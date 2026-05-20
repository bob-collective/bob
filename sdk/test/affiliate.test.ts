import { getAddress } from 'viem';
import { describe, expect, it } from 'vitest';
import { GatewaySDK, instanceOfGatewayQuoteV2OneOf, instanceOfGatewayQuoteV2OneOf1 } from '../src/gateway';
import { ETHEREUM_USDT_ADDRESS } from '../src/gateway/client';

const ETHEREUM_GATEWAY_BASE_URL = 'https://gateway-api-ethereum.gobob.xyz';

const ADDR_A = '0x1111111111111111111111111111111111111111';
const ADDR_B = '0x2222222222222222222222222222222222222222';
const BTC_SENDER = 'bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq';
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
            amount: 100_000, // 0.001 BTC
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
            amount: 100_000_000, // 100 USDT
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
