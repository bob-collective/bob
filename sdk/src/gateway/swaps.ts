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
        this.basePath = 'https://api-v2.swaps.xyz/api/';
    }

    async getAction(params: ActionsParams): Promise<ActionsResponse> {
        const url = new URL('getAction', this.basePath);
        url.search = new URLSearchParams(params as unknown as Record<string, string>).toString();
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
        if (!process.env.SWAPS_API_KEY) throw new Error('process.env.SWAPS_API_KEY is missing');

        const response = await fetch(url, {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
                Accept: 'application/json',
                'x-api-key': process.env.SWAPS_API_KEY as string,
            },
        });
        if (!response.ok) {
            throw new Error(response.statusText);
        }
        return response.json() as Promise<T>;
    }
}
