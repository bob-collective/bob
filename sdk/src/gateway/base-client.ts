/**
 * Base class providing common HTTP client functionality
 * with error handling for Gateway-related classes.
 */
export abstract class BaseClient {
    /**
     * Safe fetch wrapper with consistent error handling
     * @param url The URL to fetch from
     * @param init Optional fetch configuration
     * @param errorMessage Custom error message for network failures
     * @returns Promise<Response>
     * @throws Error with custom message for network failures, original error otherwise
     */
    protected async safeFetch(url: URL | string, init: RequestInit | undefined, errorMessage: string) {
        const defaultInit = {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
                Accept: 'application/json',
            },
        };

        try {
            // NOTE: await inside try/catch to handle runtime error
            const response = await fetch(url, init || defaultInit);
            return response;
        } catch (err) {
            this.handleFetchError(err, errorMessage);
        }
    }

    /**
     * Handle fetch errors with consistent error transformation
     * @param err The caught error
     * @param message Custom error message to use for network failures
     * @throws Error with custom message for TypeError('Failed to fetch'), original error otherwise
     */
    protected handleFetchError(err: unknown, message: string): never {
        if (err instanceof TypeError && err.message === 'Failed to fetch') {
            throw new Error(message);
        }
        throw err;
    }
}
