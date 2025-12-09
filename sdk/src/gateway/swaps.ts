import { Chain } from 'viem';
import {
    ActionsParams,
    ActionsResponse,
    PathsParams,
    PathsResponse,
    StatusParams,
    StatusResponse,
    TransactionParams,
    TransactionResponse,
} from './types';

export class SwapsClient {
    private basePath: string;

    constructor() {
        this.basePath = 'https://box-api-git-zev-swaps-v2chains-decent-webapp.vercel.app/api/';
    }

    async getAction(params: ActionsParams): Promise<ActionsResponse> {
        const url = new URL('getAction', this.basePath);

        // Convert params to URLSearchParams-compatible format
        const searchParams = new URLSearchParams();
        Object.entries(params).forEach(([key, value]) => {
            if (value === undefined || value === null) {
                return; // Skip undefined/null values
            }
            if (Array.isArray(value)) {
                // Handle arrays (e.g., bridgeIds)
                value.forEach((item) => searchParams.append(key, String(item)));
            } else {
                // Convert all values to strings
                searchParams.append(key, String(value));
            }
        });

        url.search = searchParams.toString();
        return this.getJson(url.toString());
    }

    async getStatus(params: StatusParams): Promise<StatusResponse> {
        const url = new URL('getStatus', this.basePath);
        url.search = new URLSearchParams(params as unknown as Record<string, string>).toString();
        return this.getJson(url.toString());
    }

    async getTransactions(params: TransactionParams): Promise<TransactionResponse> {
        const url = new URL('getTransactions', this.basePath);
        url.search = new URLSearchParams(params as unknown as Record<string, string>).toString();
        return this.getJson(url.toString());
    }

    async getChainList(): Promise<Chain> {
        const url = new URL('getChainList', this.basePath);
        return this.getJson(url.toString());
    }

    async getPaths(params: PathsParams): Promise<PathsResponse> {
        const url = new URL('getPaths', this.basePath);
        url.search = new URLSearchParams(params as unknown as Record<string, string>).toString();
        return this.getJson(url.toString());
    }

    private async getJson<T>(url: string): Promise<T> {
        if (!process.env.NEXT_PUBLIC_SWAPS_API_KEY && !process.env.SWAPS_API_KEY)
            throw new Error('process.env.NEXT_PUBLIC_SWAPS_API_KEY or process.env.SWAPS_API_KEY is missing');

        const response = await fetch(url, {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
                Accept: 'application/json',
                'x-api-key': process.env.NEXT_PUBLIC_SWAPS_API_KEY || (process.env.SWAPS_API_KEY as string),
            },
        });
        if (!response.ok) {
            let errorMessage = response.statusText;
            try {
                const errorBody = await response.text();
                if (errorBody) {
                    errorMessage = `${response.statusText}: ${errorBody}`;
                }
            } catch {
                // If we can't parse the error body, use statusText
            }
            throw new Error(`Swaps API error (${response.status}): ${errorMessage}. URL: ${url}`);
        }
        return response.json() as Promise<T>;
    }
}
