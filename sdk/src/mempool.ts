/**
 * Base path for the mainnet Memopool API.
 * @default "https://btc-mainnet.gobob.xyz"
 */
export const MAINNET_MEMPOOL_BASE_PATH = 'https://mempool.space/api/v1';
/**
 * Base path for the testnet Memopool API.
 * @default "https://btc-testnet.gobob.xyz"
 */
export const TESTNET_MEMPOOL_BASE_PATH = 'https://mempool.space/testnet4/api/v1';
/**
 * Base path for the regtest Memopool API.
 * @default "http://localhost:3003"
 */
export const REGTEST_MEMPOOL_BASE_PATH = 'http://localhost:3003';

/**
 * @ignore
 */
export type MempoolRecommendedFee = {
    fastestFee: number;
    halfHourFee: number;
    hourFee: number;
    economyFee: number;
    minimumFee: number;
};

export class MempoolClient {
    private basePath: string;

    /**
     * Create an instance of the `MempoolPool` with the specified network or URL.
     * If the `networkOrUrl` parameter is omitted, it defaults to "mainnet."
     *
     * @param networkOrUrl The Bitcoin network (e.g., "mainnet," "testnet," "regtest")
     *
     * @returns An instance of the `MempoolPool` configured for the specified network or URL.
     *
     * @example
     * const BITCOIN_NETWORK = "regtest";
     * const mempoolClient = new MempoolPool(BITCOIN_NETWORK);
     *
     * @example
     * // Create a client for the mainnet using the default URL.
     * const mempoolClientMainnet = new MempoolPool();
     */
    constructor(networkOrUrl: string = 'mainnet') {
        switch (networkOrUrl) {
            case 'mainnet':
                this.basePath = MAINNET_MEMPOOL_BASE_PATH;
                break;
            case 'testnet':
                this.basePath = TESTNET_MEMPOOL_BASE_PATH;
                break;
            case 'regtest':
                this.basePath = REGTEST_MEMPOOL_BASE_PATH;
                break;
            default:
                this.basePath = networkOrUrl;
        }
    }

    /**
     * Get the recommended Bitcoin transaction fee rates from the Mempool API.
     *
     * This method returns the fee estimates in satoshis per virtual byte (sat/vB)
     * for different confirmation targets, including:
     *
     * - `fastestFee`: The fee rate for transactions that are likely to be included
     *   in the next block (fastest possible confirmation).
     * - `halfHourFee`: The fee rate for transactions that are likely to be confirmed
     *   within 30 minutes.
     * - `hourFee`: The fee rate for transactions that are likely to be confirmed
     *   within an hour.
     * - `economyFee`: The fee rate for transactions that are likely to be confirmed
     *   in a longer period (low priority).
     * - `minimumFee`: The lowest fee rate that is still accepted by miners.
     *
     * @returns {Promise<MempoolRecommendedFee>} A promise that resolves to an object containing
     * the recommended fees for various confirmation times.
     *
     * @example
     * const mempoolClient = new MempoolClient();
     * mempoolClient.getRecommendedFees()
     *   .then(fees => console.log(fees))
     *   .catch(error => console.error('Failed to fetch fees:', error));
     */
    async getRecommendedFees(): Promise<MempoolRecommendedFee> {
        return this.getJson<MempoolRecommendedFee>(`${this.basePath}/fees/recommended`);
    }

    /**
     * @ignore
     */
    private async getJson<T>(url: string): Promise<T> {
        const response = await fetch(url);
        if (!response.ok) {
            throw new Error(response.statusText);
        }
        return (await response.json()) as Promise<T>;
    }
}
