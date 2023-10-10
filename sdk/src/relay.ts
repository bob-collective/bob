import { Transaction } from "bitcoinjs-lib";
import { ElectrsClient } from "./electrs";
import { encodeRawInput, encodeRawOutput } from "./utils";

export interface BitcoinTxInfo {
    version: string,
    inputVector: string,
    outputVector: string,
    locktime: string,
}

export async function getBitcoinTxInfo(
    electrsClient: ElectrsClient,
    txId: string,
): Promise<BitcoinTxInfo> {
    const txHex = await electrsClient.getTransactionHex(txId);
    const tx = Transaction.fromHex(txHex);

    const versionBuffer = Buffer.allocUnsafe(4);
    versionBuffer.writeInt32LE(tx.version);

    const locktimeBuffer = Buffer.allocUnsafe(4);
    locktimeBuffer.writeInt32LE(tx.locktime);

    return {
        version: versionBuffer.toString("hex"),
        inputVector: encodeRawInput(tx).toString("hex"),
        outputVector: encodeRawOutput(tx).toString("hex"),
        locktime: locktimeBuffer.toString("hex")
    }
}

export interface BitcoinTxProof {
    merkleProof: string,
    txIndexInBlock: number,
    bitcoinHeaders: string,
}

export async function getBitcoinTxProof(
    electrsClient: ElectrsClient,
    txId: string,
    txProofDifficultyFactor: number,
): Promise<BitcoinTxProof> {
    const merkleProof = await electrsClient.getMerkleProof(txId);

    const range = (start: number, end: number) => Array.from({ length: end - start }, (_element, index) => index + start);
    const blockHeights = range(merkleProof.blockHeight, merkleProof.blockHeight + txProofDifficultyFactor);

    const bitcoinHeaders = await Promise.all(blockHeights.map(async height => {
        const hash = await electrsClient.getBlockHash(height);
        return electrsClient.getBlockHeader(hash);
    }));

    return {
        merkleProof: merkleProof.merkle,
        txIndexInBlock: merkleProof.pos,
        bitcoinHeaders: bitcoinHeaders.join(''),
    }
}
