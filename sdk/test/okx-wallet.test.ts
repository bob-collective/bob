import { afterEach, assert, describe, expect, it, vi } from 'vitest';
import { EsploraClient } from '../src/esplora';
import { BitcoinBroadcastError, OkxWalletAdapter } from '../src/gateway';

describe('OkxWalletAdapter', () => {
    const txhash = 'aa'.repeat(32);

    const sendBitcoin = () => {
        const adapter = new OkxWalletAdapter({ send: vi.fn().mockResolvedValue({ txhash }) });

        // Settled into a value up front, so a rejection while timers advance is not unhandled.
        const result = adapter
            .sendBitcoin({
                from: 'tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx',
                to: 'tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx',
                value: '0.0001',
            })
            .catch((thrown: unknown) => thrown);

        return { adapter, result };
    };

    afterEach(() => {
        vi.useRealTimers();
        vi.restoreAllMocks();
    });

    it('retries fetching the hex until Esplora has the broadcast transaction', async () => {
        vi.useFakeTimers();
        const getTransactionHex = vi
            .spyOn(EsploraClient.prototype, 'getTransactionHex')
            .mockRejectedValueOnce(new Error('Transaction not found'))
            .mockRejectedValueOnce(new Error('Transaction not found'))
            .mockResolvedValue('0200');

        const { result } = sendBitcoin();
        await vi.runAllTimersAsync();

        expect(await result).toBe('0200');
        expect(getTransactionHex).toHaveBeenCalledTimes(3);
        expect(getTransactionHex).toHaveBeenCalledWith(txhash);
    });

    it('names the broadcast transaction when its hex never becomes available', async () => {
        vi.useFakeTimers();
        const notFound = new Error('Transaction not found');
        const getTransactionHex = vi.spyOn(EsploraClient.prototype, 'getTransactionHex').mockRejectedValue(notFound);

        const { result } = sendBitcoin();
        await vi.runAllTimersAsync();
        const error = await result;

        expect(error).toBeInstanceOf(BitcoinBroadcastError);
        assert(error instanceof BitcoinBroadcastError);
        expect(error.txid).toBe(txhash);
        expect(error.cause).toBe(notFound);
        expect(getTransactionHex).toHaveBeenCalledTimes(5);
    });
});
