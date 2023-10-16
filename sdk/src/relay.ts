import { Transaction } from "bitcoinjs-lib";
import { ElectrsClient } from "./electrs";
import { encodeRawInput, encodeRawOutput, encodeRawWitness } from "./utils";

export interface BitcoinTxInfo {
    version: string,
    inputVector: string,
    outputVector: string,
    locktime: string,
    witnessVector?: string,
}

export async function getBitcoinTxInfo(
    electrsClient: ElectrsClient,
    txId: string,
    forWitness?: boolean,
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
        locktime: locktimeBuffer.toString("hex"),
        witnessVector: forWitness ? encodeRawWitness(tx).toString("hex") : undefined,
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
    const bitcoinHeaders = await getBitcoinHeaders(electrsClient, merkleProof.blockHeight, txProofDifficultyFactor);

    return {
        merkleProof: merkleProof.merkle,
        txIndexInBlock: merkleProof.pos,
        bitcoinHeaders: bitcoinHeaders,
    }
}

export async function getBitcoinHeaders(
    electrsClient: ElectrsClient,
    startHeight: number,
    numBlocks: number,
): Promise<string> {
    const range = (start: number, end: number) => Array.from({ length: end - start }, (_element, index) => index + start);
    const blockHeights = range(startHeight, startHeight + numBlocks);

    const bitcoinHeaders = await Promise.all(blockHeights.map(async height => {
        const hash = await electrsClient.getBlockHash(height);
        return electrsClient.getBlockHeader(hash);
    }));

    return bitcoinHeaders.join('');
}
