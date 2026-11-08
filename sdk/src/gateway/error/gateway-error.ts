import {
    GatewayErrorCode,
    GatewayErrorCodeV2Variants as GatewayErrorCodeV2,
    GatewayErrorCodeV3Variants as GatewayErrorCodeV3,
    GatewayErrorCodeV4Variants as GatewayErrorCodeV4,
    GatewayErrorDetailsV2OneOf,
    GatewayErrorDetailsV2OneOfFromJSON,
    GatewayErrorDetailsV2OneOf1,
    GatewayErrorDetailsV2OneOf1FromJSON,
    GatewayErrorDetailsV2OneOf2,
    GatewayErrorDetailsV2OneOf2FromJSON,
    GatewayErrorDetailsV2OneOf3,
    GatewayErrorDetailsV2OneOf3FromJSON,
    GatewayErrorDetailsV2OneOf4,
    GatewayErrorDetailsV2OneOf4FromJSON,
    GatewayErrorDetailsV2OneOf5,
    GatewayErrorDetailsV2OneOf5FromJSON,
    GatewayErrorDetailsV2OneOf6,
    GatewayErrorDetailsV2OneOf6FromJSON,
    GatewayErrorDetailsV2OneOf7,
    GatewayErrorDetailsV2OneOf7FromJSON,
    GatewayErrorDetailsV3OneOf,
    GatewayErrorDetailsV3OneOfFromJSON,
    GatewayErrorDetailsV3OneOf1,
    GatewayErrorDetailsV3OneOf1FromJSON,
    GatewayErrorDetailsV3OneOf2,
    GatewayErrorDetailsV3OneOf2FromJSON,
} from '../generated-client';
import type { GatewayErrorV4 as GatewayErrorInterface } from '../generated-client/models/GatewayErrorV4';
import { instanceOfGatewayErrorV4 } from '../generated-client/models/GatewayErrorV4';

export { GatewayErrorCode, GatewayErrorCodeV2, GatewayErrorCodeV3, GatewayErrorCodeV4 };

// ─── Named detail interfaces (mirror the Rust GatewayErrorDetails enum) ──────

/** Details for {@link GatewayErrorCode.InsufficientAmount} */
export type InsufficientAmountDetails = GatewayErrorDetailsV2OneOf;

/** Details for {@link GatewayErrorCode.InsufficientPaymentAmount} */
export type InsufficientPaymentAmountDetails = GatewayErrorDetailsV2OneOf;

/** Details for {@link GatewayErrorCode.InsufficientSwapAmount} */
export type InsufficientSwapAmountDetails = GatewayErrorDetailsV2OneOf1;

/** Details for {@link GatewayErrorCode.UnableToCoverFees} */
export type UnableToCoverFeesDetails = GatewayErrorDetailsV2OneOf2;

/** Details for {@link GatewayErrorCode.SimulationFailed} */
export type SimulationFailedDetails = GatewayErrorDetailsV2OneOf3;

/** Details for {@link GatewayErrorCode.GasEstimateFailed} */
export type GasEstimateFailedDetails = GatewayErrorDetailsV2OneOf3;

/** Details for {@link GatewayErrorCode.NoRoute} */
export type NoRouteDetails = GatewayErrorDetailsV2OneOf4;

/** Details for {@link GatewayErrorCodeV2.AffiliateFeesNotSupportedForRoute} */
export type AffiliateFeesNotSupportedForRouteDetails = GatewayErrorDetailsV2OneOf4;

/** Details for {@link GatewayErrorCode.ExceededLimit} */
export type ExceededLimitDetails = GatewayErrorDetailsV2OneOf5;

/** Details for {@link GatewayErrorCodeV2.InsufficientSolverBalance} */
export type InsufficientSolverBalanceDetails = GatewayErrorDetailsV2OneOf7;

/** Details for {@link GatewayErrorCode.QuoteAmountTooLow} */
export type QuoteAmountTooLowDetails = GatewayErrorDetailsV2OneOf6;

/** Details for {@link GatewayErrorCode.SlippageTooLow} */
export type SlippageTooLowDetails = GatewayErrorDetailsV3OneOf;

/** Details for {@link GatewayErrorCodeV3.NonCompliantAddresses} */
export type NonCompliantAddressesDetails = GatewayErrorDetailsV3OneOf1;

/** Details for {@link GatewayErrorCodeV3.TooManyAffiliates} */
export type TooManyAffiliatesDetails = GatewayErrorDetailsV3OneOf2;

// ─── Code → details type mapping ─────────────────────────────────────────────

/**
 * Maps each {@link GatewayErrorCode} that carries structured details to its
 * corresponding detail interface. Codes absent from this map carry `null` details.
 */
