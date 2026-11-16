import { NETWORK, TEST_NETWORK } from '@scure/btc-signer';
import { describe, expect, it } from 'vitest';
import { toHexScriptPubKey } from '../src/gateway/utils/common';
import { EsploraClient } from '../src/esplora';
import { getBitcoinTxInfo } from '../src/relay';
import { encodeRawInput, encodeRawOutput, encodeRawWitness, getMerkleProof } from '../src/utils';
import fixtures from './fixtures/bitcoin-encoding.json';
import { merkleBlock, parseBlock, parseTx, txHash } from './helpers/bitcoin';

// Expected values were produced by the bitcoinjs-lib implementation these functions replaced, from mainnet
// transactions of each input and output kind (`fixtures.source`). Blocks are passed in the shape of the bitcoinjs-lib
// `Block` callers use, built by `./helpers/bitcoin`.

function esploraReturning(txHex: string): EsploraClient {
    return { getTransactionHex: async () => txHex } as unknown as EsploraClient;
}

describe('Bitcoin transaction encoding', () => {
    describe.each(Object.entries(fixtures.txs))('%s transaction', (_name, tx) => {
        it('encodes the same transaction info', async () => {
            expect(await getBitcoinTxInfo(esploraReturning(tx.hex), tx.txid)).toEqual(tx.info);
            expect(await getBitcoinTxInfo(esploraReturning(tx.hex), tx.txid, true)).toEqual(tx.infoWithWitness);
        });

        it('reassembles into the raw transaction', async () => {
            const info = await getBitcoinTxInfo(esploraReturning(tx.hex), tx.txid, true);
            const isSegwit = tx.hex.slice(8, 12) === '0001';

            expect(
                [
                    info.version,
                    isSegwit ? '0001' : '',
                    info.inputVector,
                    info.outputVector,
                    isSegwit ? info.witnessVector : '',
                    info.locktime,
                ].join('')
            ).toBe(tx.hex);
        });
    });

    it('encodes a transaction with 253 inputs (3-byte CompactSize)', () => {
        const { inputVector, outputVector, witnessVector } = fixtures.largeInputCount;
        // The transaction the fixture was generated from.
        const tx = {
            ins: Array.from({ length: 253 }, (_, i) => ({
                hash: Buffer.alloc(32, i),
                index: i,
                script: Buffer.from([i]),
                sequence: 0xfffffffe,
                witness: [],
            })),
            outs: [{ script: Buffer.from('0014' + '11'.repeat(20), 'hex'), value: 1234 }],
        };

        expect(inputVector.startsWith('fdfd00')).toBe(true);
        expect(encodeRawInput(tx).toString('hex')).toBe(inputVector);
        expect(encodeRawOutput(tx).toString('hex')).toBe(outputVector);
        expect(encodeRawWitness(tx).toString('hex')).toBe(witnessVector);
    });

    it('decodes a transaction with 253 inputs', async () => {
        const { hex, inputVector, outputVector, witnessVector } = fixtures.largeInputCount;

        expect(await getBitcoinTxInfo(esploraReturning(hex), '', true)).toMatchObject({
            inputVector,
            outputVector,
            witnessVector,
        });
    });
});

describe('getMerkleProof', () => {
    it('proves each transaction of block 100000 against its header', () => {
        const { header, txs } = parseBlock(fixtures.merkle.block100000.hex);
        const block = merkleBlock(txs);
        const proofs = txs.map((tx) => getMerkleProof(block, txHash(tx).toString('hex')));

        expect(proofs).toEqual(fixtures.merkle.block100000.proofs);
        for (const proof of proofs) {
            expect(proof.root).toBe(header.subarray(36, 68).toString('hex'));
        }
    });

    it('builds the same txid and wtxid proofs', () => {
        const fixtureTxs = fixtures.merkle.synthetic.order.map(
            (name) => fixtures.txs[name as keyof typeof fixtures.txs]
        );
        const txs = fixtureTxs.map((tx) => parseTx(tx.hex));
        const block = merkleBlock(txs);

        // The helper hashes as bitcoinjs-lib did when the fixtures were generated.
        expect(txs.map((tx) => Buffer.from(txHash(tx)).reverse().toString('hex'))).toEqual(
            fixtureTxs.map((tx) => tx.txid)
        );
        expect(txs.map((tx) => Buffer.from(txHash(tx, true)).reverse().toString('hex'))).toEqual(
            fixtureTxs.map((tx) => tx.wtxid)
        );

        expect(txs.map((tx) => getMerkleProof(block, txHash(tx).toString('hex')))).toEqual(
            fixtures.merkle.synthetic.txidProofs
        );
        expect(txs.map((tx) => getMerkleProof(block, txHash(tx, true).toString('hex'), true))).toEqual(
            fixtures.merkle.synthetic.wtxidProofs
        );
    });
});

describe('toHexScriptPubKey', () => {
    it.each(Object.entries(fixtures.scriptPubKeys))('encodes a %s address', (_type, { address, network, script }) => {
        expect(toHexScriptPubKey(address, network === 'bitcoin' ? NETWORK : TEST_NETWORK)).toBe(script);
    });

    // BIP86 first receive address. bitcoinjs-lib needed an ECC library initialised for taproot.
    it('encodes a taproot address', () => {
        expect(toHexScriptPubKey('bc1p5cyxnuxmeuwuvkwfem96lqzszd02n6xdcjrs20cac6yqjjwudpxqkedrcr', NETWORK)).toBe(
            '0x225120a60869f0dbcf1dc659c9cecbaf8050135ea9e8cdc487053f1dc6880949dc684c'
        );
    });

    it('rejects an address from another network', () => {
        expect(() => toHexScriptPubKey('tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx', NETWORK)).toThrow();
    });
});
