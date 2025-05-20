/**
 * Base path for the mainnet Memopool API.
 * @default "https://mempool.space/api"
 */
export const MAINNET_MEMPOOL_BASE_PATH = 'https://mempool.space/api';
/**
 * Base path for the testnet Memopool API.
 * @default "https://mempool.space/testnet4/api"
 */
export const TESTNET_MEMPOOL_BASE_PATH = 'https://mempool.space/testnet4/api';
/**
 * Base path for the signet Memopool API.
 * @default "https://mempool.space/signet/api"
 */
export const SIGNET_MEMPOOL_BASE_PATH = 'https://mempool.space/signet/api';
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

/**
 * @ignore
 */
export type Vout = {
    scriptpubkey: string;
    scriptpubkey_asm: string;
    scriptpubkey_type: string;
    scriptpubkey_address: string;
    value: number;
};

/**
 * @ignore
 */
export type Vin = {
    txid: string;
    vout: number;
    prevout: Vout;
    scriptsig: string;
    scriptsig_asm: string;
    is_coinbase: boolean;
    sequence: number;
};

/**
 * @ignore
 */
export type TxStatus = {
    confirmed: boolean;
    block_height?: number;
    block_hash?: string;
    block_time?: number;
};

/**
 * @ignore
 */
export type MempoolTxInfo = {
    txid: string;
    version: number;
    locktime: number;
    vin: Vin[];
    vout: Vout[];
    size: number;
    weight: number;
    fee: number;
    status: TxStatus;
};

/**
 * @ignore
 */
export type BlockDetails = {
    id: string;
    height: number;
    version: number;
    timestamp: number;
    tx_count: number;
    size: number;
    weight: number;
    merkle_root: string;
    previousblockhash: string;
    mediantime: number;
    nonce: number;
    bits: number;
    difficulty: number;
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
            case 'signet':
                this.basePath = SIGNET_MEMPOOL_BASE_PATH;
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
        return this.getJson<MempoolRecommendedFee>(`${this.basePath}/v1/fees/recommended`);
    }

    /**
     * Returns details about a transaction.
     *
     * @param {string} txid
     * @returns {Promise<MempoolTxInfo>}
     *
     * @example
     * const mempoolClient = new MempoolClient();
     * mempoolClient.getTxInfo()
     *   .then(txInfo => console.log(txInfo))
     *   .catch(error => console.error('Failed to fetch transaction info:', error));
     */
    async getTxInfo(txid: string): Promise<MempoolTxInfo> {
        return this.getJson<MempoolTxInfo>(`${this.basePath}/v1/tx/${txid}`);
    }

    /**
     * Returns details about a block.
     *
     * @param {string} hash
     * @returns {Promise<BlockDetails>}
     *
     * @example
     * const mempoolClient = new MempoolClient();
     * mempoolClient.getBlock()
     *   .then(block => console.log(block))
     *   .catch(error => console.error('Failed to fetch block:', error));
     */
    async getBlock(hash: string): Promise<BlockDetails> {
        return this.getJson<BlockDetails>(`${this.basePath}/block/${hash}`);
    }

    /**
     * Returns the hash of the last block.
     *
     * @returns {Promise<string>}
     *
     * @example
     * const mempoolClient = new MempoolClient();
     * mempoolClient.getBlocksTipHeight()
     *   .then(blocksHeigh => console.log(blocksHeigh))
     *   .catch(error => console.error('Failed to fetch blocks height:', error));
     */
    async getBlocksTipHash() {
        return this.getText(`${this.basePath}/blocks/tip/hash`);
    }

    /**
     * Estimate transaction time to be mined
     *
     * This method returns estimated timestamp for given txid to be mined in seconds based on {@link getRecommendedFees}.
     *
     * @param {string} txid
     * @returns {Promise<number>} A timestamp when tx is expected to be mined in seconds
     *
     * - `Infinity`: the fee rate is < economyFee
     *
     * @example
     * const mempoolClient = new MempoolClient();
     * mempoolClient.estimateTxTime()
     *   .then(timestamp => console.log(timestamp))
     *   .catch(error => console.error('Failed to fetch tx time:', error));
     */
    async estimateTxTime(txid: string): Promise<number> {
        const [lastBlockHash, recommendedFees, txInfo] = await Promise.all([
            this.getBlocksTipHash(),
            this.getRecommendedFees(),
            this.getTxInfo(txid),
        ]);

        const lastBlockDetails = await this.getBlock(lastBlockHash);

        if (txInfo.status.confirmed) return txInfo.status.block_time || ~~(Date.now() / 1000);

        const blockTime = 10 * 60;
        const feeRate = txInfo.fee / txInfo.size;

        if (feeRate >= recommendedFees.fastestFee) return lastBlockDetails.timestamp + blockTime;
        if (feeRate >= recommendedFees.halfHourFee) return lastBlockDetails.timestamp + 3 * blockTime;
        if (feeRate >= recommendedFees.hourFee) return lastBlockDetails.timestamp + 6 * blockTime;
        if (feeRate >= recommendedFees.economyFee) return lastBlockDetails.timestamp + 144 * blockTime;
        console.warn(
            `Fee rate (${feeRate.toFixed(2)} sat/vB) is too low. Consider using RBF (Replace-by-Fee) to increase the fee. Recommended minimum: ${recommendedFees.economyFee} sat/vB.`
        );
        return Infinity;
    }

    /**
     * @ignore
     */
    private async getJson<T>(url: string): Promise<T> {
        const response = await fetch(url);
        if (!response.ok) {
            throw new Error(response.statusText);
        }
        return (await response.json()) as T;
    }

    /**
     * @ignore
     */
    private async getText<T extends string>(url: string): Promise<T> {
        const response = await fetch(url);
        if (!response.ok) {
            throw new Error(response.statusText);
        }
        return (await response.text()) as T;
    }
}
