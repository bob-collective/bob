export const MAINNET_ESPLORA_BASE_PATH = "https://btc-mainnet.interlay.io";
export const TESTNET_ESPLORA_BASE_PATH = "https://btc-testnet.interlay.io";
export const REGTEST_ESPLORA_BASE_PATH = "http://localhost:3002";

export interface MerkleProof {
    blockHeight: number
    merkle: string,
    pos: number,
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
     * const electrsClient = new DefaultElectrsClient(BITCOIN_NETWORK);
     * const blockHeight = 123456;
     * electrs.getBlockHash(blockHeight)
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
     * const electrsClient = new DefaultElectrsClient(BITCOIN_NETWORK);
     * const blockHash = 'your_block_hash_here';
     * electrs.getBlockHeader(blockHash)
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
     * const electrsClient = new DefaultElectrsClient(BITCOIN_NETWORK);
     * const transactionId = 'your_transaction_id_here';
     * electrs.getTransactionHex(transactionId)
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
     * const electrsClient = new DefaultElectrsClient(BITCOIN_NETWORK);
     * const transactionId = 'your_transaction_id_here';
     * electrs.getMerkleProof(transactionId)
     *   .then((merkleProof) => {
     *     console.log(`Merkle inclusion proof for transaction with ID ${transactionId}: ${merkleProof}`);
     *   })
     *   .catch((error) => {
     *     console.error(`Error: ${error}`);
     *   });
     * ```
     */
    getMerkleProof(txId: string): Promise<MerkleProof>;
 }

function encodeElectrsMerkleProof(merkle: string[]): string {
    // convert to little-endian
    return merkle.map(item => Buffer.from(item, "hex").reverse().toString("hex")).join('');
}

export class DefaultElectrsClient implements ElectrsClient {
    private basePath: string;

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

    async getBlockHash(height: number): Promise<string> {
        return this.getText(`${this.basePath}/block-height/${height}`);
    }

    async getBlockHeader(hash: string): Promise<string> {
        return this.getText(`${this.basePath}/block/${hash}/header`);
    }

    async getTransactionHex(txId: string): Promise<string> {
        return this.getText(`${this.basePath}/tx/${txId}/hex`);
    }

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

    async getJson<T>(url: string): Promise<T> {
        const response = await fetch(url);
        if (!response.ok) {
            throw new Error(response.statusText);
        }
        return await response.json() as Promise<T>;
    }

    async getText(url: string): Promise<string> {
        const response = await fetch(url);
        if (!response.ok) {
            throw new Error(response.statusText);
        }
        return await response.text();
    }
}
