import { base58, base64 } from '@scure/base';
import nock from 'nock';
import { Account, Address, PublicClient, Transport, Chain as ViemChain, WalletClient } from 'viem';
import { afterEach, assert, describe, expect, it, vi } from 'vitest';
import {
    ExecuteQuoteError,
    ExecuteQuoteStepType,
    GatewaySDK,
    type ExecuteQuoteStep,
    type SolanaConnection,
    type SolanaWallet,
} from '../src/gateway';
import { MAINNET_GATEWAY_BASE_URL } from '../src/gateway/client';
import { GatewayQuoteV4OneOf, GatewayQuoteV4OneOf1 } from '../src/gateway/generated-client';
import { waitForSolanaConfirmation } from '../src/gateway/solana-order';
import { isValidSolanaAddress, readSolanaTransactionHeader } from '../src/gateway/utils';

const FEE_PAYER = new Uint8Array(32).fill(1);
const OTHER_KEY = new Uint8Array(32).fill(2);
const PROGRAM_ID = new Uint8Array(32).fill(3);
const WALLET_ADDRESS = base58.encode(FEE_PAYER);
const SIGNATURE = base58.encode(new Uint8Array(64).fill(7));
const USDC_MINT = 'EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v';
const SOLANA_CHAIN_ID = 900;

function compactU16(value: number): number[] {
    const out: number[] = [];
    for (;;) {
        const byte = value & 0x7f;
        value >>= 7;
        if (value === 0) return [...out, byte];
        out.push(byte | 0x80);
    }
}

/** An unsigned transaction calling one program, in the wire format Gateway sends (base64 of its bincode). */
function buildTransaction({
    signers = [FEE_PAYER],
    version = 0,
}: { signers?: Uint8Array[]; version?: 0 | 'legacy' } = {}): Uint8Array {
    const keys = [...signers, PROGRAM_ID];
    const message = [
        ...(version === 'legacy' ? [] : [0x80 | version]),
        signers.length,
        0,
        1,
        ...compactU16(keys.length),
        ...keys.flatMap((key) => [...key]),
        ...new Uint8Array(32).fill(9), // recent blockhash
        ...compactU16(1), // one instruction
        keys.length - 1,
        ...compactU16(0),
        ...compactU16(3),
        1,
        2,
        3,
        ...(version === 'legacy' ? [] : compactU16(0)), // no address table lookups
    ];

    return new Uint8Array([...compactU16(signers.length), ...new Array(64 * signers.length).fill(0), ...message]);
}

function solanaTx(transaction = buildTransaction()) {
    return { type: 'solana', chainId: SOLANA_CHAIN_ID, transaction: base64.encode(transaction) };
}

function offrampQuote(srcChain = 'solana'): GatewayQuoteV4OneOf {
    const amount = (value: string) => ({ address: USDC_MINT, amount: value, chain: srcChain });

    return {
        offramp: {
            srcChain,
            tokenAddress: USDC_MINT,
            txTo: base58.encode(PROGRAM_ID),
            recipient: 'bc1qafk4yhqvj4wep57m62dgrmutldusqde8adh20d',
            sender: WALLET_ADDRESS,
            slippage: 300,
            inputAmount: amount('1000000'),
            outputAmount: { address: '0x0000000000000000000000000000000000000000', amount: '1000', chain: 'bitcoin' },
            totalFeeUsd: '1',
            feeBreakdown: {
                protocolFee: amount('1'),
                affiliateFee: amount('0'),
                inclusionFee: amount('1'),
                solverFee: amount('1'),
                fastestFeeRate: '6',
            },
        },
    };
}

