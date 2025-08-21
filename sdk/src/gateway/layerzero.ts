export class LayerZeroClient {
    private basePath: string;

    constructor() {
        this.basePath = 'https://metadata.layerzero-api.com/v1/metadata';
    }

    async getSupportedChains(): Promise<Array<string>> {
        const params = new URLSearchParams({
            symbols: 'WBTC',
        });

        const data = await this.getJson<{
            WBTC: [{ deployments: { [chainKey: string]: { address: string } } }];
        }>(`${this.basePath}/experiment/ofts/list?${params.toString()}`);

        return Object.keys(data.WBTC[0].deployments);
    }

    async getEidForChain(chainKey: string) {
        const data = await this.getJson<{
            [chainKey: string]: {
                deployments: [
                    {
                        version: number;
                        eid: string;
                    },
                ];
            };
        }>(`${this.basePath}`);

        return data[chainKey]?.deployments.find((item) => item.version === 2)?.eid || null;
    }

    private async getJson<T>(url: string): Promise<T> {
        const response = await fetch(url);
        if (!response.ok) {
            throw new Error(response.statusText);
        }
        return (await response.json()) as Promise<T>;
    }
}
