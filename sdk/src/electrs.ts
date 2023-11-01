/**
 * Base path for the mainnet Esplora API.
 * @default "https://btc-mainnet.gobob.xyz"
 */
export const MAINNET_ESPLORA_BASE_PATH = "https://btc-mainnet.gobob.xyz";
/**
 * Base path for the testnet Esplora API.
 * @default "https://btc-testnet.gobob.xyz"
 */
export const TESTNET_ESPLORA_BASE_PATH = "https://btc-testnet.gobob.xyz";
/**
 * Base path for the regtest Esplora API.
 * @default "http://localhost:3003"
 */
export const REGTEST_ESPLORA_BASE_PATH = "http://localhost:3003";

/**
 * @ignore
 */
export interface MerkleProof {
    blockHeight: number
    merkle: string,
    pos: number,
}

/**
 * @ignore
 */
export interface UTXO {
    txid: string
    vout: number,
    value: number,
}

export interface ElectrsClient {
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
     * const electrsClient = new DefaultElectrsClient(BITCOIN_NETWORK);
     * const blockHeight = 123456;
     * electrsClient.getBlockHash(blockHeight)
     *   .then((blockHash) => {
     *     console.log(`Block hash at height ${blockHeight}: ${blockHash}`);
     *   })
     *   .catch((error) => {
     *     console.error(`Error: ${error}`);
     *   });
     * ```
     */
    getBlockHash(height: number): Promise<string>;

    /**
     * Get the raw block header, represented as a hex string, for a Bitcoin block with a given hash.
     *
     * @param {string} hash - The hash of the Bitcoin block.
     * @returns {Promise<string>} A promise that resolves to the raw block header as a hex string.
     *
     * @example
     * ```typescript
     * const BITCOIN_NETWORK = "regtest";
     * const electrsClient = new DefaultElectrsClient(BITCOIN_NETWORK);
     * const blockHash = 'your_block_hash_here';
     * electrsClient.getBlockHeader(blockHash)
     *   .then((blockHeader) => {
     *     console.log(`Raw block header for block with hash ${blockHash}: ${blockHeader}`);
     *   })
     *   .catch((error) => {
     *     console.error(`Error: ${error}`);
     *   });
     * ```
     */
    getBlockHeader(hash: string): Promise<string>;

    /**
     * Get the transaction data, represented as a hex string, for a Bitcoin transaction with a given ID (txId).
     *
     * @param {string} txId - The ID of a Bitcoin transaction.
     * @returns {Promise<string>} A promise that resolves to the transaction data as a hex string.
     *
     * @example
     * ```typescript
     * const BITCOIN_NETWORK = "regtest";
     * const electrsClient = new DefaultElectrsClient(BITCOIN_NETWORK);
     * const transactionId = 'your_transaction_id_here';
     * electrsClient.getTransactionHex(transactionId)
     *   .then((transactionHex) => {
     *     console.log(`Transaction hex for transaction with ID ${transactionId}: ${transactionHex}`);
     *   })
     *   .catch((error) => {
     *     console.error(`Error: ${error}`);
     *   });
     * ```
     */
    getTransactionHex(txId: string): Promise<string>;

    /**
     * Get the encoded merkle inclusion proof for a Bitcoin transaction with a given ID (txId).
     *
     * @param {string} txId - The ID of a Bitcoin transaction.
     * @returns {Promise<MerkleProof>} A promise that resolves to the encoded merkle inclusion proof.
     *
     * @example
     * ```typescript
     * const BITCOIN_NETWORK = "regtest";
     * const electrsClient = new DefaultElectrsClient(BITCOIN_NETWORK);
     * const transactionId = 'your_transaction_id_here';
     * electrsClient.getMerkleProof(transactionId)
     *   .then((merkleProof) => {
     *     console.log(`Merkle inclusion proof for transaction with ID ${transactionId}: ${merkleProof}`);
     *   })
     *   .catch((error) => {
     *     console.error(`Error: ${error}`);
     *   });
     * ```
     */
    getMerkleProof(txId: string): Promise<MerkleProof>;

    /**
     * Get the fee estimate (in sat/vB) for the given confirmation target.
     *
     * @param {number} confirmationTarget - The number of blocks to be included in.
     * @returns {Promise<number>} A promise that resolves to the fee rate.
     */
    getFeeEstimate(confirmationTarget: number): Promise<number>;

    /**
     * Get the Unspent Transaction Outputs (UTXOs) for an address.
     *
     * @param {string} address - The Bitcoin address to checl.
     * @returns {Promise<Array<UTXO>>} A promise that resolves to an array of UTXOs.
     */
    getAddressUtxos(address: string): Promise<Array<UTXO>>;

    /**
     * Broadcast a raw transaction to the network.
     *
     * @param {string} txHex - The hex encoded transaction.
     * @returns {Promise<string>} A promise that resolves to the txid.
     */
    broadcastTx(txHex: string): Promise<string>;
}

/**
* @ignore
*/
function encodeElectrsMerkleProof(merkle: string[]): string {
    // convert to little-endian
    return merkle.map(item => Buffer.from(item, "hex").reverse().toString("hex")).join('');
}

/**
 * The `DefaultElectrsClient` class provides a client for interacting with an Esplora API
 * for Bitcoin network data retrieval.
 */
export class DefaultElectrsClient implements ElectrsClient {
    private basePath: string;

    /**
         * Create an instance of the `DefaultElectrsClient` with the specified network or URL.
         * If the `networkOrUrl` parameter is omitted, it defaults to "mainnet."
         *
         * @param networkOrUrl The Bitcoin network (e.g., "mainnet," "testnet," "regtest") 
         * 
         * @returns An instance of the `DefaultElectrsClient` configured for the specified network or URL.
         *
         * @example
         * const BITCOIN_NETWORK = "regtest";
         * const electrsClient = new DefaultElectrsClient(BITCOIN_NETWORK);
         *
         * @example
         * // Create a client for the mainnet using the default URL.
         * const electrsClientMainnet = new DefaultElectrsClient();
    */
    constructor(networkOrUrl: string = "mainnet") {
        switch (networkOrUrl) {
            case "mainnet":
                this.basePath = MAINNET_ESPLORA_BASE_PATH;
                break;
            case "testnet":
                this.basePath = TESTNET_ESPLORA_BASE_PATH;
                break;
            case "regtest":
                this.basePath = REGTEST_ESPLORA_BASE_PATH;
                break;
            default:
                this.basePath = networkOrUrl;
        }
    }

    /**
     * @ignore
     */
    async getBlockHash(height: number): Promise<string> {
        return this.getText(`${this.basePath}/block-height/${height}`);
    }

    /**
     * @ignore
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
     * @ignore
     */
    async getTransactionHex(txId: string): Promise<string> {
        return this.getText(`${this.basePath}/tx/${txId}/hex`);
    }

    /**
     * @ignore
     */
    async getMerkleProof(txId: string): Promise<MerkleProof> {
        const response = await this.getJson<{
            "block_height": number,
            "merkle": string[],
            "pos": number,
        }>(`${this.basePath}/tx/${txId}/merkle-proof`);
        return {
            blockHeight: response.block_height,
            merkle: encodeElectrsMerkleProof(response.merkle),
            pos: response.pos,
        };
    }

    /**
     * @ignore
     */
    async getFeeEstimate(confirmationTarget: number): Promise<number> {
        const response = await this.getJson<any>(`${this.basePath}/fee-estimates`);
        return response[confirmationTarget];
    }

    /**
     * @ignore
     */
    async getAddressUtxos(address: string, confirmed?: boolean): Promise<Array<UTXO>> {
        const response = await this.getJson<Array<{
            txid: string,
            vout: number,
            status: {
                confirmed: boolean,
                block_height: number,
                block_hash: string,
                block_time: number
            },
            value: number,
        }>>(`${this.basePath}/address/${address}/utxo`);
        return response
            .filter(utxo => (typeof confirmed !== "undefined") ? confirmed === utxo.status.confirmed : true)
            .map<UTXO>(utxo => {
                return {
                    txid: utxo.txid,
                    vout: utxo.vout,
                    value: utxo.value
                }
            });
    }

    /**
     * @ignore
     */
    async broadcastTx(txHex: string): Promise<string> {
        const res = await fetch(`${this.basePath}/tx`, {
            method: 'POST',
            body: txHex
        });
        return await res.text();
    }

    /**
     * @ignore
     */
    async getJson<T>(url: string): Promise<T> {
        const response = await fetch(url);
        if (!response.ok) {
            throw new Error(response.statusText);
        }
        return await response.json() as Promise<T>;
    }

    /**
     * @ignore
     */
    async getText(url: string): Promise<string> {
        const response = await fetch(url);
        if (!response.ok) {
            throw new Error(response.statusText);
        }
        return await response.text();
    }
}
