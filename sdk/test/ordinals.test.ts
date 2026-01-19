import * as ecc from 'tiny-secp256k1';
import * as ECPairFactory from 'ecpair';
import { RemoteSigner, inscribeData } from '../src/ordinals';
import { Network, Psbt, Transaction, address, initEccLib } from 'bitcoinjs-lib';
import { bitcoin } from 'bitcoinjs-lib/src/networks';
import { chunkContent, MAX_CHUNK_SIZE } from '../src/inscription';
import { assert, describe, it } from 'vitest';
import { Inscription } from '../src/inscription';

const ECPair = ECPairFactory.default(ecc);
initEccLib(ecc);

class StaticSigner implements RemoteSigner {
    keyPair: ECPairFactory.ECPairInterface;
    txs: Map<string, Transaction>;

    constructor(secret: string) {
        const privateKey = Buffer.from(secret, 'hex');
        this.keyPair = ECPair.fromPrivateKey(privateKey);
        this.txs = new Map();
    }

    async getNetwork(): Promise<Network> {
        return bitcoin;
    }

    async getPublicKey(): Promise<string> {
        return this.keyPair.publicKey.toString('hex');
    }

    async sendToAddress(toAddress: string, amount: number): Promise<string> {
        const tx = new Transaction();
        tx.addInput(Buffer.alloc(32, 0), 0);
        tx.addOutput(address.toOutputScript(toAddress), amount);
        const txId = tx.getId();
        this.txs.set(txId, tx);
        return txId;
    }

    async getTransaction(txId: string): Promise<Transaction> {
        return this.txs.get(txId)!;
    }

    async signInput(inputIndex: number, psbt: Psbt): Promise<Psbt> {
        psbt.signInput(inputIndex, this.keyPair);
        return psbt;
    }
}

describe.skip('Ordinal Tests', () => {
    it('should inscribe text', async () => {
        const secret = 'fc7458de3d5616e7803fdc81d688b9642641be32fee74c4558ce680cac3d4111';
        const signer = new StaticSigner(secret);
        const toAddress = 'bc1pxaneaf3w4d27hl2y93fuft2xk6m4u3wc4rafevc6slgd7f5tq2dqyfgy06';
        const tx = await inscribeData(signer, toAddress, 1, Inscription.createTextInscription('Hello World!'), 546);
        assert(tx.getId() == '9312bc8a9541dd3e4b22993740ff96449a52dbca00b8be22b2979bb25053f7d6');
    });

    it('should chunk large data', async () => {
        const data = Buffer.alloc(MAX_CHUNK_SIZE * 2 + 20, 0);
        const chunks = chunkContent(data);
        assert.equal(chunks.length, 3);
        assert.equal(chunks[0].length, MAX_CHUNK_SIZE);
        assert.equal(chunks[1].length, MAX_CHUNK_SIZE);
        assert.equal(chunks[2].length, 20);
    });
});
