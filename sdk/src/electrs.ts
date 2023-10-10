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
     * @param height The height of the Bitcoin block
     * @returns The block hash of the Bitcoin block
     */
    getBlockHash(height: number): Promise<string>;
    /**
     * @param height The hash of the Bitcoin block
     * @returns The raw block header, represented as a hex string
     */
    getBlockHeader(hash: string): Promise<string>;
    /**
     * @param txId The ID of a Bitcoin transaction
     * @returns The transaction data, represented as a hex string
     */
    getTransactionHex(txId: string): Promise<string>;
    /**
     * @param txId The ID of a Bitcoin transaction
     * @returns The encoded merkle inclusion proof for the transaction
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
        return await response.json();
    }

    async getText(url: string): Promise<string> {
        const response = await fetch(url);
        if (!response.ok) {
            throw new Error(response.statusText);
        }
        return await response.text();
    }
}
