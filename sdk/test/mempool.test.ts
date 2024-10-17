import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { MempoolClient } from '../src/mempool';

const MOCKS = {
    fees: {
        recommended: {
            fastestFee: 100,
            halfHourFee: 80,
            hourFee: 60,
            economyFee: 40,
            minimumFee: 10,
        },
    },
};

describe('Mempool Tests', () => {
    const client = new MempoolClient();

    beforeEach(() => {
        // Mock the fetch API only for URLs including /fees/recommended
        global.fetch = vi.fn((url) => {
            if (url.includes('/fees/recommended')) {
                return Promise.resolve({
                    ok: true,
                    json: () => Promise.resolve(MOCKS.fees.recommended),
                } as Response);
            }
            return Promise.reject(new Error('Unexpected URL'));
        });
    });

    afterEach(() => {
        vi.clearAllMocks();
    });

    it('should get recommended fee', async () => {
        const fees = await client.getRecommendedFees();

        expect(fees).toEqual(MOCKS.fees.recommended);
    });
});
