import * as bitcoin from "bitcoinjs-lib";
import { EsploraClient } from "./esplora";
import { InscriptionId } from "./ordinal-api";

const textEncoder = new TextEncoder();

const OP_INT_BASE = bitcoin.opcodes.OP_RESERVED;

// https://github.com/rust-bitcoin/rust-bitcoin/blob/64bd34cffb54603db58efd718bd0b44c86366529/bitcoin/src/taproot.rs#L146
const TAPROOT_ANNEX_PREFIX = 0x50;

// https://github.com/ordinals/ord/blob/3bdbb00f57d06935f6ea78f115fca471ca8941be/src/envelope.rs#L9-L17
export const PROTOCOL_ID = Buffer.from("6f7264", "hex");

const CONTENT_TYPE_TAG = bitcoin.opcodes.OP_1;
const CONTENT_ENCODING_TAG = bitcoin.opcodes.OP_9;

// individual data pushes may not be larger than 520 bytes
export const MAX_CHUNK_SIZE = 520;

export function chunkContent(data: Buffer) {
    const body: Buffer[] = [];
    let start = 0;
    while (start < data.length) {
        body.push(data.subarray(start, start + MAX_CHUNK_SIZE));
        start += MAX_CHUNK_SIZE;
    }
    return body;
}

function convertOpInt(value: number) {
    if (value >= bitcoin.opcodes.OP_1 && value <= bitcoin.opcodes.OP_16) {
        return value - OP_INT_BASE;
    }
    return value;
}

export class Inscription {
    tags: Map<number, Buffer>;
    body: Buffer;

    constructor(tags?: Map<number, Buffer>, body?: Buffer) {
        this.tags = tags ?? new Map();
        this.body = body;
    }

    getContentType(): string | null {
        const data: Buffer | null = this.tags[CONTENT_TYPE_TAG];
        if (Buffer.isBuffer(data)) {
            return data.toString("utf-8");
        }
        return null;
    }

    getContentEncoding(): string | null {
        const data: Buffer | null = this.tags[CONTENT_ENCODING_TAG];
        if (Buffer.isBuffer(data)) {
            return data.toString("utf-8");
        }
        return null;
    }

    setContentType(contentType: string) {
        this.tags[CONTENT_TYPE_TAG] = Buffer.from(textEncoder.encode(contentType));
    }

    setContentEncoding(contentEncoding: string) {
        this.tags[CONTENT_ENCODING_TAG] = Buffer.from(textEncoder.encode(contentEncoding));
    }

    private getTags(): [number, Buffer][] {
        const tags = this.tags;
        return Object.keys(this.tags).map(function (key) {
            return [convertOpInt(Number(key)), tags[key]];
        });
    }

    toScript(xOnlyPublicKey: Buffer) {
        return [
            xOnlyPublicKey,
            bitcoin.opcodes.OP_CHECKSIG,
            bitcoin.opcodes.OP_0,
            bitcoin.opcodes.OP_IF,
            PROTOCOL_ID,
            ...this.getTags().map(([key, value]) => [
                1,
                key,
                value,
            ]).flat(),
            bitcoin.opcodes.OP_0,
            ...chunkContent(this.body),
            bitcoin.opcodes.OP_ENDIF,
        ];
    }
}

export module Inscription {
    /**
     * Create a basic text inscription.
     */
    export function createTextInscription(text: string): Inscription {
        return Inscription.createInscription(
            "text/plain;charset=utf-8",
            Buffer.from(textEncoder.encode(text))
        );
    }

    /**
     * Create an inscription.
     */
    export function createInscription(contentType: string, content: Buffer): Inscription {
        const inscription = new Inscription;
        // e.g. `image/png`
        inscription.setContentType(contentType);
        inscription.body = content;
        return inscription;
    }
}

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

            let tags: Map<number, Buffer> = new Map();
            let body: Buffer[] = [];
            let isBody = false;
            for (let chunk = chunks.next(); !chunk.done; chunk = chunks.next()) {
                if (chunk.value == bitcoin.opcodes.OP_ENDIF) {
                    inscriptions.push(new Inscription(tags, Buffer.concat(body)));
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

export async function getTxInscriptions(esploraClient: EsploraClient, txid: string) {
    const txHex = await esploraClient.getTransactionHex(txid);
    const tx = bitcoin.Transaction.fromHex(txHex);
    return parseInscriptions(tx);
}

export async function getInscriptionFromId(esploraClient: EsploraClient, inscriptionId: string) {
    const { txid, index } = InscriptionId.fromString(inscriptionId);
    const inscriptions = await getTxInscriptions(esploraClient, txid);
    return inscriptions[index];
}