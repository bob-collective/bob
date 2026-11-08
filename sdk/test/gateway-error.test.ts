import { describe, expect, it } from 'vitest';
import {
    ApiError,
    GatewayError,
    GatewayErrorCode,
    GatewayErrorCodeV2,
    GatewayErrorCodeV3,
    isApiError,
} from '../src/gateway/error';

// Every body here is copied from a live mainnet response. The Gateway serialises detail fields in
// snake_case, so a camelCase read silently yields nothing and the figures never reach the caller.
describe('GatewayError.fromResponse — snake_case detail bodies', () => {
    it('reads the slippage a route requires', () => {
        const error = GatewayError.fromResponse({
            code: GatewayErrorCode.SlippageTooLow,
            error: 'Across requires a slippage of at least 550 bps, got 500 bps',
            details: { required_bps: '550', requested_bps: '500' },
        });

        expect(error.code).toBe(GatewayErrorCode.SlippageTooLow);
        expect(error.details).toEqual({ requiredBps: '550', requestedBps: '500' });
    });

    it('reads the pair a missing route refers to', () => {
        const error = GatewayError.fromResponse({
            code: GatewayErrorCode.NoRoute,
            error: 'No route found',
            details: {
                src_chain: 'bitcoin',
                src_token: '0x0000000000000000000000000000000000000000',
                dst_chain: 'base',
                dst_token: '0xecAc9C5F704e954931349Da37F60E39f515c11c1',
            },
        });

        expect(error.details).toEqual({
            srcChain: 'bitcoin',
            srcToken: '0x0000000000000000000000000000000000000000',
            dstChain: 'base',
            dstToken: '0xecAc9C5F704e954931349Da37F60E39f515c11c1',
        });
    });

    it('reads the chain a solver ceiling applies to', () => {
        const error = GatewayError.fromResponse({
            code: GatewayErrorCodeV2.InsufficientSolverBalance,
            error: 'Insufficient solver balance',
            details: { limit: '1000000', token: '0xdAC17F958D2ee523a2206206994597C13D831ec7', chain_id: '1' },
        });

        expect(error.details).toMatchObject({ chainId: '1', limit: '1000000' });
    });

    it('reads the fee figures a shortfall reports', () => {
        const error = GatewayError.fromResponse({
            code: GatewayErrorCode.UnableToCoverFees,
            error: 'Unable to cover fees',
            details: { total_fees: '5000', available_amount: '1200' },
        });

        expect(error.details).toEqual({ totalFees: '5000', availableAmount: '1200' });
    });

    it('reads the tenderly url a simulation failure carries', () => {
        const error = GatewayError.fromResponse({
            code: GatewayErrorCode.SimulationFailed,
            error: 'Simulation failed',
            details: { tenderly_url: 'https://dashboard.tenderly.co/tx/1' },
        });

        expect(error.details).toEqual({ tenderlyUrl: 'https://dashboard.tenderly.co/tx/1' });
    });

    it('still gives an object when the response omits details entirely', () => {
        const error = GatewayError.fromResponse({
            code: GatewayErrorCode.SlippageTooLow,
            error: 'Across requires a slippage of at least 550 bps, got 500 bps',
        });

        // `DetailsFor` promises an object for any code that carries details, so a caller may read a
        // field without a null check. Passing the body straight through handed them `null`.
        expect(error.details).not.toBeNull();
        expect(error.details.requiredBps).toBeUndefined();
    });

    it('keeps the compliance addresses iterable when the gateway omits them', () => {
        const omitted = GatewayError.fromResponse({
            code: GatewayErrorCodeV3.NonCompliantAddresses,
            error: 'Non-compliant addresses',
            details: {},
        });

        // Declared as an array, so callers map over it without a guard.
        expect(omitted.details.addresses).toEqual([]);
        expect(() => omitted.details.addresses.map((a) => a)).not.toThrow();
    });

    it('leaves an absent detail undefined rather than the string "undefined"', () => {
        const error = GatewayError.fromResponse({
            code: GatewayErrorCodeV3.TooManyAffiliates,
            error: 'Too many affiliates',
            details: {},
        });

        expect(error.details).toStrictEqual({ max: undefined, actual: undefined });
    });
});

// The UI treats a 403 as the geoblock verdict, and only the status tells a 403 apart from a 5xx
// once the body is not a Gateway error, so every factory carries the status it was handed.
describe('GatewayError — HTTP status code', () => {
    it('keeps the status beside the code of a Gateway error body', () => {
        const error = GatewayError.fromResponse(
            { code: 'GEOBLOCKED', error: 'Service not available in your region' },
            { httpCode: 403 }
        );

        expect(error.httpCode).toBe(403);
        expect(error.code).toBe('GEOBLOCKED');
    });

    it('keeps the status when a JSON body is not a Gateway error', () => {
        const error = GatewayError.fromResponse({ message: 'Forbidden' }, { httpCode: 403 });

        expect(error.code).toBe(GatewayErrorCode.InternalError);
        expect(error.httpCode).toBe(403);
    });

    it('keeps the status when the JSON body is not an object', () => {
        const error = GatewayError.fromResponse(null as unknown as object, { httpCode: 502 });

        expect(error.httpCode).toBe(502);
    });

    it('keeps the status and the cause on an error built from text', () => {
        const cause = new SyntaxError('Unexpected token <');
        const error = GatewayError.fromText('Non-JSON response: HTTP 403', { httpCode: 403, cause });

        expect(error.httpCode).toBe(403);
        expect(error.cause).toBe(cause);
    });

    it('leaves the status undefined for an error not built from a response', () => {
        expect(GatewayError.fromText('boom').httpCode).toBeUndefined();
        expect(
            GatewayError.fromResponse({ code: GatewayErrorCode.InternalError, error: 'boom' }).httpCode
        ).toBeUndefined();
        expect(new GatewayError(GatewayErrorCode.InternalError, 'boom', null).httpCode).toBeUndefined();
    });
});

describe('ApiError', () => {
    it('keeps the status and the cause it was built with', () => {
        const cause = new SyntaxError('Unexpected token <');
        const error = new ApiError('Non-JSON response: HTTP 403', { httpCode: 403, cause });

        expect(error.httpCode).toBe(403);
        expect(error.cause).toBe(cause);
        expect(error.message).toBe('Non-JSON response: HTTP 403');
        expect(error.name).toBe('ApiError');
    });

    it('is the base of GatewayError, so one check covers both', () => {
        const error = GatewayError.fromResponse({ code: 'GEOBLOCKED', error: 'blocked' }, { httpCode: 403 });

        expect(error).toBeInstanceOf(ApiError);
        expect(isApiError(error)).toBe(true);
        expect(error.name).toBe('GatewayError');
    });

    it('does not match an unrelated error', () => {
        expect(isApiError(new Error('boom'))).toBe(false);
        expect(isApiError('boom')).toBe(false);
    });
});
