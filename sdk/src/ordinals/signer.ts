import { Network, Psbt, Signer, Transaction } from 'bitcoinjs-lib';

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
        return Buffer.alloc(64, 0);
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
     * Get the transaction based on its ID.
     *
     * @param {string} txId - The transaction ID to fetch.
     * @returns {Promise<number>} A promise that resolves to the hex encoded transaction.
     */
    getTransaction(txId: string): Promise<Transaction>;
    /**
     * Sign the PSBT at the specified input index.
     *
     * @param {number} inputIndex - The input index to sign for.
     * @param {Psbt} psbt - The PSBT containing that input.
     * @returns {Promise<Psbt>} A promise that resolves to the signed PSBT.
     */
    signInput(inputIndex: number, psbt: Psbt): Promise<Psbt>;
}