export type GatewayErrorDetailsMap = {
    [GatewayErrorCode.InsufficientAmount]: InsufficientAmountDetails;
    [GatewayErrorCodeV2.InsufficientSolverBalance]: InsufficientSolverBalanceDetails;
    [GatewayErrorCode.InsufficientPaymentAmount]: InsufficientPaymentAmountDetails;
    [GatewayErrorCode.InsufficientSwapAmount]: InsufficientSwapAmountDetails;
    [GatewayErrorCode.UnableToCoverFees]: UnableToCoverFeesDetails;
    [GatewayErrorCode.SimulationFailed]: SimulationFailedDetails;
    [GatewayErrorCode.GasEstimateFailed]: GasEstimateFailedDetails;
    [GatewayErrorCode.NoRoute]: NoRouteDetails;
    [GatewayErrorCodeV2.AffiliateFeesNotSupportedForRoute]: AffiliateFeesNotSupportedForRouteDetails;
    [GatewayErrorCode.ExceededLimit]: ExceededLimitDetails;
    [GatewayErrorCode.QuoteAmountTooLow]: QuoteAmountTooLowDetails;
    [GatewayErrorCode.SlippageTooLow]: SlippageTooLowDetails;
    [GatewayErrorCodeV3.NonCompliantAddresses]: NonCompliantAddressesDetails;
    [GatewayErrorCodeV3.TooManyAffiliates]: TooManyAffiliatesDetails;
};

/**
 * Resolves to the detail interface for a known code, or `null` for codes
 * that carry no structured details (e.g. `InternalError`, `InvalidRequest`).
 */
export type DetailsFor<C extends GatewayErrorCode | GatewayErrorCodeV2 | GatewayErrorCodeV3 | GatewayErrorCodeV4> =
    C extends keyof GatewayErrorDetailsMap ? GatewayErrorDetailsMap[C] : null;

type AnyGatewayErrorCode = GatewayErrorCode | GatewayErrorCodeV2 | GatewayErrorCodeV3 | GatewayErrorCodeV4;

// `raw` is the JSON body exactly as the Gateway sent it, which serialises detail fields in
// snake_case. Typing it as `DetailsFor<C>` claimed the camelCase shape the SDK returns and let
// every multi-word key be read under a name the wire never uses.
type ParseDetailsArgs = {
    [C in AnyGatewayErrorCode]: { code: C; raw: Record<string, unknown> | null };
}[AnyGatewayErrorCode];

// ─── Base class ──────────────────────────────────────────────────────────────
// ApiError lives beside GatewayError rather than in its own module: `fromResponse` builds a
// GatewayError, which extends ApiError, so two modules would have to import each other.

/**
 * Error thrown by {@link GatewayApiClient} for a 4xx/5xx HTTP response.
 *
 * A body in the Gateway's error shape (`{ code, error, details? }`) is thrown as the subclass
 * {@link GatewayError}. Any other body — an HTML page, an empty body, JSON without a `code` — did
 * not come from the Gateway but from something in front of it (a CDN, a proxy, a load balancer),
 * and is thrown as a plain `ApiError`.
 *
 * Check {@link isGatewayError} first when the Gateway's `code` matters; check {@link isApiError}
 * to handle every HTTP failure, whatever answered it:
 *
 * @example
 * ```typescript
 * try {
 *   await gatewaySDK.getRoutes();
 * } catch (err) {
 *   if (isApiError(err) && err.httpCode === 403) {
 *     // refused, by the Gateway (`GEOBLOCKED`) or by the proxy in front of it
 *   }
 * }
 * ```
 */
export class ApiError extends Error {
    /**
     * HTTP status code of the response the error was built from. Always set on an error the
     * client throws; `undefined` only for a {@link GatewayError} built without a response.
     */
    readonly httpCode?: number;

    constructor(message: string, httpCode?: number, options?: ErrorOptions) {
        super(message, options);
        this.name = 'ApiError';
        this.httpCode = httpCode;
    }

    /**
     * Read a 4xx/5xx response into an error carrying its status.
     *
     * A body in the Gateway's error shape gives a {@link GatewayError}, its `details` parsed by
     * `code`. Any other body gives a plain `ApiError`: for JSON, its message is the body's `error`
     * or `message` string, else the body itself; for a body that is not JSON, it names the status
     * and carries the parse failure as `cause`.
     *
     * Reads the body, so pass a response whose body is unread (or a clone). Also reachable as
     * `GatewayError.fromResponse`.
     */
    static async fromResponse(response: Response): Promise<ApiError> {
        const httpCode = response.status;

        let body: unknown;
        try {
            body = await response.json();
        } catch (parseError) {
            return new ApiError(`Non-JSON response: HTTP ${httpCode} ${response.statusText}`, httpCode, {
                cause: parseError,
            });
        }

        // The Gateway codes every error it returns, so a body without a `code` came from something
        // in front of it (CDN, proxy, ingress).
        if (body && typeof body === 'object' && instanceOfGatewayErrorV4(body)) {
            return gatewayErrorFromBody(body, httpCode);
        }

        return new ApiError(describeErrorBody(body), httpCode);
    }
}

