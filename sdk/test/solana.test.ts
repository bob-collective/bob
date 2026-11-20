import { base58, base64 } from '@scure/base';
import nock from 'nock';
import { Account, bytesToHex, PublicClient, Transport, Chain as ViemChain, WalletClient } from 'viem';
import { afterEach, assert, describe, expect, it, vi } from 'vitest';
import { ExecuteQuoteError, ExecuteQuoteStepType, GatewaySDK, type ExecuteQuoteStep } from '../src/gateway';
import { MAINNET_GATEWAY_BASE_URL } from '../src/gateway/client';
import { GatewayQuoteV4OneOf, GatewayQuoteV4OneOf1 } from '../src/gateway/generated-client';
import { readSolanaTransactionHeader, solanaTransactionData } from '../src/gateway/utils';

const FEE_PAYER = new Uint8Array(32).fill(1);
const OTHER_KEY = new Uint8Array(32).fill(2);
const PROGRAM_ID = new Uint8Array(32).fill(3);
const WALLET_ADDRESS = base58.encode(FEE_PAYER);
const SIGNATURE = base58.encode(new Uint8Array(64).fill(7));
const USDC_MINT = 'EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v';

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
    return { type: 'solana', chainId: 900, transaction: base64.encode(transaction) };
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

/** A viem-shaped Solana adapter, as an app builds one: Base58 account, signature returned as the hash. */
function solanaClients({
    sendTransaction = vi.fn().mockResolvedValue(SIGNATURE),
    receipt = { status: 'success' },
}: { sendTransaction?: ReturnType<typeof vi.fn>; receipt?: object } = {}) {
    const walletClient = {
        account: { address: WALLET_ADDRESS, type: 'json-rpc' },
        sendTransaction,
        writeContract: vi.fn(),
    };
    const publicClient = {
        readContract: vi.fn(),
        simulateContract: vi.fn(),
        waitForTransactionReceipt: vi.fn().mockResolvedValue(receipt),
    };

    return {
        walletClient,
        publicClient,
        params: {
            walletClient: walletClient as unknown as WalletClient<Transport, ViemChain, Account>,
            publicClient: publicClient as unknown as PublicClient<Transport>,
        },
    };
}

function mockCreateOrder(body: object) {
    return nock(MAINNET_GATEWAY_BASE_URL).post('/v4/create-order').reply(201, body);
}

