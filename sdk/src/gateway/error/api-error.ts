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
}

/** Type guard for {@link ApiError}, including its subclass {@link GatewayError}. */
export function isApiError(err: unknown): err is ApiError {
    return err instanceof ApiError;
}

/**
 * Message for an error body that is not in the Gateway's shape: its `error` or `message` string
 * when it has one, otherwise the body itself.
 */
export function describeErrorBody(body: unknown): string {
    if (!body || typeof body !== 'object') {
        return String(body ?? '');
    }

    const fields = body as Record<string, unknown>;
    if (typeof fields.error === 'string') return fields.error;
    if (typeof fields.message === 'string') return fields.message;
    return JSON.stringify(body);
}
