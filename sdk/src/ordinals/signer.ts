import { Network, Psbt, Signer } from "bitcoinjs-lib";

/**
 * Dummy signer implementation used to estimate tx fees.
 */
export class DummySigner implements Signer {
    publicKey: Buffer;
    constructor(publicKey: Buffer) {
        this.publicKey = publicKey;
    }
    sign(_hash: Buffer, _lowR?: boolean | undefined): Buffer {
        // https://github.com/bitcoin/bitcoin/blob/607d5a46aa0f5053d8643a3e2c31a69bfdeb6e9f/src/script/sign.cpp#L611
        return Buffer.from("304502210100000000000000000000000000000000000000000000000000000000000000000220010000000000000000000000000000000000000000000000000000000000000001", "hex");
    }
    signSchnorr(hash: Buffer): Buffer {
        // https://github.com/bitcoin/bitcoin/blob/607d5a46aa0f5053d8643a3e2c31a69bfdeb6e9f/src/script/sign.cpp#L626
        return Buffer.alloc(64, 0);
    }
}

export interface RemoteSigner {
    /**
     * Get the configured Bitcoin network.
     *
     * @returns {Promise<string>} A promise that resolves to the current network.
     */
    getNetwork(): Promise<Network>;
    /**
     * Get the configured public key of the signer.
     *
     * @returns {Promise<string>} A promise that resolves to the hex encoded public key.
     */
    getPublicKey(): Promise<string>;
    /**
     * Send an amount of Satoshis to the recipient.
     *
     * @param {string} toAddress - The address of the recipient.
     * @param {number} amount - The Satoshis the recipient should receive.
     * @returns {Promise<string>} A promise that resolves to the transaction ID.
     */
    sendToAddress(toAddress: string, amount: number): Promise<string>;
    /**
     * Get the index of a UTXO in a transaction based on the recipient address.
     *
     * @param {string} toAddress - The address of the recipient.
     * @param {string} txId - The transaction ID to check.
     * @returns {Promise<number>} A promise that resolves to the UTXO index.
     */
    getUtxoIndex(toAddress: string, txId: string): Promise<number>;
    /**
     * Sign the PSBT at the specified input index.
     *
     * @param {number} inputIndex - The input index to sign for.
     * @param {Psbt} psbt - The PSBT containing that input.
     * @returns {Promise<Psbt>} A promise that resolves to the signed PSBT.
     */
    signPsbt(inputIndex: number, psbt: Psbt): Promise<Psbt>;
};
