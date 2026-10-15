import { describe, expect, it, vi } from 'vitest';
import { parseEther, parseGwei, type Account, type PublicClient, type Transport } from 'viem';
import { privateKeyToAccount } from 'viem/accounts';
import { mainnet } from 'viem/chains';
import { estimateSendFees, withSendFees } from '../src/gateway/utils';

// Throwaway placeholder key (= 1); only used to build a local viem account.
const local = privateKeyToAccount('0x0000000000000000000000000000000000000000000000000000000000000001');

// A zero-value write whose cap fits easily, unless a test says otherwise.
const tx = { value: 0n, gas: 100_000n };

function publicClientWith(
    estimateFeesPerGas: ReturnType<typeof vi.fn>,
    overrides: Record<string, unknown> = {}
): PublicClient<Transport> {
    return {
        chain: mainnet,
        estimateFeesPerGas,
        getBalance: vi.fn().mockResolvedValue(parseEther('1')),
        ...overrides,
    } as unknown as PublicClient<Transport>;
}

describe('estimateSendFees', () => {
    it('floors a zero Ethereum tip and keeps the base-fee part of maxFeePerGas', async () => {
        // eth_maxPriorityFeePerGas returns 0 on common Ethereum RPCs (bob#1214).
        const estimate = vi.fn().mockResolvedValue({ maxFeePerGas: parseGwei('0.16'), maxPriorityFeePerGas: 0n });

        expect(await estimateSendFees(publicClientWith(estimate), 'ethereum', local, tx)).toEqual({
            maxPriorityFeePerGas: parseGwei('0.1'),
            maxFeePerGas: parseGwei('0.26'),
        });
    });

    it('asks viem for 2x base-fee headroom instead of its 1.2x default', async () => {
        const estimate = vi
            .fn()
            .mockResolvedValue({ maxFeePerGas: parseGwei('1'), maxPriorityFeePerGas: parseGwei('0.5') });

        await estimateSendFees(publicClientWith(estimate), 'ethereum', local, tx);

        expect(estimate).toHaveBeenCalledWith(
            expect.objectContaining({
                type: 'eip1559',
                chain: expect.objectContaining({
                    id: mainnet.id,
                    fees: expect.objectContaining({ baseFeeMultiplier: 2 }),
                }),
            })
        );
    });

    it('keeps an estimated Ethereum tip that is already above the floor', async () => {
        const estimate = vi
            .fn()
            .mockResolvedValue({ maxFeePerGas: parseGwei('3'), maxPriorityFeePerGas: parseGwei('0.5') });

        expect(await estimateSendFees(publicClientWith(estimate), 'ethereum', local, tx)).toEqual({
            maxPriorityFeePerGas: parseGwei('0.5'),
            maxFeePerGas: parseGwei('3'),
        });
    });

    it('does not floor tips on chains without a configured minimum', async () => {
        // L2 tips are orders of magnitude below Ethereum's; a mainnet floor would overpay there.
        const estimate = vi.fn().mockResolvedValue({ maxFeePerGas: parseGwei('0.002'), maxPriorityFeePerGas: 0n });

        expect(await estimateSendFees(publicClientWith(estimate), 'base', local, tx)).toEqual({
            maxPriorityFeePerGas: 0n,
            maxFeePerGas: parseGwei('0.002'),
        });
    });

    it('falls back to the caller defaults when fees cannot be estimated', async () => {
        // e.g. a legacy (non-EIP-1559) chain, or an RPC error: never a new failure mode.
        const estimate = vi.fn().mockRejectedValue(new Error('Chain does not support EIP-1559 fees.'));

        expect(await estimateSendFees(publicClientWith(estimate), 'ethereum', local, tx)).toBeUndefined();
    });

    it('skips chains whose adapter takes no fee fields', async () => {
        const estimate = vi.fn();

        expect(await estimateSendFees(publicClientWith(estimate), 'tron', local, tx)).toBeUndefined();
        expect(estimate).not.toHaveBeenCalled();
    });

    it('leaves fees to JSON-RPC (browser) wallets, which choose their own', async () => {
        // An explicit field would override the wallet's own tip, and could pin a zero tip off Ethereum.
        const estimate = vi.fn().mockResolvedValue({ maxFeePerGas: parseGwei('0.16'), maxPriorityFeePerGas: 0n });
        const jsonRpc = { address: local.address, type: 'json-rpc' } as Account;

        expect(await estimateSendFees(publicClientWith(estimate), 'ethereum', jsonRpc, tx)).toBeUndefined();
        expect(estimate).not.toHaveBeenCalled();
    });

    describe('balance check: the higher cap never rejects a send that fit before', () => {
        const estimate = () =>
            vi.fn().mockResolvedValue({ maxFeePerGas: parseGwei('10'), maxPriorityFeePerGas: parseGwei('1') });
        const fees = { maxFeePerGas: parseGwei('10'), maxPriorityFeePerGas: parseGwei('1') };
        const nativeSend = { value: parseEther('1'), gas: 500_000n };

        it('keeps the fees when value + gas * maxFeePerGas fits the balance', async () => {
            // 1 ETH + 500k * 10 gwei = 1.005 ETH
            const getBalance = vi.fn().mockResolvedValue(parseEther('1.005'));

            expect(
                await estimateSendFees(publicClientWith(estimate(), { getBalance }), 'ethereum', local, nativeSend)
            ).toEqual(fees);
            expect(getBalance).toHaveBeenCalledWith({ address: local.address });
        });

        it('falls back when a native-value send would no longer fit', async () => {
            const getBalance = vi.fn().mockResolvedValue(parseEther('1.004'));

            expect(
                await estimateSendFees(publicClientWith(estimate(), { getBalance }), 'ethereum', local, nativeSend)
            ).toBeUndefined();
        });

        it('falls back when a zero-value write would no longer fit (gas paid from a near-empty balance)', async () => {
            // 500k * 10 gwei = 0.005 ETH needed; 0.004 ETH held.
            const getBalance = vi.fn().mockResolvedValue(parseEther('0.004'));

            expect(
                await estimateSendFees(publicClientWith(estimate(), { getBalance }), 'ethereum', local, {
                    value: 0n,
                    gas: 500_000n,
                })
            ).toBeUndefined();
        });

        it('falls back when the gas limit or balance is unknown', async () => {
            const failingBalance = vi.fn().mockRejectedValue(new Error('rpc unavailable'));

            expect(
                await estimateSendFees(publicClientWith(estimate()), 'ethereum', local, { value: 0n, gas: undefined })
            ).toBeUndefined();
            expect(
                await estimateSendFees(
                    publicClientWith(estimate(), { getBalance: failingBalance }),
                    'ethereum',
                    local,
                    nativeSend
                )
            ).toBeUndefined();
        });
    });
});

describe('withSendFees', () => {
    const estimate = () => vi.fn().mockResolvedValue({ maxFeePerGas: parseGwei('0.16'), maxPriorityFeePerGas: 0n });
    const request = { address: '0x0000000000000000000000000000000000000001', functionName: 'approve' };

    it('estimates the write gas, checks it against the balance and merges the fees', async () => {
        const estimateContractGas = vi.fn().mockResolvedValue(50_000n);
        const getBalance = vi.fn().mockResolvedValue(parseEther('0.01'));

        const result = await withSendFees(
            publicClientWith(estimate(), { estimateContractGas, getBalance }),
            'ethereum',
            local,
            request
        );

        expect(estimateContractGas).toHaveBeenCalledWith(request);
        expect(result).toEqual({ ...request, maxFeePerGas: parseGwei('0.26'), maxPriorityFeePerGas: parseGwei('0.1') });
    });

    it('returns the request unchanged when its gas cannot be estimated', async () => {
        const estimateContractGas = vi.fn().mockRejectedValue(new Error('execution reverted'));

        expect(
            await withSendFees(publicClientWith(estimate(), { estimateContractGas }), 'ethereum', local, request)
        ).toBe(request);
    });
});
