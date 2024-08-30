import * as bitcoin from "bitcoinjs-lib";
import * as psbtUtils from "bitcoinjs-lib/src/psbt/psbtutils";

import { RemoteSigner } from "./signer";
import { CommitTxData } from "./commit";

const { witnessStackToScriptWitness } = psbtUtils;

export interface CommitTxResult {
    tx: bitcoin.Transaction;
    outputIndex: number;
    outputAmount: number;
}

/**
 * Create the reveal tx which spends the commit tx.
 */
export function createRevealTx(
    network: bitcoin.Network,
    commitTxData: CommitTxData,
    commitTxResult: CommitTxResult,
    toAddress: string,
    amount: number,
) {
    const { scriptTaproot, tapLeafScript } = commitTxData;

    const psbt = new bitcoin.Psbt({ network });

    psbt.addInput({
        hash: commitTxResult.tx.getId(),
        index: commitTxResult.outputIndex,
        witnessUtxo: {
            value: commitTxResult.outputAmount,
            script: scriptTaproot.output!,
        },
        nonWitnessUtxo: commitTxResult.tx.toBuffer(),
        tapLeafScript: [tapLeafScript],
    });

    psbt.addOutput({
        value: amount,
        address: toAddress,
    });

    return psbt;
}

export const customFinalizer = (commitTxData: CommitTxData) => {
    const { tapLeafScript } = commitTxData;

    return (inputIndex: number, input: any) => {
        const witness = [input.tapScriptSig[inputIndex].signature]
            .concat(tapLeafScript.script)
            .concat(tapLeafScript.controlBlock);

        return {
            finalScriptWitness: witnessStackToScriptWitness(witness),
        };
    };
};

export async function signRevealTx(signer: RemoteSigner, commitTxData: CommitTxData, psbt: bitcoin.Psbt) {
    // reveal should only have one input
    psbt = await signer.signInput(0, psbt);

    // we have to construct our witness script in a custom finalizer
    psbt.finalizeInput(0, customFinalizer(commitTxData));

    return psbt.extractTransaction();
}
