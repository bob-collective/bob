import * as bitcoin from "bitcoinjs-lib";
import { ElectrsClient } from "./electrs";
import { InscriptionId } from "./ordinal-api";

// https://github.com/rust-bitcoin/rust-bitcoin/blob/64bd34cffb54603db58efd718bd0b44c86366529/bitcoin/src/taproot.rs#L146
const TAPROOT_ANNEX_PREFIX = 0x50;

// https://github.com/ordinals/ord/blob/3bdbb00f57d06935f6ea78f115fca471ca8941be/src/envelope.rs#L9-L17
export const PROTOCOL_ID = Buffer.from("6f7264", "hex");

const CONTENT_TYPE_TAG = bitcoin.opcodes.OP_1;
const CONTENT_ENCODING_TAG = bitcoin.opcodes.OP_9;

// https://github.com/rust-bitcoin/rust-bitcoin/blob/8aa5501827a0dd5b27abf304a5f9bdefb07a2cc6/bitcoin/src/blockdata/witness.rs#L386-L406
function getTapscript(witness: Buffer[]) {
    const len = witness.length;
    const last = witness[len - 1];
    if (typeof last === 'undefined') {
        return null;
    }
    let scriptPosFromLast = 2;
    if (len >= 2 && last[0] == TAPROOT_ANNEX_PREFIX) {
        scriptPosFromLast = 3
    }
    if (typeof witness[len - scriptPosFromLast] === 'undefined') {
        return null;
    }
    return bitcoin.script.decompile(witness[len - scriptPosFromLast]);
}

interface Tags { [key: number]: Buffer | null };
interface Inscription {
    tags: Tags,
    body: Buffer[],
}

export function getContentType(inscription: Inscription): string | null {
    const data: Buffer | null = inscription.tags[CONTENT_TYPE_TAG];
    if (Buffer.isBuffer(data)) {
        return data.toString("utf-8");
    }
    return null;
}

export function getContentEncoding(inscription: Inscription): string | null {
    const data: Buffer | null = inscription.tags[CONTENT_ENCODING_TAG];
    if (Buffer.isBuffer(data)) {
        return data.toString("utf-8");
    }
    return null;
}

export function parseInscriptions(tx: bitcoin.Transaction) {
    let inscriptions: Inscription[] = [];

    for (const txInput of tx.ins) {
        const tapscript = getTapscript(txInput.witness);
        if (tapscript == null) {
            continue;
        }

        const chunks = tapscript.values();
        for (let chunk = chunks.next(); !chunk.done; chunk = chunks.next()) {
            // envelope is `OP_FALSE OP_IF ... OP_ENDIF`
            if (chunk.value != bitcoin.opcodes.OP_FALSE) {
                continue;
            }
            if (chunks.next().value != bitcoin.opcodes.OP_IF) {
                continue;
            }

            // check next push is "ord"
            const data = chunks.next().value;
            if (!Buffer.isBuffer(data) && !data.equals(PROTOCOL_ID)) {
                continue;
            }

            let tags: Tags = {};
            let body: Buffer[] = [];
            let isBody = false;
            for (let chunk = chunks.next(); !chunk.done; chunk = chunks.next()) {
                if (chunk.value == bitcoin.opcodes.OP_ENDIF) {
                    inscriptions.push({ tags, body });
                    break;
                } else if (chunk.value == bitcoin.opcodes.OP_0) {
                    // `OP_PUSH 0` indicates that subsequent data pushes contain the content itself
                    isBody = true;
                    continue;
                }
                // fields are before body, e.g. `OP_PUSH 1` is content type
                if (!isBody) {
                    const data = chunks.next().value;
                    if (typeof chunk.value == 'number' && Buffer.isBuffer(data)) {
                        tags[chunk.value] = data;
                    }
                } else if (Buffer.isBuffer(chunk.value)) {
                    body.push(chunk.value);
                }
            }
        }
    }

    return inscriptions;
}

export async function getTxInscriptions(electrsClient: ElectrsClient, txid: string) {
    const txHex = await electrsClient.getTransactionHex(txid);
    const tx = bitcoin.Transaction.fromHex(txHex);
    return parseInscriptions(tx);
}

export async function getInscriptionFromId(electrsClient: ElectrsClient, inscriptionId: string) {
    const { txid, index } = InscriptionId.fromString(inscriptionId);
    const inscriptions = await getTxInscriptions(electrsClient, txid);
    return inscriptions[index];
}