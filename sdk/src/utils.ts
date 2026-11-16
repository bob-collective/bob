/**
 * @ignore
 */
import { CompactSize } from '@scure/btc-signer';
/**
 * @ignore
 */
import { sha256 } from 'viem';

/**
 * A transaction input, as far as the raw encoders read it. A bitcoinjs-lib `Transaction` input satisfies it.
 */
export interface RawTxInput {
    /** Previous transaction hash, in internal (little-endian) byte order. */
    hash: Uint8Array;
    index: number;
    script: Uint8Array;
    sequence: number;
    witness: Uint8Array[];
}

/**
 * A transaction output, as far as the raw encoders read it. A bitcoinjs-lib `Transaction` output satisfies it.
 */
export interface RawTxOutput {
    /** Amount in satoshis. */
    value: number | bigint;
    script: Uint8Array;
}

/**
 * A block, as far as {@link getMerkleProof} reads it. A bitcoinjs-lib `Block` satisfies it.
 */
export interface MerkleBlock {
    transactions?: ReadonlyArray<{ getHash(forWitness?: boolean): Buffer }>;
}

/**
 * @ignore
 */
function compactSize(n: number): Buffer {
    return Buffer.from(CompactSize.encode(BigInt(n)));
}

/**
 * @ignore
 */
function varSlice(bytes: Uint8Array): Buffer {
    return Buffer.concat([compactSize(bytes.length), bytes]);
}

/**
 * @ignore
 */
function uint32LE(n: number): Buffer {
    const buffer = Buffer.allocUnsafe(4);
    buffer.writeUInt32LE(n);
    return buffer;
}

/**
 * @ignore
 */
function uint64LE(n: number | bigint): Buffer {
    const buffer = Buffer.allocUnsafe(8);
    buffer.writeBigUInt64LE(BigInt(n));
    return buffer;
}

/**
 * @ignore
 */
export function encodeRawInput(tx: { ins: ReadonlyArray<Omit<RawTxInput, 'witness'>> }): Buffer {
    return Buffer.concat([
        compactSize(tx.ins.length),
        ...tx.ins.flatMap((txIn) => [txIn.hash, uint32LE(txIn.index), varSlice(txIn.script), uint32LE(txIn.sequence)]),
    ]);
}

/**
 * @ignore
 */
export function encodeRawOutput(tx: { outs: ReadonlyArray<RawTxOutput> }): Buffer {
    return Buffer.concat([
        compactSize(tx.outs.length),
        ...tx.outs.flatMap((txOut) => [uint64LE(txOut.value), varSlice(txOut.script)]),
    ]);
}

/**
 * @ignore
 */
export function encodeRawWitness(tx: { ins: ReadonlyArray<Pick<RawTxInput, 'witness'>> }): Buffer {
    return Buffer.concat(
        tx.ins.flatMap((input) => [compactSize(input.witness.length), ...input.witness.map((item) => varSlice(item))])
    );
}

/**
 * @ignore
 */
function hash256(data: Buffer): Buffer {
    return Buffer.from(sha256(sha256(data, 'bytes'), 'bytes'));
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
        hashes = chunkArray(hashes, 2).map((pair) => hash256(Buffer.concat([pair[0], pair[1]])));
    }
    return {
        merkle,
        root: hashes[0],
    };
}

/**
 * Retrieve a Merkle proof for a Bitcoin transaction from a block's raw data.
 *
 * @param block - The Bitcoin block containing the transaction, e.g. a bitcoinjs-lib `Block`.
 * @param txHash - The transaction hash to construct a proof for.
 * @param forWitness - Set to `true` to construct a witness proof (default is `false`).
 * @returns An object containing the position, proof, and root of the Merkle proof.
 */
export function getMerkleProof(block: MerkleBlock, txHash: string, forWitness?: boolean) {
    const txIds = block.transactions!.map((tx) => tx.getHash(forWitness));
    const pos = txIds.map((value) => value.toString('hex')).indexOf(txHash);

    const merkleAndRoot = createMerkleBranchAndRoot(txIds, pos);
    return {
        pos: pos,
        // hashes are already little-endian
        proof: merkleAndRoot.merkle.map((value) => value.toString('hex')).join(''),
        root: merkleAndRoot.root.toString('hex'),
    };
}
