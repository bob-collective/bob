export const MAINNET_ESPLORA_BASE_PATH = "https://btc-mainnet.interlay.io";
export const TESTNET_ESPLORA_BASE_PATH = "https://btc-testnet.interlay.io";
export const REGTEST_ESPLORA_BASE_PATH = "http://localhost:3002";

export interface ElectrsClient {
    /**
     * @param txId The ID of a Bitcoin transaction
     * @returns The encoded merkle inclusion proof for the transaction
     */
    getMerkleProof(txId: string): Promise<string>;

}

async function get<T>(url: string): Promise<T> {
    const response = await fetch(url);
    if (!response.ok) {
        throw new Error(response.statusText);
    }
    return await response.json();
}

function encodeMerkleProof(merkle: string[]): string {
    let proof = Buffer.from("")
    merkle.forEach(function (item) {
        proof = Buffer.concat([proof, Buffer.from(item, "hex").reverse()])
    })
    return proof.toString("hex")
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

    async getMerkleProof(txId: string): Promise<string> {
        const response = await get<{
            "block_height": number,
            "merkle": string[],
            "pos": number,
        }>(`${this.basePath}/tx/${txId}/merkle-proof`);
        return encodeMerkleProof(response.merkle);
    }
}
