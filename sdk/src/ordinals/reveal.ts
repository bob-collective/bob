import * as bitcoinjsLib from "bitcoinjs-lib";
import * as psbtUtils from "bitcoinjs-lib/src/psbt/psbtutils";

import { RemoteSigner } from "./signer";
import { CommitTxData, CommitTxResult } from "./commit";

const { witnessStackToScriptWitness } = psbtUtils;

/**
 * Create the reveal tx which spends the commit tx.
 */
export function createRevealTx(
    network: bitcoinjsLib.Network,
    commitTxData: CommitTxData,
    commitTxResult: CommitTxResult,
    toAddress: string,
    amount: number,
) {
    const { scriptTaproot, tapLeafScript } = commitTxData;

    const psbt = new bitcoinjsLib.Psbt({ network });

    psbt.addInput({
        hash: commitTxResult.txId,
        index: commitTxResult.sendUtxoIndex,
        witnessUtxo: {
            value: commitTxResult.sendAmount,
            script: scriptTaproot.output!,
        },
        tapLeafScript: [tapLeafScript],
    });

    psbt.addOutput({
        value: amount,
        address: toAddress,
    });

    return psbt;
}

export const customFinalizer = (commitTxData: CommitTxData) => {
    const { outputScript, tapLeafScript } = commitTxData;

    return (inputIndex: number, input: any) => {
        const witness = [input.tapScriptSig[inputIndex].signature]
            .concat(outputScript)
            .concat(tapLeafScript.controlBlock);

        return {
            finalScriptWitness: witnessStackToScriptWitness(witness),
        };
    };
}

export async function signRevealTx(
    signer: RemoteSigner,
    commitTxData: CommitTxData,
    psbt: bitcoinjsLib.Psbt
) {
    // reveal should only have one input
    psbt = await signer.signPsbt(0, psbt);

    // we have to construct our witness script in a custom finalizer
    psbt.finalizeInput(0, customFinalizer(commitTxData));

    return psbt.extractTransaction();
}