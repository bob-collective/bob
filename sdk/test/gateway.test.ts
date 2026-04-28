import nock from 'nock';
import {
    Account,
    Address,
    maxUint256,
    PublicClient,
    Transport,
    Chain as ViemChain,
    WalletClient,
    zeroAddress,
} from 'viem';
import { afterEach, assert, describe, expect, it, vi } from 'vitest';
import { GatewayError, GatewayErrorCode, GatewaySDK, isGatewayError } from '../src/gateway';
import { ETHEREUM_USDT_ADDRESS, MAINNET_GATEWAY_BASE_URL } from '../src/gateway/client';
import {
    GatewayOrderInfo,
    GatewayQuoteOneOf,
    GatewayQuoteOneOf1,
    GatewayQuoteOneOf2,
    GatewayQuoteV2OneOf,
    instanceOfGatewayQuoteOneOf,
    instanceOfGatewayQuoteOneOf1,
    instanceOfGatewayQuoteOneOf2,
} from '../src/gateway/generated-client';
import { BitcoinSigner } from '../src/gateway/types';

const WBTC_OFT_ADDRESS = '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c';
const MOCK_SIGNED_QUOTE_DATA = 'signed-quote-data';

afterEach(() => {
    nock.cleanAll();
});

describe('Gateway Tests', () => {
    it('should get quote', async () => {
        const gatewaySDK = new GatewaySDK();

        const mockOnrampQuote: GatewayQuoteOneOf = {
            onramp: {
                dstChain: 'bob',
                dstToken: WBTC_OFT_ADDRESS,
                executionFees: {
                    address: zeroAddress,
                    amount: '10',
                    chain: 'bob',
                },
                feeBreakdown: {
                    protocolFee: {
                        address: zeroAddress,
                        amount: '5',
                        chain: 'bob',
                    },
                    affiliateFee: {
                        address: zeroAddress,
                        amount: '2',
                        chain: 'bob',
                    },
                    executionFee: {
                        address: zeroAddress,
                        amount: '1',
                        chain: 'bob',
                    },
                    layerzeroFee: {
                        address: zeroAddress,
                        amount: '1',
                        chain: 'bob',
                    },
                    solverFee: {
                        address: zeroAddress,
                        amount: '1',
                        chain: 'bob',
                    },
                },
                fees: {
                    address: zeroAddress,
                    amount: '3',
                    chain: 'bob',
                },
                inputAmount: {
                    address: zeroAddress,
                    amount: '1000',
                    chain: 'bob',
                },
                outputAmount: {
                    address: zeroAddress,
                    amount: '990',
                    chain: 'bob',
                },
                recipient: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                sender: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                signedQuoteData: MOCK_SIGNED_QUOTE_DATA,
                slippage: '0',
                token: '0x0000000000000000000000000000000000000000',
            },
        };

        const mockOfframpQuote: GatewayQuoteOneOf1 = {
            offramp: {
                txTo: '0x1234567890123456789012345678901234567890',
                recipient: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                slippage: 300,
                srcChain: 'bob',
                feeBreakdown: {
                    protocolFee: {
                        address: zeroAddress,
                        amount: '5',
                        chain: 'bob',
                    },
                    affiliateFee: {
                        address: zeroAddress,
                        amount: '2',
                        chain: 'bob',
                    },
                    inclusionFee: {
                        address: zeroAddress,
                        amount: '1',
                        chain: 'bob',
                    },
                    solverFee: {
                        address: zeroAddress,
                        amount: '1',
                        chain: 'bob',
                    },
                    fastestFeeRate: '6',
                },
                fees: {
                    address: zeroAddress,
                    amount: '3',
                    chain: 'bob',
                },
                inputAmount: {
                    address: zeroAddress,
                    amount: '1000',
                    chain: 'bob',
                },
                outputAmount: {
                    address: zeroAddress,
                    amount: '990',
                    chain: 'bob',
                },
                tokenAddress: WBTC_OFT_ADDRESS,
            },
        };

        const mockLayerZeroQuote: GatewayQuoteOneOf2 = {
            layerZero: {
                fees: {
                    address: zeroAddress,
                    amount: '3',
                    chain: 'bob',
                },
                inputAmount: {
                    address: zeroAddress,
                    amount: '1000',
                    chain: 'bob',
                },
                outputAmount: {
                    address: zeroAddress,
                    amount: '990',
                    chain: 'bob',
                },
                tx: {
                    to: '0x1234567890123456789012345678901234567890',
                    data: '0xabcdef',
                    value: '0',
                    chain: 'bob',
                },
                recipient: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
            },
        };

        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .get('/v1/get-quote')
            .query((q) => q.srcChain === 'bitcoin')
            .reply(200, mockOnrampQuote);

        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .get('/v1/get-quote')
            .query((q) => q.dstChain === 'bitcoin')
            .reply(200, mockOfframpQuote);

        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .get('/v1/get-quote')
            .query((q) => q.srcChain === 'bsc')
            .reply(200, mockLayerZeroQuote);

        const result1 = await gatewaySDK.getQuote({
            fromChain: 'bitcoin',
            fromToken: '0x0000000000000000000000000000000000000000',
            toChain: 'bob',
            toToken: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c',
            fromUserAddress: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
            toUserAddress: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
            amount: 1000,
        });

        assert(instanceOfGatewayQuoteOneOf(result1));

        const result2 = await gatewaySDK.getQuote({
            fromChain: 'bob',
            fromToken: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c',
            toChain: 'bitcoin',
            toToken: '0x0000000000000000000000000000000000000000',
            toUserAddress: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
            amount: 1000,
        });

        assert(instanceOfGatewayQuoteOneOf1(result2));

        const result3 = await gatewaySDK.getQuote({
            fromChain: 'bsc',
            fromToken: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c',
            toChain: 'bob',
            toToken: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c',
            fromUserAddress: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
            toUserAddress: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
            amount: 1000,
        });

        assert(instanceOfGatewayQuoteOneOf2(result3));
    });

    it('should get orders', async () => {
        const mockOrders: GatewayOrderInfo[] = [
            {
                id: 'order-1',
                status: 'btc-confirmation',
                dstInfo: {
                    chain: 'bob',
                    token: WBTC_OFT_ADDRESS,
                    txHash: '0xabc123',
                    amount: '1000',
                },
                srcInfo: {
                    chain: 'bitcoin',
                    txHash: '0xdef456',
                    token: '0x0000000000000000000000000000000000000000',
                    amount: '1000',
                },
                timestamp: 1625247600,
                estimatedTimeInSecs: 3600,
            },
            {
                id: 'order-2',
                status: 'Accepted',
                timestamp: 1625247600,
                estimatedTimeInSecs: 3600,
                dstInfo: {
                    chain: 'bob',
                    token: WBTC_OFT_ADDRESS,
                    txHash: '0xabc123',
                    amount: '1000',
                },
                srcInfo: {
                    chain: 'bitcoin',
                    txHash: '0xdef456',
                    token: '0x0000000000000000000000000000000000000000',
                    amount: '1000',
                },
            },
            {
                id: 'order-3',
                estimatedTimeInSecs: 600,
                dstInfo: {
                    chain: 'bsc',
                    token: WBTC_OFT_ADDRESS,
                    txHash: '0xlzabc123',
                    amount: '50',
                },
                srcInfo: {
                    chain: 'bitcoin',
                    txHash: '0xlzdef456',
                    token: '0x0000000000000000000000000000000000000000',
                    amount: '50',
                },
                timestamp: 1625247600,
                status: 'destinationConfirmed',
            },
        ];

        nock(`${MAINNET_GATEWAY_BASE_URL}`).get(`/v1/get-orders/${zeroAddress}`).reply(200, mockOrders);

        const gatewaySDK = new GatewaySDK();
        const orders = await gatewaySDK.getOrders(zeroAddress);
        expect(orders).toBeDefined();
        assert(Array.isArray(orders));
    });

    it('should get routes', async () => {
        nock(`${MAINNET_GATEWAY_BASE_URL}/v1`)
            .get('/get-routes')
            .reply(200, [
                {
                    srcChain: 'bitcoin',
                    dstChain: 'bob',
                    srcToken: '0x0000000000000000000000000000000000000000',
                    dstToken: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c',
                },
            ]);

        const gatewaySDK = new GatewaySDK();
        const routes = await gatewaySDK.getRoutes();
        expect(routes).toBeDefined();
        expect(Array.isArray(routes)).toBe(true);
    });

    it('should execute onramp quote with btcSigner.signAllInputs', async () => {
        const gatewaySDK = new GatewaySDK();

        const mockQuote: GatewayQuoteOneOf = {
            onramp: {
                dstChain: 'bob',
                dstToken: WBTC_OFT_ADDRESS,
                executionFees: {
                    address: zeroAddress,
                    amount: '10',
                    chain: 'bob',
                },
                feeBreakdown: {
                    protocolFee: {
                        address: zeroAddress,
                        amount: '5',
                        chain: 'bob',
                    },
                    affiliateFee: {
                        address: zeroAddress,
                        amount: '2',
                        chain: 'bob',
                    },
                    executionFee: {
                        address: zeroAddress,
                        amount: '1',
                        chain: 'bob',
                    },
                    layerzeroFee: {
                        address: zeroAddress,
                        amount: '1',
                        chain: 'bob',
                    },
                    solverFee: {
                        address: zeroAddress,
                        amount: '1',
                        chain: 'bob',
                    },
                },
                fees: {
                    address: zeroAddress,
                    amount: '3',
                    chain: 'bob',
                },
                inputAmount: {
                    address: zeroAddress,
                    amount: '1000',
                    chain: 'bob',
                },
                outputAmount: {
                    address: zeroAddress,
                    amount: '990',
                    chain: 'bob',
                },
                recipient: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                sender: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                signedQuoteData: MOCK_SIGNED_QUOTE_DATA,
                slippage: '0',
                token: '0x0000000000000000000000000000000000000000',
            },
        };

        const mockOrderId = 'order-123';
        const mockPsbt = 'cHNidP8BAH0CAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAzwAAAAA=';
        const signedTx = '02000000010000000000000000000000000000000000000000000000000000000000000000';

        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .post('/v1/create-order')
            .reply(200, {
                onramp: {
                    order_id: mockOrderId,
                    psbt_hex: mockPsbt,
                    address: 'tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx',
                    op_return_data: '',
                },
            });

        nock(`${MAINNET_GATEWAY_BASE_URL}`).patch('/v1/register-tx').reply(200, JSON.stringify('tx-hash-123'));

        const mockBtcSigner: BitcoinSigner = {
            signAllInputs: async (psbt: string) => {
                expect(psbt).toBe(mockPsbt);
                return signedTx;
            },
        };

        const mockWalletClient: WalletClient<Transport, ViemChain, Account> = {
            account: { address: '0x1234567890123456789012345678901234567890' as Address },
        } as WalletClient<Transport, ViemChain, Account>;

        const mockPublicClient = {} as PublicClient<Transport>;

        const result = await gatewaySDK.executeQuote({
            quote: mockQuote,
            walletClient: mockWalletClient,
            publicClient: mockPublicClient,
            btcSigner: mockBtcSigner,
        });

        expect(result).toEqual({
            order: expect.objectContaining({ onramp: expect.objectContaining({ orderId: 'order-123' }) }),
            tx: 'tx-hash-123',
        });
    });

    it('should execute walletless onramp without btcSigner', async () => {
        const gatewaySDK = new GatewaySDK();

        const mockQuote: GatewayQuoteOneOf = {
            onramp: {
                dstChain: 'bob',
                dstToken: WBTC_OFT_ADDRESS,
                executionFees: {
                    address: zeroAddress,
                    amount: '10',
                    chain: 'bob',
                },
                feeBreakdown: {
                    protocolFee: {
                        address: zeroAddress,
                        amount: '5',
                        chain: 'bob',
                    },
                    affiliateFee: {
                        address: zeroAddress,
                        amount: '2',
                        chain: 'bob',
                    },
                    executionFee: {
                        address: zeroAddress,
                        amount: '1',
                        chain: 'bob',
                    },
                    layerzeroFee: {
                        address: zeroAddress,
                        amount: '1',
                        chain: 'bob',
                    },
                    solverFee: {
                        address: zeroAddress,
                        amount: '1',
                        chain: 'bob',
                    },
                },
                fees: {
                    address: zeroAddress,
                    amount: '3',
                    chain: 'bob',
                },
                inputAmount: {
                    address: zeroAddress,
                    amount: '1000',
                    chain: 'bob',
                },
                outputAmount: {
                    address: zeroAddress,
                    amount: '990',
                    chain: 'bob',
                },
                recipient: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                sender: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                signedQuoteData: MOCK_SIGNED_QUOTE_DATA,
                slippage: '0',
                token: '0x0000000000000000000000000000000000000000',
            },
        };

        const mockOrderId = 'walletless-order-123';

        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .post('/v1/create-order')
            .reply(200, {
                onramp: {
                    order_id: mockOrderId,
                    address: 'tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx',
                    op_return_data: '',
                },
            });

        const registerTxScope = nock(`${MAINNET_GATEWAY_BASE_URL}`).patch('/v1/register-tx').reply(200, 'ok');

        const mockWalletClient: WalletClient<Transport, ViemChain, Account> = {
            account: { address: '0x1234567890123456789012345678901234567890' as Address },
        } as WalletClient<Transport, ViemChain, Account>;

        const mockPublicClient = {} as PublicClient<Transport>;

        const result = await gatewaySDK.executeQuote({
            quote: mockQuote,
            walletClient: mockWalletClient,
            publicClient: mockPublicClient,
        });

        expect(result).toEqual({
            order: expect.objectContaining({
                onramp: expect.objectContaining({
                    orderId: mockOrderId,
                    address: 'tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx',
                }),
            }),
        });
        expect(result.tx).toBeUndefined();
        expect(registerTxScope.isDone()).toBe(false);
    });

    it('should throw error when btcSigner returns empty transaction', async () => {
        const gatewaySDK = new GatewaySDK();

        const mockQuote: GatewayQuoteOneOf = {
            onramp: {
                dstChain: 'bob',
                dstToken: WBTC_OFT_ADDRESS,
                executionFees: {
                    address: zeroAddress,
                    amount: '10',
                    chain: 'bob',
                },
                feeBreakdown: {
                    protocolFee: {
                        address: zeroAddress,
                        amount: '5',
                        chain: 'bob',
                    },
                    affiliateFee: {
                        address: zeroAddress,
                        amount: '2',
                        chain: 'bob',
                    },
                    executionFee: {
                        address: zeroAddress,
                        amount: '1',
                        chain: 'bob',
                    },
                    layerzeroFee: {
                        address: zeroAddress,
                        amount: '1',
                        chain: 'bob',
                    },
                    solverFee: {
                        address: zeroAddress,
                        amount: '1',
                        chain: 'bob',
                    },
                },
                fees: {
                    address: zeroAddress,
                    amount: '10',
                    chain: 'bob',
                },
                inputAmount: {
                    address: zeroAddress,
                    amount: '1000',
                    chain: 'bob',
                },
                outputAmount: {
                    address: zeroAddress,
                    amount: '990',
                    chain: 'bob',
                },
                recipient: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                sender: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                signedQuoteData: MOCK_SIGNED_QUOTE_DATA,
                slippage: '0',
                token: '0x0000000000000000000000000000000000000000',
            },
        };

        const mockOrderId = 'order-123';
        const mockPsbt = 'cHNidP8BAH0CAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAzwAAAAA=';

        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .post('/v1/create-order')
            .reply(200, {
                onramp: {
                    order_id: mockOrderId,
                    psbt_hex: mockPsbt,
                    address: 'tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx',
                    op_return_data: '',
                },
            });

        const mockBtcSigner: BitcoinSigner = {
            signAllInputs: async () => '',
        };

        const mockWalletClient: WalletClient<Transport, ViemChain, Account> = {
            account: { address: '0x1234567890123456789012345678901234567890' as Address },
        } as WalletClient<Transport, ViemChain, Account>;

        const mockPublicClient = {} as PublicClient<Transport>;

        await expect(
            gatewaySDK.executeQuote({
                quote: mockQuote,
                walletClient: mockWalletClient,
                publicClient: mockPublicClient,
                btcSigner: mockBtcSigner,
            })
        ).rejects.toThrow('Failed to get signed transaction');
    });

    it('should execute offramp quote with token approval', async () => {
        const gatewaySDK = new GatewaySDK();

        const mockQuote: GatewayQuoteV2OneOf = {
            offramp: {
                txTo: '0x1234567890123456789012345678901234567890',
                recipient: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                slippage: 300,
                srcChain: 'bob',
                feeBreakdown: {
                    protocolFee: {
                        address: zeroAddress,
                        amount: '5',
                        chain: 'bob',
                    },
                    affiliateFee: {
                        address: zeroAddress,
                        amount: '2',
                        chain: 'bob',
                    },
                    solverFee: {
                        address: zeroAddress,
                        amount: '1',
                        chain: 'bob',
                    },
                    inclusionFee: {
                        address: zeroAddress,
                        amount: '1',
                        chain: 'bob',
                    },
                    fastestFeeRate: '6',
                },
                inputAmount: {
                    address: zeroAddress,
                    amount: '1000',
                    chain: 'bob',
                },
                outputAmount: {
                    address: zeroAddress,
                    amount: '990',
                    chain: 'bob',
                },
                tokenAddress: WBTC_OFT_ADDRESS,
                totalFeeUsd: '3',
            },
        };

        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .post('/v1/create-order')
            .reply(200, {
                offramp: {
                    order_id: 'offramp-order-123',
                    tx: {
                        to: '0x1234567890123456789012345678901234567890',
                        data: '0xabcdef',
                        value: '0',
                    },
                },
            });

        nock(`${MAINNET_GATEWAY_BASE_URL}`).patch('/v1/register-tx').reply(200, JSON.stringify('ok'));

        const mockWalletClient = {
            account: { address: '0xabcd1234abcd1234abcd1234abcd1234abcd1234' as Address },
            writeContract: async () => '0xapprovehash' as `0x${string}`,
            sendTransaction: async () => '0xtxhash' as `0x${string}`,
        } as unknown as WalletClient<Transport, ViemChain, Account>;

        const mockPublicClient = {
            multicall: async () => [0n, 8], // allowance: 0 (BigInt), decimals: 8 (number)
            simulateContract: async () => ({ request: {} }),
            waitForTransactionReceipt: async () => ({}),
        } as unknown as PublicClient<Transport>;

        const result = await gatewaySDK.executeQuote({
            quote: mockQuote,
            walletClient: mockWalletClient,
            publicClient: mockPublicClient,
        });

        expect(result.tx).toBe('0xtxhash');
        expect(result.order).toEqual(
            expect.objectContaining({ offramp: expect.objectContaining({ orderId: 'offramp-order-123' }) })
        );
    });

    it('should approve WBTC on bob offramp', async () => {
        const gatewaySDK = new GatewaySDK();

        const mockQuote: GatewayQuoteV2OneOf = {
            offramp: {
                srcChain: 'bob',
                feeBreakdown: {
                    protocolFee: {
                        address: zeroAddress,
                        amount: '5',
                        chain: 'bob',
                    },
                    affiliateFee: {
                        address: zeroAddress,
                        amount: '2',
                        chain: 'bob',
                    },
                    solverFee: {
                        address: zeroAddress,
                        amount: '1',
                        chain: 'bob',
                    },
                    inclusionFee: {
                        address: zeroAddress,
                        amount: '1',
                        chain: 'bob',
                    },
                    fastestFeeRate: '6',
                },
                inputAmount: {
                    address: zeroAddress,
                    amount: '1000',
                    chain: 'bob',
                },
                outputAmount: {
                    address: zeroAddress,
                    amount: '990',
                    chain: 'bob',
                },
                tokenAddress: WBTC_OFT_ADDRESS,
                recipient: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                slippage: 0,
                totalFeeUsd: '3',
                txTo: zeroAddress,
            },
        };

        const spenderAddress = '0x1234567890123456789012345678901234567890';

        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .post('/v1/create-order')
            .reply(200, {
                offramp: {
                    order_id: 'offramp-order-789',
                    tx: {
                        to: spenderAddress,
                        data: '0xabcdef',
                        value: '0',
                    },
                },
            });

        nock(`${MAINNET_GATEWAY_BASE_URL}`).patch('/v1/register-tx').reply(200, JSON.stringify('ok'));

        const mockWalletClient = {
            account: { address: '0xabcd1234abcd1234abcd1234abcd1234abcd1234' as Address },
            writeContract: vi.fn().mockResolvedValue('0xapprovehash' as `0x${string}`),
            sendTransaction: vi.fn().mockResolvedValue('0xtxhash' as `0x${string}`),
        } as unknown as WalletClient<Transport, ViemChain, Account>;

        const simulateContractMock = vi.fn().mockResolvedValue({ request: {} });
        const mockPublicClient = {
            multicall: async () => [0n],
            simulateContract: simulateContractMock,
            waitForTransactionReceipt: vi.fn().mockResolvedValue({}),
        } as unknown as PublicClient<Transport>;

        const result = await gatewaySDK.executeQuote({
            quote: mockQuote,
            walletClient: mockWalletClient,
            publicClient: mockPublicClient,
        });

        expect(result.tx).toBe('0xtxhash');
        expect(simulateContractMock).toHaveBeenCalledTimes(1);
        expect(simulateContractMock.mock.calls[0][0].args).toEqual([spenderAddress, maxUint256]);
        expect(mockWalletClient.writeContract).toHaveBeenCalledTimes(1);
    });

    it('should skip approval for WBTC when srcChain is not bob', async () => {
        const gatewaySDK = new GatewaySDK();

        const mockQuote: GatewayQuoteV2OneOf = {
            offramp: {
                srcChain: 'ethereum',
                feeBreakdown: {
                    protocolFee: {
                        address: zeroAddress,
                        amount: '5',
                        chain: 'ethereum',
                    },
                    affiliateFee: {
                        address: zeroAddress,
                        amount: '2',
                        chain: 'ethereum',
                    },
                    solverFee: {
                        address: zeroAddress,
                        amount: '1',
                        chain: 'ethereum',
                    },
                    inclusionFee: {
                        address: zeroAddress,
                        amount: '1',
                        chain: 'ethereum',
                    },
                    fastestFeeRate: '6',
                },
                inputAmount: {
                    address: zeroAddress,
                    amount: '1000',
                    chain: 'ethereum',
                },
                outputAmount: {
                    address: zeroAddress,
                    amount: '990',
                    chain: 'ethereum',
                },
                tokenAddress: WBTC_OFT_ADDRESS,
                recipient: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                slippage: 0,
                totalFeeUsd: '3',
                txTo: zeroAddress,
            },
        };

        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .post('/v1/create-order')
            .reply(200, {
                offramp: {
                    order_id: 'offramp-order-790',
                    tx: {
                        to: '0x1234567890123456789012345678901234567890',
                        data: '0xabcdef',
                        value: '0',
                    },
                },
            });

        nock(`${MAINNET_GATEWAY_BASE_URL}`).patch('/v1/register-tx').reply(200, JSON.stringify('ok'));

        const mockWalletClient = {
            account: { address: '0xabcd1234abcd1234abcd1234abcd1234abcd1234' as Address },
            writeContract: vi.fn().mockResolvedValue('0xapprovehash' as `0x${string}`),
            sendTransaction: vi.fn().mockResolvedValue('0xtxhash' as `0x${string}`),
        } as unknown as WalletClient<Transport, ViemChain, Account>;

        const simulateContractMock = vi.fn().mockResolvedValue({ request: {} });
        const mockPublicClient = {
            multicall: async () => [0n],
            simulateContract: simulateContractMock,
            waitForTransactionReceipt: vi.fn().mockResolvedValue({}),
        } as unknown as PublicClient<Transport>;

        const result = await gatewaySDK.executeQuote({
            quote: mockQuote,
            walletClient: mockWalletClient,
            publicClient: mockPublicClient,
        });

        expect(result.tx).toBe('0xtxhash');
        expect(simulateContractMock).not.toHaveBeenCalled();
        expect(mockWalletClient.writeContract).not.toHaveBeenCalled();
    });

    it('should reset USDT allowance before approving', async () => {
        const gatewaySDK = new GatewaySDK();

        const mockQuote: GatewayQuoteV2OneOf = {
            offramp: {
                recipient: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                srcChain: 'ethereum',
                feeBreakdown: {
                    protocolFee: {
                        address: zeroAddress,
                        amount: '5',
                        chain: 'ethereum',
                    },
                    affiliateFee: {
                        address: zeroAddress,
                        amount: '2',
                        chain: 'ethereum',
                    },
                    solverFee: {
                        address: zeroAddress,
                        amount: '1',
                        chain: 'ethereum',
                    },
                    inclusionFee: {
                        address: zeroAddress,
                        amount: '1',
                        chain: 'ethereum',
                    },
                    fastestFeeRate: '6',
                },
                inputAmount: {
                    address: zeroAddress,
                    amount: '1000',
                    chain: 'ethereum',
                },
                outputAmount: {
                    address: zeroAddress,
                    amount: '990',
                    chain: 'ethereum',
                },
                tokenAddress: ETHEREUM_USDT_ADDRESS,
                slippage: 0,
                totalFeeUsd: '3',
                txTo: zeroAddress,
            },
        };

        const spenderAddress = '0x1234567890123456789012345678901234567890';

        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .post('/v1/create-order')
            .reply(200, {
                offramp: {
                    order_id: 'offramp-order-791',
                    tx: {
                        to: spenderAddress,
                        data: '0xabcdef',
                        value: '0',
                    },
                },
            });

        nock(`${MAINNET_GATEWAY_BASE_URL}`).patch('/v1/register-tx').reply(200, JSON.stringify('ok'));

        const mockWalletClient = {
            account: { address: '0xabcd1234abcd1234abcd1234abcd1234abcd1234' as Address },
            writeContract: vi.fn().mockResolvedValue('0xapprovehash' as `0x${string}`),
            sendTransaction: vi.fn().mockResolvedValue('0xtxhash' as `0x${string}`),
        } as unknown as WalletClient<Transport, ViemChain, Account>;

        const simulateContractMock = vi.fn().mockResolvedValue({ request: {} });
        const mockPublicClient = {
            multicall: async () => [1n],
            simulateContract: simulateContractMock,
            waitForTransactionReceipt: vi.fn().mockResolvedValue({}),
        } as unknown as PublicClient<Transport>;

        const result = await gatewaySDK.executeQuote({
            quote: mockQuote,
            walletClient: mockWalletClient,
            publicClient: mockPublicClient,
        });

        expect(result.tx).toBe('0xtxhash');
        expect(simulateContractMock).toHaveBeenCalledTimes(2);
        expect(simulateContractMock.mock.calls[0][0].args).toEqual([spenderAddress, 0n]);
        expect(simulateContractMock.mock.calls[1][0].args).toEqual([spenderAddress, maxUint256]);
        expect(mockWalletClient.writeContract).toHaveBeenCalledTimes(2);
    });

    it('should execute offramp quote without approval when allowance is sufficient', async () => {
        const gatewaySDK = new GatewaySDK();

        const mockQuote: GatewayQuoteV2OneOf = {
            offramp: {
                txTo: '0x1234567890123456789012345678901234567890',
                recipient: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                slippage: 300,
                srcChain: 'bob',
                feeBreakdown: {
                    protocolFee: {
                        address: zeroAddress,
                        amount: '5',
                        chain: 'bob',
                    },
                    affiliateFee: {
                        address: zeroAddress,
                        amount: '2',
                        chain: 'bob',
                    },
                    inclusionFee: {
                        address: zeroAddress,
                        amount: '1',
                        chain: 'bob',
                    },
                    solverFee: {
                        address: zeroAddress,
                        amount: '1',
                        chain: 'bob',
                    },
                    fastestFeeRate: '6',
                },
                inputAmount: {
                    address: zeroAddress,
                    amount: '1000',
                    chain: 'bob',
                },
                outputAmount: {
                    address: zeroAddress,
                    amount: '990',
                    chain: 'bob',
                },
                tokenAddress: WBTC_OFT_ADDRESS,
                totalFeeUsd: '3',
            },
        };

        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .post('/v1/create-order')
            .reply(200, {
                offramp: {
                    order_id: 'offramp-order-456',
                    tx: {
                        to: '0x1234567890123456789012345678901234567890',
                        data: '0xabcdef',
                        value: '0',
                    },
                },
            });

        nock(`${MAINNET_GATEWAY_BASE_URL}`).patch('/v1/register-tx').reply(200, JSON.stringify('ok'));

        const mockWalletClient = {
            account: { address: '0xabcd1234abcd1234abcd1234abcd1234abcd1234' as Address },
            sendTransaction: async () => '0xtxhash' as `0x${string}`,
        } as unknown as WalletClient<Transport, ViemChain, Account>;

        const mockPublicClient = {
            multicall: async () => [maxUint256, 8],
            waitForTransactionReceipt: async () => ({}),
        } as unknown as PublicClient<Transport>;

        const result = await gatewaySDK.executeQuote({
            quote: mockQuote,
            walletClient: mockWalletClient,
            publicClient: mockPublicClient,
        });

        expect(result.tx).toBe('0xtxhash');
        expect(result.order).toEqual(
            expect.objectContaining({ offramp: expect.objectContaining({ orderId: 'offramp-order-456' }) })
        );
    });

    it('should throw error when invalid quote type is provided', async () => {
        const gatewaySDK = new GatewaySDK();

        const invalidQuote = {
            someInvalidField: 'invalid',
        } as unknown as GatewayQuoteOneOf;

        const mockWalletClient = {
            account: { address: '0xabcd1234abcd1234abcd1234abcd1234abcd1234' as Address },
        } as WalletClient<Transport, ViemChain, Account>;

        const mockPublicClient = {} as PublicClient<Transport>;

        await expect(
            gatewaySDK.executeQuote({
                quote: invalidQuote,
                walletClient: mockWalletClient,
                publicClient: mockPublicClient,
            })
        ).rejects.toThrow('Invalid quote type');
    });

    it('should throw error when btcSigner has neither method', async () => {
        const gatewaySDK = new GatewaySDK();

        const mockQuote: GatewayQuoteOneOf = {
            onramp: {
                dstChain: 'bob',
                dstToken: WBTC_OFT_ADDRESS,
                executionFees: {
                    address: zeroAddress,
                    amount: '10',
                    chain: 'bob',
                },
                feeBreakdown: {
                    protocolFee: {
                        address: zeroAddress,
                        amount: '5',
                        chain: 'bob',
                    },
                    affiliateFee: {
                        address: zeroAddress,
                        amount: '2',
                        chain: 'bob',
                    },
                    executionFee: {
                        address: zeroAddress,
                        amount: '1',
                        chain: 'bob',
                    },
                    layerzeroFee: {
                        address: zeroAddress,
                        amount: '1',
                        chain: 'bob',
                    },
                    solverFee: {
                        address: zeroAddress,
                        amount: '1',
                        chain: 'bob',
                    },
                },
                fees: {
                    address: zeroAddress,
                    amount: '3',
                    chain: 'bob',
                },
                inputAmount: {
                    address: zeroAddress,
                    amount: '1000',
                    chain: 'bob',
                },
                outputAmount: {
                    address: zeroAddress,
                    amount: '990',
                    chain: 'bob',
                },
                recipient: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                sender: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                signedQuoteData: MOCK_SIGNED_QUOTE_DATA,
                slippage: '0',
                token: '0x0000000000000000000000000000000000000000',
            },
        };

        const mockOrderId = 'order-123';
        const mockPsbt = 'cHNidP8BAH0CAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAzwAAAAA=';

        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .post('/v1/create-order')
            .reply(200, {
                onramp: {
                    order_id: mockOrderId,
                    psbt_hex: mockPsbt,
                    address: 'tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx',
                    op_return_data: '',
                },
            });

        const mockBtcSigner = {} as BitcoinSigner;

        const mockWalletClient = {
            account: { address: '0x1234567890123456789012345678901234567890' as Address },
        } as WalletClient<Transport, ViemChain, Account>;

        const mockPublicClient = {} as PublicClient<Transport>;

        await expect(
            gatewaySDK.executeQuote({
                quote: mockQuote,
                walletClient: mockWalletClient,
                publicClient: mockPublicClient,
                btcSigner: mockBtcSigner,
            })
        ).rejects.toThrow('btcSigner must implement either sendBitcoin or signAllInputs method');
    });

    it('should get error', async () => {
        // Mock the GET request to /v1/get-quote
        const errorMessage =
            'No route found from bitcoin (0x0000000000000000000000000000000000000001) to bob (0x0555E30da8f98308EdB960aa94C0Db47230d2B9c)';
        nock(MAINNET_GATEWAY_BASE_URL).get('/v1/get-quote').query(true).reply(400, {
            error: errorMessage,
        });

        const gatewaySDK = new GatewaySDK();

        await expect(
            gatewaySDK.getQuote({
                fromChain: 'bitcoin',
                toChain: 'bob',
                fromToken: '0x0000000000000000000000000000000000000001',
                toToken: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c',
                amount: 100000,
                fromUserAddress: 'bc1qyhc4uslh46axl553pq3mjclrt7dcgmlzxv0ktx',
                toUserAddress: '0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266',
                maxSlippage: 300,
            })
        ).rejects.toThrow(errorMessage);
    });

    it('should parse QuoteAmountTooLow gateway errors with typed details', () => {
        const error = GatewayError.fromResponse({
            code: GatewayErrorCode.QuoteAmountTooLow,
            error: 'Quote amount too low',
            details: {
                minimum: '1000',
                actual: '10',
            },
        });

        expect(isGatewayError(error)).toBe(true);
        expect(error.code).toBe(GatewayErrorCode.QuoteAmountTooLow);
        expect(error.details).toEqual({
            minimum: '1000',
            actual: '10',
        });
    });

    it('should parse camelCase gateway error details', () => {
        const error = GatewayError.fromResponse({
            code: GatewayErrorCode.NoRoute,
            error: 'No route',
            details: {
                srcChain: 'bitcoin',
                srcToken: '0x0000000000000000000000000000000000000000',
                dstChain: 'bob',
                dstToken: WBTC_OFT_ADDRESS,
            },
        });

        expect(isGatewayError(error)).toBe(true);
        expect(error.code).toBe(GatewayErrorCode.NoRoute);
        expect(error.details).toEqual({
            srcChain: 'bitcoin',
            srcToken: '0x0000000000000000000000000000000000000000',
            dstChain: 'bob',
            dstToken: WBTC_OFT_ADDRESS,
        });
    });

    it('should approve and send transaction for layerzero swap when allowance is low', async () => {
        const mockedQuote: GatewayQuoteOneOf2 = {
            layerZero: {
                recipient: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                tx: {
                    to: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c',
                    data: '0xc7c7f5b3000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000016967ac72e86a000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000007596000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000186a000000000000000000000000000000000000000000000000000000000000186a000000000000000000000000000000000000000000000000000000000000000e000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000120000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000',
                    value: '397368972470378',
                    chain: 'ethereum',
                },
                fees: {
                    amount: '0',
                    address: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c',
                    chain: 'bob',
                },
                inputAmount: {
                    amount: '100000',
                    address: '0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599',
                    chain: 'ethereum',
                },
                outputAmount: {
                    amount: '100000',
                    address: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c',
                    chain: 'bob',
                },
            },
        };

        const gatewaySDK = new GatewaySDK();

        const readContract = vi.fn().mockResolvedValue(0n);
        const simulateContract = vi.fn().mockResolvedValue({ request: {} });
        const waitForTransactionReceipt = vi.fn().mockResolvedValue({});
        const sendTransaction = vi.fn().mockResolvedValue('0xsendhash');
        const writeContract = vi.fn().mockResolvedValue('0xapprovehash');

        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .post('/v1/create-order')
            .reply(200, {
                layerZero: {
                    order_id: 'layerzero-order-123',
                },
            });

        nock(`${MAINNET_GATEWAY_BASE_URL}`).patch('/v1/register-tx').reply(200, JSON.stringify('tx-hash-123'));

        const mockWalletClient = {
            account: { address: '0x1234567890123456789012345678901234567890' as Address },
            sendTransaction,
            writeContract,
        } as unknown as WalletClient<Transport, ViemChain, Account>;

        const mockPublicClient = {
            readContract,
            simulateContract,
            waitForTransactionReceipt,
        } as unknown as PublicClient<Transport>;

        const result = await gatewaySDK.executeQuote({
            quote: mockedQuote,
            walletClient: mockWalletClient,
            publicClient: mockPublicClient,
        });

        expect(result.tx).toBe('0xsendhash');
        expect(result.order).toEqual(
            expect.objectContaining({ layerZero: expect.objectContaining({ orderId: 'layerzero-order-123' }) })
        );
        expect(readContract).toHaveBeenCalledTimes(1);
        expect(simulateContract).toHaveBeenCalledTimes(1);
        expect(writeContract).toHaveBeenCalledTimes(1);
        expect(sendTransaction).toHaveBeenCalledTimes(1);
        expect(waitForTransactionReceipt).toHaveBeenCalledTimes(2);
    });

    it('should skip approval when allowance is sufficient for layerzero swap', async () => {
        const mockedQuote: GatewayQuoteOneOf2 = {
            layerZero: {
                recipient: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                tx: {
                    to: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c',
                    data: '0xc7c7f5b3000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000016967ac72e86a000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000007596000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000186a000000000000000000000000000000000000000000000000000000000000186a000000000000000000000000000000000000000000000000000000000000000e000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000120000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000',
                    value: '397368972470378',
                    chain: 'ethereum',
                },
                fees: {
                    amount: '0',
                    address: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c',
                    chain: 'bob',
                },
                inputAmount: {
                    amount: '100000',
                    address: '0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599',
                    chain: 'ethereum',
                },
                outputAmount: {
                    amount: '100000',
                    address: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c',
                    chain: 'bob',
                },
            },
        };

        const gatewaySDK = new GatewaySDK();

        const readContract = vi.fn().mockResolvedValue(100000n);
        const simulateContract = vi.fn().mockResolvedValue({ request: {} });
        const waitForTransactionReceipt = vi.fn().mockResolvedValue({});
        const sendTransaction = vi.fn().mockResolvedValue('0xsendhash');
        const writeContract = vi.fn().mockResolvedValue('0xapprovehash');

        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .post('/v1/create-order')
            .reply(200, {
                layerZero: {
                    order_id: 'layerzero-order-123',
                },
            });

        nock(`${MAINNET_GATEWAY_BASE_URL}`).patch('/v1/register-tx').reply(200, JSON.stringify('tx-hash-123'));

        const mockWalletClient = {
            account: { address: '0x1234567890123456789012345678901234567890' as Address },
            sendTransaction,
            writeContract,
        } as unknown as WalletClient<Transport, ViemChain, Account>;

        const mockPublicClient = {
            readContract,
            simulateContract,
            waitForTransactionReceipt,
        } as unknown as PublicClient<Transport>;

        const result = await gatewaySDK.executeQuote({
            quote: mockedQuote,
            walletClient: mockWalletClient,
            publicClient: mockPublicClient,
        });

        expect(result.tx).toBe('0xsendhash');
        expect(readContract).toHaveBeenCalledTimes(1);
        expect(simulateContract).not.toHaveBeenCalled();
        expect(writeContract).not.toHaveBeenCalled();
        expect(sendTransaction).toHaveBeenCalledTimes(1);
        expect(waitForTransactionReceipt).toHaveBeenCalledTimes(1);
    });

    it('should skip allowance check for WBTC OFT token in layerzero swap', async () => {
        const mockedQuote: GatewayQuoteOneOf2 = {
            layerZero: {
                recipient: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                tx: {
                    to: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c',
                    data: '0xc7c7f5b3000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000016967ac72e86a000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000007596000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000186a000000000000000000000000000000000000000000000000000000000000186a000000000000000000000000000000000000000000000000000000000000000e000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000120000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000',
                    value: '397368972470378',
                    chain: 'bob',
                },
                fees: {
                    amount: '0',
                    address: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c',
                    chain: 'bob',
                },
                inputAmount: {
                    amount: '100000',
                    address: WBTC_OFT_ADDRESS,
                    chain: 'bob',
                },
                outputAmount: {
                    amount: '100000',
                    address: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c',
                    chain: 'ethereum',
                },
            },
        };

        const gatewaySDK = new GatewaySDK();

        const readContract = vi.fn().mockResolvedValue(0n);
        const waitForTransactionReceipt = vi.fn().mockResolvedValue({});
        const sendTransaction = vi.fn().mockResolvedValue('0xsendhash');

        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .post('/v1/create-order')
            .reply(200, {
                layerZero: {
                    order_id: 'layerzero-order-123',
                },
            });

        nock(`${MAINNET_GATEWAY_BASE_URL}`).patch('/v1/register-tx').reply(200, JSON.stringify('tx-hash-123'));

        const mockWalletClient = {
            account: { address: '0x1234567890123456789012345678901234567890' as Address },
            sendTransaction,
        } as unknown as WalletClient<Transport, ViemChain, Account>;

        const mockPublicClient = {
            readContract,
            waitForTransactionReceipt,
        } as unknown as PublicClient<Transport>;

        const result = await gatewaySDK.executeQuote({
            quote: mockedQuote,
            walletClient: mockWalletClient,
            publicClient: mockPublicClient,
        });

        expect(result.tx).toBe('0xsendhash');
        expect(readContract).not.toHaveBeenCalled();
        expect(sendTransaction).toHaveBeenCalledTimes(1);
        expect(waitForTransactionReceipt).toHaveBeenCalledTimes(1);
    });

    it.each([
        { field: 'fromToken', value: 'BTC', validToken: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c' },
        { field: 'toToken', value: 'WBTC', validToken: '0x0000000000000000000000000000000000000000' },
    ])('should throw error when $field is not an address', async ({ field, value, validToken }) => {
        const gatewaySDK = new GatewaySDK();

        const quoteParams = {
            fromChain: 'bitcoin',
            fromToken: field === 'fromToken' ? value : validToken,
            toChain: 'bob',
            toToken: field === 'toToken' ? value : validToken,
            fromUserAddress: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
            toUserAddress: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
            amount: 1000,
        };

        await expect(gatewaySDK.getQuote(quoteParams)).rejects.toThrow(
            `Invalid ${field}: '${value}'. Expected a token address (e.g. '0x0000000000000000000000000000000000000000'), not a symbol. Use getRoutes() to find supported token addresses.`
        );
    });

    it('should throw error when apiKey is not exactly 32 characters', () => {
        expect(() => new GatewaySDK(undefined, 'short')).toThrow('apiKey must be exactly 32 characters');
        expect(() => new GatewaySDK(undefined, '0x1234567890')).toThrow('apiKey must be exactly 32 characters');
        expect(() => new GatewaySDK(undefined, 'a'.repeat(34))).toThrow('apiKey must be exactly 32 characters');
        expect(() => new GatewaySDK(undefined, 'a'.repeat(36))).toThrow('apiKey must be exactly 32 characters');
    });

    it('should accept apiKey with exactly 32 characters', () => {
        const validApiKey = 'a'.repeat(32);
        expect(validApiKey.length).toBe(32);
        expect(() => new GatewaySDK(undefined, validApiKey)).not.toThrow();
    });

    it('should include Authorization header when apiKey is provided', async () => {
        const validApiKey = 'a'.repeat(32);
        const gatewaySDK = new GatewaySDK(undefined, validApiKey);

        const mockOnrampQuote: GatewayQuoteOneOf = {
            onramp: {
                dstChain: 'bob',
                dstToken: WBTC_OFT_ADDRESS,
                executionFees: {
                    address: zeroAddress,
                    amount: '10',
                    chain: 'bob',
                },
                feeBreakdown: {
                    protocolFee: {
                        address: zeroAddress,
                        amount: '5',
                        chain: 'bob',
                    },
                    affiliateFee: {
                        address: zeroAddress,
                        amount: '2',
                        chain: 'bob',
                    },
                    executionFee: {
                        address: zeroAddress,
                        amount: '1',
                        chain: 'bob',
                    },
                    layerzeroFee: {
                        address: zeroAddress,
                        amount: '1',
                        chain: 'bob',
                    },
                    solverFee: {
                        address: zeroAddress,
                        amount: '1',
                        chain: 'bob',
                    },
                },
                fees: {
                    address: zeroAddress,
                    amount: '3',
                    chain: 'bob',
                },
                inputAmount: {
                    address: zeroAddress,
                    amount: '1000',
                    chain: 'bob',
                },
                outputAmount: {
                    address: zeroAddress,
                    amount: '990',
                    chain: 'bob',
                },
                recipient: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                sender: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                signedQuoteData: MOCK_SIGNED_QUOTE_DATA,
                slippage: '0',
                token: '0x0000000000000000000000000000000000000000',
            },
        };

        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .get('/v1/get-quote')
            .query(true)
            .matchHeader('Authorization', `Bearer ${validApiKey}`)
            .reply(200, mockOnrampQuote);

        const result = await gatewaySDK.getQuote({
            fromChain: 'bitcoin',
            fromToken: '0x0000000000000000000000000000000000000000',
            toChain: 'bob',
            toToken: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c',
            fromUserAddress: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
            toUserAddress: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
            amount: 1000,
        });

        expect(result).toBeDefined();
        assert(instanceOfGatewayQuoteOneOf(result));
    });

    it('should not include Authorization header when apiKey is not provided', async () => {
        const gatewaySDK = new GatewaySDK();

        const mockOnrampQuote: GatewayQuoteOneOf = {
            onramp: {
                dstChain: 'bob',
                dstToken: WBTC_OFT_ADDRESS,
                executionFees: {
                    address: zeroAddress,
                    amount: '10',
                    chain: 'bob',
                },
                feeBreakdown: {
                    protocolFee: {
                        address: zeroAddress,
                        amount: '5',
                        chain: 'bob',
                    },
                    affiliateFee: {
                        address: zeroAddress,
                        amount: '2',
                        chain: 'bob',
                    },
                    executionFee: {
                        address: zeroAddress,
                        amount: '1',
                        chain: 'bob',
                    },
                    layerzeroFee: {
                        address: zeroAddress,
                        amount: '1',
                        chain: 'bob',
                    },
                    solverFee: {
                        address: zeroAddress,
                        amount: '1',
                        chain: 'bob',
                    },
                },
                fees: {
                    address: zeroAddress,
                    amount: '3',
                    chain: 'bob',
                },
                inputAmount: {
                    address: zeroAddress,
                    amount: '1000',
                    chain: 'bob',
                },
                outputAmount: {
                    address: zeroAddress,
                    amount: '990',
                    chain: 'bob',
                },
                recipient: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                sender: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                signedQuoteData: MOCK_SIGNED_QUOTE_DATA,
                slippage: '0',
                token: '0x0000000000000000000000000000000000000000',
            },
        };

        nock(`${MAINNET_GATEWAY_BASE_URL}`).get('/v1/get-quote').query(true).reply(200, mockOnrampQuote);

        const result = await gatewaySDK.getQuote({
            fromChain: 'bitcoin',
            fromToken: '0x0000000000000000000000000000000000000000',
            toChain: 'bob',
            toToken: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c',
            fromUserAddress: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
            toUserAddress: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
            amount: 1000,
        });

        expect(result).toBeDefined();
        assert(instanceOfGatewayQuoteOneOf(result));
    });
});
