import nock from 'nock';
import { afterAll, afterEach, beforeAll, describe, expect, it } from 'vitest';
import { EsploraClient, MAINNET_ESPLORA_BASE_PATH, REGTEST_ESPLORA_BASE_PATH } from '../src/esplora';
import { ApiError, isApiError } from '../src/gateway';
import { MAINNET_MEMPOOL_BASE_PATH, MempoolClient } from '../src/mempool';

const TXID = '82420f81f3d956b464dee4a1eb2bee4810d88140dfda2e67f0fc4e32996045c6';

async function catchError(promise: Promise<unknown>): Promise<unknown> {
    try {
        await promise;
    } catch (err) {
        return err;
    }
    throw new Error('expected promise to reject');
}

beforeAll(() => {
    nock.disableNetConnect();
});

afterAll(() => {
    nock.enableNetConnect();
});

afterEach(() => {
    nock.cleanAll();
});

describe('HTTP client errors', () => {
    it('EsploraClient JSON method throws ApiError with httpCode and body text', async () => {
        nock(MAINNET_ESPLORA_BASE_PATH).get(`/tx/${TXID}`).reply(404, 'Transaction not found');

        const err = await catchError(new EsploraClient().getTransaction(TXID));

        expect(isApiError(err)).toBe(true);
        expect((err as ApiError).httpCode).toBe(404);
        expect((err as ApiError).message).toBe('Esplora error: Transaction not found (HTTP 404)');
    });

    it('EsploraClient text method throws ApiError with httpCode, falling back to the status text', async () => {
        nock(MAINNET_ESPLORA_BASE_PATH).get('/blocks/tip/height').reply(503, '');

        const err = await catchError(new EsploraClient().getLatestHeight());

        expect(err).toBeInstanceOf(ApiError);
        expect((err as ApiError).httpCode).toBe(503);
        expect((err as ApiError).message).toMatch(/^Esplora error: .+ \(HTTP 503\)$/);
    });

    it('EsploraClient.broadcastTx throws ApiError with the node rejection reason', async () => {
        nock(MAINNET_ESPLORA_BASE_PATH)
            .post('/tx', 'deadbeef')
            .reply(400, 'sendrawtransaction RPC error: {"code":-26,"message":"min relay fee not met"}');

        const err = await catchError(new EsploraClient().broadcastTx('deadbeef'));

        expect(err).toBeInstanceOf(ApiError);
        expect((err as ApiError).httpCode).toBe(400);
        expect((err as ApiError).message).toContain('Esplora broadcast error: sendrawtransaction RPC error');
    });

    it('EsploraClient.broadcastTx returns the txid on success', async () => {
        nock(MAINNET_ESPLORA_BASE_PATH).post('/tx', 'deadbeef').reply(200, TXID);

        await expect(new EsploraClient().broadcastTx('deadbeef')).resolves.toBe(TXID);
    });

    it('MempoolClient throws ApiError with httpCode and body text', async () => {
        nock(MAINNET_MEMPOOL_BASE_PATH.replace('/api', '')).get(`/api/tx/${TXID}`).reply(404, 'Transaction not found');

        const err = await catchError(new MempoolClient().getTxInfo(TXID));

        expect(err).toBeInstanceOf(ApiError);
        expect((err as ApiError).httpCode).toBe(404);
        expect((err as ApiError).message).toBe('Mempool error: Transaction not found (HTTP 404)');
    });

    it('MempoolClient text method throws ApiError', async () => {
        nock(MAINNET_MEMPOOL_BASE_PATH.replace('/api', '')).get('/api/blocks/tip/hash').reply(500, 'Internal error');

        const err = await catchError(new MempoolClient().getBlocksTipHash());

        expect(err).toBeInstanceOf(ApiError);
        expect((err as ApiError).httpCode).toBe(500);
    });
});

describe('HTTP client initOverrides', () => {
    const MEMPOOL_ORIGIN = MAINNET_MEMPOOL_BASE_PATH.replace('/api', '');

    it('EsploraClient.broadcastTx forwards initOverrides and keeps POST + body', async () => {
        nock(MAINNET_ESPLORA_BASE_PATH).post('/tx', 'deadbeef').matchHeader('x-test', 'esplora').reply(200, TXID);

        await expect(
            new EsploraClient().broadcastTx('deadbeef', { method: 'GET', headers: { 'x-test': 'esplora' } })
        ).resolves.toBe(TXID);
    });

    it('EsploraClient.broadcastTx can be aborted', async () => {
        nock(MAINNET_ESPLORA_BASE_PATH).post('/tx').delay(1000).reply(200, TXID);
        const controller = new AbortController();

        const promise = new EsploraClient().broadcastTx('deadbeef', { signal: controller.signal });
        controller.abort();

        await expect(promise).rejects.toMatchObject({ name: 'AbortError' });
    });

    it('MempoolClient methods forward initOverrides', async () => {
        const scope = nock(MEMPOOL_ORIGIN, { reqheaders: { 'x-test': 'mempool' } })
            .get('/api/v1/fees/recommended')
            .reply(200, { fastestFee: 1, halfHourFee: 1, hourFee: 1, economyFee: 1, minimumFee: 1 })
            .get(`/api/tx/${TXID}`)
            .reply(200, { txid: TXID })
            .get('/api/block/abc')
            .reply(200, { id: 'abc' })
            .get('/api/blocks/tip/hash')
            .reply(200, 'abc')
            .get('/api/address/bc1qtest/txs/mempool')
            .reply(200, []);
        const client = new MempoolClient();
        const init = { headers: { 'x-test': 'mempool' } };

        await client.getRecommendedFees(init);
        await client.getTxInfo(TXID, init);
        await client.getBlock('abc', init);
        await expect(client.getBlocksTipHash(init)).resolves.toBe('abc');
        await expect(client.getAddressMempoolTxs('bc1qtest', init)).resolves.toEqual([]);
        expect(scope.isDone()).toBe(true);
    });

    it('MempoolClient.estimateTxTime passes the signal to every request', async () => {
        nock(MEMPOOL_ORIGIN).get(/.*/).times(4).delay(1000).reply(200, {});
        const controller = new AbortController();

        const promise = new MempoolClient().estimateTxTime(TXID, { signal: controller.signal });
        controller.abort();

        await expect(promise).rejects.toMatchObject({ name: 'AbortError' });
    });
});

describe('EsploraClient base URL', () => {
    it('uses a custom base URL, dropping a trailing slash', async () => {
        nock('http://esplora.example:3002').get('/api/blocks/tip/height').reply(200, '42');

        await expect(new EsploraClient('http://esplora.example:3002/api/').getLatestHeight()).resolves.toBe(42);
    });

    it('keeps the built-in defaults for network names', async () => {
        nock(REGTEST_ESPLORA_BASE_PATH).get('/blocks/tip/height').reply(200, '7');

        await expect(new EsploraClient('regtest').getLatestHeight()).resolves.toBe(7);
    });

    it('still rejects an unknown network name', () => {
        expect(() => new EsploraClient('mainet')).toThrow('Invalid chain');
    });
});
