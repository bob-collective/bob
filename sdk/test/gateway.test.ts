import nock from 'nock';
import {
    Account,
    Address,
    ContractFunctionExecutionError,
    maxUint256,
    PublicClient,
    Transport,
    Chain as ViemChain,
    WalletClient,
    zeroAddress,
} from 'viem';
import { afterEach, assert, describe, expect, it, vi } from 'vitest';
import { GatewaySDK } from '../src/gateway';
import { MAINNET_GATEWAY_BASE_URL } from '../src/gateway/client';
import {
    GatewayOrderInfo,
    GatewayQuoteOneOf,
    GatewayQuoteOneOf1,
    GatewayQuoteOneOf2,
    instanceOfGatewayQuoteOneOf,
    instanceOfGatewayQuoteOneOf1,
    instanceOfGatewayQuoteOneOf2,
} from '../src/gateway/generated-client';
import { BitcoinSigner } from '../src/gateway/types';

export const WBTC_OFT_ADDRESS = '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c';

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

        const txHash = await gatewaySDK.executeQuote({
            quote: mockQuote,
            walletClient: mockWalletClient,
            publicClient: mockPublicClient,
            btcSigner: mockBtcSigner,
        });

        expect(txHash).toBe('tx-hash-123');
    });

    it('should throw error for onramp without btcSigner', async () => {
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
                slippage: '0',
                token: '0x0000000000000000000000000000000000000000',
            },
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
            })
        ).rejects.toThrow('btcSigner is required for onramp order');
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

        const mockQuote: GatewayQuoteOneOf1 = {
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

        const txHash = await gatewaySDK.executeQuote({
            quote: mockQuote,
            walletClient: mockWalletClient,
            publicClient: mockPublicClient,
        });

        expect(txHash).toBe('0xtxhash');
    });

    it('should execute offramp quote without approval when allowance is sufficient', async () => {
        const gatewaySDK = new GatewaySDK();

        const mockQuote: GatewayQuoteOneOf1 = {
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

        const txHash = await gatewaySDK.executeQuote({
            quote: mockQuote,
            walletClient: mockWalletClient,
            publicClient: mockPublicClient,
        });

        expect(txHash).toBe('0xtxhash');
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

    it('should approve and send transaction for layerzero swap when allowance is low', async () => {
        const mockedQuote: GatewayQuoteOneOf2 = {
            layerZero: {
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

        const txHash = await gatewaySDK.executeQuote({
            quote: mockedQuote,
            walletClient: mockWalletClient,
            publicClient: mockPublicClient,
        });

        expect(txHash).toBe('0xsendhash');
        expect(readContract).toHaveBeenCalledTimes(1);
        expect(simulateContract).toHaveBeenCalledTimes(1);
        expect(writeContract).toHaveBeenCalledTimes(1);
        expect(sendTransaction).toHaveBeenCalledTimes(1);
        expect(waitForTransactionReceipt).toHaveBeenCalledTimes(2);
    });

    it('should skip approval when allowance is sufficient for layerzero swap', async () => {
        const mockedQuote: GatewayQuoteOneOf2 = {
            layerZero: {
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

        const txHash = await gatewaySDK.executeQuote({
            quote: mockedQuote,
            walletClient: mockWalletClient,
            publicClient: mockPublicClient,
        });

        expect(txHash).toBe('0xsendhash');
        expect(readContract).toHaveBeenCalledTimes(1);
        expect(simulateContract).not.toHaveBeenCalled();
        expect(writeContract).not.toHaveBeenCalled();
        expect(sendTransaction).toHaveBeenCalledTimes(1);
        expect(waitForTransactionReceipt).toHaveBeenCalledTimes(1);
    });

    it('should skip allowance check for WBTC OFT token in layerzero swap', async () => {
        const mockedQuote: GatewayQuoteOneOf2 = {
            layerZero: {
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

        const mockWalletClient = {
            account: { address: '0x1234567890123456789012345678901234567890' as Address },
            sendTransaction,
        } as unknown as WalletClient<Transport, ViemChain, Account>;

        const mockPublicClient = {
            readContract,
            waitForTransactionReceipt,
        } as unknown as PublicClient<Transport>;

        const txHash = await gatewaySDK.executeQuote({
            quote: mockedQuote,
            walletClient: mockWalletClient,
            publicClient: mockPublicClient,
        });

        expect(txHash).toBe('0xsendhash');
        expect(readContract).not.toHaveBeenCalled();
        expect(sendTransaction).toHaveBeenCalledTimes(1);
        expect(waitForTransactionReceipt).toHaveBeenCalledTimes(1);
    });
});
