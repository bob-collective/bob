import * as bitcoin from 'bitcoinjs-lib';
import { MaxUint256, ZeroAddress } from 'ethers';
import nock from 'nock';
import { createPublicClient, encodeAbiParameters, http, keccak256, maxUint256, numberToHex, zeroAddress } from 'viem';
import { Address } from 'viem/accounts';
import { bobSepolia } from 'viem/chains';
import { afterEach, assert, describe, expect, it, vi } from 'vitest';
import { GatewaySDK } from '../src/gateway';
import { MAINNET_GATEWAY_BASE_URL, SIGNET_GATEWAY_BASE_URL, toHexScriptPubKey } from '../src/gateway/client';
import { getTokenAddress, SYMBOL_LOOKUP } from '../src/gateway/tokens';
import {
    StakeParams,
    Chain,
    ChainId,
    GatewayQuote,
    GatewayTokensInfo,
    OfframpOrder,
    OfframpOrderStatus,
    StakeTransactionParams,
} from '../src/gateway/types';
import { access } from 'fs';

const TBTC = SYMBOL_LOOKUP[ChainId.BOB]['tbtc'];
const TBTC_ADDRESS = TBTC.address;
const SOLVBTC = SYMBOL_LOOKUP[ChainId.BOB]['solvbtc'];
const SOLVBTC_ADDRESS = SOLVBTC.address;

afterEach(() => {
    nock.cleanAll();
});

