import * as bitcoin from 'bitcoinjs-lib';
import nock from 'nock';
import { zeroAddress } from 'viem';
import { Address } from 'viem/accounts';
import { bob, bobSepolia } from 'viem/chains';
import { afterEach, assert, describe, expect, it, vi } from 'vitest';
import { GatewaySDK } from '../src/gateway';
import { MAINNET_GATEWAY_BASE_URL, SIGNET_GATEWAY_BASE_URL } from '../src/gateway/client';
import { getTokenAddress, SYMBOL_LOOKUP } from '../src/gateway/tokens';
import {
    GatewayQuote,
    GatewayTokensInfo,
    OfframpOrder,
    OfframpOrderStatus,
    OrderDetailsRaw,
} from '../src/gateway/types';
import { toHexScriptPubKey, convertOrderDetailsRawToOrderDetails } from '../src/gateway/utils';

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
        };

        const assertMockQuote = {
            ...mockQuote,
            orderDetails: orderDetails,
        };

        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .get(`/v4/quote/${TBTC_ADDRESS}?userAddress=${zeroAddress}&satoshis=1000`)
            .times(5)
            .reply(200, mockQuote);

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
                onrampQuote: assertMockQuote,
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
                onrampQuote: assertMockQuote,
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
                onrampQuote: assertMockQuote,
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
                onrampQuote: assertMockQuote,
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
            .reply(200, mockQuote);
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
                onrampQuote: assertMockQuote,
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
            fee: 10,
            bitcoinAddress: 'bc1qafk4yhqvj4wep57m62dgrmutldusqde8adh20d',
            txProofDifficultyFactor: 3,
            strategyAddress: zeroAddress,
            orderDetails: convertOrderDetailsRawToOrderDetails(MOCK_ORDER_DETAILS_RAW),
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

        assert.lengthOf(enrichedTokens, tokens.length);

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
            ]);

        const gatewaySDK = new GatewaySDK(bob.id);
        const orders = await gatewaySDK.getOnrampOrders(zeroAddress);
        assert.lengthOf(orders, 7);

        assert.strictEqual(orders[0].getTokenAmount(), '2000'); // success (staking)
        assert.strictEqual(orders[1].getTokenAmount(), undefined); // pending (staking)
        assert.strictEqual(orders[2].getTokenAmount(), 10000000000000); // failed (staking)
        assert.strictEqual(orders[3].getTokenAmount(), 10000000000000); // pending (swapping)
        assert.strictEqual(orders[4].getTokenAmount(), 10000000000000); // success (swapping)
        assert.strictEqual(orders[5].getTokenAmount(), 1000); // failed (staking)
        assert.strictEqual(orders[6].getTokenAmount(), 1000); // success (swapping)

        assert.strictEqual(orders[0].getToken()!.address, SOLVBTC_ADDRESS);
        assert.strictEqual(orders[1].getToken(), undefined);
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
            canOrderBeUnlocked: true,
            shouldFeesBeBumped: false,
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

    it('should return error for taproot address', async () => {
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

        await expect(
            gatewaySDK.createOfframpOrder(
                {
                    amountLockInSat: 0,
                    deadline: 0,
                    registryAddress: '0x',
                    token: '0x',
                    feeBreakdown: {
                        overallFeeSats: 0,
                        inclusionFeeSats: 0,
                        protocolFeeSats: 0,
                        affiliateFeeSats: 0,
                        fastestFeeRate: 0,
                    },
                },
                {
                    fromToken: '0xda472456b1a6a2fc9ae7edb0e007064224d4284c',
                    amount: 100000000000000,
                    fromUserAddress: '0xFAEe001465dE6D7E8414aCDD9eF4aC5A35B2B808',
                    toUserAddress: 'tb1p5d2m6d7yje35xqnk2wczghak6q20c6rqw303p58wrlzhue8t4z9s9y304z', // P2TR taproot address
                    fromChain: 'bob',
                    toChain: 'bitcoin',
                    toToken: 'bitcoin',
                }
            )
        ).rejects.toThrowError('Only following bitcoin address types are supported P2PKH, P2WPKH, P2SH or P2WSH.');
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

        const mockQuote: GatewayQuote & GatewayTokensInfo = {
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
                onrampQuote: mockQuote,
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
                offrampQuote: {
                    amountLockInSat: 0,
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

    it('should return onramp and offramp orders', async () => {
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
});
