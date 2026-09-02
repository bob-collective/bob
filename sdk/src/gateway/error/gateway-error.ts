import {
    GatewayErrorCode,
    GatewayErrorCodeV2Variants as GatewayErrorCodeV2,
    GatewayErrorCodeV3Variants as GatewayErrorCodeV3,
    GatewayErrorDetailsOneOf,
    GatewayErrorDetailsOneOf1,
    GatewayErrorDetailsOneOf2,
    GatewayErrorDetailsOneOf3,
    GatewayErrorDetailsOneOf4,
    GatewayErrorDetailsOneOf5,
    GatewayErrorDetailsOneOf6,
    GatewayErrorDetailsV2OneOf,
    GatewayErrorDetailsV3OneOf,
    GatewayErrorDetailsV3OneOf1,
    GatewayErrorDetailsV3OneOf2,
} from '../generated-client';
import type { GatewayError as GatewayErrorInterface } from '../generated-client/models/GatewayError';
import { instanceOfGatewayError } from '../generated-client/models/GatewayError';

export { GatewayErrorCode, GatewayErrorCodeV2, GatewayErrorCodeV3 };

// ─── Named detail interfaces (mirror the Rust GatewayErrorDetails enum) ──────

/** Details for {@link GatewayErrorCode.InsufficientAmount} */
export type InsufficientAmountDetails = GatewayErrorDetailsOneOf;

/** Details for {@link GatewayErrorCode.InsufficientPaymentAmount} */
export type InsufficientPaymentAmountDetails = GatewayErrorDetailsOneOf;

/** Details for {@link GatewayErrorCode.InsufficientSwapAmount} */
export type InsufficientSwapAmountDetails = GatewayErrorDetailsOneOf1;

/** Details for {@link GatewayErrorCode.UnableToCoverFees} */
export type UnableToCoverFeesDetails = GatewayErrorDetailsOneOf2;

/** Details for {@link GatewayErrorCode.SimulationFailed} */
export type SimulationFailedDetails = GatewayErrorDetailsOneOf3;

/** Details for {@link GatewayErrorCode.GasEstimateFailed} */
export type GasEstimateFailedDetails = GatewayErrorDetailsOneOf3;

/** Details for {@link GatewayErrorCode.NoRoute} */
export type NoRouteDetails = GatewayErrorDetailsOneOf4;

/** Details for {@link GatewayErrorCodeV2.AffiliateFeesNotSupportedForRoute} */
export type AffiliateFeesNotSupportedForRouteDetails = GatewayErrorDetailsOneOf4;

/** Details for {@link GatewayErrorCode.ExceededLimit} */
export type ExceededLimitDetails = GatewayErrorDetailsOneOf5;

/** Details for {@link GatewayErrorCodeV2.InsufficientSolverBalance} */
export type InsufficientSolverBalanceDetails = GatewayErrorDetailsV2OneOf;

/** Details for {@link GatewayErrorCode.QuoteAmountTooLow} */
export type QuoteAmountTooLowDetails = GatewayErrorDetailsOneOf6;

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
export type DetailsFor<C extends GatewayErrorCode | GatewayErrorCodeV2 | GatewayErrorCodeV3> =
    C extends keyof GatewayErrorDetailsMap ? GatewayErrorDetailsMap[C] : null;

type AnyGatewayErrorCode = GatewayErrorCode | GatewayErrorCodeV2 | GatewayErrorCodeV3;

type ParseDetailsArgs = {
    [C in AnyGatewayErrorCode]: { code: C; raw: DetailsFor<C> | null };
}[AnyGatewayErrorCode];

// ─── Class ───────────────────────────────────────────────────────────────────

/**
 * Typed error thrown by {@link GatewayApiClient} for every 4xx/5xx HTTP response.
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
    C extends GatewayErrorCode | GatewayErrorCodeV2 | GatewayErrorCodeV3 =
        | GatewayErrorCode
        | GatewayErrorCodeV2
        | GatewayErrorCodeV3,
> extends Error {
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

    constructor(code: C, message: string, details: DetailsFor<C>, options?: ErrorOptions) {
        super(message, options);
        this.name = 'GatewayError';
        this.code = code;
        this.error = message;
        this.details = details;
    }

    /**
     * Parse a 4xx/5xx API response body into a typed `GatewayError`.
     *
     * Details are extracted by `code`, matching the Rust `GatewayErrorDetails` enum exactly.
     * Falls back to `InternalError` for non-conforming bodies, preserving any `error` or
     * `message` string present in the payload.
     *
     * Returns {@link AnyGatewayError} — a discriminated union over all error codes —
     * so callers can switch on `.code` and have `.details` automatically narrowed.
     */
    static fromResponse(json: GatewayErrorInterface | object): AnyGatewayError {
        if (!json || typeof json !== 'object') {
            return GatewayError.fromText(String(json ?? ''));
        }

        const body = json as Record<string, unknown>;

        if (!instanceOfGatewayError(body)) {
            const message =
                typeof body.error === 'string'
                    ? body.error
                    : typeof body.message === 'string'
                      ? body.message
                      : JSON.stringify(json);
            return GatewayError.fromText(message);
        }

        const code = body.code as AnyGatewayErrorCode;
        const message = body.error as string;
        const raw =
            body.details != null && typeof body.details === 'object' ? (body.details as Record<string, unknown>) : null;

        return new GatewayError(code, message, parseDetails({ code, raw } as ParseDetailsArgs)) as AnyGatewayError;
    }

    static fromText(message: string, options?: ErrorOptions): GatewayError<(typeof GatewayErrorCode)['InternalError']> {
        return new GatewayError(GatewayErrorCode.InternalError, message, null, options);
    }
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
    | { [C3 in GatewayErrorCodeV3]: GatewayError<C3> }[GatewayErrorCodeV3];

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
// Reads detail fields using generated-client property names.
// Each case corresponds to a GatewayErrorDetails enum variant in error.rs.