describe('orders from Solana', () => {
    afterEach(() => {
        nock.cleanAll();
    });

    it('sends an offramp from Solana through the wallet client and waits for its receipt', async () => {
        const transaction = buildTransaction();
        mockCreateOrder({ offramp: { order_id: 'sol-offramp-1', tx: solanaTx(transaction) } });
        const { walletClient, publicClient, params } = solanaClients();
        const steps: ExecuteQuoteStep[] = [];

        const result = await new GatewaySDK().executeQuote({
            quote: offrampQuote(),
            ...params,
            callback: (step) => steps.push(step),
        });

        expect(result.tx).toBe(SIGNATURE);
        expect(result.order).toEqual(
            expect.objectContaining({ offramp: expect.objectContaining({ orderId: 'sol-offramp-1' }) })
        );
        expect(walletClient.sendTransaction).toHaveBeenCalledExactlyOnceWith({
            account: WALLET_ADDRESS,
            data: bytesToHex(transaction),
        });
        expect(publicClient.waitForTransactionReceipt).toHaveBeenCalledWith(
            expect.objectContaining({ hash: SIGNATURE })
        );
        // No approvals on Solana.
        expect(publicClient.readContract).not.toHaveBeenCalled();
        expect(publicClient.simulateContract).not.toHaveBeenCalled();
        expect(walletClient.writeContract).not.toHaveBeenCalled();
        expect(steps).toEqual([
            { step: 1, type: ExecuteQuoteStepType.SendTransaction, totalSteps: 1, orderId: 'sol-offramp-1' },
        ]);
    });

    it('sends a token swap from Solana, legacy message included', async () => {
        const transaction = buildTransaction({ version: 'legacy' });
        mockCreateOrder({ tokenSwap: { order_id: 'sol-swap-1', tx: solanaTx(transaction) } });
        const { walletClient, publicClient, params } = solanaClients();

        const result = await new GatewaySDK().executeQuote({ quote: tokenSwapQuote(), ...params });

        expect(result.tx).toBe(SIGNATURE);
        expect(walletClient.sendTransaction).toHaveBeenCalledExactlyOnceWith({
            account: WALLET_ADDRESS,
            data: bytesToHex(transaction),
        });
        expect(publicClient.readContract).not.toHaveBeenCalled();
    });

    it('reports a transaction that failed on-chain as reverted', async () => {
        mockCreateOrder({ tokenSwap: { order_id: 'sol-swap-2', tx: solanaTx() } });
        const { params } = solanaClients({ receipt: { status: 'reverted' } });

        const error = await new GatewaySDK()
            .executeQuote({ quote: tokenSwapQuote(), ...params })
            .catch((thrown: unknown) => thrown);

        assert(error instanceof ExecuteQuoteError);
        expect(error.orderId).toBe('sol-swap-2');
        expect(error.step).toBe(ExecuteQuoteStepType.SendTransaction);
        expect(error.txHash).toBe(SIGNATURE);
        expect(error.txStatus).toBe('reverted');
    });

    it('reports a confirmation that could not be awaited as unconfirmed, naming the signature', async () => {
        mockCreateOrder({ offramp: { order_id: 'sol-offramp-4', tx: solanaTx() } });
        const { publicClient, params } = solanaClients();
        const timeout = new Error('Transaction was not confirmed in 30.00 seconds');
        publicClient.waitForTransactionReceipt.mockRejectedValue(timeout);

        const error = await new GatewaySDK()
            .executeQuote({ quote: offrampQuote(), ...params })
            .catch((thrown: unknown) => thrown);

        assert(error instanceof ExecuteQuoteError);
        expect(error.orderId).toBe('sol-offramp-4');
        expect(error.cause).toBe(timeout);
        expect(error.step).toBe(ExecuteQuoteStepType.SendTransaction);
        expect(error.txHash).toBe(SIGNATURE);
        expect(error.txStatus).toBe('unconfirmed');
    });

    it('reports a wallet rejection on the send step without a transaction', async () => {
        mockCreateOrder({ offramp: { order_id: 'sol-offramp-2', tx: solanaTx() } });
        const rejection = new Error('User rejected the request.');
        const { publicClient, params } = solanaClients({ sendTransaction: vi.fn().mockRejectedValue(rejection) });

        const error = await new GatewaySDK()
            .executeQuote({ quote: offrampQuote(), ...params })
            .catch((thrown: unknown) => thrown);

        assert(error instanceof ExecuteQuoteError);
        expect(error.message).toBe('User rejected the request.');
        expect(error.cause).toBe(rejection);
        expect(error.step).toBe(ExecuteQuoteStepType.SendTransaction);
        expect(error.txHash).toBeUndefined();
        expect(publicClient.waitForTransactionReceipt).not.toHaveBeenCalled();
    });

    it.each([
        [
            'a malformed transaction',
            { type: 'solana', chainId: 900, transaction: base64.encode(new Uint8Array([1, 0, 0])) },
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
        [
            'a transaction of another chain',
            { type: 'evm', chain: 'solana', to: '0x1234567890123456789012345678901234567890', data: '0x', value: '0' },
            'Gateway returned a transaction of type evm for an order from solana',
        ],
    ])('refuses %s before the wallet signs', async (_, tx, message) => {
        mockCreateOrder({ offramp: { order_id: 'sol-offramp-3', tx } });
        const { walletClient, params } = solanaClients();

        const error = await new GatewaySDK()
            .executeQuote({ quote: offrampQuote(), ...params })
            .catch((thrown: unknown) => thrown);

        assert(error instanceof ExecuteQuoteError);
        expect(error.orderId).toBe('sol-offramp-3');
        expect(error.message).toBe(message);
        expect(error.txHash).toBeUndefined();
        expect(walletClient.sendTransaction).not.toHaveBeenCalled();
    });

    it.each([
        ['offramp', () => offrampQuote('ethereum'), (tx: object) => ({ offramp: { order_id: 'evm-order-1', tx } })],
        [
            'token swap',
            () => {
                const nativeEth = {
                    amount: '1',
                    address: '0x0000000000000000000000000000000000000000',
                    chain: 'ethereum',
                };
                return { tokenSwap: { ...tokenSwapQuote('ethereum').tokenSwap, inputAmount: nativeEth } };
            },
            (tx: object) => ({ tokenSwap: { order_id: 'evm-order-1', tx } }),
        ],
    ])('refuses a Solana transaction for an %s from an EVM chain', async (_, quote, order) => {
        mockCreateOrder(order(solanaTx()));
        const { walletClient, params } = solanaClients();

        const error = await new GatewaySDK()
            .executeQuote({ quote: quote(), ...params })
            .catch((thrown: unknown) => thrown);

        assert(error instanceof ExecuteQuoteError);
        expect(error.orderId).toBe('evm-order-1');
        expect(error.message).toBe('Gateway returned a Solana transaction for an order from ethereum');
        expect(walletClient.sendTransaction).not.toHaveBeenCalled();
    });

    it('refuses an EVM transaction for a token swap from Solana', async () => {
        const evmTx = {
            type: 'evm',
            chain: 'solana',
            to: '0x1234567890123456789012345678901234567890',
            data: '0x',
            value: '0',
        };
        mockCreateOrder({ tokenSwap: { order_id: 'sol-swap-3', tx: evmTx } });
        const { walletClient, publicClient, params } = solanaClients();

        const error = await new GatewaySDK()
            .executeQuote({ quote: tokenSwapQuote(), ...params })
            .catch((thrown: unknown) => thrown);

        assert(error instanceof ExecuteQuoteError);
        expect(error.orderId).toBe('sol-swap-3');
        expect(error.message).toBe('Gateway returned a transaction of type evm for an order from solana');
        expect(walletClient.sendTransaction).not.toHaveBeenCalled();
        expect(walletClient.writeContract).not.toHaveBeenCalled();
        expect(publicClient.simulateContract).not.toHaveBeenCalled();
    });
});

describe('Solana transaction utils', () => {
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

    it('turns the base64 transaction into hex data for the wallet client', () => {
        const transaction = buildTransaction();
        expect(solanaTransactionData(base64.encode(transaction), WALLET_ADDRESS)).toBe(bytesToHex(transaction));
    });
});
