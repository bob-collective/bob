import {
    GatewayErrorDetailsOneOf,
    GatewayErrorDetailsOneOf1,
    GatewayErrorDetailsOneOf2,
    GatewayErrorDetailsOneOf3,
    GatewayErrorDetailsOneOf4,
    GatewayErrorDetailsOneOf5,
    GatewayErrorDetailsOneOf6,
} from '../generated-client';
import type { GatewayError as GatewayErrorInterface } from '../generated-client/models/GatewayError';
import { instanceOfGatewayError } from '../generated-client/models/GatewayError';
import { GatewayErrorCode } from '../generated-client/models/GatewayErrorCode';

export { GatewayErrorCode };

// ─── Named detail interfaces (mirror the Rust GatewayErrorDetails enum) ──────

/** Details for {@link GatewayErrorCode.InsufficientAmount} and {@link GatewayErrorCode.InsufficientPaymentAmount} */
export interface InsufficientAmountDetails extends GatewayErrorDetailsOneOf {}

/** Details for {@link GatewayErrorCode.InsufficientSwapAmount} */
export interface InsufficientSwapAmountDetails extends GatewayErrorDetailsOneOf1 {}

/** Details for {@link GatewayErrorCode.InsufficientFunds} */
export interface InsufficientFundsDetails extends GatewayErrorDetailsOneOf2 {}

/** Details for {@link GatewayErrorCode.UnableToCoverFees} */
export interface UnableToCoverFeesDetails extends GatewayErrorDetailsOneOf3 {}

/** Details for {@link GatewayErrorCode.SimulationFailed} and {@link GatewayErrorCode.GasEstimateFailed} */
export interface SimulationFailedDetails extends GatewayErrorDetailsOneOf4 {}

/** Details for {@link GatewayErrorCode.NoRoute} */
export interface NoRouteDetails extends GatewayErrorDetailsOneOf5 {}

/** Details for {@link GatewayErrorCode.ExceededLimit} */
export interface ExceededLimitDetails extends GatewayErrorDetailsOneOf6 {}

// ─── Code → details type mapping ─────────────────────────────────────────────

/**
 * Maps each {@link GatewayErrorCode} that carries structured details to its
 * corresponding detail interface. Codes absent from this map carry `null` details.
 */
export type GatewayErrorDetailsMap = {
    [GatewayErrorCode.InsufficientAmount]: InsufficientAmountDetails;
    [GatewayErrorCode.InsufficientPaymentAmount]: InsufficientAmountDetails;
    [GatewayErrorCode.InsufficientSwapAmount]: InsufficientSwapAmountDetails;
    [GatewayErrorCode.InsufficientFunds]: InsufficientFundsDetails;
    [GatewayErrorCode.UnableToCoverFees]: UnableToCoverFeesDetails;
    [GatewayErrorCode.SimulationFailed]: SimulationFailedDetails;
    [GatewayErrorCode.GasEstimateFailed]: SimulationFailedDetails;
    [GatewayErrorCode.NoRoute]: NoRouteDetails;
    [GatewayErrorCode.ExceededLimit]: ExceededLimitDetails;
};

/**
 * Resolves to the detail interface for a known code, or `null` for codes
 * that carry no structured details (e.g. `InternalError`, `InvalidRequest`).
 */
export type DetailsFor<C extends GatewayErrorCode> = C extends keyof GatewayErrorDetailsMap
    ? GatewayErrorDetailsMap[C]
    : null;

// ─── Class ───────────────────────────────────────────────────────────────────

/**
 * Typed error thrown by {@link GatewayApiClient} for every 4xx/5xx HTTP response.
 *
 * The class is generic over the error code `C` so that `details` is
 * automatically narrowed to the correct shape when the caller discriminates on
 * `code`. Use the {@link AnyGatewayError} union type (returned by
 * {@link GatewayError.fromResponse}) to get full switch-narrowing:
 *
 * @example
 * ```typescript
 * try {
 *   await gatewaySDK.getQuote(params);
 * } catch (err) {
 *   if (err instanceof GatewayError) {
 *     switch (err.code) {
 *       case GatewayErrorCode.NoRoute:
 *         // err.details is NoRouteDetails — no cast needed
 *         console.log(err.details.srcChain);
 *         break;
 *       case GatewayErrorCode.InsufficientFunds:
 *         // err.details is InsufficientFundsDetails
 *         console.log(err.details.minRequiredSats);
 *         break;
 *     }
 *   }
 * }
 * ```
 */
