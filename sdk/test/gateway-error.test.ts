import { describe, expect, it } from 'vitest';
import {
    AnyGatewayError,
    ApiError,
    GatewayError,
    GatewayErrorCode,
    GatewayErrorCodeV2,
    GatewayErrorCodeV3,
    isApiError,
    isGatewayError,
} from '../src/gateway/error';

/** An error body as the middleware sees it: a JSON response with an error status. */
const jsonResponse = (body: unknown, status = 400) =>
    new Response(JSON.stringify(body), { status, headers: { 'Content-Type': 'application/json' } });

/** Reads an error body through `GatewayError.fromResponse`, as the client does for a coded one. */
const gatewayErrorFrom = (body: object, status = 400): Promise<AnyGatewayError> =>
    GatewayError.fromResponse(jsonResponse(body, status));

// Every body here is copied from a live mainnet response. The Gateway serialises detail fields in
// snake_case, so a camelCase read silently yields nothing and the figures never reach the caller.
describe('GatewayError.fromResponse — snake_case detail bodies', () => {
    it('reads the slippage a route requires', async () => {
        const error = await gatewayErrorFrom({
            code: GatewayErrorCode.SlippageTooLow,
            error: 'Across requires a slippage of at least 550 bps, got 500 bps',
            details: { required_bps: '550', requested_bps: '500' },
        });

        expect(error.code).toBe(GatewayErrorCode.SlippageTooLow);
        expect(error.details).toEqual({ requiredBps: '550', requestedBps: '500' });
    });

    it('reads the pair a missing route refers to', async () => {
        const error = await gatewayErrorFrom({
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

    it('reads the chain a solver ceiling applies to', async () => {
        const error = await gatewayErrorFrom({
            code: GatewayErrorCodeV2.InsufficientSolverBalance,
            error: 'Insufficient solver balance',
            details: { limit: '1000000', token: '0xdAC17F958D2ee523a2206206994597C13D831ec7', chain_id: '1' },
        });

        expect(error.details).toMatchObject({ chainId: '1', limit: '1000000' });
    });

    it('reads the fee figures a shortfall reports', async () => {
        const error = await gatewayErrorFrom({
            code: GatewayErrorCode.UnableToCoverFees,
            error: 'Unable to cover fees',
            details: { total_fees: '5000', available_amount: '1200' },
        });

        expect(error.details).toEqual({ totalFees: '5000', availableAmount: '1200' });
    });

    it('reads the tenderly url a simulation failure carries', async () => {
        const error = await gatewayErrorFrom({
            code: GatewayErrorCode.SimulationFailed,
            error: 'Simulation failed',
            details: { tenderly_url: 'https://dashboard.tenderly.co/tx/1' },
        });

        expect(error.details).toEqual({ tenderlyUrl: 'https://dashboard.tenderly.co/tx/1' });
    });

    it('still gives an object when the response omits details entirely', async () => {
        const error = await gatewayErrorFrom({
            code: GatewayErrorCode.SlippageTooLow,
            error: 'Across requires a slippage of at least 550 bps, got 500 bps',
        });

        // `DetailsFor` promises an object for any code that carries details, so a caller may read a
        // field without a null check. Passing the body straight through handed them `null`.
        expect(error.details).not.toBeNull();
        expect(error.details.requiredBps).toBeUndefined();
    });

    it('keeps the compliance addresses iterable when the gateway omits them', async () => {
        const omitted = await gatewayErrorFrom({
            code: GatewayErrorCodeV3.NonCompliantAddresses,
            error: 'Non-compliant addresses',
            details: {},
        });

        // Declared as an array, so callers map over it without a guard.
        expect(omitted.details.addresses).toEqual([]);
        expect(() => omitted.details.addresses.map((a) => a)).not.toThrow();
    });

    it('leaves an absent detail undefined rather than the string "undefined"', async () => {
        const error = await gatewayErrorFrom({
            code: GatewayErrorCodeV3.TooManyAffiliates,
            error: 'Too many affiliates',
            details: {},
        });

        expect(error.details).toStrictEqual({ max: undefined, actual: undefined });
    });
});

// The UI treats a 403 as the geoblock verdict, and only the status tells a 403 apart from a 5xx
// once the body is not a Gateway error, so every error read from a response carries its status.
// Each factory returns its own class; choosing between them is the client's job.
describe('ApiError.fromResponse', () => {
    it('gives a plain ApiError even for a coded body', async () => {
        const error = await ApiError.fromResponse(
            jsonResponse({ code: 'GEOBLOCKED', error: 'This service is not available in your location' }, 403)
        );

        expect(isGatewayError(error)).toBe(false);
        expect(error.httpCode).toBe(403);
        expect(error.message).toBe('This service is not available in your location');
    });

    it('carries the status and the message of JSON without a code', async () => {
        const error = await ApiError.fromResponse(jsonResponse({ message: 'Forbidden' }, 403));

        expect(error.httpCode).toBe(403);
        expect(error.message).toBe('Forbidden');
    });

    it('carries the status of JSON that is not an object', async () => {
        const error = await ApiError.fromResponse(jsonResponse(null, 502));

        expect(error.httpCode).toBe(502);
    });

    it('carries the status and the parse failure of a body that is not JSON', async () => {
        const error = await ApiError.fromResponse(
            new Response('<html>Access denied</html>', { status: 403, statusText: 'Forbidden' })
        );

        expect(error.httpCode).toBe(403);
        expect(error.message).toBe('Non-JSON response: HTTP 403 Forbidden');
        expect(error.cause).toBeInstanceOf(SyntaxError);
    });
});

describe('GatewayError.fromResponse', () => {
    it('carries the status beside the code of a coded body', async () => {
        const error = await GatewayError.fromResponse(
            jsonResponse({ code: 'GEOBLOCKED', error: 'This service is not available in your location' }, 403)
        );

        expect(isGatewayError(error)).toBe(true);
        expect(error.code).toBe('GEOBLOCKED');
        expect(error.httpCode).toBe(403);
    });

    it('falls back to an InternalError carrying the status for JSON without a code', async () => {
        const error = await GatewayError.fromResponse(jsonResponse({ message: 'Forbidden' }, 403));

        expect(isGatewayError(error)).toBe(true);
        expect(error.code).toBe(GatewayErrorCode.InternalError);
        expect(error.httpCode).toBe(403);
        expect(error.message).toBe('Forbidden');
    });

    it('falls back to an InternalError carrying the parse failure for a body that is not JSON', async () => {
        const error = await GatewayError.fromResponse(
            new Response('<html>Access denied</html>', { status: 403, statusText: 'Forbidden' })
        );

        expect(isGatewayError(error)).toBe(true);
        expect(error.code).toBe(GatewayErrorCode.InternalError);
        expect(error.httpCode).toBe(403);
        expect(error.message).toBe('Non-JSON response: HTTP 403 Forbidden');
        expect(error.cause).toBeInstanceOf(SyntaxError);
    });
});

describe('GatewayError — HTTP status code', () => {
    it('builds an InternalError from the text alone, with no status', () => {
        const error = GatewayError.fromText('boom');

        expect(error.code).toBe(GatewayErrorCode.InternalError);
        expect(error.message).toBe('boom');
        expect(error.details).toBeNull();
        expect(error.httpCode).toBeUndefined();
    });

    it('leaves the status undefined for an error built without one', () => {
        expect(new GatewayError(GatewayErrorCode.InternalError, 'boom', null).httpCode).toBeUndefined();
    });

    it('takes the status apart from the standard error options', () => {
        const cause = new Error('upstream');
        const error = new GatewayError(GatewayErrorCode.InternalError, 'boom', null, { cause }, 500);

        expect(error.cause).toBe(cause);
        expect(error.httpCode).toBe(500);
    });
});

describe('ApiError', () => {
    it('keeps the status and the cause it was built with', () => {
        const cause = new SyntaxError('Unexpected token <');
        const error = new ApiError('Non-JSON response: HTTP 403', 403, { cause });

        expect(error.httpCode).toBe(403);
        expect(error.cause).toBe(cause);
        expect(error.message).toBe('Non-JSON response: HTTP 403');
        expect(error.name).toBe('ApiError');
    });

    it('is the base of GatewayError, so one check covers both', () => {
        const error = GatewayError.fromText('blocked');

        expect(error).toBeInstanceOf(ApiError);
        expect(isApiError(error)).toBe(true);
        expect(error.name).toBe('GatewayError');
    });

    it('does not match an unrelated error', () => {
        expect(isApiError(new Error('boom'))).toBe(false);
        expect(isApiError('boom')).toBe(false);
    });
});