function tokenSwapQuote(srcChain = 'solana'): GatewayQuoteV4OneOf1 {
    return {
        tokenSwap: {
            srcChain,
            dstChain: 'bob',
            estimatedTimeInSecs: 60,
            fees: { amount: '0', address: USDC_MINT, chain: srcChain },
            inputAmount: { amount: '1000000', address: USDC_MINT, chain: srcChain },
            outputAmount: { amount: '990000', address: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c', chain: 'bob' },
            recipient: '0x1F5fF4a5B9C15d5C78Fd492e6FCF25905eB3eCFF',
            sender: WALLET_ADDRESS,
            slippage: 100,
            txTo: base58.encode(PROGRAM_ID),
        },
    };
}

function mockWallet(signAndSendTransaction = vi.fn().mockResolvedValue(SIGNATURE)) {
    return { address: WALLET_ADDRESS, signAndSendTransaction } satisfies SolanaWallet;
}

function mockConnection(...statuses: Array<{ confirmationStatus?: string | null; err: unknown } | null>) {
    const getSignatureStatuses = vi.fn();
    for (const status of statuses) getSignatureStatuses.mockResolvedValueOnce({ value: [status] });
    return { getSignatureStatuses } as unknown as SolanaConnection & {
        getSignatureStatuses: typeof getSignatureStatuses;
    };
}

const CONFIRMED = { confirmationStatus: 'confirmed', err: null };

const evmWalletClient = {
    account: { address: '0x1234567890123456789012345678901234567890' as Address },
    sendTransaction: vi.fn(),
} as unknown as WalletClient<Transport, ViemChain, Account>;

const evmPublicClient = {
    readContract: vi.fn(),
    waitForTransactionReceipt: vi.fn(),
} as unknown as PublicClient<Transport>;

function mockCreateOrder(body: object) {
    return nock(MAINNET_GATEWAY_BASE_URL).post('/v4/create-order').reply(201, body);
}

describe('Solana source orders', () => {
    afterEach(() => {
        nock.cleanAll();
        vi.clearAllMocks();
    });

    it('signs and sends an offramp from Solana with the SolanaWallet and confirms it', async () => {
        const transaction = buildTransaction();
        mockCreateOrder({ offramp: { order_id: 'sol-offramp-1', tx: solanaTx(transaction) } });
        const walletClient = mockWallet();
        const publicClient = mockConnection(CONFIRMED);
        const steps: ExecuteQuoteStep[] = [];

        const result = await new GatewaySDK().executeQuote({
            quote: offrampQuote(),
            walletClient,
            publicClient,
            callback: (step) => steps.push(step),
        });

        expect(result.tx).toBe(SIGNATURE);
        expect(result.order).toEqual(
            expect.objectContaining({ offramp: expect.objectContaining({ orderId: 'sol-offramp-1' }) })
        );
        expect(walletClient.signAndSendTransaction).toHaveBeenCalledOnce();
        expect(walletClient.signAndSendTransaction.mock.calls[0][0]).toEqual(transaction);
        expect(publicClient.getSignatureStatuses).toHaveBeenCalledWith([SIGNATURE]);
        expect(steps).toEqual([
            { step: 1, type: ExecuteQuoteStepType.SendTransaction, totalSteps: 1, orderId: 'sol-offramp-1' },
        ]);
    });

    it('signs and sends a token swap from Solana, legacy message included', async () => {
        const transaction = buildTransaction({ version: 'legacy' });
        mockCreateOrder({ tokenSwap: { order_id: 'sol-swap-1', tx: solanaTx(transaction) } });
        const walletClient = mockWallet();

        const result = await new GatewaySDK().executeQuote({
            quote: tokenSwapQuote(),
            walletClient,
            publicClient: mockConnection({ confirmationStatus: 'finalized', err: null }),
        });

        expect(result.tx).toBe(SIGNATURE);
        expect(walletClient.signAndSendTransaction.mock.calls[0][0]).toEqual(transaction);
    });

    it.each([
        ['an EVM wallet client', evmWalletClient, mockConnection(), 'walletClient must be a SolanaWallet'],
        [
            'a SolanaWallet with an invalid address',
            { ...mockWallet(), address: '0x1234567890123456789012345678901234567890' },
            mockConnection(),
            'SolanaWallet address is not a valid Solana address',
        ],
        ['an EVM public client', mockWallet(), evmPublicClient, 'publicClient must be a SolanaConnection'],
    ])('refuses %s for an order from Solana before creating it', async (_, walletClient, publicClient, message) => {
        const scope = mockCreateOrder({ offramp: { order_id: 'never', tx: solanaTx() } });

        await expect(
            new GatewaySDK().executeQuote({ quote: offrampQuote(), walletClient, publicClient })
        ).rejects.toThrow(message);
        expect(scope.isDone()).toBe(false);
    });

    it.each([
        ['offramp', offrampQuote('ethereum')],
        ['token swap', tokenSwapQuote('ethereum')],
    ])('refuses Solana clients for a %s from an EVM chain before creating it', async (_, quote) => {
        const scope = mockCreateOrder({});

        await expect(
            new GatewaySDK().executeQuote({ quote, walletClient: mockWallet(), publicClient: mockConnection() })
        ).rejects.toThrow('Solana clients cannot send an order from ethereum');
        expect(scope.isDone()).toBe(false);
    });

    it('refuses a Solana transaction for an order from an EVM chain', async () => {
        mockCreateOrder({ offramp: { order_id: 'evm-offramp-1', tx: solanaTx() } });

        const error = await new GatewaySDK()
            .executeQuote({
                quote: {
                    offramp: {
                        ...offrampQuote('bob').offramp,
                        tokenAddress: '0x0000000000000000000000000000000000000000',
                    },
                },
                walletClient: evmWalletClient,
                publicClient: evmPublicClient,
            })
            .catch((thrown: unknown) => thrown);

        assert(error instanceof ExecuteQuoteError);
        expect(error.orderId).toBe('evm-offramp-1');
        expect(error.message).toBe('Gateway returned a Solana transaction for an order from bob');
        expect(evmWalletClient.sendTransaction).not.toHaveBeenCalled();
    });

    it.each([
        [
            'an EVM transaction',
            { type: 'evm', chain: 'solana', to: '0x1234567890123456789012345678901234567890', data: '0x', value: '0' },
            'Gateway returned a evm transaction for an order from Solana',
        ],
        [
            'a malformed transaction',
            { type: 'solana', chainId: SOLANA_CHAIN_ID, transaction: base64.encode(new Uint8Array([1, 0, 0])) },
            'Gateway returned a malformed Solana transaction',
        ],
        [
            'a transaction paid by another account',
            solanaTx(buildTransaction({ signers: [OTHER_KEY] })),
            'Solana transaction fee payer is not the wallet address',
        ],
        [
            'a transaction needing a second signer',
            solanaTx(buildTransaction({ signers: [FEE_PAYER, OTHER_KEY] })),
            "Solana transaction requires 2 signatures; only the wallet's is supported",
        ],
    ])('refuses %s before the wallet signs', async (_, tx, message) => {
        mockCreateOrder({ offramp: { order_id: 'sol-offramp-2', tx } });
        const walletClient = mockWallet();

        const error = await new GatewaySDK()
            .executeQuote({ quote: offrampQuote(), walletClient, publicClient: mockConnection() })
            .catch((thrown: unknown) => thrown);

        assert(error instanceof ExecuteQuoteError);
        expect(error.orderId).toBe('sol-offramp-2');
        expect(error.message).toBe(message);
        expect(error.txHash).toBeUndefined();
        expect(walletClient.signAndSendTransaction).not.toHaveBeenCalled();
    });

    it('reports a wallet rejection on the send step without a transaction', async () => {
        mockCreateOrder({ offramp: { order_id: 'sol-offramp-3', tx: solanaTx() } });
        const rejection = new Error('User rejected the request.');
        const publicClient = mockConnection();

        const error = await new GatewaySDK()
            .executeQuote({
                quote: offrampQuote(),
                walletClient: mockWallet(vi.fn().mockRejectedValue(rejection)),
                publicClient,
            })
            .catch((thrown: unknown) => thrown);

        assert(error instanceof ExecuteQuoteError);
        expect(error.message).toBe('User rejected the request.');
        expect(error.cause).toBe(rejection);
        expect(error.step).toBe(ExecuteQuoteStepType.SendTransaction);
        expect(error.txHash).toBeUndefined();
        expect(publicClient.getSignatureStatuses).not.toHaveBeenCalled();
    });

    it('reports a transaction that failed on-chain as reverted', async () => {
        mockCreateOrder({ tokenSwap: { order_id: 'sol-swap-2', tx: solanaTx() } });

        const error = await new GatewaySDK()
            .executeQuote({
                quote: tokenSwapQuote(),
                walletClient: mockWallet(),
                publicClient: mockConnection({
                    confirmationStatus: 'processed',
                    err: { InstructionError: [0, { Custom: 1 }] },
                }),
            })
            .catch((thrown: unknown) => thrown);

        assert(error instanceof ExecuteQuoteError);
        expect(error.orderId).toBe('sol-swap-2');
        expect(error.message).toBe('Transaction reverted: {"InstructionError":[0,{"Custom":1}]}');
        expect(error.step).toBe(ExecuteQuoteStepType.SendTransaction);
        expect(error.txHash).toBe(SIGNATURE);
        expect(error.txStatus).toBe('reverted');
    });
});

describe('waitForSolanaConfirmation', () => {
    it('keeps polling past unseen and processed statuses and RPC errors', async () => {
        const connection = mockConnection(null, { confirmationStatus: 'processed', err: null });
        connection.getSignatureStatuses.mockRejectedValueOnce(new Error('429 Too Many Requests'));
        connection.getSignatureStatuses.mockResolvedValueOnce({ value: [CONFIRMED] });

        await waitForSolanaConfirmation('order', connection, SIGNATURE, { pollIntervalMs: 1 });

        expect(connection.getSignatureStatuses).toHaveBeenCalledTimes(4);
    });

    it('names the transaction as unconfirmed once the deadline passes, with the last RPC error', async () => {
        const rpcError = new Error('fetch failed');
        const getSignatureStatuses = vi.fn().mockRejectedValue(rpcError);

        const error = await waitForSolanaConfirmation('order', { getSignatureStatuses }, SIGNATURE, {
            timeoutMs: 20,
            pollIntervalMs: 1,
        }).catch((thrown: unknown) => thrown);

        assert(error instanceof ExecuteQuoteError);
        expect(error.message).toBe('Transaction not confirmed in time');
        expect(error.cause).toBe(rpcError);
        expect(error.step).toBe(ExecuteQuoteStepType.SendTransaction);
        expect(error.txHash).toBe(SIGNATURE);
        expect(error.txStatus).toBe('unconfirmed');
    });
});

describe('Solana utils', () => {
    it('validates Base58 public keys', () => {
        expect(isValidSolanaAddress(WALLET_ADDRESS)).toBe(true);
        expect(isValidSolanaAddress(USDC_MINT)).toBe(true);
        expect(isValidSolanaAddress(SIGNATURE)).toBe(false);
        expect(isValidSolanaAddress('0x1234567890123456789012345678901234567890')).toBe(false);
        expect(isValidSolanaAddress('')).toBe(false);
    });

    it('reads the fee payer and required signatures of legacy and v0 transactions', () => {
        for (const version of [0, 'legacy'] as const) {
            expect(readSolanaTransactionHeader(buildTransaction({ version, signers: [FEE_PAYER, OTHER_KEY] }))).toEqual(
                {
                    feePayer: WALLET_ADDRESS,
                    numRequiredSignatures: 2,
                }
            );
        }
    });

    it('rejects truncated transactions and unsupported versions', () => {
        const transaction = buildTransaction();
        expect(() => readSolanaTransactionHeader(transaction.subarray(0, 70))).toThrow('truncated');
        expect(() => readSolanaTransactionHeader(new Uint8Array([]))).toThrow('truncated');

        const v1 = transaction.slice();
        v1[65] = 0x81;
        expect(() => readSolanaTransactionHeader(v1)).toThrow('Unsupported Solana transaction version 1');
    });
});
