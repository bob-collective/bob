import { parseAbi } from 'viem';
import { assert, describe, it } from 'vitest';
import { oftApprovalRequiredAbi } from '../src/gateway/abi';

describe('ABI literals', () => {
    // The literal replaced this `parseAbi` call so bundles skip viem's runtime ABI parser.
    it('oftApprovalRequiredAbi equals its human-readable form', () => {
        assert.deepStrictEqual(
            oftApprovalRequiredAbi,
            parseAbi(['function approvalRequired() view returns (bool)']) as unknown as typeof oftApprovalRequiredAbi
        );
    });
});
