import * as bitcoin from 'bitcoinjs-lib';
import nock from 'nock';
import { getAddress, zeroAddress } from 'viem';
import { bob, bobSepolia } from 'viem/chains';
import { afterEach, assert, describe, expect, it } from 'vitest';
import { GatewaySDK } from '../src/gateway';
import { MAINNET_GATEWAY_BASE_URL } from '../src/gateway/client';
import { getTokenAddress, SYMBOL_LOOKUP } from '../src/gateway/tokens';
import { toHexScriptPubKey } from '../src/gateway/utils';

const TBTC = SYMBOL_LOOKUP[bob.id]['tbtc'];
const TBTC_ADDRESS = TBTC.address;

afterEach(() => {
    nock.cleanAll();
});

describe('Gateway Tests', () => {
    it('should reject invalid chain', async () => {
        expect(() => {
            new GatewaySDK(109209);
        }).toThrowError('Invalid chain');
    });

    it('should get quote', async () => {
        const gatewaySDK = new GatewaySDK(bob.id);

        const mockQuote = {
            onramp: {
                inputAmount: '1000',
                outputAmount: '990',
                outputToken: TBTC_ADDRESS,
                fees: {
                    protocolFee: '5',
                    affiliateFee: '2',
                    networkFee: '3',
                },
            },
        };

        nock(`${MAINNET_GATEWAY_BASE_URL}/api`)
            .get('/get-quote')
            .query(true)
            .times(3)
            .reply(200, mockQuote);

        const result1 = await gatewaySDK.getQuote({
            fromChain: 'Bitcoin',
            fromToken: 'bitcoin',
            toChain: 'BOB',
            toToken: 'tBTC',
            toUserAddress: zeroAddress,
            amount: 1000,
        });

        expect(result1).toBeDefined();
        expect(result1.onramp).toBeDefined();

        const result2 = await gatewaySDK.getQuote({
            fromChain: 'Bitcoin',
            fromToken: 'bitcoin',
            toChain: 'bob',
            toToken: 'tbtc',
            toUserAddress: zeroAddress,
            amount: 1000,
        });

        expect(result2).toBeDefined();

        const result3 = await gatewaySDK.getQuote({
            fromChain: 'Bitcoin',
            fromToken: 'bitcoin',
            toChain: '60808',
            toToken: 'tbtc',
            toUserAddress: zeroAddress,
            amount: 1000,
        });

        expect(result3).toBeDefined();
    });

    it('should get tokens', async () => {
        nock(`${MAINNET_GATEWAY_BASE_URL}/api`).get(`/get-tokens`).reply(200, [zeroAddress]);

        const gatewaySDK = new GatewaySDK(bob.id);
        const tokens = await gatewaySDK.getTokens();
        assert.deepEqual(tokens, [zeroAddress]);
    });

    it.skip('should get enriched tokens', async () => {
        const gatewaySDK = new GatewaySDK(bob.id);

        const enrichedTokens = await gatewaySDK.getEnrichedTokens();

        enrichedTokens.forEach((enrichedToken) => {
            assert.isDefined(enrichedToken.address);
            assert.isDefined(enrichedToken.name);
            assert.isDefined(enrichedToken.symbol);
            assert.isDefined(enrichedToken.decimals);
            assert.isDefined(enrichedToken.tvl);
        });
    });

    it('should get orders', async () => {
        const mockOrders = [
            {
                id: '1',
                type: 'onramp',
                status: 'completed',
                amount: '1000',
            },
        ];

        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .get(`/api/get-orders/${zeroAddress}`)
            .reply(200, mockOrders);

        const gatewaySDK = new GatewaySDK(bob.id);
        const orders = await gatewaySDK.getOrders(zeroAddress);
        expect(orders).toBeDefined();
        expect(Array.isArray(orders)).toBe(true);
    });

    it('should return valid btc pub key', async () => {
        let userAddress = 'tb1qn40xpua4eskjgmueq6fwujex05wdtprh46vkpc'; //P2WPKH Native Segwit
        let scriptPubKey = toHexScriptPubKey(userAddress, bitcoin.networks.testnet);
        expect(scriptPubKey).to.deep.equal('0x1600149d5e60f3b5cc2d246f990692ee4b267d1cd58477');

        userAddress = '2NEFg6PA1ZBkNC4dFhXf2uYrKuSxXwPiWEh'; //P2SH-P2WPKH Nested Segwit
        scriptPubKey = toHexScriptPubKey(userAddress, bitcoin.networks.testnet);
        expect(scriptPubKey).to.deep.equal('0x17a914e6706ed55fed3e5136644bf42e3a878371afd93187');

        userAddress = 'mxd8LwdL2EMSNQ6RkxQedqdKkPskrRoYDx'; //P2PKH Legacy
        scriptPubKey = toHexScriptPubKey(userAddress, bitcoin.networks.testnet);
        expect(scriptPubKey).to.deep.equal('0x1976a914bba505f3a2730254053081318cade0672ffe31d888ac');
    });

    it('resolves token address for symbol or address input', async () => {
        expect(getTokenAddress(bob.id, 'WBTC')).toBe(getAddress('0x0555e30da8f98308edb960aa94c0db47230d2b9c'));
        expect(getTokenAddress(bob.id, 'wbtc')).toBe(getAddress('0x0555e30da8f98308edb960aa94c0db47230d2b9c'));
        expect(getTokenAddress(bob.id, '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c')).toBe(
            getAddress('0x0555e30da8f98308edb960aa94c0db47230d2b9c')
        );
    });

    it('should get chains', async () => {
        nock(`${MAINNET_GATEWAY_BASE_URL}/api`)
            .get('/get-chains')
            .reply(200, ['Bitcoin', 'BOB', 'Ethereum']);

        const gatewaySDK = new GatewaySDK(bob.id);
        const chains = await gatewaySDK.getChains();
        expect(chains).toBeDefined();
        expect(Array.isArray(chains)).toBe(true);
    });

    it('should get routes', async () => {
        nock(`${MAINNET_GATEWAY_BASE_URL}/api`)
            .get('/get-routes')
            .reply(200, [
                {
                    srcChain: 'Bitcoin',
                    dstChain: 'BOB',
                    srcToken: 'bitcoin',
                    dstToken: 'tBTC',
                },
            ]);

        const gatewaySDK = new GatewaySDK(bob.id);
        const routes = await gatewaySDK.getRoutes();
        expect(routes).toBeDefined();
        expect(Array.isArray(routes)).toBe(true);
    });
});
