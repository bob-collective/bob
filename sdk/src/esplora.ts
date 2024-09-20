/**
 * Base path for the mainnet Esplora API.
 * @default "https://btc-mainnet.gobob.xyz"
 */
export const MAINNET_ESPLORA_BASE_PATH = 'https://btc-mainnet.gobob.xyz';
/**
 * Base path for the testnet Esplora API.
 * @default "https://btc-testnet.gobob.xyz"
 */
export const TESTNET_ESPLORA_BASE_PATH = 'https://btc-testnet.gobob.xyz';
/**
 * Base path for the regtest Esplora API.
 * @default "http://localhost:3003"
 */
export const REGTEST_ESPLORA_BASE_PATH = 'http://localhost:3003';

/**
 * @ignore
 */
export interface MerkleProof {
    blockHeight: number;
    merkle: string;
    pos: number;
}

/**
 * @ignore
 */
export interface UTXO {
    txid: string;
    vout: number;
    value: number;
    confirmed: boolean;
    height?: number;
}

/**
 * @ignore
 */
export interface Transaction {
    txid: string;
    version: number;
    locktime: number;
    size: number;
    weight: number;
    fee: number;
    vin: Array<{
        txid: string;
        vout: number;
        is_coinbase: boolean;
        scriptsig: string;
        scriptsig_asm: string;
        inner_redeemscript_asm?: string;
        inner_witnessscript_asm?: string;
        sequence?: number;
        witness?: string[];
        prevout: {
            scriptpubkey: string;
            scriptpubkey_asm: string;
            scriptpubkey_type: string;
            scriptpubkey_address: string;
            value: number;
        } | null;
    }>;
    vout: Array<{
        scriptpubkey: string;
        scriptpubkey_asm?: string;
        scriptpubkey_type?: string;
        scriptpubkey_address?: string;
        value: number;
    }>;
    status: TransactionStatus;
}

/**
 * @ignore
 */
export interface TransactionStatus {
    confirmed: boolean;
    block_height?: number;
    block_hash?: string;
    block_time?: number;
}

/**
 * @ignore
 */
export interface Block {
    id: string;
    height: number;
    version: number;
    timestamp: number;
    bits: number;
    nonce: number;
    difficulty: number;
    merkle_root: string;
    tx_count: number;
    size: number;
    weight: number;
    previousblockhash: string | null;
    mediantime: number;
}

/**
 * @ignore
 */
function encodeEsploraMerkleProof(merkle: string[]): string {
    // convert to little-endian
    return merkle.map((item) => Buffer.from(item, 'hex').reverse().toString('hex')).join('');
}

/**
 * The `EsploraClient` interface provides a set of methods for interacting with an Esplora API
 * for Bitcoin network data retrieval.
 * See https://github.com/blockstream/esplora/blob/master/API.md for more information.
 */
export class EsploraClient {
    private basePath: string;

    /**
     * Create an instance of the `EsploraClient` with the specified network or URL.
     * If the `networkOrUrl` parameter is omitted, it defaults to "mainnet."
     *
     * @param networkOrUrl The Bitcoin network (e.g., "mainnet," "testnet," "regtest")
     *
     * @returns An instance of the `EsploraClient` configured for the specified network or URL.
     *
     * @example
     * const BITCOIN_NETWORK = "regtest";
     * const esploraClient = new EsploraClient(BITCOIN_NETWORK);
     *
     * @example
     * // Create a client for the mainnet using the default URL.
     * const esploraClientMainnet = new EsploraClient();
     */
    constructor(networkOrUrl: string = 'mainnet') {
        switch (networkOrUrl) {
            case 'mainnet':
                this.basePath = MAINNET_ESPLORA_BASE_PATH;
                break;
            case 'testnet':
                this.basePath = TESTNET_ESPLORA_BASE_PATH;
                break;
            case 'regtest':
                this.basePath = REGTEST_ESPLORA_BASE_PATH;
                break;
            default:
                this.basePath = networkOrUrl;
        }
    }

    /**
     * Get the latest block height of the Bitcoin chain.
     *
     * @returns {Promise<number>} A promise that resolves to the latest block number.
     */
    async getLatestHeight(): Promise<number> {
        return parseInt(await this.getText(`${this.basePath}/blocks/tip/height`), 10);
    }

