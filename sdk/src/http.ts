import { ApiError } from './gateway/error/api-error';

/**
 * Fetch `url` and return the response, throwing an {@link ApiError} carrying the HTTP status as
 * `httpCode` for any non-2xx response.
 *
 * The error message is `<label> error: <body> (HTTP <status>)`, with the response body read as
 * text (Esplora and mempool.space answer errors in plain text, e.g. a node's RPC rejection
 * reason). An empty body is replaced by the status text.
 *
 * Shared by {@link EsploraClient} and {@link MempoolClient} so both report failures the same way
 * as the Gateway client.
 *
 * @param label Name of the service, used as the message prefix (e.g. `Esplora`).
 * @param url Request URL.
 * @param init Optional request init passed to `fetch` (e.g. `{ signal }` to abort the request).
 * @ignore
 */
export async function fetchOrThrow(label: string, url: string, init?: RequestInit): Promise<Response> {
    const response = await fetch(url, init);
    if (!response.ok) {
        throw await apiErrorFromResponse(label, response);
    }
    return response;
}

/**
 * Fetch `url` and parse the response body as JSON. See {@link fetchOrThrow} for errors.
 * @ignore
 */
export async function fetchJson<T>(label: string, url: string, init?: RequestInit): Promise<T> {
    const response = await fetchOrThrow(label, url, init);
    return (await response.json()) as T;
}

/**
 * Fetch `url` and return the response body as text. See {@link fetchOrThrow} for errors.
 * @ignore
 */
export async function fetchText(label: string, url: string, init?: RequestInit): Promise<string> {
    const response = await fetchOrThrow(label, url, init);
    return response.text();
}

async function apiErrorFromResponse(label: string, response: Response): Promise<ApiError> {
    const httpCode = response.status;
    let body = '';
    let cause: unknown;
    try {
        body = (await response.text()).trim();
    } catch (err) {
        cause = err;
    }
    const detail = body || response.statusText || 'request failed';
    return new ApiError(`${label} error: ${detail} (HTTP ${httpCode})`, httpCode, cause ? { cause } : undefined);
}
