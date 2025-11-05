import * as bitcoin from 'bitcoinjs-lib';
import nock from 'nock';
import { zeroAddress } from 'viem';
import { Address } from 'viem/accounts';
import { bob, bobSepolia } from 'viem/chains';
import { afterEach, assert, describe, expect, it, vi } from 'vitest';
import { GatewaySDK, LayerZeroGatewayClient } from '../src/gateway';
import { MAINNET_GATEWAY_BASE_URL, SIGNET_GATEWAY_BASE_URL } from '../src/gateway/client';
import { getTokenAddress, SYMBOL_LOOKUP } from '../src/gateway/tokens';
import {
    GatewayOrderType,
    GatewayTokensInfo,
    OfframpOrder,
    OfframpOrderStatus,
    OnrampQuote,
    OrderDetailsRaw,
} from '../src/gateway/types';
import { convertOrderDetailsRawToOrderDetails, toHexScriptPubKey } from '../src/gateway/utils';

const TBTC = SYMBOL_LOOKUP[bob.id]['tbtc'];
const TBTC_ADDRESS = TBTC.address;
const SOLVBTC = SYMBOL_LOOKUP[bob.id]['solvbtc'];
const SOLVBTC_ADDRESS = SOLVBTC.address;

const MOCK_ORDER_DETAILS_RAW: OrderDetailsRaw = {
    version: 'v4',
    data: {
        ethAmountToReceive: '0x0',
        satsToSwapToEth: 0,
        ethTransferGasLimit: '0x8fc',
        strategyGasLimit: '0x0',
        totalUserGasLimit: '0x30d40',
        userGasPriceLimit: '0x100699',
        l1DataFee: '0x18187',
        extraSatsFee: null,
        extraSatsFeeRecipient: null,
    },
};

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

        const orderDetails = convertOrderDetailsRawToOrderDetails(MOCK_ORDER_DETAILS_RAW);

        const mockQuote = {
            gatewayAddress: zeroAddress,
            baseTokenAddress: TBTC_ADDRESS as Address,
            dustThreshold: 1000,
            satoshis: 1000,
            fee: 10,
            bitcoinAddress: '',
            txProofDifficultyFactor: 3,
            strategyAddress: zeroAddress,
            orderDetails: MOCK_ORDER_DETAILS_RAW,
            baseToken: TBTC,
            outputToken: TBTC,
            feeBreakdown: {
                overallFeeSats: 10,
                protocolFeeSats: 0,
                affiliateFeeSats: 0,
                executionFeeWei: BigInt(0),
                l1DataFeeWei: BigInt(0),
            },
        };

        const assertMockQuote = {
            ...mockQuote,
            outputSatoshis: mockQuote.satoshis - mockQuote.fee,
            orderDetails: orderDetails,
        };

        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .get(`/v4/quote/${TBTC_ADDRESS}?userAddress=${zeroAddress}&satoshis=1000`)
            .times(5)
            .reply(200, {
                ...mockQuote,
                feeBreakdown: {
                    overallFeeSats: mockQuote.feeBreakdown.overallFeeSats,
                    protocolFeeSats: mockQuote.feeBreakdown.protocolFeeSats,
                    affiliateFeeSats: mockQuote.feeBreakdown.affiliateFeeSats,
                    executionFeeWei: '0x0',
                    l1DataFeeWei: '0x0',
                },
            });

        assert.deepEqual(
            await gatewaySDK.getQuote({
                fromChain: 'Bitcoin',
                fromToken: 'bitcoin',
                toChain: 'BOB',
                toToken: 'tBTC',
                toUserAddress: zeroAddress,
                amount: 1000,
            }),
            {
                type: GatewayOrderType.Onramp,
                data: assertMockQuote,
                finalOutputSats: mockQuote.satoshis - mockQuote.fee,
                finalFeeSats: mockQuote.fee,
                params: {
                    fromChain: 'Bitcoin',
                    fromToken: 'bitcoin',
                    toChain: 'BOB',
                    toToken: 'tBTC',
                    toUserAddress: zeroAddress,
                    amount: 1000,
                },
            }
        );
        assert.deepEqual(
            await gatewaySDK.getQuote({
                fromChain: 'Bitcoin',
                fromToken: 'bitcoin',
                toChain: 'bob',
                toToken: 'tbtc',
                toUserAddress: zeroAddress,
                amount: 1000,
            }),
            {
                type: GatewayOrderType.Onramp,
                data: assertMockQuote,
                finalOutputSats: mockQuote.satoshis - mockQuote.fee,
                finalFeeSats: mockQuote.fee,
                params: {
                    fromChain: 'Bitcoin',
                    fromToken: 'bitcoin',
                    toChain: 'bob',
                    toToken: 'tbtc',
                    toUserAddress: zeroAddress,
                    amount: 1000,
                },
            }
        );
        assert.deepEqual(
            await gatewaySDK.getQuote({
                fromChain: 'Bitcoin',
                fromToken: 'bitcoin',
                toChain: 60808,
                toToken: 'tbtc',
                toUserAddress: zeroAddress,
                amount: 1000,
            }),
            {
                type: GatewayOrderType.Onramp,
                data: assertMockQuote,
                finalOutputSats: mockQuote.satoshis - mockQuote.fee,
                finalFeeSats: mockQuote.fee,
                params: {
                    fromChain: 'Bitcoin',
                    fromToken: 'bitcoin',
                    toChain: 60808,
                    toToken: 'tbtc',
                    toUserAddress: zeroAddress,
                    amount: 1000,
                },
            }
        );
        assert.deepEqual(
            await gatewaySDK.getQuote({
                fromChain: 'Bitcoin',
                fromToken: 'bitcoin',
                toChain: 'BOB',
                toToken: TBTC_ADDRESS,
                toUserAddress: zeroAddress,
                amount: 1000,
            }),
            {
                type: GatewayOrderType.Onramp,
                data: assertMockQuote,
                finalOutputSats: mockQuote.satoshis - mockQuote.fee,
                finalFeeSats: mockQuote.fee,
                params: {
                    fromChain: 'Bitcoin',
                    fromToken: 'bitcoin',
                    toChain: 'BOB',
                    toToken: TBTC_ADDRESS,
                    toUserAddress: zeroAddress,
                    amount: 1000,
                },
            }
        );

        // get the total available without amount
        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .get(`/v4/quote/${TBTC_ADDRESS}?userAddress=${zeroAddress}`)
            .reply(200, {
                ...mockQuote,
                feeBreakdown: {
                    overallFeeSats: mockQuote.feeBreakdown.overallFeeSats,
                    protocolFeeSats: mockQuote.feeBreakdown.protocolFeeSats,
                    affiliateFeeSats: mockQuote.feeBreakdown.affiliateFeeSats,
                    executionFeeWei: '0x0',
                    l1DataFeeWei: '0x0',
                },
            });

        assert.deepEqual(
            await gatewaySDK.getQuote({
                fromChain: 'Bitcoin',
                fromToken: 'bitcoin',
                amount: 0,
                toChain: 'BOB',
                toToken: TBTC_ADDRESS,
                toUserAddress: zeroAddress,
            }),
            {
                type: GatewayOrderType.Onramp,
                data: assertMockQuote,
                finalOutputSats: mockQuote.satoshis - mockQuote.fee,
                finalFeeSats: mockQuote.fee,
                params: {
                    fromChain: 'Bitcoin',
                    fromToken: 'bitcoin',
                    amount: 0,
                    toChain: 'BOB',
                    toToken: TBTC_ADDRESS,
                    toUserAddress: zeroAddress,
                },
            }
        );
    });

    it('should throw error on invalid token', async () => {
        const gatewaySDK = new GatewaySDK(bob.id);
        await expect(async () => {
            await gatewaySDK.getQuote({
                fromChain: 'Bitcoin',
                toChain: 'BOB',
                toToken: 'unknownToken',
                toUserAddress: zeroAddress,
                amount: 1000,
                fromToken: 'bitcoin',
            });
        }).rejects.toThrowError('Unknown output token');
    });

    it('should reject invalid network', async () => {
        const gatewaySDK = new GatewaySDK(bobSepolia.id);
        await expect(async () => {
            await gatewaySDK.getQuote({
                fromChain: 'Bitcoin',
                toChain: 'BOB',
                toToken: 'tbtc',
                toUserAddress: zeroAddress,
                amount: 1000,
                fromToken: 'bitcoin',
            });
        }).rejects.toThrowError('Invalid output chain');
    });

    it('should start order', { timeout: 50000 }, async () => {
        const gatewaySDK = new GatewaySDK(bob.id);
        const mockQuote = {
            gatewayAddress: zeroAddress,
            baseTokenAddress: TBTC_ADDRESS as Address,
            dustThreshold: 1000,
            satoshis: 1000,
            outputSatoshis: 990,
            fee: 10,
            bitcoinAddress: 'bc1qafk4yhqvj4wep57m62dgrmutldusqde8adh20d',
            txProofDifficultyFactor: 3,
            strategyAddress: zeroAddress,
            orderDetails: convertOrderDetailsRawToOrderDetails(MOCK_ORDER_DETAILS_RAW),
            feeBreakdown: {
                overallFeeSats: 10,
                protocolFeeSats: 0,
                affiliateFeeSats: 0,
                executionFeeWei: BigInt(0),
                l1DataFeeWei: BigInt(0),
            },
        };

        nock(`${MAINNET_GATEWAY_BASE_URL}`).post(`/v4/order`).reply(201, {
            uuid: '00000000-0000-0000-0000-000000000000',
            opReturnHash: '0x8d0fd89210149d4219c87fa814a4bcde0c6a36b8fe2dff52b1d3eaa9e7cf0a9a',
        });

        await expect(async () => {
            await gatewaySDK.startOnrampOrder(mockQuote, {
                toChain: 'BOB',
                toToken: 'tBTC',
                toUserAddress: '2N8DbeaBdjkktkRzaKL1tHj9FQELV7jA8Re',
                amount: 1000,
                fromChain: 'Bitcoin',
                fromToken: 'BTC',
                fromUserAddress: 'bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq',
            });
        }).rejects.toThrowError('Invalid user address');

        const result = await gatewaySDK.startOnrampOrder(mockQuote, {
            toChain: 'BOB',
            toToken: 'tBTC',
            toUserAddress: zeroAddress,
            amount: 1000,
            fromChain: 'Bitcoin',
            fromToken: 'BTC',
            fromUserAddress: 'bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq',
        });

        assert.isDefined(result.psbtBase64);
        const psbt = bitcoin.Psbt.fromBase64(result.psbtBase64!);
        assert.deepEqual(
            psbt.txOutputs[0].script,
            bitcoin.script.compile([bitcoin.opcodes.OP_RETURN, Buffer.from(result.opReturnHash.slice(2), 'hex')])
        );
    });

    it('should get strategies', async () => {
        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .get(`/strategies`)
            .reply(200, [
                {
                    strategyAddress: zeroAddress,
                    inputTokenAddress: TBTC_ADDRESS,
                    strategyName: 'Pell Network (tBTC)',
                    strategyType: 'staking',
                },
            ]);
        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .get(`/v4/quote/${TBTC_ADDRESS}?satoshis=1000&strategy=${zeroAddress}&userAddress=${zeroAddress}`)
            .times(4)
            .reply(200, {
                gatewayAddress: zeroAddress,
                dustThreshold: 1000,
                satoshis: 1000,
                fee: 10,
                bitcoinAddress: '',
                txProofDifficultyFactor: 3,
                strategyAddress: zeroAddress,
                orderDetails: MOCK_ORDER_DETAILS_RAW,
                feeBreakdown: {
                    overallFeeSats: 10,
                    protocolFeeSats: 0,
                    affiliateFeeSats: 0,
                    executionFeeWei: '0x0',
                    l1DataFeeWei: '0x0',
                },
            });

        const gatewaySDK = new GatewaySDK(bob.id);
        const strategies = await gatewaySDK.getStrategies();

        assert.lengthOf(strategies, 1);
        assert.isDefined(strategies[0].inputToken);
        assert.isNull(strategies[0].outputToken);
        assert.equal(strategies[0].integration.slug, 'pell-network-tbtc');

        const strategy = strategies[0];
        await gatewaySDK.getQuote({
            fromChain: 'Bitcoin',
            fromToken: 'bitcoin',
            toUserAddress: zeroAddress,
            amount: 1000,
            toChain: strategy.chain.chainId,
            toToken: strategy.inputToken.symbol,
            strategyAddress: strategy.address,
        });
    });

    it('should get tokens', async () => {
        nock(`${MAINNET_GATEWAY_BASE_URL}`).get(`/tokens?includeStrategies=false`).reply(200, [zeroAddress]);

        const gatewaySDK = new GatewaySDK(bob.id);
        assert.deepEqual(await gatewaySDK.getTokenAddresses(false), [zeroAddress]);
    });

    it.skip('should get enriched tokens', async () => {
        const gatewaySDK = new GatewaySDK(bob.id);

        const tokens = await gatewaySDK.getTokens(true);
        const enrichedTokens = await gatewaySDK.getEnrichedTokens(true);

        enrichedTokens.forEach((enrichedToken, i) => {
            assert.strictEqual(tokens[i].address, enrichedToken.address);
            assert.strictEqual(tokens[i].name, enrichedToken.name);
            assert.strictEqual(tokens[i].symbol, enrichedToken.symbol);
            assert.strictEqual(tokens[i].decimals, enrichedToken.decimals);
            assert.isDefined(enrichedToken.tvl);
            assert.notEqual(enrichedToken.tvl, 0);
        });
    });

    it('should get orders', async () => {
        const mockOrder = {
            gatewayAddress: zeroAddress,
            baseTokenAddress: TBTC_ADDRESS,
            txid: '',
            status: false,
            timestamp: 0,
            tokens: '0',
            satoshis: 0,
            fee: 0,
            txProofDifficultyFactor: 0,
            satsToConvertToEth: 0,
            strategyAddress: null,
            outputEthAmount: null,
            outputTokenAddress: null,
            outputTokenAmount: null,
            txHash: null,
            layerzeroDstEid: null,
            orderDetails: null,
            tokensReceived: null,
        };
        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .get(`/orders/${zeroAddress}`)
            .reply(200, [
                // staking - success
                {
                    ...mockOrder,
                    satoshis: 1000,
                    fee: 0,
                    status: true,
                    strategyAddress: zeroAddress,
                    outputTokenAmount: '2000',
                    outputTokenAddress: SOLVBTC_ADDRESS,
                },
                // staking - pending
                {
                    ...mockOrder,
                    satoshis: 1000,
                    fee: 0,
                    strategyAddress: zeroAddress,
                },
                // staking - failed
                {
                    ...mockOrder,
                    satoshis: 1000,
                    fee: 0,
                    status: true,
                    strategyAddress: zeroAddress,
                },
                // swapping - pending
                {
                    ...mockOrder,
                    satoshis: 1000,
                    fee: 0,
                },
                // swapping - success
                {
                    ...mockOrder,
                    satoshis: 1000,
                    fee: 0,
                    status: true,
                },
                // staking - failed (wBTC)
                {
                    ...mockOrder,
                    baseTokenAddress: SYMBOL_LOOKUP[bob.id]['wbtc'],
                    satoshis: 1000,
                    fee: 0,
                    status: true,
                    strategyAddress: zeroAddress,
                },
                // swapping - success (wBTC)
                {
                    ...mockOrder,
                    baseTokenAddress: SYMBOL_LOOKUP[bob.id]['wbtc'],
                    satoshis: 1000,
                    fee: 0,
                    status: true,
                },
                // swapping - success (layerzero wBTC optimism)
                {
                    ...mockOrder,
                    baseTokenAddress: SYMBOL_LOOKUP[bob.id]['wbtc'],
                    satoshis: 1000,
                    fee: 0,
                    status: true,
                    layerzeroDstEid: 30111,
                },
                // staking - unknown token
                {
                    ...mockOrder,
                    satoshis: 1000,
                    fee: 0,
                    status: true,
                    strategyAddress: zeroAddress,
                    outputTokenAmount: '2000',
                    outputTokenAddress: '0x1111111111111111111111111111111111111111',
                },
            ]);

        const gatewaySDK = new GatewaySDK(bob.id);
        const orders = await gatewaySDK.getOnrampOrders(zeroAddress);
        assert.lengthOf(orders, 9);

        assert.strictEqual(orders[0].getTokenAmount(), '2000'); // success (staking)
        assert.strictEqual(orders[1].getTokenAmount(), null); // pending (staking)
        assert.strictEqual(orders[2].getTokenAmount(), 10000000000000); // failed (staking)
        assert.strictEqual(orders[3].getTokenAmount(), 10000000000000); // pending (swapping)
        assert.strictEqual(orders[4].getTokenAmount(), 10000000000000); // success (swapping)
        assert.strictEqual(orders[5].getTokenAmount(), 1000); // failed (staking)
        assert.strictEqual(orders[6].getTokenAmount(), 1000); // success (swapping)
        assert.strictEqual(orders[8].getTokenAmount(), '2000'); // success (swapping)

        assert.strictEqual(orders[0].getToken()!.address, SOLVBTC_ADDRESS);
        assert.strictEqual(orders[1].getToken(), undefined);
        assert.strictEqual(orders[8].getToken(), undefined);

        assert.strictEqual(orders[7].layerzeroDstEid, 30111);
    });

    it('should return multiple output tokens', async () => {
        const mockOrder = {
            gatewayAddress: zeroAddress,
            baseTokenAddress: TBTC_ADDRESS,
            txid: '',
            status: true,
            timestamp: 0,
            tokens: '0',
            satoshis: 0,
            fee: 0,
            txProofDifficultyFactor: 0,
            satsToConvertToEth: 0,
            strategyAddress: zeroAddress,
            outputEthAmount: null,
            outputTokenAddress: null,
            outputTokenAmount: null,
            txHash: null,
            layerzeroDstEid: null,
            orderDetails: null,
            tokensReceived: null,
        };
        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .get(`/orders/${zeroAddress}`)
            .reply(200, [
                // pending
                {
                    ...mockOrder,
                    status: false,
                    strategyAddress: null,
                    satoshis: 2000,
                    outputTokenAddress: null,
                    outputTokenAmount: null,
                    tokensReceived: null,
                },
                // swapped - gateway v3
                {
                    ...mockOrder,
                    satoshis: 2000,
                    outputTokenAddress: TBTC_ADDRESS,
                    outputTokenAmount: '2000',
                    tokensReceived: null,
                },
                // swapped - gateway v4
                {
                    ...mockOrder,
                    satoshis: 2000,
                    outputTokenAddress: null,
                    outputTokenAmount: null,
                    tokensReceived: [{ amount: '2000', tokenAddress: TBTC_ADDRESS }],
                },
                // swapped unknown token - gateway v4
                {
                    ...mockOrder,
                    satoshis: 2000,
                    outputTokenAddress: null,
                    outputTokenAmount: null,
                    tokensReceived: [{ amount: '2000', tokenAddress: '0x1111111111111111111111111111111111111111' }],
                },
            ]);

        const gatewaySDK = new GatewaySDK(bob.id);
        const orders = await gatewaySDK.getOnrampOrders(zeroAddress);
        assert.lengthOf(orders, 4);

        assert.strictEqual(orders[0].getTokenAmount(), 20000000000000);
        assert.strictEqual(orders[1].getTokenAmount(), '2000');
        assert.strictEqual(orders[2].getTokenAmount(), '2000');

        assert.strictEqual(orders[0].getTokenAddress(), TBTC_ADDRESS);
        assert.strictEqual(orders[1].getTokenAddress(), TBTC_ADDRESS);
        assert.strictEqual(orders[2].getTokenAddress(), TBTC_ADDRESS);

        assert.strictEqual(orders[0].getToken()!.address, TBTC_ADDRESS);
        assert.strictEqual(orders[1].getToken()!.address, TBTC_ADDRESS);
        assert.strictEqual(orders[2].getToken()!.address, TBTC_ADDRESS);

        assert.deepEqual(await orders[0].getOutputTokens(), []);
        assert.deepEqual(await orders[1].getOutputTokens(), [
            {
                amount: '2000',
                token: TBTC,
            },
        ]);
        assert.deepEqual(await orders[2].getOutputTokens(), [
            {
                amount: '2000',
                token: TBTC,
            },
        ]);
        assert.deepEqual(await orders[3].getOutputTokens(), []);

        assert.deepEqual(await orders[0].getTokens(), [
            {
                amount: 20000000000000,
                token: TBTC,
            },
        ]);
        assert.deepEqual(await orders[1].getTokens(), [
            {
                amount: '2000',
                token: TBTC,
            },
        ]);
        assert.deepEqual(await orders[2].getTokens(), [
            {
                amount: '2000',
                token: TBTC,
            },
        ]);
    });

    it('should get valid create offramp quote', async () => {
        const gatewaySDK = new GatewaySDK(bobSepolia.id);

        nock(`${SIGNET_GATEWAY_BASE_URL}`)
            .get('/offramp-quote')
            .query(true)
            .reply(200, {
                amountLockInSat: 10000000000000,
                registryAddress: '0xd7b27b178f6bf290155201109906ad203b6d99b1',
                feeBreakdown: {
                    overallFeeSats: 385,
                    inclusionFeeSats: 384,
                    protocolFeeSats: 1,
                    affiliateFeeSats: 0,
                    fastestFeeRate: 1,
                },
            });

        const result = await gatewaySDK.createOfframpOrder(
            {
                amountLockInSat: 10,
                amountReceiveInSat: 0,
                deadline: 0,
                feeBreakdown: {
                    affiliateFeeSats: 0,
                    overallFeeSats: 100,
                    inclusionFeeSats: 0,
                    protocolFeeSats: 0,
                    fastestFeeRate: 0,
                },
                registryAddress: '0x',
                token: '0xda472456b1a6a2fc9ae7edb0e007064224d4284c',
            },
            {
                fromToken: '0xda472456b1a6a2fc9ae7edb0e007064224d4284c',
                amount: 100000000000000,
                fromUserAddress: '0xFAEe001465dE6D7E8414aCDD9eF4aC5A35B2B808',
                toUserAddress: 'tb1qn40xpua4eskjgmueq6fwujex05wdtprh46vkpc',
                fromChain: 'bob',
                toChain: 'bitcoin',
                toToken: 'bitcoin',
            }
        );

        expect(result.offrampArgs[0]).to.deep.equal({
            satAmountToLock: BigInt('10'),
            satFeesMax: BigInt('100'),
            creationDeadline: result.offrampArgs[0].creationDeadline, // timestamp is dynamic
            outputScript: '0x1600149d5e60f3b5cc2d246f990692ee4b267d1cd58477',
            token: '0xda472456b1a6a2fc9ae7edb0e007064224d4284c',
            owner: '0xFAEe001465dE6D7E8414aCDD9eF4aC5A35B2B808',
        });
    });

    it('should return valid offramp orders', async () => {
        const gatewaySDK = new GatewaySDK(bobSepolia.id);
        const userAddress = '0xFAEe001465dE6D7E8414aCDD9eF4aC5A35B2B808';

        // Mock response data
        const mockResponse = [
            {
                orderId: '0x0',
                token: '0x4496ebE7C8666a8103713EE6e0c08cA0cD25b888',
                satAmountLocked: '0x3e8',
                satFeesMax: '0x181',
                status: 'Processed',
                btcTx: 'e8d52d6ef6ebf079f2d082dc683c9455178b64e0685c10e93882effaedde4474',
                evmTx: null,
                orderTimestamp: 1743679342,
                shouldFeesBeBumped: false,
            },
        ];

        // Dynamically parse expected result from mockResponse
        const expectedResult = mockResponse.map((order: any) => ({
            ...order,
            token: order.token as Address,
            orderId: BigInt(order.orderId.toString()),
            satAmountLocked: BigInt(order.satAmountLocked.toString()),
            satFeesMax: BigInt(order.satFeesMax.toString()),
            orderTimestamp: order.orderTimestamp,
            canOrderBeUnlocked: false,
            shouldFeesBeBumped: false,
            offrampRegistryAddress: '0xb74a5af78520075f90f4be803153673a162a9776',
        }));

        nock(SIGNET_GATEWAY_BASE_URL).get(`/offramp-orders/${userAddress}`).reply(200, mockResponse);
        nock(`${SIGNET_GATEWAY_BASE_URL}`).get('/offramp-quote').query(true).reply(200, {
            amountLockInSat: 10000000000000,
            feesInSat: 385,
            registryAddress: '0xd7b27b178f6bf290155201109906ad203b6d99b1',
            feeRate: 1,
        });
        nock(SIGNET_GATEWAY_BASE_URL)
            .get('/offramp-registry-address')
            .reply(200, '0xb74a5af78520075f90f4be803153673a162a9776');

        const result: OfframpOrder[] = await gatewaySDK.getOfframpOrders(userAddress);

        // Assertion
        expect(result).to.deep.equal(expectedResult);
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

    it('fetches the correct offramp registry address', async () => {
        const gatewaySDK = new GatewaySDK(bobSepolia.id);

        nock(SIGNET_GATEWAY_BASE_URL)
            .get('/offramp-registry-address')
            .reply(200, '0xb74a5af78520075f90f4be803153673a162a9776');

        const result = await gatewaySDK.fetchOfframpRegistryAddress();

        expect(result).toBe('0xb74a5af78520075f90f4be803153673a162a9776');
    });

    it('should return true when the order has passed the claim delay', async () => {
        const status: OfframpOrderStatus = 'Accepted';
        const orderTimestamp = Math.floor(Date.now() / 1000) - 7 * 24 * 60 * 60 - 1; // Ensure the timestamp is more than 7 days ago
        const gatewaySDK = new GatewaySDK(bobSepolia.id);

        // Run the function
        const result = await gatewaySDK.canOrderBeUnlocked(
            status,
            orderTimestamp,
            '0xb74a5af78520075f90f4be803153673a162a9776' as Address
        );

        // Assert the result is true (claim delay has passed)
        expect(result).toBe(true);
    });

    it('fetches the correct offramp liquidity', async () => {
        const gatewaySDK = new GatewaySDK(bobSepolia.id);
        const tokenAddress = '0x4496ebE7C8666a8103713EE6e0c08cA0cD25b888'.toLowerCase();
        nock(SIGNET_GATEWAY_BASE_URL).persist().get(`/offramp-liquidity/${tokenAddress}`).reply(200, {
            tokenAddress,
            maxOrderAmount: '861588',
            totalOfframpLiquidity: '861588',
        });

        const offrampLiquidityTokenAddressAsParam = await gatewaySDK.fetchOfframpLiquidity(tokenAddress);

        expect(offrampLiquidityTokenAddressAsParam).toEqual({
            token: tokenAddress,
            maxOrderAmount: BigInt('861588'),
            totalOfframpLiquidity: BigInt('861588'),
        });

        const offrampLiquidityTokenSymbolAsParam = await gatewaySDK.fetchOfframpLiquidity('bobBTC');

        expect(offrampLiquidityTokenSymbolAsParam).toEqual({
            token: tokenAddress,
            maxOrderAmount: BigInt('861588'),
            totalOfframpLiquidity: BigInt('861588'),
        });
    });

    it('should return btc txid for onramp', async () => {
        const gatewaySDK = new GatewaySDK(bob.id);
        const mockBtcSigner = {
            signAllInputs: vi.fn((val) => val),
        };

        const mockWalletClient = {} as Parameters<typeof gatewaySDK.executeQuote>[0]['walletClient'];
        const mockPublicClient = {
            readContract: () => Promise.resolve(10_000n),
            simulateContract: () => Promise.resolve({ request: '🎉' }),
            waitForTransactionReceipt: () =>
                Promise.resolve('0x35f5bca7f984f4ed97888944293b979f3abb198a5716d04e10c6bdc023080075'),
        } as unknown as Parameters<typeof gatewaySDK.executeQuote>[0]['publicClient'];

        const startOrderSpy = vi.spyOn(gatewaySDK, 'startOnrampOrder');
        const finalizeOrderSpy = vi.spyOn(gatewaySDK, 'finalizeOnrampOrder');

        const mockQuote: OnrampQuote & GatewayTokensInfo = {
            outputSatoshis: 990,
            feeBreakdown: {
                overallFeeSats: 10,
                protocolFeeSats: 0,
                affiliateFeeSats: 0,
                executionFeeWei: BigInt(0),
                l1DataFeeWei: BigInt(0),
            },
            gatewayAddress: zeroAddress,
            baseTokenAddress: TBTC_ADDRESS as Address,
            dustThreshold: 1000,
            satoshis: 1000,
            fee: 10,
            bitcoinAddress: 'bc1qafk4yhqvj4wep57m62dgrmutldusqde8adh20d',
            txProofDifficultyFactor: 3,
            strategyAddress: zeroAddress,
            baseToken: TBTC,
            orderDetails: convertOrderDetailsRawToOrderDetails(MOCK_ORDER_DETAILS_RAW),
        };

        nock(`${MAINNET_GATEWAY_BASE_URL}`).post(`/v4/order`).reply(201, {
            uuid: '00000000-0000-0000-0000-000000000000',
            opReturnHash: '0x8d0fd89210149d4219c87fa814a4bcde0c6a36b8fe2dff52b1d3eaa9e7cf0a9a',
        });

        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .patch(`/order/00000000-0000-0000-0000-000000000000`)
            .reply(201, '"f8c934f181cb88ce910f31bda1a6a8c27fdf5fe9c650edad1ccf4c4e0c89f863"');

        const btcTxId = await gatewaySDK.executeQuote({
            quote: {
                type: GatewayOrderType.Onramp,
                data: mockQuote,
                finalOutputSats: mockQuote.satoshis - mockQuote.fee,
                finalFeeSats: mockQuote.fee,
                params: {
                    toChain: 60808,
                    toToken: 'tBTC',
                    toUserAddress: zeroAddress,
                    amount: 1000,
                    fromChain: 'bitcoin',
                    fromToken: 'BTC',
                    fromUserAddress: 'bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq',
                },
            },
            walletClient: mockWalletClient,
            publicClient: mockPublicClient,
            btcSigner: mockBtcSigner,
        });

        expect(btcTxId).toBe('f8c934f181cb88ce910f31bda1a6a8c27fdf5fe9c650edad1ccf4c4e0c89f863');
        expect(startOrderSpy).toHaveBeenCalledOnce();
        expect(finalizeOrderSpy).toHaveBeenCalledOnce();
    });

    it('should return evm txid for offramp', async () => {
        const gatewaySDK = new GatewaySDK(bobSepolia.id);

        const mockWalletClient = {
            writeContract: () => Promise.resolve('0x35f5bca7f984f4ed97888944293b979f3abb198a5716d04e10c6bdc023080075'),
        } as unknown as Parameters<typeof gatewaySDK.executeQuote>[0]['walletClient'];

        const mockPublicClient = {
            multicall: () => Promise.resolve([10_000n, 8]),
            readContract: () => Promise.resolve(10_000n),
            getBalance: () => Promise.resolve(1000000000000000000n), // 1 ETH in wei
            estimateContractGas: () => Promise.resolve(21000n),
            estimateFeesPerGas: () =>
                Promise.resolve({
                    maxFeePerGas: 20000000000n,
                    maxPriorityFeePerGas: 1000000000n,
                }),
            getGasPrice: () => Promise.resolve(20000000000n),
            simulateContract: () => Promise.resolve({ request: '🎉' }),
            waitForTransactionReceipt: () =>
                Promise.resolve('0x35f5bca7f984f4ed97888944293b979f3abb198a5716d04e10c6bdc023080075'),
        } as unknown as Parameters<typeof gatewaySDK.executeQuote>[0]['publicClient'];

        const createOfframpOrderSpy = vi.spyOn(gatewaySDK, 'createOfframpOrder');
        const fetchOfframpRegistryAddressSpy = vi.spyOn(gatewaySDK, 'fetchOfframpRegistryAddress');

        const outputTokenAddress = getTokenAddress(bobSepolia.id, 'bobbtc');
        const searchParams = new URLSearchParams({
            amountInWrappedToken: '1000',
            token: outputTokenAddress,
        });
        nock(`${SIGNET_GATEWAY_BASE_URL}`)
            .get(`/offramp-quote?${searchParams}`)
            .reply(201, {
                amountLockInSat: 10_000,
                registryAddress: zeroAddress,
                feeBreakdown: {
                    overallFeeSats: 932,
                    inclusionFeeSats: 930,
                    protocolFeeSats: 1,
                    affiliateFeeSats: 1,
                    fastestFeeRate: 1000,
                },
            });

        nock(`${SIGNET_GATEWAY_BASE_URL}`).get(`/offramp-registry-address`).reply(201, `${zeroAddress}`);

        const evmTxId = await gatewaySDK.executeQuote({
            quote: {
                type: GatewayOrderType.Offramp,
                data: {
                    amountLockInSat: 0,
                    amountReceiveInSat: 0,
                    deadline: 0,
                    registryAddress: '0x',
                    token: '0x',
                    feeBreakdown: {
                        affiliateFeeSats: 0,
                        fastestFeeRate: 0,
                        inclusionFeeSats: 0,
                        overallFeeSats: 0,
                        protocolFeeSats: 0,
                    },
                },
                finalOutputSats: 0,
                finalFeeSats: 0,
                params: {
                    toChain: 'bitcoin',
                    toToken: 'BTC',
                    toUserAddress: 'tb1qn40xpua4eskjgmueq6fwujex05wdtprh46vkpc',
                    amount: 1000,
                    fromChain: 'bob',
                    fromToken: 'bobBTC',
                    fromUserAddress: zeroAddress,
                },
            },
            walletClient: mockWalletClient,
            publicClient: mockPublicClient,
        });

        expect(evmTxId).toBe('0x35f5bca7f984f4ed97888944293b979f3abb198a5716d04e10c6bdc023080075');
        expect(createOfframpOrderSpy).toHaveBeenCalledOnce();
        expect(fetchOfframpRegistryAddressSpy).toHaveBeenCalledOnce();
    });

    it('should return onramp, offramp and cross-chain orders', async () => {
        const gatewaySDK = new GatewaySDK(bobSepolia.id);

        const getOnrampOrdersSpy = vi
            .spyOn(gatewaySDK, 'getOnrampOrders')
            .mockImplementationOnce(() => Promise.resolve([]));
        const getOfframpOrdersSpy = vi
            .spyOn(gatewaySDK, 'getOfframpOrders')
            .mockImplementationOnce(() => Promise.resolve([]));

        const result = await gatewaySDK.getOrders(zeroAddress);

        expect(result).toEqual([]);
        expect(getOnrampOrdersSpy).toHaveBeenCalledOnce();
        expect(getOfframpOrdersSpy).toHaveBeenCalledOnce();
    });

    it(
        'get offramp gas cost for BOB and Ethereum',
        async () => {
            const gatewaySDK = new GatewaySDK(bob.id);

            // deployed offramp registry address
            nock(MAINNET_GATEWAY_BASE_URL)
                .persist()
                .get('/offramp-registry-address')
                .reply(200, '0x3d65cd168f27aeddeb08ca31cac5e5c12f3bb16d');

            nock(MAINNET_GATEWAY_BASE_URL)
                .persist()
                .get('/offramp-quote')
                .query(true)
                .reply(200, {
                    amountLockInSat: 4000,
                    registryAddress: '0xd7b27b178f6bf290155201109906ad203b6d99b1',
                    feeBreakdown: {
                        overallFeeSats: 886,
                        inclusionFeeSats: 886,
                        protocolFeeSats: 0,
                        affiliateFeeSats: 0,
                        fastestFeeRate: 2,
                    },
                });

            // Test for BOB
            const bobQuote = await gatewaySDK.getQuote({
                fromChain: 'BOB',
                fromToken: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c',
                toChain: 'Bitcoin',
                toToken: 'bitcoin',
                toUserAddress: 'bc1qcwcjsc0mltyt293877552grdktjhnvnn2zh52t',
                fromUserAddress: '0xFAEe001465dE6D7E8414aCDD9eF4aC5A35B2B808',
                amount: 40000,
            });
            if ('gasFee' in bobQuote.data.feeBreakdown) {
                expect(bobQuote.data.feeBreakdown.gasFee).toBeGreaterThan(0);
            } else {
                throw new Error('Expected gasFee to exist on feeBreakdown, but it does not.');
            }

            // Test for Ethereum
            const ethQuote = await gatewaySDK.getQuote({
                fromChain: 'ethereum',
                fromToken: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c',
                toChain: 'Bitcoin',
                toToken: 'bitcoin',
                toUserAddress: 'bc1qcwcjsc0mltyt293877552grdktjhnvnn2zh52t',
                fromUserAddress: '0xFAEe001465dE6D7E8414aCDD9eF4aC5A35B2B808',
                amount: 40000,
            });
            if ('gasFee' in ethQuote.data.feeBreakdown) {
                expect(ethQuote.data.feeBreakdown.gasFee).toBeGreaterThan(0);
            } else {
                throw new Error('Expected gasFee to exist on feeBreakdown, but it does not.');
            }

            // Test for BOB with no from user address
            const bobQuoteWithNoUserAddress = await gatewaySDK.getQuote({
                fromChain: 'BOB',
                fromToken: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c',
                toChain: 'Bitcoin',
                toToken: 'bitcoin',
                toUserAddress: 'bc1qcwcjsc0mltyt293877552grdktjhnvnn2zh52t',
                amount: 40000,
            });
            if ('gasFee' in bobQuoteWithNoUserAddress.data.feeBreakdown) {
                expect(bobQuoteWithNoUserAddress.data.feeBreakdown.gasFee).toBeGreaterThan(0);
            } else {
                throw new Error('Expected gasFee to exist on feeBreakdown, but it does not.');
            }
        },
        { timeout: 30000 } // 30 seconds
    );

    it('should get cross-chain swap orders', async () => {
        const gatewaySDK = new LayerZeroGatewayClient(bob.id);
        const userAddress = '0xFAEe001465dE6D7E8414aCDD9eF4aC5A35B2B808';

        const mockLayerZeroResponse = {
            data: [
                {
                    pathway: {
                        srcEid: 40184, // BOB chain ID
                        dstEid: 30111, // Optimism chain ID
                        sender: {
                            address: '0x123...abc',
                            id: 'sender-1',
                            name: 'BOB Sender',
                            chain: 'bob',
                        },
                        receiver: {
                            address: '0x456...def',
                            id: 'receiver-1',
                            name: 'Optimism Receiver',
                            chain: 'optimism',
                        },
                        id: 'pathway-1',
                        nonce: 1,
                    },
                    source: {
                        status: 'SUCCEEDED',
                        tx: {
                            txHash: '0xsource123',
                            blockHash: '0xblock123',
                            blockNumber: '12345',
                            blockTimestamp: 1700000000,
                            from: userAddress,
                            payload: '0x1234567890abcdef0000000000000064', // Last 16 chars = 0x64 = 100
                            readinessTimestamp: 1700000100,
                        },
                    },
                    destination: {
                        status: 'SUCCEEDED',
                        tx: {
                            txHash: '0xdest456',
                            blockHash: '0xdestblock456',
                            blockNumber: '67890',
                            blockTimestamp: 1700000200,
                            from: '0xdestfrom',
                            payload: '0x',
                            readinessTimestamp: 1700000300,
                        },
                        lzCompose: {
                            status: 'N/A',
                        },
                    },
                },
                {
                    pathway: {
                        srcEid: 30111, // Optimism chain ID
                        dstEid: 40184, // BOB chain ID
                        sender: {
                            address: '0x789...ghi',
                            id: 'sender-2',
                            name: 'Optimism Sender',
                            chain: 'optimism',
                        },
                        receiver: {
                            address: '0xabc...123',
                            id: 'receiver-2',
                            name: 'BOB Receiver',
                            chain: 'bob',
                        },
                        id: 'pathway-2',
                        nonce: 2,
                    },
                    source: {
                        status: 'SUCCEEDED',
                        tx: {
                            txHash: '0xsource789',
                            blockHash: '0xblock789',
                            blockNumber: '54321',
                            blockTimestamp: 1700001000,
                            from: userAddress,
                            payload: '0xabcdef1234567890000000000000012c', // Last 16 chars = 0x12c = 300
                            readinessTimestamp: 1700001100,
                        },
                    },
                    destination: {
                        status: 'WAITING',
                        tx: {
                            txHash: '',
                            blockHash: '',
                            blockNumber: '',
                            blockTimestamp: 0,
                            from: '',
                            payload: '',
                            readinessTimestamp: 0,
                        },
                        lzCompose: {
                            status: 'N/A',
                        },
                    },
                },
                // This order should be filtered out because lzCompose.status is not 'N/A'
                {
                    pathway: {
                        srcEid: 40184,
                        dstEid: 30111,
                        sender: {
                            address: '0xfiltered',
                            id: 'sender-filtered',
                            name: 'Filtered Sender',
                            chain: 'bob',
                        },
                        receiver: {
                            address: '0xfiltered',
                            id: 'receiver-filtered',
                            name: 'Filtered Receiver',
                            chain: 'optimism',
                        },
                        id: 'pathway-filtered',
                        nonce: 3,
                    },
                    source: {
                        status: 'SUCCEEDED',
                        tx: {
                            txHash: '0xfiltered',
                            blockHash: '0xfiltered',
                            blockNumber: '99999',
                            blockTimestamp: 1700002000,
                            from: userAddress,
                            payload: '0x',
                            readinessTimestamp: 1700002100,
                        },
                    },
                    destination: {
                        status: 'SUCCEEDED',
                        tx: {
                            txHash: '0xfiltered',
                            blockHash: '0xfiltered',
                            blockNumber: '88888',
                            blockTimestamp: 1700002200,
                            from: '0xfiltered',
                            payload: '0x',
                            readinessTimestamp: 1700002300,
                        },
                        lzCompose: {
                            status: 'COMPOSE_SUBMITTED',
                        },
                    },
                },
            ],
        };

        nock('https://scan.layerzero-api.com')
            .get(`/v1/messages/wallet/${userAddress}`)
            .reply(200, mockLayerZeroResponse);

        const orders = await gatewaySDK.getCrossChainSwapOrders(userAddress);

        expect(orders).toHaveLength(2); // Third order should be filtered out

        // First order (completed)
        expect(orders[0]).toEqual({
            amount: BigInt(100), // 0x64 = 100
            timestamp: 1700000000,
            status: 'destination-confirmed',
            source: {
                eid: 40184,
                txHash: '0xsource123',
                token: '0x123...abc',
            },
            destination: {
                eid: 30111,
                txHash: '0xdest456',
                token: '0x456...def',
            },
        });

        // Second order (pending)
        expect(orders[1]).toEqual({
            amount: BigInt(300), // 0x12c = 300
            timestamp: 1700001000,
            status: 'destination-pending',
            source: {
                eid: 30111,
                txHash: '0xsource789',
                token: '0x789...ghi',
            },
            destination: {
                eid: 40184,
                txHash: '',
                token: '0xabc...123',
            },
        });
    });

    it('should handle edge cases in getCrossChainSwapOrders', async () => {
        const gatewaySDK = new LayerZeroGatewayClient(bob.id);
        const userAddress = '0xFAEe001465dE6D7E8414aCDD9eF4aC5A35B2B808';

        const mockResponse = {
            data: [
                {
                    pathway: {
                        srcEid: 40184,
                        dstEid: 30111,
                        sender: {
                            address: '0x123',
                            id: 'sender-1',
                            name: 'Sender',
                            chain: 'bob',
                        },
                        receiver: {
                            address: '0x456',
                            id: 'receiver-1',
                            name: 'Receiver',
                            chain: 'optimism',
                        },
                        id: 'pathway-1',
                        nonce: 1,
                    },
                    source: {
                        status: 'WAITING',
                        tx: {
                            txHash: '0xsource123',
                            blockHash: '0xblock123',
                            blockNumber: '12345',
                            blockTimestamp: 1700000000,
                            from: userAddress,
                            payload: '0x', // Empty payload should result in amount 0
                            readinessTimestamp: 1700000100,
                        },
                    },
                    destination: {
                        status: 'WAITING',
                        tx: {
                            txHash: '',
                            blockHash: '',
                            blockNumber: '',
                            blockTimestamp: 0,
                            from: '',
                            payload: '',
                            readinessTimestamp: 0,
                        },
                        lzCompose: {
                            status: 'N/A',
                        },
                    },
                },
                {
                    pathway: {
                        srcEid: 40184,
                        dstEid: 30111,
                        sender: {
                            address: '0x789',
                            id: 'sender-2',
                            name: 'Sender 2',
                            chain: 'bob',
                        },
                        receiver: {
                            address: '0xabc',
                            id: 'receiver-2',
                            name: 'Receiver 2',
                            chain: 'optimism',
                        },
                        id: 'pathway-2',
                        nonce: 2,
                    },
                    source: {
                        status: 'SIMULATION_REVERTED',
                        tx: {
                            txHash: '0xfailedsource',
                            blockHash: '0xfailedblock',
                            blockNumber: '99999',
                            blockTimestamp: 1700001000,
                            from: userAddress,
                            payload: '0xinvalidhex', // Invalid hex should result in amount 0
                            readinessTimestamp: 1700001100,
                        },
                    },
                    destination: {
                        status: 'WAITING',
                        tx: {
                            txHash: '',
                            blockHash: '',
                            blockNumber: '',
                            blockTimestamp: 0,
                            from: '',
                            payload: '',
                            readinessTimestamp: 0,
                        },
                        lzCompose: {
                            status: 'N/A',
                        },
                    },
                },
            ],
        };

        nock('https://scan.layerzero-api.com').get(`/v1/messages/wallet/${userAddress}`).reply(200, mockResponse);

        const orders = await gatewaySDK.getCrossChainSwapOrders(userAddress);

        expect(orders).toHaveLength(2);

        // First order (source pending)
        expect(orders[0]).toEqual({
            amount: BigInt(0), // Empty payload
            timestamp: 1700000000,
            status: 'source-pending',
            source: {
                eid: 40184,
                txHash: '0xsource123',
                token: '0x123',
            },
            destination: {
                eid: 30111,
                txHash: '',
                token: '0x456',
            },
        });

        // Second order (source failed)
        expect(orders[1]).toEqual({
            amount: BigInt(0), // Invalid hex parsing
            timestamp: 1700001000,
            status: 'source-failed',
            source: {
                eid: 40184,
                txHash: '0xfailedsource',
                token: '0x789',
            },
            destination: {
                eid: 30111,
                txHash: '',
                token: '0xabc',
            },
        });
    });

    it('should handle API error for getCrossChainSwapOrders', async () => {
        const gatewaySDK = new LayerZeroGatewayClient(bob.id);
        const userAddress = '0xFAEe001465dE6D7E8414aCDD9eF4aC5A35B2B808';

        nock('https://scan.layerzero-api.com')
            .get(`/v1/messages/wallet/${userAddress}`)
            .reply(500, { message: 'Internal Server Error' });

        await expect(gatewaySDK.getCrossChainSwapOrders(userAddress)).rejects.toThrow('Internal Server Error');
    });

    it('should handle network error for getCrossChainSwapOrders', async () => {
        const gatewaySDK = new LayerZeroGatewayClient(bob.id);
        const userAddress = '0xFAEe001465dE6D7E8414aCDD9eF4aC5A35B2B808';

        nock('https://scan.layerzero-api.com')
            .get(`/v1/messages/wallet/${userAddress}`)
            .replyWithError(new TypeError('Failed to fetch'));

        await expect(gatewaySDK.getCrossChainSwapOrders(userAddress)).rejects.toThrow(
            'Failed to fetch LayerZero send orders'
        );
    });

    it('should return empty array when no cross-chain orders exist', async () => {
        const gatewaySDK = new LayerZeroGatewayClient(bob.id);
        const userAddress = '0xFAEe001465dE6D7E8414aCDD9eF4aC5A35B2B808';

        nock('https://scan.layerzero-api.com').get(`/v1/messages/wallet/${userAddress}`).reply(200, { data: [] });

        const orders = await gatewaySDK.getCrossChainSwapOrders(userAddress);

        expect(orders).toEqual([]);
    });

    it('should return mocked offramp liquidity v2', async () => {
        const gatewaySDK = new GatewaySDK(bob.id);
        const tokenAddress = '0x0555e30da8f98308edb960aa94c0db47230d2b9c';
        const userAddress = zeroAddress;

        const mockk_offramp_liquidity = {
            tokenAddress,
            maxOrderAmountInSats: 50000000,
            totalOfframpLiquidityInSats: 53304097,
            minimumOfframpQuote: {
                minimumAmountInSats: 666,
                calculatedForFeeRate: 1,
            },
        };

        nock(MAINNET_GATEWAY_BASE_URL)
            .get('/v2/offramp-liquidity')
            .query({
                tokenAddress,
                userAddress,
            })
            .reply(200, mockk_offramp_liquidity);

        const offrampLiquidityV2 = await gatewaySDK.fetchOfframpLiquidityV2(tokenAddress, userAddress);

        expect(offrampLiquidityV2).toEqual(mockk_offramp_liquidity);
    });

    it('should return mocked onramp liquidity', async () => {
        const gatewaySDK = new GatewaySDK(bob.id);
        const tokenAddress = '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c'.toLowerCase() as Address;
        const userAddress = '0xFAEe001465dE6D7E8414aCDD9eF4aC5A35B2B808' as Address;

        const mock_onramp_liquidity = {
            tokenAddress,
            maxOrderAmountInSats: 50000000,
            totalOnrampLiquidityInSats: 53408586,
            minSatsAmount: 86,
        };

        nock(MAINNET_GATEWAY_BASE_URL)
            .get('/onramp-liquidity')
            .query({
                tokenAddress,
                userAddress,
            })
            .reply(200, mock_onramp_liquidity);

        const onrampLiquidity = await gatewaySDK.fetchOnrampLiquidity(tokenAddress, userAddress, null);

        expect(onrampLiquidity).toEqual(mock_onramp_liquidity);
    });

    it.skip(
        'get offramp liquidity and get quote e2e',
        async () => {
            const gatewaySDK = new GatewaySDK(bob.id);
            const tokenAddress = '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c'.toLowerCase() as Address;
            const offrampLiquidityV2 = await gatewaySDK.fetchOfframpLiquidityV2(tokenAddress, zeroAddress);

            const minGatewayQuoteOnly = await gatewaySDK.fetchOfframpQuote(
                tokenAddress,
                offrampLiquidityV2.minimumOfframpQuote.minimumAmountInSats,
                zeroAddress
            );
            expect(minGatewayQuoteOnly.amountLockInSat).toEqual(
                offrampLiquidityV2.minimumOfframpQuote.minimumAmountInSats
            );

            const maxGatewayQuoteOnly = await gatewaySDK.fetchOfframpQuote(
                tokenAddress,
                offrampLiquidityV2.maxOrderAmountInSats,
                zeroAddress
            );
            expect(maxGatewayQuoteOnly.amountLockInSat).toEqual(offrampLiquidityV2.maxOrderAmountInSats);
        },
        { timeout: 30000 } // 30 seconds
    );

    it.skip(
        'get onramp liquidity and get quote e2e',
        async () => {
            const gatewaySDK = new GatewaySDK(bob.id);
            const tokenAddress = '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c'.toLowerCase() as Address;
            const userAddress = '0xFAEe001465dE6D7E8414aCDD9eF4aC5A35B2B808' as Address;
            const onrampLiquidity = await gatewaySDK.fetchOnrampLiquidity(tokenAddress, userAddress, null);

            const minGatewayQuoteOnly = await gatewaySDK.getOnrampQuote({
                fromUserAddress: userAddress,
                fromChain: 'Bitcoin',
                fromToken: 'bitcoin',
                toChain: bob.id,
                toToken: 'WBTC (OFT)',
                toUserAddress: tokenAddress,
                amount: onrampLiquidity.minSatsAmount,
            });
            expect(minGatewayQuoteOnly.satoshis).toEqual(onrampLiquidity.minSatsAmount);

            const maxGatewayQuoteOnly = await gatewaySDK.getOnrampQuote({
                fromUserAddress: userAddress,
                fromChain: 'Bitcoin',
                fromToken: 'bitcoin',
                toChain: bob.id,
                toToken: 'WBTC (OFT)',
                toUserAddress: tokenAddress,
                amount: onrampLiquidity.maxOrderAmountInSats,
            });
            expect(maxGatewayQuoteOnly.satoshis).toEqual(onrampLiquidity.maxOrderAmountInSats);
        },
        { timeout: 30000 } // 30 seconds
    );
});
