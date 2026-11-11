/**
 * @ignore
 */
import { Block, crypto, Transaction } from 'bitcoinjs-lib';

/**
 * Bitcoin CompactSize (varint) encoding.
 * @ignore
 */
function encodeVarInt(n: number): Buffer {
    if (n < 0xfd) {
        return Buffer.from([n]);
    }
    if (n <= 0xffff) {
        const buf = Buffer.allocUnsafe(3);
        buf.writeUInt8(0xfd, 0);
        buf.writeUInt16LE(n, 1);
        return buf;
    }
    if (n <= 0xffffffff) {
        const buf = Buffer.allocUnsafe(5);
        buf.writeUInt8(0xfe, 0);
        buf.writeUInt32LE(n, 1);
        return buf;
    }
    const buf = Buffer.allocUnsafe(9);
    buf.writeUInt8(0xff, 0);
    buf.writeBigUInt64LE(BigInt(n), 1);
    return buf;
}

/**
 * @ignore
 */
function encodeUInt32LE(n: number): Buffer {
    const buf = Buffer.allocUnsafe(4);
    buf.writeUInt32LE(n, 0);
    return buf;
}

/**
 * @ignore
 */
function encodeUInt64LE(n: bigint): Buffer {
    const buf = Buffer.allocUnsafe(8);
    buf.writeBigUInt64LE(n, 0);
    return buf;
}

/**
 * @ignore
 */
function encodeVarSlice(slice: Uint8Array): Buffer {
    return Buffer.concat([encodeVarInt(slice.length), slice]);
}

/**
 * @ignore
 */
export function encodeRawInput(tx: Transaction): Buffer {
    return Buffer.concat([
        encodeVarInt(tx.ins.length),
        ...tx.ins.flatMap((txIn) => [
            txIn.hash,
            encodeUInt32LE(txIn.index),
            encodeVarSlice(txIn.script),
            encodeUInt32LE(txIn.sequence),
        ]),
    ]);
}

/**
 * @ignore
 */
export function encodeRawOutput(tx: Transaction): Buffer {
    return Buffer.concat([
        encodeVarInt(tx.outs.length),
        ...tx.outs.flatMap((txOut) => [encodeUInt64LE(txOut.value), encodeVarSlice(txOut.script)]),
    ]);
}

/**
 * @ignore
 */
export function encodeRawWitness(tx: Transaction): Buffer {
    return Buffer.concat(
        tx.ins.flatMap((txIn) => [
            encodeVarInt(txIn.witness.length),
            ...txIn.witness.map((witness) => encodeVarSlice(witness)),
        ])
    );
}

/**
 * @ignore
 */
function chunkArray<T>(array: T[], chunkSize: number): T[][] {
    const chunkedArray: T[][] = [];
    let index = 0;
    while (index < array.length) {
        chunkedArray.push(array.slice(index, index + chunkSize));
        index += chunkSize;
    }
    return chunkedArray;
}

/**
 * Create a Merkle branch and root based on a list of hashes and a specific index.
 *
 * @param hashes - An array of hashes for Merkle construction.
 * @param index - The index of the hash for which the branch and root are calculated.
 * @returns An object containing the Merkle branch and root.
 */
// https://github.com/Blockstream/electrs/blob/fd35014283c7d3a7a85c77b9fd647c9f09de12c9/src/util/electrum_merkle.rs#L86-L105
function createMerkleBranchAndRoot(
    hashes: Buffer[],
    index: number
): {
    merkle: Buffer[];
    root: Buffer;
} {
    const merkle: Buffer[] = [];
    while (hashes.length > 1) {
        if (hashes.length % 2 != 0) {
            const last = hashes[hashes.length - 1];
            hashes.push(last);
        }
        if (index % 2 == 0) {
            index++;
        } else {
            index--;
        }
        merkle.push(hashes[index]);
        index = Math.floor(index / 2);
        hashes = chunkArray(hashes, 2).map((pair) => Buffer.from(crypto.hash256(Buffer.concat([pair[0], pair[1]]))));
    }
    return {
        merkle,
        root: hashes[0],
    };
}

/**
 * Retrieve a Merkle proof for a Bitcoin transaction from a block's raw data.
 *
 * @param block - The Bitcoin block containing the transaction.
 * @param txHash - The transaction hash to construct a proof for.
 * @param forWitness - Set to `true` to construct a witness proof (default is `false`).
 * @returns An object containing the position, proof, and root of the Merkle proof.
 */
export function getMerkleProof(block: Block, txHash: string, forWitness?: boolean) {
    const txIds = block.transactions!.map((tx) => Buffer.from(tx.getHash(forWitness)));
    const pos = txIds.map((value) => value.toString('hex')).indexOf(txHash);

    const merkleAndRoot = createMerkleBranchAndRoot(txIds, pos);
    return {
        pos: pos,
        // hashes are already little-endian
        proof: merkleAndRoot.merkle.map((value) => value.toString('hex')).join(''),
        root: merkleAndRoot.root.toString('hex'),
    };
}
