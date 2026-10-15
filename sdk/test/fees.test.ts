import { describe, expect, it, vi } from 'vitest';
import { parseEther, parseGwei, type Account, type PublicClient, type Transport } from 'viem';
import { privateKeyToAccount } from 'viem/accounts';
import { mainnet } from 'viem/chains';
import { estimateSendFees } from '../src/gateway/utils';

// Throwaway placeholder key (= 1); only used to build a local viem account.
const local = privateKeyToAccount('0x0000000000000000000000000000000000000000000000000000000000000001');

function publicClientWith(estimateFeesPerGas: ReturnType<typeof vi.fn>, getBalance = vi.fn()) {
    return { chain: mainnet, estimateFeesPerGas, getBalance } as unknown as PublicClient<Transport>;
}

describe('estimateSendFees', () => {
    it('floors a zero Ethereum tip and keeps the base-fee part of maxFeePerGas', async () => {
        // eth_maxPriorityFeePerGas returns 0 on common Ethereum RPCs (bob#1214).
        const estimate = vi.fn().mockResolvedValue({ maxFeePerGas: parseGwei('0.16'), maxPriorityFeePerGas: 0n });

        const fees = await estimateSendFees(publicClientWith(estimate), 'ethereum', local);

        expect(fees).toEqual({
            maxPriorityFeePerGas: parseGwei('0.1'),
            maxFeePerGas: parseGwei('0.26'),
        });
    });

    it('asks viem for 2x base-fee headroom instead of its 1.2x default', async () => {
        const estimate = vi
            .fn()
            .mockResolvedValue({ maxFeePerGas: parseGwei('1'), maxPriorityFeePerGas: parseGwei('0.5') });

        await estimateSendFees(publicClientWith(estimate), 'ethereum', local);

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

        expect(await estimateSendFees(publicClientWith(estimate), 'ethereum', local)).toEqual({
            maxPriorityFeePerGas: parseGwei('0.5'),
            maxFeePerGas: parseGwei('3'),
        });
    });

    it('does not floor tips on chains without a configured minimum', async () => {
        // L2 tips are orders of magnitude below Ethereum's; a mainnet floor would overpay there.
        const estimate = vi.fn().mockResolvedValue({ maxFeePerGas: parseGwei('0.002'), maxPriorityFeePerGas: 0n });

        expect(await estimateSendFees(publicClientWith(estimate), 'base', local)).toEqual({
            maxPriorityFeePerGas: 0n,
            maxFeePerGas: parseGwei('0.002'),
        });
    });

    it('falls back to the caller defaults when fees cannot be estimated', async () => {
        // e.g. a legacy (non-EIP-1559) chain, or an RPC error: never a new failure mode.
        const estimate = vi.fn().mockRejectedValue(new Error('Chain does not support EIP-1559 fees.'));

        expect(await estimateSendFees(publicClientWith(estimate), 'ethereum', local)).toBeUndefined();
    });

    it('skips chains whose adapter takes no fee fields', async () => {
        const estimate = vi.fn();

        expect(await estimateSendFees(publicClientWith(estimate), 'tron', local)).toBeUndefined();
        expect(estimate).not.toHaveBeenCalled();
    });

    it('leaves fees to JSON-RPC (browser) wallets, which choose their own', async () => {
        // An explicit field would override the wallet's own tip, and could pin a zero tip off Ethereum.
        const estimate = vi.fn().mockResolvedValue({ maxFeePerGas: parseGwei('0.16'), maxPriorityFeePerGas: 0n });
        const jsonRpc = { address: local.address, type: 'json-rpc' } as Account;

        expect(await estimateSendFees(publicClientWith(estimate), 'ethereum', jsonRpc)).toBeUndefined();
        expect(estimate).not.toHaveBeenCalled();
    });

    describe('order sends carrying native value', () => {
        const estimate = () =>
            vi.fn().mockResolvedValue({ maxFeePerGas: parseGwei('10'), maxPriorityFeePerGas: parseGwei('1') });
        const send = { value: parseEther('1'), gas: 500_000n };

        it('keeps the fees when value + gas * maxFeePerGas fits the balance', async () => {
            // 1 ETH + 500k * 10 gwei = 1.005 ETH
            const getBalance = vi.fn().mockResolvedValue(parseEther('1.005'));

            expect(await estimateSendFees(publicClientWith(estimate(), getBalance), 'ethereum', local, send)).toEqual({
                maxFeePerGas: parseGwei('10'),
                maxPriorityFeePerGas: parseGwei('1'),
            });
            expect(getBalance).toHaveBeenCalledWith({ address: local.address });
        });

        it('falls back to the wallet defaults when the higher cap would not fit the balance', async () => {
            // A near-max send sized against the old ~1.2x cap must not start failing.
            const getBalance = vi.fn().mockResolvedValue(parseEther('1.004'));

            expect(
                await estimateSendFees(publicClientWith(estimate(), getBalance), 'ethereum', local, send)
            ).toBeUndefined();
        });

        it('falls back when the gas limit or balance is unknown', async () => {
            const failingBalance = vi.fn().mockRejectedValue(new Error('rpc unavailable'));

            expect(
                await estimateSendFees(publicClientWith(estimate()), 'ethereum', local, { value: send.value })
            ).toBeUndefined();
            expect(
                await estimateSendFees(publicClientWith(estimate(), failingBalance), 'ethereum', local, send)
            ).toBeUndefined();
        });

        it('skips the balance check for sends without native value', async () => {
            const getBalance = vi.fn();

            await estimateSendFees(publicClientWith(estimate(), getBalance), 'ethereum', local, {
                value: 0n,
                gas: 500_000n,
            });
            expect(getBalance).not.toHaveBeenCalled();
        });
    });
});
