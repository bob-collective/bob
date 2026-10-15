import { describe, expect, it, vi } from 'vitest';
import { parseGwei, type PublicClient, type Transport } from 'viem';
import { mainnet } from 'viem/chains';
import { estimateSendFees } from '../src/gateway/utils';

function publicClientWith(estimateFeesPerGas: ReturnType<typeof vi.fn>) {
    return { chain: mainnet, estimateFeesPerGas } as unknown as PublicClient<Transport>;
}

describe('estimateSendFees', () => {
    it('floors a zero Ethereum tip and keeps the base-fee part of maxFeePerGas', async () => {
        // eth_maxPriorityFeePerGas returns 0 on common Ethereum RPCs (bob#1214).
        const estimate = vi.fn().mockResolvedValue({ maxFeePerGas: parseGwei('0.16'), maxPriorityFeePerGas: 0n });

        const fees = await estimateSendFees(publicClientWith(estimate), 'ethereum');

        expect(fees).toEqual({
            maxPriorityFeePerGas: parseGwei('0.1'),
            maxFeePerGas: parseGwei('0.26'),
        });
    });

    it('asks viem for 2x base-fee headroom instead of its 1.2x default', async () => {
        const estimate = vi
            .fn()
            .mockResolvedValue({ maxFeePerGas: parseGwei('1'), maxPriorityFeePerGas: parseGwei('0.5') });

        await estimateSendFees(publicClientWith(estimate), 'ethereum');

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

        expect(await estimateSendFees(publicClientWith(estimate), 'ethereum')).toEqual({
            maxPriorityFeePerGas: parseGwei('0.5'),
            maxFeePerGas: parseGwei('3'),
        });
    });

    it('does not floor tips on chains without a configured minimum', async () => {
        // L2 tips are orders of magnitude below Ethereum's; a mainnet floor would overpay there.
        const estimate = vi.fn().mockResolvedValue({ maxFeePerGas: parseGwei('0.002'), maxPriorityFeePerGas: 0n });

        expect(await estimateSendFees(publicClientWith(estimate), 'base')).toEqual({
            maxPriorityFeePerGas: 0n,
            maxFeePerGas: parseGwei('0.002'),
        });
    });

    it('falls back to the caller defaults when fees cannot be estimated', async () => {
        // e.g. a legacy (non-EIP-1559) chain, or an RPC error: never a new failure mode.
        const estimate = vi.fn().mockRejectedValue(new Error('Chain does not support EIP-1559 fees.'));

        expect(await estimateSendFees(publicClientWith(estimate), 'ethereum')).toBeUndefined();
    });

    it('skips chains whose adapter takes no fee fields', async () => {
        const estimate = vi.fn();

        expect(await estimateSendFees(publicClientWith(estimate), 'tron')).toBeUndefined();
        expect(estimate).not.toHaveBeenCalled();
    });
});