function parseDetails({ code, raw }: ParseDetailsArgs): DetailsFor<AnyGatewayErrorCode> {
    switch (code) {
        // Rust: GatewayErrorDetails::InsufficientAmount { expected, actual }
        case GatewayErrorCode.InsufficientAmount:
        case GatewayErrorCode.InsufficientPaymentAmount:
            return {
                expected: String(raw?.expected ?? ''),
                actual: String(raw?.actual ?? ''),
            } satisfies InsufficientAmountDetails;

        // Rust: GatewayErrorDetails::InsufficientSwapAmount { required, available }
        case GatewayErrorCode.InsufficientSwapAmount:
            return {
                required: String(raw?.required ?? ''),
                available: String(raw?.available ?? ''),
            } satisfies InsufficientSwapAmountDetails;

        // Rust: GatewayErrorDetails::UnableToCoverFees { total_fees, available_amount }
        case GatewayErrorCode.UnableToCoverFees:
            return {
                totalFees: String(raw?.totalFees ?? ''),
                availableAmount: String(raw?.availableAmount ?? ''),
            } satisfies UnableToCoverFeesDetails;

        // Rust: GatewayErrorDetails::SimulationFailed { tenderly_url }
        // GasEstimateFailed also uses this shape (TenderlyError::GasEstimateFailed)
        case GatewayErrorCode.SimulationFailed:
        case GatewayErrorCode.GasEstimateFailed:
            return {
                tenderlyUrl: typeof raw?.tenderlyUrl === 'string' ? raw?.tenderlyUrl : null,
            } satisfies SimulationFailedDetails;

        // Rust: GatewayErrorDetails::NoRoute { src_chain, src_token, dst_chain, dst_token }
        case GatewayErrorCode.NoRoute:
        case GatewayErrorCodeV2.AffiliateFeesNotSupportedForRoute:
            return {
                srcChain: String(raw?.srcChain ?? ''),
                srcToken: String(raw?.srcToken ?? ''),
                dstChain: String(raw?.dstChain ?? ''),
                dstToken: String(raw?.dstToken ?? ''),
            } satisfies NoRouteDetails;

        // Rust: GatewayErrorDetailsV2::InsufficientSolverBalance { limit, token, chain_id },
        case GatewayErrorCode.InsufficientSolverBalance:
            return {
                limit: String(raw?.limit ?? ''),
                token: String(raw?.token ?? ''),
                chainId: String(raw?.chainId ?? ''),
            } satisfies InsufficientSolverBalanceDetails;

        // Rust: GatewayErrorDetails::ExceededLimit { limit }
        case GatewayErrorCode.ExceededLimit:
            return {
                limit: String(raw?.limit ?? ''),
            } satisfies ExceededLimitDetails;

        // Rust: GatewayErrorDetails::QuoteAmountTooLow { minimum, actual }
        case GatewayErrorCode.QuoteAmountTooLow:
            return {
                minimum: String(raw?.minimum ?? ''),
                actual: String(raw?.actual ?? ''),
            } satisfies QuoteAmountTooLowDetails;

        case GatewayErrorCode.SlippageTooLow:
            return {
                requestedBps: String(raw?.requestedBps),
                requiredBps: String(raw?.requiredBps),
            } satisfies SlippageTooLowDetails;

        case GatewayErrorCodeV3.NonCompliantAddresses:
            return {
                addresses: raw?.addresses || [],
            } satisfies NonCompliantAddressesDetails;

        case GatewayErrorCodeV3.TooManyAffiliates:
            return {
                max: String(raw?.max),
                actual: String(raw?.actual),
            } satisfies TooManyAffiliatesDetails;

        // Codes with no details in Rust (details field absent or unit variant → {}):
        //   InsufficientConfirmedFunds, PerAccountLimitExceeded, GlobalLimitExceeded,
        //   InvalidRequest, InvalidOrderArgs, InvalidAffiliateFee, SlippageTooHigh, DisabledChain,
        //   InvalidDestinationChainId, OrderNotFound, OrderExpired, DuplicateOrder, InternalError
        default:
            return null;
    }
}