    /**
     * Get the complete block data for a Bitcoin block with a given hash.
     *
     * @param {string} hash - The hash of the Bitcoin block.
     * @returns {Promise<Block>} A promise that resolves to the block data.
     *
     * @example
     * ```typescript
     * const BITCOIN_NETWORK = "regtest";
     * const esploraClient = new EsploraClient(BITCOIN_NETWORK);
     * const blockHash = 'your_block_hash_here';
     * esploraClient.getBlock(blockHash)
     *  .then((block) => {
     *  console.log(`Block data for block with hash ${blockHash}: ${JSON.stringify(block)}`);
     * })
     * .catch((error) => {
     * console.error(`Error: ${error}`);
     * });
     * ```
     */
    async getBlock(blockHash: string): Promise<Block> {
        return this.getJson(`${this.basePath}/block/${blockHash}`);
    }

    /**
     * Get the block hash of the Bitcoin block at a specific height.
     *
     * This function retrieves the block hash for the Bitcoin block at the given height.
     *
     * @param {number} height - The height of the Bitcoin block.
     * @returns {Promise<string>} A promise that resolves to the block hash of the Bitcoin block.
     *
     * @example
     * ```typescript
     * const BITCOIN_NETWORK = "regtest";
     * const esploraClient = new EsploraClient(BITCOIN_NETWORK);
     * const blockHeight = 123456;
     * esploraClient.getBlockHash(blockHeight)
     *   .then((blockHash) => {
     *     console.log(`Block hash at height ${blockHeight}: ${blockHash}`);
     *   })
     *   .catch((error) => {
     *     console.error(`Error: ${error}`);
     *   });
     * ```
     */
    async getBlockHash(height: number): Promise<string> {
        return this.getText(`${this.basePath}/block-height/${height}`);
    }

    /**
     * Get the raw block header, represented as a hex string, for a Bitcoin block with a given hash.
     *
     * @param {string} hash - The hash of the Bitcoin block.
     * @returns {Promise<string>} A promise that resolves to the raw block header as a hex string.
     *
     * @example
     * ```typescript
     * const BITCOIN_NETWORK = "regtest";
     * const esploraClient = new EsploraClient(BITCOIN_NETWORK);
     * const blockHash = 'your_block_hash_here';
     * esploraClient.getBlockHeader(blockHash)
     *   .then((blockHeader) => {
     *     console.log(`Raw block header for block with hash ${blockHash}: ${blockHeader}`);
     *   })
     *   .catch((error) => {
     *     console.error(`Error: ${error}`);
     *   });
     * ```
     */
    async getBlockHeader(hash: string): Promise<string> {
        return this.getText(`${this.basePath}/block/${hash}/header`);
    }

    /**
     * @ignore
     */
    async getBlockHeaderAt(height: number): Promise<string> {
        const blockHash = await this.getBlockHash(height);
        return await this.getBlockHeader(blockHash);
    }

    /**
     * Get the complete transaction data for a Bitcoin transaction with a given ID (txId).
     *
     * @param txId {string} - The ID of a Bitcoin transaction.
     * @returns {Promise<Transaction>} A promise that resolves to the transaction data.
     *
     * @example
     * ```typescript
     * const BITCOIN_NETWORK = "regtest";
     * const esploraClient = new EsploraClient(BITCOIN_NETWORK);
     * const transactionId = 'your_transaction_id_here';
     * esploraClient.getTransaction(transactionId)
     *  .then((transaction) => {
     *   console.log(`Transaction data for transaction with ID ${transactionId}: ${JSON.stringify(transaction)}`);
     * })
     * .catch((error) => {
     *  console.error(`Error: ${error}`);
     * });
     * ```
     */
    async getTransaction(txId: string): Promise<Transaction> {
        return this.getJson(`${this.basePath}/tx/${txId}`);
    }

    async getTransactionStatus(txId: string): Promise<TransactionStatus> {
        return this.getJson(`${this.basePath}/tx/${txId}/status`);
    }

    /**
     * Get the transaction data, represented as a hex string, for a Bitcoin transaction with a given ID (txId).
     *
     * @param {string} txId - The ID of a Bitcoin transaction.
     * @returns {Promise<string>} A promise that resolves to the transaction data as a hex string.
     *
     * @example
     * ```typescript
     * const BITCOIN_NETWORK = "regtest";
     * const esploraClient = new EsploraClient(BITCOIN_NETWORK);
     * const transactionId = 'your_transaction_id_here';
     * esploraClient.getTransactionHex(transactionId)
     *   .then((transactionHex) => {
     *     console.log(`Transaction hex for transaction with ID ${transactionId}: ${transactionHex}`);
     *   })
     *   .catch((error) => {
     *     console.error(`Error: ${error}`);
     *   });
     * ```
     */
    async getTransactionHex(txId: string): Promise<string> {
        return this.getText(`${this.basePath}/tx/${txId}/hex`);
    }

