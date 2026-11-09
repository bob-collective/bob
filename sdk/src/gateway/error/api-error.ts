/**
 * Error thrown by {@link GatewayApiClient} for a 4xx/5xx HTTP response.
 *
 * A body in the Gateway's error shape (`{ code, error, details? }`) is thrown as the subclass
 * {@link GatewayError}. Any other body — an HTML page, an empty body, JSON without a `code` — did
 * not come from the Gateway but from something in front of it (Cloudflare, a proxy, a load
 * balancer), and is thrown as a plain `ApiError`.
 *
 * A blocked location is refused by Cloudflare with a 403 HTML page before the request reaches the
 * Gateway, which never answers 403 itself. That refusal is therefore always a plain `ApiError`,
 * recognisable only by its `httpCode`.
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
 *     // geoblocked: Cloudflare refused the request
 *   }
 * }
 * ```
 */
export class ApiError extends Error {
    /**
     * HTTP status code of the response the error was built from. Always set on an error the
     * client throws; `undefined` only for an error built without a response.
     */
    readonly httpCode?: number;

    constructor(message: string, httpCode?: number, options?: ErrorOptions) {
        super(message, options);
        this.name = 'ApiError';
        this.httpCode = httpCode;
    }

    /**
     * Read a 4xx/5xx response into a plain `ApiError` carrying its status, whatever the body holds.
     *
     * For JSON, the message is the body's `error` or `message` string, else the body itself; for a
     * body that is not JSON, it names the status and carries the parse failure as `cause`. Reads the
     * body, so pass a response whose body is unread (or a clone).
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

        return new ApiError(describeErrorBody(body), httpCode);
    }
}

/** Type guard for {@link ApiError}, including its subclass {@link GatewayError}. */
export function isApiError(err: unknown): err is ApiError {
    return err instanceof ApiError;
}

/**
 * Message for an error body not in the Gateway's shape: its `error` or `message` string when it has
 * one, otherwise the body itself.
 */
function describeErrorBody(body: unknown): string {
    if (!body || typeof body !== 'object') {
        return String(body ?? '');
    }

    const fields = body as Record<string, unknown>;
    if (typeof fields.error === 'string') return fields.error;
    if (typeof fields.message === 'string') return fields.message;
    return JSON.stringify(body);
}
