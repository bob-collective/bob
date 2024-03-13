import { ElectrsClient, UTXO } from "./electrs";
import * as bitcoin from "bitcoinjs-lib";
import { InscriptionId, OrdinalsClient } from "./ordinal-api";
import { getTxInscriptions } from "./inscription";

import coinSelect = require('coinselect');

export interface SendParams {
    network: bitcoin.Network,
    fromBip32Derivation?: {
        masterFingerprint: Buffer,
        path: string
        pubkey: Buffer,
    },
    fromTapBip32Derivation?: {
        masterFingerprint: Buffer,
        path: string
        pubkey: Buffer,
        leafHashes: Buffer[],
    },
    tapInternalKey?: Buffer,
    fromAddress: string,
    toAddress: string,
    amount: number,
    feeRate: number,
    checkNoInscriptions: boolean,
}

export async function sendToAddress(
    electrsClient: ElectrsClient,
    ordinalsClient: OrdinalsClient,
    params: SendParams,
): Promise<bitcoin.Psbt> {
    const allConfirmedUtxos = await electrsClient.getAddressUtxos(params.fromAddress);
    const utxos = await findUtxosWithoutInscriptions(
        electrsClient,
        ordinalsClient,
        allConfirmedUtxos
    );

    const txOutputs = [
        {
            address: params.toAddress,
            value: params.amount,
        },
    ];

    const { inputs, outputs } = coinSelect(
        utxos.map((utxo) => {
            return {
                txId: utxo.txid,
                vout: utxo.vout,
                value: utxo.value,
            };
        }),
        txOutputs,
        params.feeRate,
    );

    if (inputs === undefined) {
        throw Error("No inputs returned/selected by coinSelect");
    }

    if (outputs === undefined) {
        throw Error("No outputs returned/selected by coinSelect");
    }

    const psbt = new bitcoin.Psbt({ network: params.network });

    for (const input of inputs) {
        const txHex = await electrsClient.getTransactionHex(input.txId);
        const utx = bitcoin.Transaction.fromHex(txHex);

        const witnessUtxo = {
            script: utx.outs[input.vout].script,
            value: input.value,
        };
        const nonWitnessUtxo = utx.toBuffer();

        psbt.addInput({
            hash: input.txId,
            index: input.vout,
            nonWitnessUtxo,
            witnessUtxo,
            bip32Derivation: params.fromBip32Derivation ? [params.fromBip32Derivation] : undefined,
            tapBip32Derivation: params.fromTapBip32Derivation ? [params.fromTapBip32Derivation] : undefined,
            tapInternalKey: params.tapInternalKey,
        });
    }

    outputs.forEach((output) => {
        // output may have been added for change
        if (!output.address) {
            output.address = params.fromAddress;
        }

        psbt.addOutput({
            address: output.address,
            value: output.value,
        });
    });

    return psbt;
}

// NOTE: requires higher postage since we don't include cardinals
// TODO: fund transfer with cardinals (non-inscription UTXOs)
export async function sendInscription(
    electrsClient: ElectrsClient,
    ordinalsClient: OrdinalsClient,
    params: SendParams,
    inscriptionId: string,
): Promise<bitcoin.Psbt> {
    const utxos = await electrsClient.getAddressUtxos(params.fromAddress);
    const inscriptionUtxo = await findUtxoForInscriptionId(
        electrsClient,
        ordinalsClient,
        utxos,
        inscriptionId
    );

    if (inscriptionUtxo === undefined) {
        throw Error(
            `Unable to find utxo owned by address ${params.fromAddress} containing inscription id ${inscriptionId}`
        );
    }

    const psbt = new bitcoin.Psbt({ network: params.network });
    const txHex = await electrsClient.getTransactionHex(inscriptionUtxo.txid);
    const utx = bitcoin.Transaction.fromHex(txHex);

    // prepare single input
    psbt.addInput({
        hash: inscriptionUtxo.txid,
        index: inscriptionUtxo.vout,
        nonWitnessUtxo: utx.toBuffer(),
        witnessUtxo: {
            script: utx.outs[inscriptionUtxo.vout].script,
            value: inscriptionUtxo.value,
        },
        bip32Derivation: params.fromBip32Derivation ? [params.fromBip32Derivation] : undefined,
        tapBip32Derivation: params.fromTapBip32Derivation ? [params.fromTapBip32Derivation] : undefined,
        tapInternalKey: params.tapInternalKey,
    });

    const txSize = estimateTxSize(params.network, params.toAddress);
    const fee = txSize * params.feeRate;

    psbt.addOutput({
        address: params.toAddress,
        value: inscriptionUtxo.value - fee,
    });

    return psbt;
}

// NOTE: will only work for P2WSH
function estimateTxSize(network: bitcoin.Network, toAddress: string) {
    const tx = new bitcoin.Transaction();
    tx.addInput(Buffer.alloc(32, 0), 0);
    tx.ins[0].witness = [Buffer.alloc(71, 0), Buffer.alloc(33, 0)];
    tx.addOutput(bitcoin.address.toOutputScript(toAddress, network), 0);
    return tx.virtualSize();
}

/**
 * Given an array of UTXOs passed in, return those that do not contain any inscriptions.
 */
async function findUtxosWithoutInscriptions(
    electrsClient: ElectrsClient,
    ordinalsClient: OrdinalsClient,
    utxos: UTXO[]
): Promise<UTXO[]> {
    const safeUtxos = [];

    for (const utxo of utxos) {
        if (utxo.confirmed) {
            const inscriptionUtxo = await ordinalsClient.getInscriptionsFromOutPoint(utxo);
            if (inscriptionUtxo.inscriptions.length === 0) {
                safeUtxos.push(utxo);
            }
        } else {
            // we can't use the ord indexer if the tx is unconfirmed
            const inscriptions = await getTxInscriptions(electrsClient, utxo.txid);
            // TODO: we can use this if the sats in the utxo aren't inscribed
            if (inscriptions.length === 0) {
                safeUtxos.push(utxo);
            }
        }
    }

    return safeUtxos;
}

async function findUtxoForInscriptionId(
    electrsClient: ElectrsClient,
    ordinalsClient: OrdinalsClient,
    utxos: UTXO[],
    inscriptionId: string
): Promise<UTXO | undefined> {
    // TODO: can we get the current UTXO of the inscription from ord?
    // we can use the satpoint for this
    const { txid, index } = InscriptionId.fromString(inscriptionId);

    for (const utxo of utxos) {
        if (utxo.confirmed) {
            const inscriptionUtxo = await ordinalsClient.getInscriptionsFromOutPoint(utxo);
            if (
                inscriptionUtxo.inscriptions &&
                inscriptionUtxo.inscriptions.includes(inscriptionId)
            ) {
                return utxo;
            }
        } else if (txid == utxo.txid) {
            const inscriptions = await getTxInscriptions(electrsClient, utxo.txid);
            if (typeof inscriptions[index] !== "undefined") {
                return utxo;
            }
        }
    }

    return undefined;
}