/** Type guard for {@link ApiError}, including its subclass {@link GatewayError}. */
export function isApiError(err: unknown): err is ApiError {
    return err instanceof ApiError;
}

/** Message for an error body not in the Gateway's shape. */
function describeErrorBody(body: unknown): string {
    if (!body || typeof body !== 'object') {
        return String(body ?? '');
    }

    const fields = body as Record<string, unknown>;
    if (typeof fields.error === 'string') return fields.error;
    if (typeof fields.message === 'string') return fields.message;
    return JSON.stringify(body);
}

// ─── Class ───────────────────────────────────────────────────────────────────

/**
 * Typed error thrown by {@link GatewayApiClient} for a 4xx/5xx HTTP response whose body is in the
 * Gateway's error shape (`{ code, error, details? }`). A response with any other body is thrown as
 * the base class {@link ApiError}; both carry the response's `httpCode`.
 *
 * The class is generic over the error code `C` so that `details` is
 * automatically narrowed to the correct shape when the caller discriminates on
 * `code`. Use {@link isGatewayError} (instead of `instanceof`) to narrow the
 * caught error to {@link AnyGatewayError} — a discriminated union over all
 * error codes — so that switching on `.code` resolves `.details` automatically:
 *
 * @example
 * ```typescript
 * try {
 *   await gatewaySDK.getQuote(params);
 * } catch (err) {
 *   if (isGatewayError(err)) {
 *     switch (err.code) {
 *       case GatewayErrorCode.NoRoute:
 *         // err.details is NoRouteDetails — no cast needed
 *         console.log(err.details.srcChain);
 *         break;
 *       case GatewayErrorCode.QuoteAmountTooLow:
 *         // err.details is QuoteAmountTooLowDetails
 *         console.log(err.details.minimum);
 *         break;
 *     }
 *   }
 * }
 * ```
 */
export class GatewayError<
    C extends GatewayErrorCode | GatewayErrorCodeV2 | GatewayErrorCodeV3 | GatewayErrorCodeV4 =
        GatewayErrorCode | GatewayErrorCodeV2 | GatewayErrorCodeV3 | GatewayErrorCodeV4,
> extends ApiError {
    /** Stable error code, safe to switch/match on. */
    readonly code: C;

    /**
     * Human-readable error message. Mirrors the standard `Error.message` property
     * and satisfies the `GatewayError` interface `error` field.
     */
    readonly error: string;

    /**
     * Structured details whose shape depends on the error code.
     * Resolves to the concrete detail interface when `code` is narrowed via
     * `switch`/`if`; resolves to `null` for codes that carry no details.
     */
    readonly details: DetailsFor<C>;

    constructor(code: C, message: string, details: DetailsFor<C>, options?: ErrorOptions, httpCode?: number) {
        super(message, httpCode, options);
        this.name = 'GatewayError';
        this.code = code;
        this.error = message;
        this.details = details;
    }

    static fromText(
        message: string,
        options?: ErrorOptions,
        httpCode?: number
    ): GatewayError<(typeof GatewayErrorCode)['InternalError']> {
        return new GatewayError(GatewayErrorCode.InternalError, message, null, options, httpCode);
    }
}

/**
 * Build the `GatewayError` a coded response body describes. Details are extracted by `code`,
 * matching the Rust `GatewayErrorDetails` enum exactly.
 */
function gatewayErrorFromBody(json: GatewayErrorInterface, httpCode: number): AnyGatewayError {
    const body = json as unknown as Record<string, unknown>;
    const code = body.code as AnyGatewayErrorCode;
    const raw =
        body.details != null && typeof body.details === 'object' ? (body.details as Record<string, unknown>) : null;

    return new GatewayError(
        code,
        json.error,
        parseDetails({ code, raw } as ParseDetailsArgs),
        undefined,
        httpCode
    ) as AnyGatewayError;
}

/**
 * Discriminated union of all `GatewayError` variants.
 *
 * When you narrow on `.code` (via `switch` or `===`), TypeScript resolves
 * `.details` to the matching detail interface automatically.
 */
export type AnyGatewayError =
    | { [C in GatewayErrorCode]: GatewayError<C> }[GatewayErrorCode]
    | { [C2 in GatewayErrorCodeV2]: GatewayError<C2> }[GatewayErrorCodeV2]
    | { [C3 in GatewayErrorCodeV3]: GatewayError<C3> }[GatewayErrorCodeV3]
    | { [C4 in GatewayErrorCodeV4]: GatewayError<C4> }[GatewayErrorCodeV4];

