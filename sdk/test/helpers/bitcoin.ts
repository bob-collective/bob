import { createHash } from 'node:crypto';
import type { MerkleBlock } from '../../src/utils';

// Reads Bitcoin's block and transaction serialization without the SDK's own decoding, so tests can check one
// against the other.

class Reader {
    offset = 0;

    constructor(readonly bytes: Buffer) {}

    take(length: number): Buffer {
        if (this.offset + length > this.bytes.length) throw new Error('Unexpected end of data');
        const out = this.bytes.subarray(this.offset, this.offset + length);
        this.offset += length;
        return out;
    }

    compactSize(): number {
        const first = this.take(1)[0];
        if (first < 0xfd) return first;
        if (first === 0xfd) return this.take(2).readUInt16LE();
        if (first === 0xfe) return this.take(4).readUInt32LE();
        return Number(this.take(8).readBigUInt64LE());
    }

    varSlice(): Buffer {
        return this.take(this.compactSize());
    }
}

export interface ParsedTx {
    /** The transaction as serialized, with its witness if it has one. */
    raw: Buffer;
    /** The serialization without marker, flag and witness, which the txid hashes. */
    stripped: Buffer;
    isCoinbase: boolean;
}

function readTx(reader: Reader): ParsedTx {
    const start = reader.offset;
    const version = reader.take(4);
    const isSegwit = reader.bytes[reader.offset] === 0x00 && reader.bytes[reader.offset + 1] === 0x01;
    if (isSegwit) reader.take(2);

    const bodyStart = reader.offset;
    const inputCount = reader.compactSize();
    let isCoinbase = false;
    for (let i = 0; i < inputCount; i++) {
        const previousTxHash = reader.take(32);
        reader.take(4); // previous output index
        reader.varSlice(); // scriptSig
        reader.take(4); // sequence
        isCoinbase = inputCount === 1 && previousTxHash.every((byte) => byte === 0);
    }
    const outputCount = reader.compactSize();
    for (let i = 0; i < outputCount; i++) {
        reader.take(8); // value
        reader.varSlice(); // scriptPubKey
    }
    const body = reader.bytes.subarray(bodyStart, reader.offset);

    if (isSegwit) {
        for (let i = 0; i < inputCount; i++) {
            const items = reader.compactSize();
            for (let j = 0; j < items; j++) reader.varSlice();
        }
    }
    const locktime = reader.take(4);

    return {
        raw: reader.bytes.subarray(start, reader.offset),
        stripped: Buffer.concat([version, body, locktime]),
        isCoinbase,
    };
}

function readToEnd<T>(hex: string, read: (reader: Reader) => T): T {
    const reader = new Reader(Buffer.from(hex, 'hex'));
    const value = read(reader);
    if (reader.offset !== reader.bytes.length) throw new Error('Trailing data');
    return value;
}

export function parseTx(txHex: string): ParsedTx {
    return readToEnd(txHex, readTx);
}

export function parseBlock(blockHex: string): { header: Buffer; txs: ParsedTx[] } {
    return readToEnd(blockHex, (reader) => {
        const header = reader.take(80);
        const txCount = reader.compactSize();
        return { header, txs: Array.from({ length: txCount }, () => readTx(reader)) };
    });
}

function hash256(data: Buffer): Buffer {
    return createHash('sha256').update(createHash('sha256').update(data).digest()).digest();
}

/**
 * Hash in internal (little-endian) byte order, as bitcoinjs-lib's `Transaction.getHash` returns it: the txid, or with
 * `forWitness` the wtxid, which is zero for the coinbase (BIP141).
 */
export function txHash(tx: ParsedTx, forWitness = false): Buffer {
    if (forWitness && tx.isCoinbase) return Buffer.alloc(32);
    return hash256(forWitness ? tx.raw : tx.stripped);
}

/** A block for `getMerkleProof`, shaped like the bitcoinjs-lib `Block` callers pass. */
export function merkleBlock(txs: ParsedTx[]): MerkleBlock {
    return { transactions: txs.map((tx) => ({ getHash: (forWitness?: boolean) => txHash(tx, forWitness) })) };
}
