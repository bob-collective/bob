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
        const body = await readErrorBody(response);
        return 'parseError' in body
            ? new ApiError(body.message, response.status, { cause: body.parseError })
            : new ApiError(describeErrorBody(body.json), response.status);
    }
}

/** Type guard for {@link ApiError}, including its subclass {@link GatewayError}. */
export function isApiError(err: unknown): err is ApiError {
    return err instanceof ApiError;
}

/**
 * Parse an error response's body as JSON, or say why it is not JSON. Shared by the `fromResponse`
 * factories, which differ only in the class they build from it.
 */
export async function readErrorBody(
    response: Response
): Promise<{ json: unknown } | { message: string; parseError: unknown }> {
    try {
        return { json: await response.json() };
    } catch (parseError) {
        return { message: `Non-JSON response: HTTP ${response.status} ${response.statusText}`, parseError };
    }
}

/**
 * Message for an error body not in the Gateway's shape: its `error` or `message` string when it has
 * one, otherwise the body itself.
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
