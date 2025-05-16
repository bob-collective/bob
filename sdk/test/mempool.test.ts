import { afterEach, beforeEach, describe, expect, it, Mock, MockedFunction, vi } from 'vitest';
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
    tipBlockHash: '00000000000000000000a1717d19d2c42681ec43504fbc41970cd9f1c4a124dd',
    blockDetails: {
        id: '00000000000000000000a1717d19d2c42681ec43504fbc41970cd9f1c4a124dd',
        timestamp: 1747412420,
    },
    txInfo: {
        txid: '82420f81f3d956b464dee4a1eb2bee4810d88140dfda2e67f0fc4e32996045c6',
        size: 223,
        fee: 14200,
        status: {
            confirmed: false,
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
            if (url.includes('/blocks/tip/hash')) {
                return Promise.resolve({
                    ok: true,
                    text: () => Promise.resolve(MOCKS.tipBlockHash),
                } as Response);
            }
            if (url.includes(`/block/${MOCKS.tipBlockHash}`)) {
                return Promise.resolve({
                    ok: true,
                    json: () => Promise.resolve(MOCKS.blockDetails),
                } as Response);
            }
            if (url.includes(`/v1/tx/${MOCKS.txInfo.txid}`)) {
                return Promise.resolve({
                    ok: true,
                    json: () => Promise.resolve(MOCKS.txInfo),
                } as Response);
            }
            return Promise.reject(new Error('Unexpected URL => ' + url));
        });
    });

    afterEach(() => {
        vi.clearAllMocks();
    });

    it('should get recommended fee', async () => {
        const fees = await client.getRecommendedFees();

        expect(fees).toEqual(MOCKS.fees.recommended);
    });

    it('should get last block hash', async () => {
        const txInfo = await client.getTxInfo(MOCKS.txInfo.txid);

        expect(txInfo).toEqual(MOCKS.txInfo);
    });

    it('should get last block hash', async () => {
        const tipBlockHash = await client.getBlocksTipHash();

        expect(tipBlockHash).toEqual(MOCKS.tipBlockHash);
    });

    it('should get block details', async () => {
        const blockDetails = await client.getBlock(MOCKS.tipBlockHash);

        expect(blockDetails).toEqual(MOCKS.blockDetails);
    });

    it('should estimate tx timestamp', async () => {
        const mockData = [
            {
                txid: '111',
                fee: 100_000,
                size: 1000,
                timestamp: 60 * 10,
                status: {
                    confirmed: false,
                },
            },
            {
                txid: '222',
                fee: 80_000,
                size: 1000,
                timestamp: 3 * 60 * 10,
                status: {
                    confirmed: false,
                },
            },
            {
                txid: '333',
                fee: 60_000,
                size: 1000,
                timestamp: 6 * 60 * 10,
                status: {
                    confirmed: false,
                },
            },
            {
                txid: '444',
                fee: 40_000,
                size: 1000,
                timestamp: 144 * 60 * 10,
                status: {
                    confirmed: false,
                },
            },
            {
                txid: '555',
                fee: 10_000,
                size: 1000,
                timestamp: Infinity,
                status: {
                    confirmed: false,
                },
            },
            {
                txid: '666',
                fee: 1,
                size: 1,
                timestamp: 0,
                status: {
                    confirmed: true,
                },
            },
        ];

        const prevGlobalFetch = global.fetch as Mock;

        global.fetch = vi.fn((url) => {
            for (let data of mockData) {
                if (url.includes(`/v1/tx/${data.txid}`)) {
                    return Promise.resolve({
                        ok: true,
                        json: () => Promise.resolve({ ...MOCKS.txInfo, ...data, timestamp: undefined }),
                    } as Response);
                }
            }

            return prevGlobalFetch(url);
        });

        const results = await Promise.all(mockData.map(({ txid }) => client.estimateTxTime(txid)));

        mockData.forEach(({ timestamp }, index) => {
            // NOTE: 0 indicates that there's no timestamp in the future when tx is expected to be mined
            if (results[index] !== 0) {
                expect(MOCKS.blockDetails.timestamp + timestamp).toEqual(results[index]);
            }
        });
    });
});