/**
 * Type guard that narrows `err` to {@link AnyGatewayError}.
 *
 * Use this instead of `err instanceof GatewayError`. The `instanceof` check
 * alone widens to `GatewayError<GatewayErrorCode>` — a single non-union type —
 * which prevents TypeScript from narrowing `.details` when you switch on
 * `.code`. This guard returns the discriminated union so narrowing works:
 *
 * @example
 * ```typescript
 * if (isGatewayError(err)) {
 *   switch (err.code) {
 *     case GatewayErrorCode.NoRoute:
 *       err.details.srcChain; // NoRouteDetails — no cast needed
 *   }
 * }
 * ```
 */
export function isGatewayError(err: unknown): err is AnyGatewayError {
    return err instanceof GatewayError;
}

// ─── Code-aware detail parser ─────────────────────────────────────────────────
// Reads the response body, so fields are spelled as the Gateway sends them (snake_case). The
// generated `…FromJSON` helpers map them onto the generated-client property names (camelCase).
// Each case corresponds to a GatewayErrorDetails enum variant in error.rs.

function parseDetails({ code, raw }: ParseDetailsArgs): DetailsFor<AnyGatewayErrorCode> {
    // The generated helpers pass `null` straight through, but `DetailsFor` promises an object for
    // every code that carries details, so a response omitting them would hand callers a `null` the
    // type told them was safe to read. An empty body gives the same shape with absent fields.
    const body: Record<string, unknown> = raw ?? {};

    switch (code) {
        // Rust: GatewayErrorDetails::InsufficientAmount { expected, actual }
        case GatewayErrorCode.InsufficientAmount:
        case GatewayErrorCode.InsufficientPaymentAmount:
            return GatewayErrorDetailsV2OneOfFromJSON(body);

        // Rust: GatewayErrorDetails::InsufficientSwapAmount { required, available }
        case GatewayErrorCode.InsufficientSwapAmount:
            return GatewayErrorDetailsV2OneOf1FromJSON(body);

        // Rust: GatewayErrorDetails::UnableToCoverFees { total_fees, available_amount }
        case GatewayErrorCode.UnableToCoverFees:
            return GatewayErrorDetailsV2OneOf2FromJSON(body);

        // Rust: GatewayErrorDetails::SimulationFailed { tenderly_url }
        // GasEstimateFailed also uses this shape (TenderlyError::GasEstimateFailed)
        case GatewayErrorCode.SimulationFailed:
        case GatewayErrorCode.GasEstimateFailed:
            return GatewayErrorDetailsV2OneOf3FromJSON(body);

        // Rust: GatewayErrorDetails::NoRoute { src_chain, src_token, dst_chain, dst_token }
        case GatewayErrorCode.NoRoute:
        case GatewayErrorCodeV2.AffiliateFeesNotSupportedForRoute:
            return GatewayErrorDetailsV2OneOf4FromJSON(body);

        // Rust: GatewayErrorDetailsV2::InsufficientSolverBalance { limit, token, chain_id },
        case GatewayErrorCode.InsufficientSolverBalance:
            return GatewayErrorDetailsV2OneOf7FromJSON(body);

        // Rust: GatewayErrorDetails::ExceededLimit { limit }
        case GatewayErrorCode.ExceededLimit:
            return GatewayErrorDetailsV2OneOf5FromJSON(body);

        // Rust: GatewayErrorDetails::QuoteAmountTooLow { minimum, actual }
        case GatewayErrorCode.QuoteAmountTooLow:
            return GatewayErrorDetailsV2OneOf6FromJSON(body);

        case GatewayErrorCode.SlippageTooLow:
            return GatewayErrorDetailsV3OneOfFromJSON(body);

        // `addresses` is declared as an array, so callers iterate it without a guard. The generated
        // mapper copies the field through, which leaves `undefined` when the Gateway omits it.
        case GatewayErrorCodeV3.NonCompliantAddresses:
            return {
                ...GatewayErrorDetailsV3OneOf1FromJSON(body),
                addresses: Array.isArray(body.addresses) ? body.addresses.map(String) : [],
            };

        case GatewayErrorCodeV3.TooManyAffiliates:
            return GatewayErrorDetailsV3OneOf2FromJSON(body);

        // Codes with no details in Rust (details field absent or unit variant → {}):
        //   InsufficientConfirmedFunds, PerAccountLimitExceeded, GlobalLimitExceeded,
        //   InvalidRequest, InvalidOrderArgs, InvalidAffiliateFee, SlippageTooHigh, DisabledChain,
        //   InvalidDestinationChainId, OrderNotFound, OrderExpired, DuplicateOrder, InternalError,
        //   MissingRefundAddress
        default:
            return null;
    }
}