    /**
     * Get the encoded merkle inclusion proof for a Bitcoin transaction with a given ID (txId).
     *
     * @param {string} txId - The ID of a Bitcoin transaction.
     * @returns {Promise<MerkleProof>} A promise that resolves to the encoded merkle inclusion proof.
     *
     * @example
     * ```typescript
     * const BITCOIN_NETWORK = "regtest";
     * const esploraClient = new EsploraClient(BITCOIN_NETWORK);
     * const transactionId = 'your_transaction_id_here';
     * esploraClient.getMerkleProof(transactionId)
     *   .then((merkleProof) => {
     *     console.log(`Merkle inclusion proof for transaction with ID ${transactionId}: ${merkleProof}`);
     *   })
     *   .catch((error) => {
     *     console.error(`Error: ${error}`);
     *   });
     * ```
     */
    async getMerkleProof(txId: string): Promise<MerkleProof> {
        const response = await this.getJson<{
            block_height: number;
            merkle: string[];
            pos: number;
        }>(`${this.basePath}/tx/${txId}/merkle-proof`);
        return {
            blockHeight: response.block_height,
            merkle: encodeEsploraMerkleProof(response.merkle),
            pos: response.pos,
        };
    }

    /**
     * Get the fee estimate (in sat/vB) for the given confirmation target.
     *
     * @param {number} confirmationTarget - The number of blocks to be included in.
     * @returns {Promise<number>} A promise that resolves to the fee rate.
     */
    async getFeeEstimate(confirmationTarget: number): Promise<number> {
        const response = await this.getJson<Record<number, number>>(`${this.basePath}/fee-estimates`);
        return response[confirmationTarget];
    }

    /**
     * Get the Unspent Transaction Outputs (UTXOs) for an address.
     *
     * @dev Should return up to 500 UTXOs - depending on the configured limit.
     * @param {string} address - The Bitcoin address to check.
     * @returns {Promise<Array<UTXO>>} A promise that resolves to an array of UTXOs.
     */
    async getAddressUtxos(address: string, confirmed?: boolean): Promise<Array<UTXO>> {
        // https://github.com/Blockstream/electrs/blob/306f66acf2ab10bcd99b8012e95a0de30b2cc012/src/rest.rs#L860
        // https://github.com/Blockstream/electrs/blob/306f66acf2ab10bcd99b8012e95a0de30b2cc012/src/new_index/query.rs#L82
        // https://github.com/Blockstream/electrs/blob/306f66acf2ab10bcd99b8012e95a0de30b2cc012/src/config.rs#L177
        const response = await this.getJson<
            Array<{
                txid: string;
                vout: number;
                status: {
                    confirmed: boolean;
                    block_height: number;
                    block_hash: string;
                    block_time: number;
                };
                value: number;
            }>
        >(`${this.basePath}/address/${address}/utxo`);
        return response
            .filter((utxo) => (typeof confirmed !== 'undefined' ? confirmed === utxo.status.confirmed : true))
            .map<UTXO>((utxo) => {
                return {
                    txid: utxo.txid,
                    vout: utxo.vout,
                    value: utxo.value,
                    confirmed: utxo.status.confirmed,
                    height: utxo.status.block_height,
                };
            });
    }

    /**
     * Broadcast a raw transaction to the network.
     *
     * @param {string} txHex - The hex encoded transaction.
     * @returns {Promise<string>} A promise that resolves to the txid.
     */
    async broadcastTx(txHex: string): Promise<string> {
        const res = await fetch(`${this.basePath}/tx`, {
            method: 'POST',
            body: txHex,
        });
        return await res.text();
    }

    /**
     * Get the balance for an account / address.
     *
     * @param {string} address - The Bitcoin address to check.
     * @returns {Promise<number>} A promise that resolves to the balance.
     */
    async getBalance(address: string): Promise<number> {
        const response = await this.getJson<{
            address: string;
            chain_stats: {
                funded_txo_count: number;
                funded_txo_sum: number;
                spent_txo_count: number;
                spent_txo_sum: number;
                tx_count: number;
            };
            mempool_stats: {
                funded_txo_count: number;
                funded_txo_sum: number;
                spent_txo_count: number;
                spent_txo_sum: number;
                tx_count: number;
            };
        }>(`${this.basePath}/address/${address}`);

        const chainBalance = response.chain_stats.funded_txo_sum - response.chain_stats.spent_txo_sum;
        const mempoolBalance = response.mempool_stats.funded_txo_sum - response.mempool_stats.spent_txo_sum;
        return chainBalance + mempoolBalance;
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

    /**
     * @ignore
     */
    private async getText(url: string): Promise<string> {
        const response = await fetch(url);
        if (!response.ok) {
            throw new Error(response.statusText);
        }
        return await response.text();
    }
}
