//@ts-nocheck
/**
 * @ignore
 */
import { Block } from "bitcoinjs-lib";
//@ts-nocheck
/**
 * @ignore
 */
import { BufferWriter, varuint } from "bitcoinjs-lib/src/bufferutils";
//@ts-nocheck
/**
 * @ignore
 */
import { hash256 } from "bitcoinjs-lib/src/crypto";
//@ts-nocheck
/**
 * @ignore
 */
import { Output, Transaction } from "bitcoinjs-lib/src/transaction";
//@ts-nocheck
/**
 * @ignore
 */
import * as bitcoin from "bitcoinjs-lib";

/**
 * @ignore
 */
function varSliceSize(someScript: Buffer): number {
    const length = someScript.length;
    return varuint.encodingLength(length) + length;
}

/**
 * @ignore
 */
export function encodeRawInput(tx: Transaction) {
    const inputSize = varuint.encodingLength(tx.ins.length) + tx.ins.reduce((sum, input) => {
        return sum + 40 + varSliceSize(input.script);
    }, 0);

    const inputBuffer = Buffer.allocUnsafe(inputSize);
    const inputBufferWriter = new BufferWriter(inputBuffer, 0);

    inputBufferWriter.writeVarInt(tx.ins.length);
    tx.ins.forEach(txIn => {
        inputBufferWriter.writeSlice(txIn.hash);
        inputBufferWriter.writeUInt32(txIn.index);
        inputBufferWriter.writeVarSlice(txIn.script);
        inputBufferWriter.writeUInt32(txIn.sequence);
    });

    return inputBuffer;
}

/**
 * @ignore
 */
function isOutput(out: Output): boolean {
    return out.value !== undefined;
}

/**
 * @ignore
 */
export function encodeRawOutput(tx: Transaction) {
    const outputSize = varuint.encodingLength(tx.outs.length) + tx.outs.reduce((sum, output) => {
        return sum + 8 + varSliceSize(output.script);
    }, 0);

    const outputBuffer = Buffer.allocUnsafe(outputSize);
    const outputBufferWriter = new BufferWriter(outputBuffer, 0);

    outputBufferWriter.writeVarInt(tx.outs.length);
    tx.outs.forEach(txOut => {
        if (isOutput(txOut)) {
            outputBufferWriter.writeUInt64(txOut.value);
        } else {
            outputBufferWriter.writeSlice((txOut as any).valueBuffer);
        }

        outputBufferWriter.writeVarSlice(txOut.script);
    });

    return outputBuffer;
}

/**
 * @ignore
 */
function vectorSize(someVector: Buffer[]): number {
    const length = someVector.length;

    return (
        varuint.encodingLength(length) +
        someVector.reduce((sum, witness) => {
            return sum + varSliceSize(witness);
        }, 0)
    );
}

/**
 * @ignore
 */
export function encodeRawWitness(tx: Transaction) {
    const witnessSize = tx.ins.reduce((sum, input) => {
        return sum + vectorSize(input.witness);
    }, 0);

    const witnessBuffer = Buffer.allocUnsafe(witnessSize);
    const witnessBufferWriter = new BufferWriter(witnessBuffer, 0);

    tx.ins.forEach(input => {
        witnessBufferWriter.writeVector(input.witness);
    });

    return witnessBuffer;
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
function createMerkleBranchAndRoot(hashes: Buffer[], index: number): {
    merkle: Buffer[],
    root: Buffer,
} {
    let merkle: Buffer[] = [];
    while (hashes.length > 1) {
        if (hashes.length % 2 != 0) {
            let last = hashes[hashes.length - 1];
            hashes.push(last);
        }
        if (index % 2 == 0) { index++ } else { index-- }
        merkle.push(hashes[index]);
        index = Math.floor(index / 2);
        hashes = chunkArray(hashes, 2).map(pair => hash256(Buffer.concat([pair[0], pair[1]])));
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
    const txIds = block.transactions!.map(tx => tx.getHash(forWitness));
    const pos = txIds.map(value => value.toString("hex")).indexOf(txHash);

    const merkleAndRoot = createMerkleBranchAndRoot(txIds, pos);
    return {
        pos: pos,
        // hashes are already little-endian
        proof: merkleAndRoot.merkle.map(value => value.toString("hex")).join(''),
        root: merkleAndRoot.root.toString("hex"),
    };
}

export function estimateTxFee(feeRate: number, numInputs: number = 1) {
    const tx = new bitcoin.Transaction();
    for (let i = 0; i < numInputs; i++) {
        tx.addInput(Buffer.alloc(32, 0), 0, 0xfffffffd, Buffer.alloc(0));
    }
    // https://github.com/interlay/interbtc-clients/blob/6bd3e81d695b93180c5aeae4f33910ad4395ff1a/bitcoin/src/light/wallet.rs#L80
    tx.ins.map(tx_input => (tx_input.witness = [Buffer.alloc(33 + 32 + 7, 0), Buffer.alloc(33, 0)]));
    tx.addOutput(Buffer.alloc(22, 0), 1000); // P2WPKH
    tx.addOutput(Buffer.alloc(22, 0), 1000); // P2WPKH (change)
    tx.addOutput(bitcoin.script.compile([bitcoin.opcodes.OP_RETURN, Buffer.alloc(20, 0)]), 0);
    const vsize = tx.virtualSize();
    const satoshis = feeRate * vsize;
    return satoshis;
}

export function estimateTxFeeConst(feeRate: number, numInputs: number = 1) {
    // hash (32 bytes)
    // + index (4 bytes)
    // + [scriptsig] length (1 byte)
    // + sequence (4 bytes)
    // + [witness] item count (1 byte)
    // + [witness] signature length (1 byte)
    // + [witness] signature (71 or 72 bytes)
    // + [witness] pubkey length (1 byte)
    // + [witness] pubkey (33 bytes)
    const inputSize = 32 + 4 + 1 + 4 + (1 + 1 + 72 + 1 + 33) / 4;
    // nValue (8 bytes)
    // + scriptPubKey length (...)
    // + scriptPubKey (P2WPKH = 22 bytes)
    const outputSize = 8 + 1 + 22;
    const opReturnSize = 8 + 1 + 1 + 20;
    // nVersion (4 bytes)
    // + input count (1 byte)
    // + input(s)
    // + output count (1 byte)
    // + output(s)
    // + nLockTime (4 bytes)
    // + segwit marker & segwit flag (1 byte)
    const vsize = 4 + 1 + inputSize * numInputs + 1 + outputSize + outputSize + opReturnSize + 4 + 1;
    const satoshis = feeRate * vsize;
    return satoshis;
}