describe('Gateway Tests', () => {
    it('should get chains', async () => {
        const gatewaySDK = new GatewaySDK('bob');
        assert.deepEqual(gatewaySDK.getChains(), Object.values(Chain));
    });

    it('should reject invalid chain', async () => {
        expect(() => {
            // @ts-expect-error invalid chain id
            new GatewaySDK('bob-testnet');
        }).toThrowError('Invalid chain');
    });

    it('should get quote', async () => {
        const gatewaySDK = new GatewaySDK('mainnet');

        const mockQuote = {
            gatewayAddress: ZeroAddress,
            baseTokenAddress: TBTC_ADDRESS,
            dustThreshold: 1000,
            satoshis: 1000,
            fee: 10,
            bitcoinAddress: '',
            txProofDifficultyFactor: 3,
            strategyAddress: ZeroAddress,
            baseToken: TBTC,
            outputToken: TBTC,
        };

        nock(`${MAINNET_GATEWAY_BASE_URL}`).get(`/quote/${TBTC_ADDRESS}?satoshis=1000`).times(5).reply(200, mockQuote);

        assert.deepEqual(
            await gatewaySDK.getQuote({
                type: 'onramp',
                params: {
                    toChain: 'BOB',
                    toToken: 'tBTC',
                    toUserAddress: ZeroAddress,
                    amount: 1000,
                },
            }),
            mockQuote
        );
        assert.deepEqual(
            await gatewaySDK.getQuote({
                type: 'onramp',
                params: {
                    toChain: 'bob',
                    toToken: 'tbtc',
                    toUserAddress: ZeroAddress,
                    amount: 1000,
                },
            }),
            mockQuote
        );
        assert.deepEqual(
            await gatewaySDK.getQuote({
                type: 'onramp',
                params: {
                    toChain: 60808,
                    toToken: 'tbtc',
                    toUserAddress: ZeroAddress,
                    amount: 1000,
                },
            }),
            mockQuote
        );
        assert.deepEqual(
            await gatewaySDK.getQuote({
                type: 'onramp',
                params: {
                    toChain: 'BOB',
                    toToken: TBTC_ADDRESS,
                    toUserAddress: ZeroAddress,
                    amount: 1000,
                },
            }),
            mockQuote
        );
        assert.deepEqual(
            await gatewaySDK.getQuote({
                type: 'onramp',
                params: {
                    toChain: 'BOB',
                    toToken: 'tBTC',
                    toUserAddress: ZeroAddress,
                    amount: 1000,
                    gasRefill: 5,
                },
            }),
            { ...mockQuote, fee: 15 }
        );

        // get the total available without amount
        nock(`${MAINNET_GATEWAY_BASE_URL}`).get(`/quote/${TBTC_ADDRESS}`).reply(200, mockQuote);
        assert.deepEqual(
            await gatewaySDK.getQuote({
                type: 'onramp',
                params: {
                    toChain: 'BOB',
                    toToken: TBTC_ADDRESS,
                    toUserAddress: ZeroAddress,
                },
            }),
            mockQuote
        );
    });

    it('should throw error on invalid token', async () => {
        const gatewaySDK = new GatewaySDK('mainnet');
        await expect(async () => {
            await gatewaySDK.getQuote({
                type: 'onramp',
                params: {
                    toChain: 'BOB',
                    toToken: 'unknownToken',
                    toUserAddress: ZeroAddress,
                    amount: 1000,
                },
            });
        }).rejects.toThrowError('Unknown output token');
    });

    it('should reject invalid network', async () => {
        const gatewaySDK = new GatewaySDK('testnet');
        await expect(async () => {
            await gatewaySDK.getQuote({
                type: 'onramp',
                params: {
                    toChain: 'BOB',
                    toToken: 'tbtc',
                    toUserAddress: ZeroAddress,
                    amount: 1000,
                },
            });
        }).rejects.toThrowError('Invalid output chain');

        await expect(async () => {
            await gatewaySDK.getQuote({
                type: 'onramp',
                params: {
                    toChain: 'unknownChain',
                    toToken: 'tbtc',
                    toUserAddress: ZeroAddress,
                    amount: 1000,
                },
            });
        }).rejects.toThrowError('Invalid output chain');
    });

    it('should start order', { timeout: 50000 }, async () => {
        const gatewaySDK = new GatewaySDK('bob');

        const mockQuote = {
            gatewayAddress: ZeroAddress,
            baseTokenAddress: TBTC_ADDRESS,
            dustThreshold: 1000,
            satoshis: 1000,
            fee: 10,
            bitcoinAddress: 'bc1qafk4yhqvj4wep57m62dgrmutldusqde8adh20d',
            txProofDifficultyFactor: 3,
            strategyAddress: ZeroAddress,
        };

        nock(`${MAINNET_GATEWAY_BASE_URL}`).post(`/order`).reply(201, {
            uuid: '00000000-0000-0000-0000-000000000000',
            opReturnHash: '0x8d0fd89210149d4219c87fa814a4bcde0c6a36b8fe2dff52b1d3eaa9e7cf0a9a',
        });

        await expect(async () => {
            await gatewaySDK.startOrder(mockQuote, {
                toChain: 'BOB',
                toToken: 'tBTC',
                toUserAddress: '2N8DbeaBdjkktkRzaKL1tHj9FQELV7jA8Re',
                amount: 1000,
                fromChain: 'Bitcoin',
                fromToken: 'BTC',
                fromUserAddress: 'bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq',
            });
        }).rejects.toThrowError('Invalid user address');

        const result = await gatewaySDK.startOrder(mockQuote, {
            toChain: 'BOB',
            toToken: 'tBTC',
            toUserAddress: ZeroAddress,
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
                    strategyAddress: ZeroAddress,
                    inputTokenAddress: TBTC_ADDRESS,
                    strategyName: 'Pell Network (tBTC)',
                    strategyType: 'staking',
                },
            ]);
        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .get(`/quote/${TBTC_ADDRESS}?satoshis=1000&strategy=${ZeroAddress}`)
            .times(4)
            .reply(200, {
                gatewayAddress: ZeroAddress,
                dustThreshold: 1000,
                satoshis: 1000,
                fee: 10,
                bitcoinAddress: '',
                txProofDifficultyFactor: 3,
                strategyAddress: ZeroAddress,
            });

        const gatewaySDK = new GatewaySDK('bob');
        const strategies = await gatewaySDK.getStrategies();

        assert.lengthOf(strategies, 1);
        assert.isDefined(strategies[0].inputToken);
        assert.isNull(strategies[0].outputToken);
        assert.equal(strategies[0].integration.slug, 'pell-network-tbtc');

        const strategy = strategies[0];
        await gatewaySDK.getQuote({
            type: 'onramp',
            params: {
                toUserAddress: ZeroAddress,
                amount: 1000,
                toChain: strategy.chain.chainId,
                toToken: strategy.inputToken.symbol,
                strategyAddress: strategy.address,
            },
        });
    });

    it('should get tokens', async () => {
        nock(`${MAINNET_GATEWAY_BASE_URL}`).get(`/tokens?includeStrategies=false`).reply(200, [ZeroAddress]);

        const gatewaySDK = new GatewaySDK('bob');
        assert.deepEqual(await gatewaySDK.getTokenAddresses(false), [ZeroAddress]);
    });

    it.skip('should get enriched tokens', async () => {
        const gatewaySDK = new GatewaySDK('bob');

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
            gatewayAddress: ZeroAddress,
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
            .get(`/orders/${ZeroAddress}`)
            .reply(200, [
                // staking - success
                {
                    ...mockOrder,
                    satoshis: 1000,
                    fee: 0,
                    status: true,
                    strategyAddress: ZeroAddress,
                    outputTokenAmount: '2000',
                    outputTokenAddress: SOLVBTC_ADDRESS,
                },
                // staking - pending
                {
                    ...mockOrder,
                    satoshis: 1000,
                    fee: 0,
                    strategyAddress: ZeroAddress,
                },
                // staking - failed
                {
                    ...mockOrder,
                    satoshis: 1000,
                    fee: 0,
                    status: true,
                    strategyAddress: ZeroAddress,
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
                    baseTokenAddress: SYMBOL_LOOKUP[ChainId.BOB]['wbtc'],
                    satoshis: 1000,
                    fee: 0,
                    status: true,
                    strategyAddress: ZeroAddress,
                },
                // swapping - success (wBTC)
                {
                    ...mockOrder,
                    baseTokenAddress: SYMBOL_LOOKUP[ChainId.BOB]['wbtc'],
                    satoshis: 1000,
                    fee: 0,
                    status: true,
                },
            ]);

        const gatewaySDK = new GatewaySDK('bob');
        const orders = await gatewaySDK.getOnrampOrders(ZeroAddress);
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

    // Skipping this test as it is likely to fail on the testnet once the user transfers their funds
    // or if the strategy is removed from the testnet. Use nock for mocking the responses in the meantime for other test.
    it.skip('should correctly retrieve stake info and simulate call testnet strategy', async () => {
        const gatewaySDK = new GatewaySDK('testnet');
        const params: StakeParams = {
            strategyAddress: '0x06cEA150E651236499319d78f92791f0FAe6FE67' as Address,
            token: '0x6744babdf02dcf578ea173a9f0637771a9e1c4d0' as Address,
            sender: '0x5e46D220eC8B01f55B70Dbb503c697f6E231eb65' as Address, // sender needs to hold the input token
            receiver: '0x5e46D220eC8B01f55B70Dbb503c697f6E231eb65' as Address,
            amount: 100n,
            amountOutMin: 0n,
        };

        const expected: StakeTransactionParams = {
            strategyAddress: params.strategyAddress,
            strategyABI: expect.any(Array), // Assuming ABI is an array
            strategyFunctionName: 'handleGatewayMessageWithSlippageArgs',
            strategyArgs: [params.token, params.amount, params.receiver, { amountOutMin: params.amountOutMin }],
            address: params.sender,
        };

        const result = await gatewaySDK.buildStake(params);
        expect(result).toMatchObject(expected);

        // call method
        const publicClient = createPublicClient({
            chain: bobSepolia,
            transport: http(),
        });

        // For the allowances mapping in OpenZeppelin ERC20, the base slot is 1.
        const baseSlot = 1n;

        // Compute the inner slot for the owner using viem's encodeAbiParameters.
        const innerSlot = keccak256(
            encodeAbiParameters(
                [
                    { name: 'owner', type: 'address' },
                    { name: 'slot', type: 'uint256' },
                ],
                [params.sender, baseSlot]
            )
        );

        // Compute the final allowance storage key using the spender (strategyAddress) and the inner slot.
        // The second parameter is encoded as bytes32.
        const allowanceSlot = keccak256(
            encodeAbiParameters(
                [
                    { name: 'spender', type: 'address' },
                    { name: 'innerSlot', type: 'bytes32' },
                ],
                [params.strategyAddress, innerSlot]
            )
        );

        const maxAllowance = numberToHex(maxUint256);
        const { request } = await publicClient.simulateContract({
            address: result.strategyAddress, // Ensure correct type
            abi: result.strategyABI,
            functionName: result.strategyFunctionName,
            args: result.strategyArgs,
            account: result.address, // Ensure correct type
            stateOverride: [
                // overriding token allowance
                {
                    address: params.token,
                    stateDiff: [
                        {
                            slot: allowanceSlot,
                            value: maxAllowance,
                        },
                    ],
                },
            ],
        });
        // Assert that the request object is valid
        expect(request).toEqual(
            expect.objectContaining({
                abi: expect.any(Array),
                address: expect.any(String),
                args: expect.any(Array),
                functionName: expect.any(String),
                account: expect.any(Object),
            })
        );
    });

    it('should throw error for invalid strategy address in buildStake', async () => {
        const gatewaySDK = new GatewaySDK('testnet');
        const params: StakeParams = {
            strategyAddress: ZeroAddress as Address,
            token: '0x6744babdf02dcf578ea173a9f0637771a9e1c4d0' as Address,
            sender: '0x5e46D220eC8B01f55B70Dbb503c697f6E231eb65' as Address,
            receiver: '0x5e46D220eC8B01f55B70Dbb503c697f6E231eb65' as Address,
            amount: 100n,
            amountOutMin: 0n,
        };

        await expect(gatewaySDK.buildStake(params)).rejects.toThrowError(
            `Strategy with address ${ZeroAddress} not found.`
        );
    });

    it('should throw error for invalid token address in buildStake', async () => {
        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .get(`/strategies`)
            .reply(200, [
                {
                    strategyAddress: ZeroAddress,
                    inputTokenAddress: TBTC_ADDRESS,
                    strategyName: 'Pell Network (tBTC)',
                    strategyType: 'staking',
                },
            ]);
        const gatewaySDK = new GatewaySDK('bob');
        const params: StakeParams = {
            strategyAddress: ZeroAddress as Address,
            token: ZeroAddress as Address,
            sender: '0x5e46D220eC8B01f55B70Dbb503c697f6E231eb65' as Address,
            receiver: '0x5e46D220eC8B01f55B70Dbb503c697f6E231eb65' as Address,
            amount: 100n,
            amountOutMin: 0n,
        };

        await expect(gatewaySDK.buildStake(params)).rejects.toThrowError(
            `Provided token ${params.token} does not match strategy's input token ${TBTC_ADDRESS}.`
        );
    });

    it('should throw error for invalid sender address in buildStake', async () => {
        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .get(`/strategies`)
            .reply(200, [
                {
                    strategyAddress: ZeroAddress,
                    inputTokenAddress: TBTC_ADDRESS,
                    strategyName: 'Pell Network (tBTC)',
                    strategyType: 'staking',
                },
            ]);
        const gatewaySDK = new GatewaySDK('bob');
        const params: StakeParams = {
            strategyAddress: ZeroAddress as Address,
            token: TBTC_ADDRESS as Address,
            sender: 'ab5e46D220eC8B01f55B70Dbb503c697f6E231eb65' as Address,
            receiver: '0x5e46D220eC8B01f55B70Dbb503c697f6E231eb65' as Address,
            amount: 100n,
            amountOutMin: 0n,
        };
        await expect(gatewaySDK.buildStake(params)).rejects.toThrowError(`Invalid EVM address detected.`);
    });

    it('should return valid staking info', async () => {
        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .get(`/strategies`)
            .reply(200, [
                {
                    strategyAddress: ZeroAddress,
                    inputTokenAddress: TBTC_ADDRESS,
                    strategyName: 'Pell Network (tBTC)',
                    strategyType: 'staking',
                },
            ]);
        const gatewaySDK = new GatewaySDK('bob');
        const params: StakeParams = {
            strategyAddress: ZeroAddress as Address,
            token: TBTC_ADDRESS as Address,
            sender: '0x5e46D220eC8B01f55B70Dbb503c697f6E231eb65' as Address,
            receiver: '0x5e46D220eC8B01f55B70Dbb503c697f6E231eb65' as Address,
            amount: 100n,
            amountOutMin: 0n,
        };

        const expected: StakeTransactionParams = {
            strategyAddress: params.strategyAddress,
            strategyABI: expect.any(Array), // Assuming ABI is an array
            strategyFunctionName: 'handleGatewayMessageWithSlippageArgs',
            strategyArgs: [params.token, params.amount, params.receiver, { amountOutMin: params.amountOutMin }],
            address: params.sender,
        };

        const result = await gatewaySDK.buildStake(params);
        expect(result).toMatchObject(expected);
    });

    it('should get valid create offramp quote', async () => {
        const gatewaySDK = new GatewaySDK('signet');
        nock(`${SIGNET_GATEWAY_BASE_URL}`).get('/offramp-quote').query(true).reply(200, {
            amountLockInSat: 10000000000000,
            feesInSat: 385,
            registryAddress: '0xd7b27b178f6bf290155201109906ad203b6d99b1',
            feeRate: 1,
        });

        const result = await gatewaySDK.createOfframpOrder({
            fromToken: '0xda472456b1a6a2fc9ae7edb0e007064224d4284c',
            amount: 100000000000000,
            fromUserAddress: '0xFAEe001465dE6D7E8414aCDD9eF4aC5A35B2B808',
            toUserAddress: 'tb1qn40xpua4eskjgmueq6fwujex05wdtprh46vkpc',
        });

        expect(result.offrampArgs[0]).to.deep.equal({
            satAmountToLock: BigInt('10000000000000'),
            satFeesMax: BigInt('385'),
            orderCreationDeadline: result.offrampArgs[0].orderCreationDeadline,
            outputScript: '0x1600149d5e60f3b5cc2d246f990692ee4b267d1cd58477',
            token: '0xda472456b1a6a2fc9ae7edb0e007064224d4284c',
            orderOwner: '0xFAEe001465dE6D7E8414aCDD9eF4aC5A35B2B808',
        });
    });

    it('should return valid offramp orders', async () => {
        const gatewaySDK = new GatewaySDK('signet');
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
            .reply(200, '"0xb74a5af78520075f90f4be803153673a162a9776"');

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
        const gatewaySDK = new GatewaySDK('signet');
        nock(`${SIGNET_GATEWAY_BASE_URL}`).get('/offramp-quote').query(true).reply(200, {
            amountLockInSat: 10000000000000,
            feesInSat: 385,
            registryAddress: '0xd7b27b178f6bf290155201109906ad203b6d99b1',
            feeRate: 1,
        });

        await expect(
            gatewaySDK.createOfframpOrder({
                fromToken: '0xda472456b1a6a2fc9ae7edb0e007064224d4284c',
                amount: 100000000000000,
                fromUserAddress: '0xFAEe001465dE6D7E8414aCDD9eF4aC5A35B2B808',
                toUserAddress: 'tb1p5d2m6d7yje35xqnk2wczghak6q20c6rqw303p58wrlzhue8t4z9s9y304z', // P2TR taproot address
            })
        ).rejects.toThrowError('Only following bitcoin address types are supported P2PKH, P2WPKH, P2SH or P2WSH.');
    });

    it('fetches the correct offramp registry address', async () => {
        const gatewaySDK = new GatewaySDK('signet');

        nock(SIGNET_GATEWAY_BASE_URL)
            .get('/offramp-registry-address')
            .reply(200, '0xb74a5af78520075f90f4be803153673a162a9776');

        const result = await gatewaySDK.fetchOfframpRegistryAddress();

        expect(result).toBe('0xb74a5af78520075f90f4be803153673a162a9776');
    });

    it('should return true when the order has passed the claim delay', async () => {
        const status: OfframpOrderStatus = 'Accepted';
        const orderTimestamp = Math.floor(Date.now() / 1000) - 7 * 24 * 60 * 60 - 1; // Ensure the timestamp is more than 7 days ago
        const gatewaySDK = new GatewaySDK('signet');

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
        const gatewaySDK = new GatewaySDK('signet');
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
        const gatewaySDK = new GatewaySDK('mainnet');

        const mockBtcSigner = {
            signAllInputs: vi.fn((val) => val),
        };

        const mockWalletClient = {};
        const mockPublicClient = {};

        const startOrderSpy = vi.spyOn(gatewaySDK, 'startOrder');
        const finalizeOrderSpy = vi.spyOn(gatewaySDK, 'finalizeOrder');

        const mockQuote = {
            gatewayAddress: ZeroAddress,
            baseTokenAddress: TBTC_ADDRESS,
            dustThreshold: 1000,
            satoshis: 1000,
            fee: 10,
            bitcoinAddress: 'bc1qafk4yhqvj4wep57m62dgrmutldusqde8adh20d',
            txProofDifficultyFactor: 3,
            strategyAddress: ZeroAddress,
        };

        nock(`${MAINNET_GATEWAY_BASE_URL}`).post(`/order`).reply(201, {
            uuid: '00000000-0000-0000-0000-000000000000',
            opReturnHash: '0x8d0fd89210149d4219c87fa814a4bcde0c6a36b8fe2dff52b1d3eaa9e7cf0a9a',
        });

        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .patch(`/order/00000000-0000-0000-0000-000000000000`)
            .reply(201, '"f8c934f181cb88ce910f31bda1a6a8c27fdf5fe9c650edad1ccf4c4e0c89f863"');

        const btcTxId = await gatewaySDK.executeQuote(
            {
                type: 'onramp',
                quote: mockQuote as GatewayQuote & GatewayTokensInfo,
                params: {
                    toChain: 'bob',
                    toToken: 'tBTC',
                    toUserAddress: zeroAddress,
                    amount: 1000,
                    fromChain: 'bitcoin',
                    fromToken: 'BTC',
                    fromUserAddress: 'bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq',
                },
            },
            mockBtcSigner,
            mockWalletClient as Parameters<typeof gatewaySDK.executeQuote>[2],
            mockPublicClient as Parameters<typeof gatewaySDK.executeQuote>[3]
        );

        expect(btcTxId).toBe('f8c934f181cb88ce910f31bda1a6a8c27fdf5fe9c650edad1ccf4c4e0c89f863');
        expect(startOrderSpy).toHaveBeenCalledOnce();
        expect(finalizeOrderSpy).toHaveBeenCalledOnce();
    });

    it('should return evm txid for offramp', async () => {
        const gatewaySDK = new GatewaySDK('signet');

        const mockBtcSigner = {};

        const mockWalletClient = {
            writeContract: () => Promise.resolve('0x35f5bca7f984f4ed97888944293b979f3abb198a5716d04e10c6bdc023080075'),
        } as unknown as Parameters<typeof gatewaySDK.executeQuote>[2];

        const mockPublicClient = {
            readContract: () => Promise.resolve(10_000n),
            simulateContract: () => Promise.resolve({ request: '🎉' }),
            waitForTransactionReceipt: () =>
                Promise.resolve('0x35f5bca7f984f4ed97888944293b979f3abb198a5716d04e10c6bdc023080075'),
        } as unknown as Parameters<typeof gatewaySDK.executeQuote>[3];

        const createOfframpOrderSpy = vi.spyOn(gatewaySDK, 'createOfframpOrder');
        const fetchOfframpRegistryAddressSpy = vi.spyOn(gatewaySDK, 'fetchOfframpRegistryAddress');

        const outputTokenAddress = getTokenAddress(ChainId.BOB_SEPOLIA, 'bobbtc');
        const searchParams = new URLSearchParams({
            amountInWrappedToken: '1000',
            token: outputTokenAddress,
        });
        nock(`${SIGNET_GATEWAY_BASE_URL}`).get(`/offramp-quote?${searchParams}`).reply(201, {
            amountLockInSat: 10_000,
            feesInSat: 932,
            feeRate: 1000,
            registryAddress: zeroAddress,
        });

        nock(`${SIGNET_GATEWAY_BASE_URL}`).get(`/offramp-registry-address`).reply(201, `"${zeroAddress}"`);

        const evmTxId = await gatewaySDK.executeQuote(
            {
                type: 'offramp',
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
            mockBtcSigner as Parameters<typeof gatewaySDK.executeQuote>[1],
            mockWalletClient as Parameters<typeof gatewaySDK.executeQuote>[2],
            mockPublicClient as Parameters<typeof gatewaySDK.executeQuote>[3]
        );

        expect(evmTxId).toBe('0x35f5bca7f984f4ed97888944293b979f3abb198a5716d04e10c6bdc023080075');
        expect(createOfframpOrderSpy).toHaveBeenCalledOnce();
        expect(fetchOfframpRegistryAddressSpy).toHaveBeenCalledOnce();
    });

    it('should return evm txid for stake', async () => {
        const params: StakeParams = {
            strategyAddress: '0x06cEA150E651236499319d78f92791f0FAe6FE67' as Address,
            token: '0x6744babdf02dcf578ea173a9f0637771a9e1c4d0' as Address,
            sender: '0x5e46D220eC8B01f55B70Dbb503c697f6E231eb65' as Address, // sender needs to hold the input token
            receiver: '0x5e46D220eC8B01f55B70Dbb503c697f6E231eb65' as Address,
            amount: 100n,
            amountOutMin: 0n,
        };

        const gatewaySDK = new GatewaySDK('testnet');

        const mockBtcSigner = {};

        const mockWalletClient = {
            account: {
                address: zeroAddress,
            },
            writeContract: () => Promise.resolve('0x35f5bca7f984f4ed97888944293b979f3abb198a5716d04e10c6bdc023080075'),
        } as unknown as Parameters<typeof gatewaySDK.executeQuote>[2];

        const mockPublicClient = {
            readContract: () => Promise.resolve(10_000n),
            simulateContract: () => Promise.resolve({ request: '🎉' }),
            waitForTransactionReceipt: () =>
                Promise.resolve('0x35f5bca7f984f4ed97888944293b979f3abb198a5716d04e10c6bdc023080075'),
        } as unknown as Parameters<typeof gatewaySDK.executeQuote>[3];

        const evmTxId = await gatewaySDK.executeQuote(
            {
                type: 'stake',
                params,
            },
            mockBtcSigner as Parameters<typeof gatewaySDK.executeQuote>[1],
            mockWalletClient as Parameters<typeof gatewaySDK.executeQuote>[2],
            mockPublicClient as Parameters<typeof gatewaySDK.executeQuote>[3]
        );

        expect(evmTxId).toBe('0x35f5bca7f984f4ed97888944293b979f3abb198a5716d04e10c6bdc023080075');
    });

    it('should return onramp and offramp orders', async () => {
        const gatewaySDK = new GatewaySDK('testnet');

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

    it('test to check getQuote types', async () => {
        const gatewaySDK = new GatewaySDK('mainnet');
        const mockQuote = {
            gatewayAddress: ZeroAddress,
            baseTokenAddress: TBTC_ADDRESS,
            dustThreshold: 1000,
            satoshis: 1000,
            fee: 10,
            bitcoinAddress: '',
            txProofDifficultyFactor: 3,
            strategyAddress: ZeroAddress,
            baseToken: TBTC,
            outputToken: TBTC,
        };
        nock(`${MAINNET_GATEWAY_BASE_URL}`).get(`/quote/${TBTC_ADDRESS}?satoshis=1000`).times(5).reply(200, mockQuote);
        const searchParams = new URLSearchParams({
            amountInWrappedToken: '1000',
            token: TBTC_ADDRESS,
        });
        nock(`${MAINNET_GATEWAY_BASE_URL}`).get(`/offramp-quote?${searchParams}`).reply(201, {
            amountLockInSat: 1000,
            feesInSat: 0,
            feeRate: 0,
            registryAddress: zeroAddress,
        });


        const result_onramp = await gatewaySDK.getQuote({
            type: 'onramp',
            params: {
                toChain: 'BOB',
                toToken: TBTC_ADDRESS,
                toUserAddress: ZeroAddress,
                amount: 1000,
            },
        });

        // Optionally, print keys or typeof each field
        console.log('Type keys offramp:', Object.keys(result_onramp));

        const result_offramp = await gatewaySDK.getQuote({
            type: 'offramp',
            params: {
                tokenAddress: TBTC_ADDRESS as Address,
                amount: 1000,
            },
        });

        // Optionally, print keys or typeof each field
        console.log('Type keys offramp:', Object.keys(result_offramp));

    });
});
