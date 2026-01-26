import nock from 'nock';
import { Account, Address, PublicClient, Transport, Chain as ViemChain, WalletClient, zeroAddress } from 'viem';
import { bob } from 'viem/chains';
import { afterEach, assert, describe, expect, it } from 'vitest';
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
import { SYMBOL_LOOKUP } from '../src/gateway/tokens';
import { BitcoinSigner } from '../src/gateway/types';

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

        const mockOnrampQuote: GatewayQuoteOneOf = {
            onramp: {
                dstChain: 'bob',
                dstToken: TBTC_ADDRESS,
                executionFees: '10',
                feeBreakdown: {
                    protocolFee: '5',
                    affiliateFee: '2',
                    executionFee: '3',
                    layerzeroFee: '3',
                    solverFee: '1',
                },
                fees: '10',
                inputAmount: '1000',
                outputAmount: '990',
                recipient: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                sender: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                slippage: '0',
                token: '0x0000000000000000000000000000000000000000',
            },
        };

        const mockOfframpQuote: GatewayQuoteOneOf1 = {
            offramp: {
                feeBreakdown: {
                    protocolFee: '5',
                    affiliateFee: '2',
                    fastestFeeRate: '2',
                    inclusionFee: '1',
                    solverFee: '3',
                },
                fees: '10',
                inputAmount: '1000',
                outputAmount: '990',
                tokenAddress: TBTC_ADDRESS,
                tx: {
                    to: '0x1234567890123456789012345678901234567890',
                    data: '0xabcdef',
                    value: '0',
                },
            },
        };

        const mockLayerZeroQuote: GatewayQuoteOneOf2 = {
            layerZero: {
                fees: '10',
                inputAmount: '1000',
                outputAmount: '990',
                tx: {
                    to: '0x1234567890123456789012345678901234567890',
                    data: '0xabcdef',
                    value: '0',
                },
            },
        };

        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .get('/api/get-quote')
            .query((q) => q.srcChain === 'bitcoin')
            .reply(200, mockOnrampQuote);

        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .get('/api/get-quote')
            .query((q) => q.dstChain === 'bitcoin')
            .reply(200, mockOfframpQuote);

        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .get('/api/get-quote')
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

    it('should get enriched tokens', async () => {
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
        const mockOrders: GatewayOrderInfo[] = [
            {
                onramp: {
                    status: 'btc-confirmation',
                    amount: '1000',
                    fees: '100',
                    orderId: 'order123',
                    dstInfo: {
                        chain: 'bob',
                        token: TBTC_ADDRESS,
                        txHash: '0xabc123',
                    },
                    srcInfo: {
                        chain: 'bitcoin',
                        txHash: '0xdef456',
                        token: '0x0000000000000000000000000000000000000000',
                    },
                    timestamp: 1625247600,
                    bitcoinExplorerUrl: 'https://blockstream.info/tx/0xdef456',
                    bobExplorerUrl: 'https://exporer.gobob.xyz/tx/0xabc123',
                    estimatedTimeInSecs: 3600,
                    layerzeroExplorerUrl: 'https://layerzero.xyz/tx/0xlayerzero123',
                },
                offramp: {
                    status: 'Accepted',
                    amount: '900',
                    fees: '100',
                    orderId: 'order456',
                    dstInfo: {
                        chain: 'bitcoin',
                        txHash: '0xghi789',
                        token: '0x0000000000000000000000000000000000000000',
                    },
                    srcInfo: {
                        chain: 'bob',
                        token: TBTC_ADDRESS,
                        txHash: '0xjkl012',
                    },
                    timestamp: 1625247600,
                    bitcoinExplorerUrl: 'https://blockstream.info/tx/0xghi789',
                    bobExplorerUrl: 'https://exporer.gobob.xyz/tx/0xjkl012',
                    estimatedTimeInSecs: 3600,
                    offrampRegistryAddress: '0xofframpRegistry123',
                    bumpFeeTx: {
                        data: '0xbumpfee123',
                        to: '0xbumpfeeToAddress',
                        value: '0',
                    },
                    refundOrderTx: {
                        data: '0xrefund123',
                        to: '0xrefundToAddress',
                        value: '0',
                    },
                },
                layerZero: {
                    amount: '1000',
                    fees: '50',
                    dstInfo: {
                        chain: 'bsc',
                        token: TBTC_ADDRESS,
                        txHash: '0xlzabc123',
                    },
                    srcInfo: {
                        chain: 'bitcoin',
                        txHash: '0xlzdef456',
                        token: '0x0000000000000000000000000000000000000000',
                    },
                    timestamp: 1625247600,
                    layerzeroExplorerUrl: 'https://layerzero.xyz/tx/0xlzdef456',
                    status: 'destinationConfirmed',
                },
            },
        ];

        nock(`${MAINNET_GATEWAY_BASE_URL}`).get(`/api/get-orders/${zeroAddress}`).reply(200, mockOrders);

        const gatewaySDK = new GatewaySDK(bob.id);
        const orders = await gatewaySDK.getOrders(zeroAddress);
        expect(orders).toBeDefined();
        assert(Array.isArray(orders));
    });

    it('should get routes', async () => {
        nock(`${MAINNET_GATEWAY_BASE_URL}/api`)
            .get('/get-routes')
            .reply(200, [
                {
                    srcChain: 'bitcoin',
                    dstChain: 'bob',
                    srcToken: '0x0000000000000000000000000000000000000000',
                    dstToken: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c',
                },
            ]);

        const gatewaySDK = new GatewaySDK(bob.id);
        const routes = await gatewaySDK.getRoutes();
        expect(routes).toBeDefined();
        expect(Array.isArray(routes)).toBe(true);
    });

    it('should execute onramp quote with btcSigner.signAllInputs', async () => {
        const gatewaySDK = new GatewaySDK(bob.id);

        const mockQuote: GatewayQuoteOneOf = {
            onramp: {
                dstChain: 'bob',
                dstToken: TBTC_ADDRESS,
                executionFees: '10',
                feeBreakdown: {
                    protocolFee: '5',
                    affiliateFee: '2',
                    executionFee: '1',
                    layerzeroFee: '1',
                    solverFee: '1',
                },
                fees: '10',
                inputAmount: '1000',
                outputAmount: '990',
                recipient: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                sender: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                slippage: '0',
                token: '0x0000000000000000000000000000000000000000',
            },
        };

        const mockOrderId = 'order-123';
        const mockPsbt = 'cHNidP8BAH0CAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAzwAAAAA=';
        const signedTx = '02000000010000000000000000000000000000000000000000000000000000000000000000';

        // Mock API calls
        nock(`${MAINNET_GATEWAY_BASE_URL}`).post('/api/start-onramp').reply(200, { id: mockOrderId, psbt: mockPsbt });

        nock(`${MAINNET_GATEWAY_BASE_URL}`).patch('/api/register-btc-tx').reply(200, 'tx-hash-123');

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
        const gatewaySDK = new GatewaySDK(bob.id);

        const mockQuote: GatewayQuoteOneOf = {
            onramp: {
                dstChain: 'bob',
                dstToken: TBTC_ADDRESS,
                executionFees: '10',
                feeBreakdown: {
                    protocolFee: '5',
                    affiliateFee: '2',
                    executionFee: '1',
                    layerzeroFee: '1',
                    solverFee: '1',
                },
                fees: '10',
                inputAmount: '1000',
                outputAmount: '990',
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
        const gatewaySDK = new GatewaySDK(bob.id);

        const mockQuote: GatewayQuoteOneOf = {
            onramp: {
                dstChain: 'bob',
                dstToken: TBTC_ADDRESS,
                executionFees: '10',
                feeBreakdown: {
                    protocolFee: '5',
                    affiliateFee: '2',
                    executionFee: '1',
                    layerzeroFee: '1',
                    solverFee: '1',
                },
                fees: '10',
                inputAmount: '1000',
                outputAmount: '990',
                recipient: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                sender: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                slippage: '0',
                token: '0x0000000000000000000000000000000000000000',
            },
        };

        const mockOrderId = 'order-123';
        const mockPsbt = 'cHNidP8BAH0CAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAzwAAAAA=';

        nock(`${MAINNET_GATEWAY_BASE_URL}`).post('/api/start-onramp').reply(200, { id: mockOrderId, psbt: mockPsbt });

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
        const gatewaySDK = new GatewaySDK(bob.id);

        const mockQuote: GatewayQuoteOneOf1 = {
            offramp: {
                feeBreakdown: {
                    protocolFee: '0',
                    affiliateFee: '0',
                    fastestFeeRate: '0',
                    inclusionFee: '0',
                    solverFee: '0',
                },
                fees: '0',
                inputAmount: '1',
                outputAmount: '1',
                tokenAddress: TBTC_ADDRESS,
                tx: {
                    to: '0x1234567890123456789012345678901234567890',
                    data: '0xabcdef',
                    value: '0',
                },
            },
        };

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
        const gatewaySDK = new GatewaySDK(bob.id);

        const mockQuote: GatewayQuoteOneOf1 = {
            offramp: {
                feeBreakdown: {
                    protocolFee: '0',
                    affiliateFee: '0',
                    fastestFeeRate: '0',
                    inclusionFee: '0',
                    solverFee: '0',
                },
                fees: '0',
                inputAmount: '1',
                outputAmount: '1',
                tokenAddress: TBTC_ADDRESS,
                tx: {
                    to: '0x1234567890123456789012345678901234567890',
                    data: '0xabcdef',
                    value: '0',
                },
            },
        };

        const mockWalletClient = {
            account: { address: '0xabcd1234abcd1234abcd1234abcd1234abcd1234' as Address },
            sendTransaction: async () => '0xtxhash' as `0x${string}`,
        } as unknown as WalletClient<Transport, ViemChain, Account>;

        const mockPublicClient = {
            multicall: async () => [BigInt('999999999999999999'), 8], // high allowance (BigInt), decimals: 8 (number)
            waitForTransactionReceipt: async () => ({}),
        } as unknown as PublicClient<Transport>;

        const txHash = await gatewaySDK.executeQuote({
            quote: mockQuote,
            walletClient: mockWalletClient,
            publicClient: mockPublicClient,
        });

        expect(txHash).toBe('0xtxhash');
    });

    it('should throw error for offramp with low decimal token', async () => {
        const gatewaySDK = new GatewaySDK(bob.id);

        const mockQuote: GatewayQuoteOneOf1 = {
            offramp: {
                feeBreakdown: {
                    protocolFee: '0',
                    affiliateFee: '0',
                    fastestFeeRate: '0',
                    inclusionFee: '0',
                    solverFee: '0',
                },
                fees: '0',
                inputAmount: '1',
                outputAmount: '1',
                tokenAddress: TBTC_ADDRESS,
                tx: {
                    to: '0x1234567890123456789012345678901234567890',
                    data: '0xabcdef',
                    value: '0',
                },
            },
        };

        const mockWalletClient = {
            account: { address: '0xabcd1234abcd1234abcd1234abcd1234abcd1234' as Address },
        } as unknown as WalletClient<Transport, ViemChain, Account>;

        const mockPublicClient = {
            multicall: async () => [0n, 6], // allowance: 0 (BigInt), decimals: 6 (number, too low)
        } as unknown as PublicClient<Transport>;
        await expect(
            gatewaySDK.executeQuote({
                quote: mockQuote,
                walletClient: mockWalletClient,
                publicClient: mockPublicClient,
            })
        ).rejects.toThrow('Tokens with less than 8 decimals are not supported');
    });

    it('should throw error when invalid quote type is provided', async () => {
        const gatewaySDK = new GatewaySDK(bob.id);

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
        const gatewaySDK = new GatewaySDK(bob.id);

        const mockQuote: GatewayQuoteOneOf = {
            onramp: {
                dstChain: 'bob',
                dstToken: TBTC_ADDRESS,
                executionFees: '10',
                feeBreakdown: {
                    protocolFee: '5',
                    affiliateFee: '2',
                    executionFee: '1',
                    layerzeroFee: '1',
                    solverFee: '1',
                },
                fees: '10',
                inputAmount: '1000',
                outputAmount: '990',
                recipient: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                sender: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
                slippage: '0',
                token: '0x0000000000000000000000000000000000000000',
            },
        };

        const mockOrderId = 'order-123';
        const mockPsbt = 'cHNidP8BAH0CAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAzwAAAAA=';

        nock(`${MAINNET_GATEWAY_BASE_URL}`).post('/api/start-onramp').reply(200, { id: mockOrderId, psbt: mockPsbt });

        const mockBtcSigner = {} as BitcoinSigner; // No methods

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
});