export class GatewayError<C extends GatewayErrorCode = GatewayErrorCode>
    extends Error
    implements GatewayErrorInterface
{
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

    constructor(code: C, message: string, details: DetailsFor<C>) {
        super(message);
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
    static fromResponse(json: unknown): AnyGatewayError {
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

        const code = body.code as GatewayErrorCode;
        const message = body.error as string;
        const raw =
            body.details != null && typeof body.details === 'object' ? (body.details as Record<string, unknown>) : null;

        return new GatewayError(code, message, parseDetails(code, raw)) as AnyGatewayError;
    }

    static fromText(message: string): GatewayError<(typeof GatewayErrorCode)['InternalError']> {
        return new GatewayError(
            GatewayErrorCode.InternalError,
            message,
            null as DetailsFor<(typeof GatewayErrorCode)['InternalError']>
        );
    }
}

/**
 * Discriminated union of all `GatewayError` variants.
 *
 * When you narrow on `.code` (via `switch` or `===`), TypeScript resolves
 * `.details` to the matching detail interface automatically.
 */
export type AnyGatewayError = { [C in GatewayErrorCode]: GatewayError<C> }[GatewayErrorCode];

// ─── Code-aware detail parser ─────────────────────────────────────────────────
// Reads raw snake_case JSON fields directly, matching Rust serde output.
// Each case corresponds to a GatewayErrorDetails enum variant in error.rs.

function parseDetails<C extends GatewayErrorCode>(code: C, raw: Record<string, unknown> | null): DetailsFor<C> {
    if (!raw) return null as DetailsFor<C>;

    switch (code) {
        // Rust: GatewayErrorDetails::InsufficientAmount { expected, actual }
        case GatewayErrorCode.InsufficientAmount:
        case GatewayErrorCode.InsufficientPaymentAmount:
            return {
                expected: String(raw.expected ?? ''),
                actual: String(raw.actual ?? ''),
            } satisfies InsufficientAmountDetails as DetailsFor<C>;

        // Rust: GatewayErrorDetails::InsufficientSwapAmount { required, available }
        case GatewayErrorCode.InsufficientSwapAmount:
            return {
                required: String(raw.required ?? ''),
                available: String(raw.available ?? ''),
            } satisfies InsufficientSwapAmountDetails as DetailsFor<C>;

        // Rust: GatewayErrorDetails::InsufficientFunds { sender, min_required_sats }
        case GatewayErrorCode.InsufficientFunds:
            return {
                sender: String(raw.sender ?? ''),
                minRequiredSats: Number(raw.min_required_sats ?? 0),
            } satisfies InsufficientFundsDetails as DetailsFor<C>;

        // Rust: GatewayErrorDetails::UnableToCoverFees { total_fees, available_amount }
        case GatewayErrorCode.UnableToCoverFees:
            return {
                totalFees: String(raw.total_fees ?? ''),
                availableAmount: String(raw.available_amount ?? ''),
            } satisfies UnableToCoverFeesDetails as DetailsFor<C>;

        // Rust: GatewayErrorDetails::SimulationFailed { tenderly_url }
        // GasEstimateFailed also uses this shape (TenderlyError::GasEstimateFailed)
        case GatewayErrorCode.SimulationFailed:
        case GatewayErrorCode.GasEstimateFailed:
            return {
                tenderlyUrl: typeof raw.tenderly_url === 'string' ? raw.tenderly_url : null,
            } satisfies SimulationFailedDetails as DetailsFor<C>;

        // Rust: GatewayErrorDetails::NoRoute { src_chain, src_token, dst_chain, dst_token }
        case GatewayErrorCode.NoRoute:
            return {
                srcChain: String(raw.src_chain ?? ''),
                srcToken: String(raw.src_token ?? ''),
                dstChain: String(raw.dst_chain ?? ''),
                dstToken: String(raw.dst_token ?? ''),
            } satisfies NoRouteDetails as DetailsFor<C>;

        // Rust: GatewayErrorDetails::ExceededLimit { limit }
        case GatewayErrorCode.ExceededLimit:
            return {
                limit: String(raw.limit ?? ''),
            } satisfies ExceededLimitDetails as DetailsFor<C>;

        // Codes with no details in Rust (details field absent or unit variant → {}):
        //   InsufficientSolverBalance, InsufficientConfirmedFunds (unit variant),
        //   PerAccountLimitExceeded, GlobalLimitExceeded, InvalidRequest, InvalidOrderArgs,
        //   InvalidAffiliateFee, SlippageTooLow, SlippageTooHigh, DisabledChain,
        //   InvalidDestinationChainId, OrderNotFound, OrderExpired, DuplicateOrder, InternalError
        default:
            return null as DetailsFor<C>;
    }
}
