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
import { privateKeyToAccount } from 'viem/accounts';
import { afterEach, assert, describe, expect, it, vi } from 'vitest';
import {
    BitcoinSigner,
    ExecuteQuoteStep,
    ExecuteQuoteStepType,
    GatewayError,
    GatewayErrorCode,
    GatewaySDK,
    isGatewayError,
} from '../src/gateway';
import { ETHEREUM_USDT_ADDRESS, MAINNET_GATEWAY_BASE_URL } from '../src/gateway/client';
import {
    GatewayOrderInfo,
    GatewayQuoteOneOf,
    GatewayQuoteOneOf1,
    GatewayQuoteV2OneOf,
    GatewayQuoteV2OneOf1,
    GatewayQuoteV2OneOf2,
    GatewayQuoteV3OneOf,
    instanceOfGatewayQuoteOneOf,
    instanceOfGatewayQuoteOneOf1,
    instanceOfGatewayQuoteV2OneOf2,
} from '../src/gateway/generated-client';

const WBTC_OFT_ADDRESS = '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c';
const MOCK_SIGNED_QUOTE_DATA = 'signed-quote-data';

function mockOftReadContract({ approvalRequired, allowance = 0n }: { approvalRequired: boolean; allowance?: bigint }) {
    return vi.fn().mockImplementation(({ functionName }: { functionName: string }) => {
        if (functionName === 'approvalRequired') {
            return Promise.resolve(approvalRequired);
        }

        return Promise.resolve(allowance);
    });
}

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

        const mockLayerZeroQuote: GatewayQuoteV2OneOf2 = {
            tokenSwap: {
                dstChain: 'bob',
                estimatedTimeInSecs: 60,
                fees: {
                    address: zeroAddress,
                    amount: '3',
                    chain: 'bob',
                },
                inputAmount: {
                    address: zeroAddress,
                    amount: '1000',
                    chain: 'bsc',
                },
                outputAmount: {
                    address: zeroAddress,
                    amount: '990',
                    chain: 'bob',
                },
                recipient: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                slippage: 100,
                srcChain: 'bsc',
                txTo: '0x1234567890123456789012345678901234567890',
            },
        };

        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .get('/v3/get-quote')
            .query((q) => q.srcChain === 'bitcoin')
            .reply(200, mockOnrampQuote);

        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .get('/v3/get-quote')
            .query((q) => q.dstChain === 'bitcoin')
            .reply(200, mockOfframpQuote);

        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .get('/v3/get-quote')
            .query((q) => q.srcChain === 'bsc')
            .reply(200, mockLayerZeroQuote);

        const result1 = await gatewaySDK.getQuote({
            fromChain: 'bitcoin',
            fromToken: '0x0000000000000000000000000000000000000000',
            toChain: 'bob',
            toToken: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c',
            fromUserAddress: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
            toUserAddress: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
            ownerAddress: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
            amount: 1000,
        });

        assert(instanceOfGatewayQuoteOneOf(result1));

        const result2 = await gatewaySDK.getQuote({
            fromChain: 'bob',
            fromToken: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c',
            toChain: 'bitcoin',
            toToken: '0x0000000000000000000000000000000000000000',
            toUserAddress: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
            ownerAddress: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
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
            ownerAddress: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
            amount: 1000,
        });

        assert(instanceOfGatewayQuoteV2OneOf2(result3));
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

        nock(`${MAINNET_GATEWAY_BASE_URL}`).get(`/v3/get-orders/${zeroAddress}`).reply(200, { orders: mockOrders });

        const gatewaySDK = new GatewaySDK();
        const result = await gatewaySDK.getOrders({ userAddress: zeroAddress });
        expect(result).toBeDefined();
        assert(Array.isArray(result.orders));
    });

    it('should get routes', async () => {
        nock(`${MAINNET_GATEWAY_BASE_URL}/v3`)
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
            .post('/v3/create-order')
            .reply(200, {
                onramp: {
                    order_id: mockOrderId,
                    psbt_hex: mockPsbt,
                    address: 'tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx',
                    op_return_data: '',
                },
            });

        nock(`${MAINNET_GATEWAY_BASE_URL}`).patch('/v3/register-tx').reply(200, JSON.stringify('tx-hash-123'));

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
            .post('/v3/create-order')
            .reply(200, {
                onramp: {
                    order_id: mockOrderId,
                    address: 'tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx',
                    op_return_data: '',
                },
            });

        const registerTxScope = nock(`${MAINNET_GATEWAY_BASE_URL}`).patch('/v3/register-tx').reply(200, 'ok');

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
            .post('/v3/create-order')
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

        const mockQuote: GatewayQuoteV3OneOf = {
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
                ownerAddress: '0xabcd1234abcd1234abcd1234abcd1234abcd1234',
                totalFeeUsd: '3',
            },
        };

        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .post('/v3/create-order')
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

        nock(`${MAINNET_GATEWAY_BASE_URL}`).patch('/v3/register-tx').reply(200, JSON.stringify('ok'));

        const mockWalletClient = {
            account: { address: '0xabcd1234abcd1234abcd1234abcd1234abcd1234' as Address },
            writeContract: async () => '0xapprovehash' as `0x${string}`,
            sendTransaction: async () => '0xtxhash' as `0x${string}`,
        } as unknown as WalletClient<Transport, ViemChain, Account>;

        const mockPublicClient = {
            readContract: mockOftReadContract({ approvalRequired: true }),
            multicall: vi.fn().mockResolvedValue([0n]),
            simulateContract: vi.fn().mockResolvedValue({ request: {} }),
            waitForTransactionReceipt: vi.fn().mockResolvedValue({}),
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

        const mockQuote: GatewayQuoteV3OneOf = {
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
                ownerAddress: '0xabcd1234abcd1234abcd1234abcd1234abcd1234',
                recipient: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                slippage: 0,
                totalFeeUsd: '3',
                txTo: zeroAddress,
            },
        };

        const spenderAddress = '0x1234567890123456789012345678901234567890';

        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .post('/v3/create-order')
            .reply(200, {
                offramp: {
                    order_id: 'offramp-order-789',
                    tx: {
                        type: 'evm',
                        chain: 'bob',
                        to: spenderAddress,
                        data: '0xabcdef',
                        value: '0',
                    },
                },
            });

        nock(`${MAINNET_GATEWAY_BASE_URL}`).patch('/v3/register-tx').reply(200, JSON.stringify('ok'));

        const mockWalletClient = {
            account: { address: '0xabcd1234abcd1234abcd1234abcd1234abcd1234' as Address },
            writeContract: vi.fn().mockResolvedValue('0xapprovehash' as `0x${string}`),
            sendTransaction: vi.fn().mockResolvedValue('0xtxhash' as `0x${string}`),
        } as unknown as WalletClient<Transport, ViemChain, Account>;

        const simulateContractMock = vi.fn().mockResolvedValue({ request: {} });
        const mockPublicClient = {
            readContract: mockOftReadContract({ approvalRequired: true }),
            multicall: vi.fn().mockResolvedValue([0n]),
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
        expect(simulateContractMock.mock.calls[0][0].args).toEqual([spenderAddress, 1000n]);
        expect(mockWalletClient.writeContract).toHaveBeenCalledTimes(1);
    });

    it('passes the local account object (not its address) to sendTransaction on offramp', async () => {
        // Regression (bob #1075): the offramp send was changed from
        //   account: walletClient.account  ->  account: walletClient.account.address
        // A bare address string is a json-rpc account in viem, forcing
        // eth_sendTransaction (node/wallet signs). Local-key callers (CLI/bots)
        // then fail on RPCs that don't support it. The account OBJECT (type
        // 'local') must reach sendTransaction so viem signs locally and uses
        // eth_sendRawTransaction. Browser (json-rpc) callers are unaffected
        // because their account stays type 'json-rpc' either way.
        const gatewaySDK = new GatewaySDK();
        // Throwaway placeholder key (= 1); only used to build a local viem account.
        const account = privateKeyToAccount(
            '0x0000000000000000000000000000000000000000000000000000000000000001'
        );

        const mockQuote: GatewayQuoteV3OneOf = {
            offramp: {
                srcChain: 'bob',
                feeBreakdown: {
                    protocolFee: { address: zeroAddress, amount: '5', chain: 'bob' },
                    affiliateFee: { address: zeroAddress, amount: '2', chain: 'bob' },
                    solverFee: { address: zeroAddress, amount: '1', chain: 'bob' },
                    inclusionFee: { address: zeroAddress, amount: '1', chain: 'bob' },
                    fastestFeeRate: '6',
                },
                inputAmount: { address: zeroAddress, amount: '1000', chain: 'bob' },
                outputAmount: { address: zeroAddress, amount: '990', chain: 'bob' },
                tokenAddress: WBTC_OFT_ADDRESS,
                ownerAddress: account.address,
                recipient: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                slippage: 0,
                totalFeeUsd: '3',
                txTo: zeroAddress,
            },
        };

        const spenderAddress = '0x1234567890123456789012345678901234567890';

        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .post('/v3/create-order')
            .reply(200, {
                offramp: {
                    order_id: 'offramp-order-local',
                    tx: { type: 'evm', chain: 'bob', to: spenderAddress, data: '0xabcdef', value: '0' },
                },
            });
        nock(`${MAINNET_GATEWAY_BASE_URL}`).patch('/v3/register-tx').reply(200, JSON.stringify('ok'));

        const sendTransactionMock = vi.fn().mockResolvedValue('0xtxhash' as `0x${string}`);
        const mockWalletClient = {
            account,
            writeContract: vi.fn().mockResolvedValue('0xapprovehash' as `0x${string}`),
            sendTransaction: sendTransactionMock,
        } as unknown as WalletClient<Transport, ViemChain, Account>;

        const mockPublicClient = {
            // approvalRequired: false -> no approve step, straight to sendTransaction
            readContract: mockOftReadContract({ approvalRequired: false }),
            simulateContract: vi.fn().mockResolvedValue({ request: {} }),
            waitForTransactionReceipt: vi.fn().mockResolvedValue({}),
        } as unknown as PublicClient<Transport>;

        await gatewaySDK.executeQuote({
            quote: mockQuote,
            walletClient: mockWalletClient,
            publicClient: mockPublicClient,
        });

        expect(sendTransactionMock).toHaveBeenCalledTimes(1);
        const passedAccount = sendTransactionMock.mock.calls[0][0].account;
        // Must be the local account object, not the downgraded address string.
        expect(passedAccount).toBe(account);
        expect((passedAccount as Account).type).toBe('local');
    });

    it('should reset USDT allowance before approving', async () => {
        const gatewaySDK = new GatewaySDK();

        const mockQuote: GatewayQuoteV3OneOf = {
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
                ownerAddress: '0xabcd1234abcd1234abcd1234abcd1234abcd1234',
                slippage: 0,
                totalFeeUsd: '3',
                txTo: zeroAddress,
            },
        };

        const spenderAddress = '0x1234567890123456789012345678901234567890';

        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .post('/v3/create-order')
            .reply(200, {
                offramp: {
                    order_id: 'offramp-order-791',
                    tx: {
                        type: 'evm',
                        chain: 'ethereum',
                        to: spenderAddress,
                        data: '0xabcdef',
                        value: '0',
                    },
                },
            });

        nock(`${MAINNET_GATEWAY_BASE_URL}`).patch('/v3/register-tx').reply(200, JSON.stringify('ok'));

        const mockWalletClient = {
            account: { address: '0xabcd1234abcd1234abcd1234abcd1234abcd1234' as Address },
            writeContract: vi.fn().mockResolvedValue('0xapprovehash' as `0x${string}`),
            sendTransaction: vi.fn().mockResolvedValue('0xtxhash' as `0x${string}`),
        } as unknown as WalletClient<Transport, ViemChain, Account>;

        const simulateContractMock = vi.fn().mockResolvedValue({ request: {} });
        const mockPublicClient = {
            readContract: mockOftReadContract({ approvalRequired: true, allowance: 1n }),
            multicall: vi.fn().mockResolvedValue([1n]),
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
        expect(simulateContractMock.mock.calls[1][0].args).toEqual([spenderAddress, 1000n]);
        expect(mockWalletClient.writeContract).toHaveBeenCalledTimes(2);
    });

    it('should execute offramp quote without approval when allowance is sufficient', async () => {
        const gatewaySDK = new GatewaySDK();

        const mockQuote: GatewayQuoteV3OneOf = {
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
                ownerAddress: '0xabcd1234abcd1234abcd1234abcd1234abcd1234',
                totalFeeUsd: '3',
            },
        };

        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .post('/v3/create-order')
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

        nock(`${MAINNET_GATEWAY_BASE_URL}`).patch('/v3/register-tx').reply(200, JSON.stringify('ok'));

        const mockWalletClient = {
            account: { address: '0xabcd1234abcd1234abcd1234abcd1234abcd1234' as Address },
            sendTransaction: async () => '0xtxhash' as `0x${string}`,
        } as unknown as WalletClient<Transport, ViemChain, Account>;

        const mockPublicClient = {
            readContract: mockOftReadContract({ approvalRequired: true, allowance: maxUint256 }),
            multicall: vi.fn().mockResolvedValue([maxUint256]),
            waitForTransactionReceipt: vi.fn().mockResolvedValue({}),
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

    it('should skip offramp approval when spender approvalRequired is false', async () => {
        const gatewaySDK = new GatewaySDK();

        const mockQuote: GatewayQuoteV3OneOf = {
            offramp: {
                txTo: '0x1234567890123456789012345678901234567890',
                recipient: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                slippage: 300,
                srcChain: 'bob',
                feeBreakdown: {
                    protocolFee: { address: zeroAddress, amount: '5', chain: 'bob' },
                    affiliateFee: { address: zeroAddress, amount: '2', chain: 'bob' },
                    inclusionFee: { address: zeroAddress, amount: '1', chain: 'bob' },
                    solverFee: { address: zeroAddress, amount: '1', chain: 'bob' },
                    fastestFeeRate: '6',
                },
                inputAmount: { address: zeroAddress, amount: '1000', chain: 'bob' },
                outputAmount: { address: zeroAddress, amount: '990', chain: 'bob' },
                tokenAddress: WBTC_OFT_ADDRESS,
                ownerAddress: '0xabcd1234abcd1234abcd1234abcd1234abcd1234',
                totalFeeUsd: '3',
            },
        };

        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .post('/v3/create-order')
            .reply(200, {
                offramp: {
                    order_id: 'offramp-order-no-approval',
                    tx: {
                        to: '0x1234567890123456789012345678901234567890',
                        data: '0xabcdef',
                        value: '0',
                    },
                },
            });

        nock(`${MAINNET_GATEWAY_BASE_URL}`).patch('/v3/register-tx').reply(200, JSON.stringify('ok'));

        const multicall = vi.fn().mockResolvedValue([0n]);
        const mockPublicClient = {
            readContract: mockOftReadContract({ approvalRequired: false }),
            multicall,
            waitForTransactionReceipt: vi.fn().mockResolvedValue({}),
        } as unknown as PublicClient<Transport>;

        const result = await gatewaySDK.executeQuote({
            quote: mockQuote,
            walletClient: {
                account: { address: '0xabcd1234abcd1234abcd1234abcd1234abcd1234' as Address },
                sendTransaction: async () => '0xtxhash' as `0x${string}`,
            } as unknown as WalletClient<Transport, ViemChain, Account>,
            publicClient: mockPublicClient,
        });

        expect(result.tx).toBe('0xtxhash');
        expect(multicall).not.toHaveBeenCalled();
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
            .post('/v3/create-order')
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
        // Mock the GET request to /v3/get-quote
        const errorMessage =
            'No route found from bitcoin (0x0000000000000000000000000000000000000001) to bob (0x0555E30da8f98308EdB960aa94C0Db47230d2B9c)';
        nock(MAINNET_GATEWAY_BASE_URL).get('/v3/get-quote').query(true).reply(400, {
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
                ownerAddress: '0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266',
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

    it('should parse gateway errors without structured details as null', () => {
        const codesWithoutDetails = [
            GatewayErrorCode.InsufficientConfirmedFunds,
            GatewayErrorCode.PerAccountLimitExceeded,
            GatewayErrorCode.GlobalLimitExceeded,
            GatewayErrorCode.InvalidRequest,
            GatewayErrorCode.InvalidOrderArgs,
            GatewayErrorCode.InvalidAffiliateFee,
            GatewayErrorCode.SlippageTooLow,
            GatewayErrorCode.SlippageTooHigh,
            GatewayErrorCode.DisabledChain,
            GatewayErrorCode.InvalidDestinationChainId,
            GatewayErrorCode.OrderNotFound,
            GatewayErrorCode.OrderExpired,
            GatewayErrorCode.DuplicateOrder,
            GatewayErrorCode.InternalError,
        ];

        for (const code of codesWithoutDetails) {
            const error = GatewayError.fromResponse({
                code,
                error: `${code} failed`,
                details: {},
            });

            expect(error.code).toBe(code);
            expect(error.details).toBeNull();
        }
    });

    it('should parse InsufficientSolverBalance gateway errors with typed details', () => {
        const error = GatewayError.fromResponse({
            code: GatewayErrorCode.InsufficientSolverBalance,
            error: 'Insufficient solver balance',
            details: {
                limit: '1000',
            },
        });

        expect(error.code).toBe(GatewayErrorCode.InsufficientSolverBalance);
        expect(error.details).toEqual({
            limit: '1000',
            token: '',
            chainId: '',
        });
    });

    it('should approve and send transaction for layerzero swap when allowance is low', async () => {
        const mockedQuote: GatewayQuoteV2OneOf2 = {
            tokenSwap: {
                dstChain: 'bob',
                estimatedTimeInSecs: 60,
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
                recipient: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                slippage: 100,
                srcChain: 'ethereum',
                txTo: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c',
            },
        };

        const gatewaySDK = new GatewaySDK();

        const readContract = mockOftReadContract({ approvalRequired: true, allowance: 0n });
        const simulateContract = vi.fn().mockResolvedValue({ request: {} });
        const waitForTransactionReceipt = vi.fn().mockResolvedValue({});
        const sendTransaction = vi.fn().mockResolvedValue('0xsendhash');
        const writeContract = vi.fn().mockResolvedValue('0xapprovehash');

        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .post('/v3/create-order')
            .reply(200, {
                tokenSwap: {
                    order_id: 'layerzero-order-123',
                    tx: {
                        to: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c',
                        data: '0xabcdef',
                        value: '0',
                    },
                },
            });

        nock(`${MAINNET_GATEWAY_BASE_URL}`).patch('/v3/register-tx').reply(200, JSON.stringify('tx-hash-123'));

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
            expect.objectContaining({ tokenSwap: expect.objectContaining({ orderId: 'layerzero-order-123' }) })
        );
        expect(readContract).toHaveBeenCalledTimes(2);
        expect(simulateContract).toHaveBeenCalledTimes(1);
        expect(writeContract).toHaveBeenCalledTimes(1);
        expect(sendTransaction).toHaveBeenCalledTimes(1);
        expect(waitForTransactionReceipt).toHaveBeenCalledTimes(2);
    });

    it('should skip approval when allowance is sufficient for layerzero swap', async () => {
        const mockedQuote: GatewayQuoteV2OneOf2 = {
            tokenSwap: {
                dstChain: 'bob',
                estimatedTimeInSecs: 60,
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
                recipient: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                slippage: 100,
                srcChain: 'ethereum',
                txTo: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c',
            },
        };

        const gatewaySDK = new GatewaySDK();

        const readContract = mockOftReadContract({ approvalRequired: true, allowance: 100000n });
        const simulateContract = vi.fn().mockResolvedValue({ request: {} });
        const waitForTransactionReceipt = vi.fn().mockResolvedValue({});
        const sendTransaction = vi.fn().mockResolvedValue('0xsendhash');
        const writeContract = vi.fn().mockResolvedValue('0xapprovehash');

        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .post('/v3/create-order')
            .reply(200, {
                tokenSwap: {
                    order_id: 'layerzero-order-123',
                    tx: {
                        to: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c',
                        data: '0xabcdef',
                        value: '0',
                    },
                },
            });

        nock(`${MAINNET_GATEWAY_BASE_URL}`).patch('/v3/register-tx').reply(200, JSON.stringify('tx-hash-123'));

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
        expect(readContract).toHaveBeenCalledTimes(2);
        expect(simulateContract).not.toHaveBeenCalled();
        expect(writeContract).not.toHaveBeenCalled();
        expect(sendTransaction).toHaveBeenCalledTimes(1);
        expect(waitForTransactionReceipt).toHaveBeenCalledTimes(1);
    });

    it('should skip allowance check for WBTC OFT token in layerzero swap', async () => {
        const mockedQuote: GatewayQuoteV2OneOf2 = {
            tokenSwap: {
                dstChain: 'ethereum',
                estimatedTimeInSecs: 60,
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
                recipient: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                slippage: 100,
                srcChain: 'bob',
                txTo: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c',
            },
        };

        const gatewaySDK = new GatewaySDK();

        const readContract = mockOftReadContract({ approvalRequired: false });
        const waitForTransactionReceipt = vi.fn().mockResolvedValue({});
        const sendTransaction = vi.fn().mockResolvedValue('0xsendhash');

        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .post('/v3/create-order')
            .reply(200, {
                tokenSwap: {
                    order_id: 'layerzero-order-123',
                    tx: {
                        to: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c',
                        data: '0xabcdef',
                        value: '0',
                    },
                },
            });

        nock(`${MAINNET_GATEWAY_BASE_URL}`).patch('/v3/register-tx').reply(200, JSON.stringify('tx-hash-123'));

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
        expect(readContract).toHaveBeenCalledTimes(1);
        expect(sendTransaction).toHaveBeenCalledTimes(1);
        expect(waitForTransactionReceipt).toHaveBeenCalledTimes(1);
    });

    it('should skip approval checks for native token layerzero swap', async () => {
        const mockedQuote: GatewayQuoteV2OneOf2 = {
            tokenSwap: {
                dstChain: 'bob',
                estimatedTimeInSecs: 60,
                fees: {
                    amount: '0',
                    address: zeroAddress,
                    chain: 'ethereum',
                },
                inputAmount: {
                    amount: '100000',
                    address: zeroAddress,
                    chain: 'ethereum',
                },
                outputAmount: {
                    amount: '100000',
                    address: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c',
                    chain: 'bob',
                },
                recipient: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                slippage: 100,
                srcChain: 'ethereum',
                txTo: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c',
            },
        };

        const gatewaySDK = new GatewaySDK();

        const readContract = vi.fn();
        const simulateContract = vi.fn();
        const waitForTransactionReceipt = vi.fn().mockResolvedValue({});
        const sendTransaction = vi.fn().mockResolvedValue('0xsendhash');
        const writeContract = vi.fn();

        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .post('/v3/create-order')
            .reply(200, {
                tokenSwap: {
                    order_id: 'layerzero-native-order-123',
                    tx: {
                        to: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c',
                        data: '0xabcdef',
                        value: '100000',
                    },
                },
            });

        nock(`${MAINNET_GATEWAY_BASE_URL}`).patch('/v3/register-tx').reply(200, JSON.stringify('tx-hash-123'));

        const result = await gatewaySDK.executeQuote({
            quote: mockedQuote,
            walletClient: {
                account: { address: '0x1234567890123456789012345678901234567890' as Address },
                sendTransaction,
                writeContract,
            } as unknown as WalletClient<Transport, ViemChain, Account>,
            publicClient: {
                readContract,
                simulateContract,
                waitForTransactionReceipt,
            } as unknown as PublicClient<Transport>,
        });

        expect(result.tx).toBe('0xsendhash');
        expect(readContract).not.toHaveBeenCalled();
        expect(simulateContract).not.toHaveBeenCalled();
        expect(writeContract).not.toHaveBeenCalled();
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
            ownerAddress: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
            amount: 1000,
        };

        const errorMessage = `Invalid ${field}: '${value}'. Expected a token address (e.g. '0x0000000000000000000000000000000000000000'), not a symbol. Use getRoutes() to find supported token addresses.`;
        nock(MAINNET_GATEWAY_BASE_URL).get('/v3/get-quote').query(true).reply(400, { error: errorMessage });

        await expect(gatewaySDK.getQuote(quoteParams)).rejects.toThrow(errorMessage);
    });

    it('should throw error when apiKey is not exactly 32 characters', () => {
        expect(() => new GatewaySDK({ apiKey: 'short' })).toThrow('apiKey must be exactly 32 characters');
        expect(() => new GatewaySDK({ apiKey: '0x1234567890' })).toThrow('apiKey must be exactly 32 characters');
        expect(() => new GatewaySDK({ apiKey: 'a'.repeat(34) })).toThrow('apiKey must be exactly 32 characters');
        expect(() => new GatewaySDK({ apiKey: 'a'.repeat(36) })).toThrow('apiKey must be exactly 32 characters');
    });

    it('should accept apiKey with exactly 32 characters', () => {
        const validApiKey = 'a'.repeat(32);
        expect(validApiKey.length).toBe(32);
        expect(() => new GatewaySDK({ apiKey: validApiKey })).not.toThrow();
    });

    it('should call callback for onramp with btcSigner (1 step: sign BTC tx)', async () => {
        const gatewaySDK = new GatewaySDK();
        const mockQuote: GatewayQuoteV2OneOf = {
            onramp: {
                dstChain: 'bob',
                dstToken: WBTC_OFT_ADDRESS,
                executionFees: { address: zeroAddress, amount: '10', chain: 'bob' },
                feeBreakdown: {
                    protocolFee: { address: zeroAddress, amount: '5', chain: 'bob' },
                    affiliateFee: { address: zeroAddress, amount: '2', chain: 'bob' },
                    executionFee: { address: zeroAddress, amount: '1', chain: 'bob' },
                    layerzeroFee: { address: zeroAddress, amount: '1', chain: 'bob' },
                    solverFee: { address: zeroAddress, amount: '1', chain: 'bob' },
                },
                fees: { address: zeroAddress, amount: '3', chain: 'bob' },
                inputAmount: { address: zeroAddress, amount: '1000', chain: 'bob' },
                outputAmount: { address: zeroAddress, amount: '990', chain: 'bob' },
                recipient: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                sender: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                signedQuoteData: MOCK_SIGNED_QUOTE_DATA,
                slippage: '0',
                token: '0x0000000000000000000000000000000000000000',
            },
        };
        const mockPsbt = 'cHNidP8BAH0CAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAzwAAAAA=';
        const signedTx = '02000000010000000000000000000000000000000000000000000000000000000000000000';

        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .post('/v3/create-order')
            .reply(200, {
                onramp: {
                    order_id: 'order-123',
                    psbt_hex: mockPsbt,
                    address: 'tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx',
                    op_return_data: '',
                },
            });
        nock(`${MAINNET_GATEWAY_BASE_URL}`).patch('/v3/register-tx').reply(200, JSON.stringify('tx-hash-123'));

        const callback = vi.fn<(step: ExecuteQuoteStep) => void>();
        await gatewaySDK.executeQuote({
            quote: mockQuote,
            walletClient: {
                account: { address: '0x1234567890123456789012345678901234567890' as Address },
            } as WalletClient<Transport, ViemChain, Account>,
            publicClient: {} as PublicClient<Transport>,
            btcSigner: { signAllInputs: async () => signedTx },
            callback,
        });

        expect(callback).toHaveBeenCalledTimes(1);
        expect(callback.mock.calls[0][0]).toEqual({
            step: 1,
            type: ExecuteQuoteStepType.SignBitcoinTransaction,
            totalSteps: 1,
        });
    });

    it('should not call callback for walletless onramp (0 wallet calls)', async () => {
        const gatewaySDK = new GatewaySDK();
        const mockQuote: GatewayQuoteV2OneOf = {
            onramp: {
                dstChain: 'bob',
                dstToken: WBTC_OFT_ADDRESS,
                executionFees: { address: zeroAddress, amount: '10', chain: 'bob' },
                feeBreakdown: {
                    protocolFee: { address: zeroAddress, amount: '5', chain: 'bob' },
                    affiliateFee: { address: zeroAddress, amount: '2', chain: 'bob' },
                    executionFee: { address: zeroAddress, amount: '1', chain: 'bob' },
                    layerzeroFee: { address: zeroAddress, amount: '1', chain: 'bob' },
                    solverFee: { address: zeroAddress, amount: '1', chain: 'bob' },
                },
                fees: { address: zeroAddress, amount: '3', chain: 'bob' },
                inputAmount: { address: zeroAddress, amount: '1000', chain: 'bob' },
                outputAmount: { address: zeroAddress, amount: '990', chain: 'bob' },
                recipient: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                sender: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                signedQuoteData: MOCK_SIGNED_QUOTE_DATA,
                slippage: '0',
                token: '0x0000000000000000000000000000000000000000',
            },
        };

        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .post('/v3/create-order')
            .reply(200, {
                onramp: {
                    order_id: 'walletless-order-123',
                    address: 'tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx',
                    op_return_data: '',
                },
            });

        const callback = vi.fn<(step: ExecuteQuoteStep) => void>();
        await gatewaySDK.executeQuote({
            quote: mockQuote,
            walletClient: {
                account: { address: '0x1234567890123456789012345678901234567890' as Address },
            } as WalletClient<Transport, ViemChain, Account>,
            publicClient: {} as PublicClient<Transport>,
            callback,
        });

        expect(callback).not.toHaveBeenCalled();
    });

    it('should call callback for offramp without approval (1 step: sendTransaction)', async () => {
        const gatewaySDK = new GatewaySDK();
        const mockQuote: GatewayQuoteV3OneOf = {
            offramp: {
                txTo: '0x1234567890123456789012345678901234567890',
                recipient: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                slippage: 300,
                srcChain: 'bob',
                feeBreakdown: {
                    protocolFee: { address: zeroAddress, amount: '5', chain: 'bob' },
                    affiliateFee: { address: zeroAddress, amount: '2', chain: 'bob' },
                    inclusionFee: { address: zeroAddress, amount: '1', chain: 'bob' },
                    solverFee: { address: zeroAddress, amount: '1', chain: 'bob' },
                    fastestFeeRate: '6',
                },
                inputAmount: { address: zeroAddress, amount: '1000', chain: 'bob' },
                outputAmount: { address: zeroAddress, amount: '990', chain: 'bob' },
                tokenAddress: WBTC_OFT_ADDRESS,
                ownerAddress: '0xabcd1234abcd1234abcd1234abcd1234abcd1234',
                totalFeeUsd: '3',
            },
        };
        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .post('/v3/create-order')
            .reply(200, {
                offramp: {
                    order_id: 'offramp-order-cb-456',
                    tx: { to: '0x1234567890123456789012345678901234567890', data: '0xabcdef', value: '0' },
                },
            });
        nock(`${MAINNET_GATEWAY_BASE_URL}`).patch('/v3/register-tx').reply(200, JSON.stringify('ok'));

        const callback = vi.fn<(step: ExecuteQuoteStep) => void>();
        await gatewaySDK.executeQuote({
            quote: mockQuote,
            walletClient: {
                account: { address: '0xabcd1234abcd1234abcd1234abcd1234abcd1234' as Address },
                sendTransaction: async () => '0xtxhash' as `0x${string}`,
            } as unknown as WalletClient<Transport, ViemChain, Account>,
            publicClient: {
                readContract: mockOftReadContract({ approvalRequired: true, allowance: maxUint256 }),
                multicall: vi.fn().mockResolvedValue([maxUint256]),
                waitForTransactionReceipt: vi.fn().mockResolvedValue({}),
            } as unknown as PublicClient<Transport>,
            callback,
        });

        expect(callback).toHaveBeenCalledTimes(1);
        expect(callback.mock.calls[0][0]).toEqual({
            step: 1,
            type: ExecuteQuoteStepType.SendTransaction,
            totalSteps: 1,
        });
    });

    it('should call callback for offramp with approval (2 steps)', async () => {
        const gatewaySDK = new GatewaySDK();
        const mockQuote: GatewayQuoteV3OneOf = {
            offramp: {
                txTo: '0x1234567890123456789012345678901234567890',
                recipient: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                slippage: 300,
                srcChain: 'bob',
                feeBreakdown: {
                    protocolFee: { address: zeroAddress, amount: '5', chain: 'bob' },
                    affiliateFee: { address: zeroAddress, amount: '2', chain: 'bob' },
                    inclusionFee: { address: zeroAddress, amount: '1', chain: 'bob' },
                    solverFee: { address: zeroAddress, amount: '1', chain: 'bob' },
                    fastestFeeRate: '6',
                },
                inputAmount: { address: zeroAddress, amount: '1000', chain: 'bob' },
                outputAmount: { address: zeroAddress, amount: '990', chain: 'bob' },
                tokenAddress: WBTC_OFT_ADDRESS,
                ownerAddress: '0xabcd1234abcd1234abcd1234abcd1234abcd1234',
                totalFeeUsd: '3',
            },
        };
        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .post('/v3/create-order')
            .reply(200, {
                offramp: {
                    order_id: 'offramp-order-cb-789',
                    tx: { to: '0x1234567890123456789012345678901234567890', data: '0xabcdef', value: '0' },
                },
            });
        nock(`${MAINNET_GATEWAY_BASE_URL}`).patch('/v3/register-tx').reply(200, JSON.stringify('ok'));

        const callback = vi.fn<(step: ExecuteQuoteStep) => void>();
        await gatewaySDK.executeQuote({
            quote: mockQuote,
            walletClient: {
                account: { address: '0xabcd1234abcd1234abcd1234abcd1234abcd1234' as Address },
                writeContract: vi.fn().mockResolvedValue('0xapprovehash' as `0x${string}`),
                sendTransaction: vi.fn().mockResolvedValue('0xtxhash' as `0x${string}`),
            } as unknown as WalletClient<Transport, ViemChain, Account>,
            publicClient: {
                readContract: mockOftReadContract({ approvalRequired: true }),
                multicall: vi.fn().mockResolvedValue([0n]),
                simulateContract: vi.fn().mockResolvedValue({ request: {} }),
                waitForTransactionReceipt: vi.fn().mockResolvedValue({}),
            } as unknown as PublicClient<Transport>,
            callback,
        });

        expect(callback).toHaveBeenCalledTimes(2);
        expect(callback.mock.calls[0][0]).toEqual({ step: 1, type: ExecuteQuoteStepType.Approve, totalSteps: 2 });
        expect(callback.mock.calls[1][0]).toEqual({
            step: 2,
            type: ExecuteQuoteStepType.SendTransaction,
            totalSteps: 2,
        });
    });

    it('should call callback for offramp with USDT reset + approval (3 steps)', async () => {
        const gatewaySDK = new GatewaySDK();
        const mockQuote: GatewayQuoteV3OneOf = {
            offramp: {
                recipient: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                srcChain: 'ethereum',
                feeBreakdown: {
                    protocolFee: { address: zeroAddress, amount: '5', chain: 'ethereum' },
                    affiliateFee: { address: zeroAddress, amount: '2', chain: 'ethereum' },
                    inclusionFee: { address: zeroAddress, amount: '1', chain: 'ethereum' },
                    solverFee: { address: zeroAddress, amount: '1', chain: 'ethereum' },
                    fastestFeeRate: '6',
                },
                inputAmount: { address: zeroAddress, amount: '1000', chain: 'ethereum' },
                outputAmount: { address: zeroAddress, amount: '990', chain: 'ethereum' },
                tokenAddress: ETHEREUM_USDT_ADDRESS,
                ownerAddress: '0xabcd1234abcd1234abcd1234abcd1234abcd1234',
                slippage: 0,
                totalFeeUsd: '3',
                txTo: zeroAddress,
            },
        };
        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .post('/v3/create-order')
            .reply(200, {
                offramp: {
                    order_id: 'offramp-usdt-cb-order',
                    tx: { to: '0x1234567890123456789012345678901234567890', data: '0xabcdef', value: '0' },
                },
            });
        nock(`${MAINNET_GATEWAY_BASE_URL}`).patch('/v3/register-tx').reply(200, JSON.stringify('ok'));

        const callback = vi.fn<(step: ExecuteQuoteStep) => void>();
        await gatewaySDK.executeQuote({
            quote: mockQuote,
            walletClient: {
                account: { address: '0xabcd1234abcd1234abcd1234abcd1234abcd1234' as Address },
                writeContract: vi.fn().mockResolvedValue('0xapprovehash' as `0x${string}`),
                sendTransaction: vi.fn().mockResolvedValue('0xtxhash' as `0x${string}`),
            } as unknown as WalletClient<Transport, ViemChain, Account>,
            publicClient: {
                readContract: mockOftReadContract({ approvalRequired: true, allowance: 1n }),
                multicall: vi.fn().mockResolvedValue([1n]),
                simulateContract: vi.fn().mockResolvedValue({ request: {} }),
                waitForTransactionReceipt: vi.fn().mockResolvedValue({}),
            } as unknown as PublicClient<Transport>,
            callback,
        });

        expect(callback).toHaveBeenCalledTimes(3);
        expect(callback.mock.calls[0][0]).toEqual({ step: 1, type: ExecuteQuoteStepType.ResetApproval, totalSteps: 3 });
        expect(callback.mock.calls[1][0]).toEqual({ step: 2, type: ExecuteQuoteStepType.Approve, totalSteps: 3 });
        expect(callback.mock.calls[2][0]).toEqual({
            step: 3,
            type: ExecuteQuoteStepType.SendTransaction,
            totalSteps: 3,
        });
    });

    it('should call callback for layerzero without approval (1 step: sendTransaction)', async () => {
        const gatewaySDK = new GatewaySDK();
        const mockedQuote: GatewayQuoteV2OneOf2 = {
            tokenSwap: {
                dstChain: 'bob',
                estimatedTimeInSecs: 60,
                fees: { amount: '0', address: WBTC_OFT_ADDRESS, chain: 'bob' },
                inputAmount: {
                    amount: '100000',
                    address: '0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599',
                    chain: 'ethereum',
                },
                outputAmount: { amount: '100000', address: WBTC_OFT_ADDRESS, chain: 'bob' },
                recipient: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                slippage: 100,
                srcChain: 'ethereum',
                txTo: WBTC_OFT_ADDRESS,
            },
        };
        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .post('/v3/create-order')
            .reply(200, {
                tokenSwap: {
                    order_id: 'lz-cb-no-approval',
                    tx: { to: WBTC_OFT_ADDRESS, data: '0xabcdef', value: '0' },
                },
            });
        nock(`${MAINNET_GATEWAY_BASE_URL}`).patch('/v3/register-tx').reply(200, JSON.stringify('tx-hash'));

        const callback = vi.fn<(step: ExecuteQuoteStep) => void>();
        await gatewaySDK.executeQuote({
            quote: mockedQuote,
            walletClient: {
                account: { address: '0x1234567890123456789012345678901234567890' as Address },
                sendTransaction: vi.fn().mockResolvedValue('0xsendhash'),
            } as unknown as WalletClient<Transport, ViemChain, Account>,
            publicClient: {
                readContract: mockOftReadContract({ approvalRequired: true, allowance: 100000n }),
                waitForTransactionReceipt: vi.fn().mockResolvedValue({}),
            } as unknown as PublicClient<Transport>,
            callback,
        });

        expect(callback).toHaveBeenCalledTimes(1);
        expect(callback.mock.calls[0][0]).toEqual({
            step: 1,
            type: ExecuteQuoteStepType.SendTransaction,
            totalSteps: 1,
        });
    });

    it('should call callback for layerzero with approval (2 steps)', async () => {
        const gatewaySDK = new GatewaySDK();
        const mockedQuote: GatewayQuoteV2OneOf2 = {
            tokenSwap: {
                dstChain: 'bob',
                estimatedTimeInSecs: 60,
                fees: { amount: '0', address: WBTC_OFT_ADDRESS, chain: 'bob' },
                inputAmount: {
                    amount: '100000',
                    address: '0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599',
                    chain: 'ethereum',
                },
                outputAmount: { amount: '100000', address: WBTC_OFT_ADDRESS, chain: 'bob' },
                recipient: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                slippage: 100,
                srcChain: 'ethereum',
                txTo: WBTC_OFT_ADDRESS,
            },
        };
        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .post('/v3/create-order')
            .reply(200, {
                tokenSwap: {
                    order_id: 'lz-cb-approval',
                    tx: { to: WBTC_OFT_ADDRESS, data: '0xabcdef', value: '0' },
                },
            });
        nock(`${MAINNET_GATEWAY_BASE_URL}`).patch('/v3/register-tx').reply(200, JSON.stringify('tx-hash'));

        const callback = vi.fn<(step: ExecuteQuoteStep) => void>();
        await gatewaySDK.executeQuote({
            quote: mockedQuote,
            walletClient: {
                account: { address: '0x1234567890123456789012345678901234567890' as Address },
                sendTransaction: vi.fn().mockResolvedValue('0xsendhash'),
                writeContract: vi.fn().mockResolvedValue('0xapprovehash'),
            } as unknown as WalletClient<Transport, ViemChain, Account>,
            publicClient: {
                readContract: mockOftReadContract({ approvalRequired: true, allowance: 0n }),
                simulateContract: vi.fn().mockResolvedValue({ request: {} }),
                waitForTransactionReceipt: vi.fn().mockResolvedValue({}),
            } as unknown as PublicClient<Transport>,
            callback,
        });

        expect(callback).toHaveBeenCalledTimes(2);
        expect(callback.mock.calls[0][0]).toEqual({ step: 1, type: ExecuteQuoteStepType.Approve, totalSteps: 2 });
        expect(callback.mock.calls[1][0]).toEqual({
            step: 2,
            type: ExecuteQuoteStepType.SendTransaction,
            totalSteps: 2,
        });
    });

    it('should include Authorization header when apiKey is provided', async () => {
        const validApiKey = 'a'.repeat(32);
        const gatewaySDK = new GatewaySDK({ apiKey: validApiKey });

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
            .get('/v3/get-quote')
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
            ownerAddress: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
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

        nock(`${MAINNET_GATEWAY_BASE_URL}`).get('/v3/get-quote').query(true).reply(200, mockOnrampQuote);

        const result = await gatewaySDK.getQuote({
            fromChain: 'bitcoin',
            fromToken: '0x0000000000000000000000000000000000000000',
            toChain: 'bob',
            toToken: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c',
            fromUserAddress: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
            toUserAddress: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
            ownerAddress: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
            amount: 1000,
        });

        expect(result).toBeDefined();
        assert(instanceOfGatewayQuoteOneOf(result));
    });
});
