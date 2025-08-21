import { assert, describe, it } from 'vitest';
import { LayerZeroClient } from '../src/gateway/layerzero';

describe('LayerZero Tests', () => {
    it('should get chains', async () => {
        const client = new LayerZeroClient();
        const chains = await client.getSupportedChains();
        assert.containSubset(chains, ['ethereum', 'bob']);

        const dstEid = await client.getEidForChain('ethereum');
        assert.equal(dstEid, '30101');
    });
});
