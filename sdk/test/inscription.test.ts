import * as bitcoin from "bitcoinjs-lib";
import { DefaultElectrsClient } from "../src/electrs";
import { PROTOCOL_ID, getContentEncoding, getContentType, parseInscriptions } from "../src/inscription";
import { createInscriptionScript, createTextInscription } from "../src/ordinals/commit";
import { assert, describe, it } from "vitest";

const encoder = new TextEncoder();

function createOrdinalTransaction(outputScript: Buffer) {
    const tx = new bitcoin.Transaction();
    tx.ins.push({
        hash: Buffer.alloc(32, 0),
        index: 0,
        script: Buffer.alloc(0),
        sequence: 0,
        witness: [
            Buffer.alloc(64, 0), // tapScriptSig
            outputScript, // outputScript
            Buffer.alloc(33, 0), // controlBlock
        ],
    });
    return tx;
}

describe("Inscription Tests", () => {
    it("should parse text inscription", async () => {
        const electrsClient = new DefaultElectrsClient("mainnet");

        const txHex = await electrsClient.getTransactionHex("b61b0172d95e266c18aea0c624db987e971a5d6d4ebc2aaed85da4642d635735");
        const tx = bitcoin.Transaction.fromHex(txHex);

        const inscriptions = parseInscriptions(tx);
        assert(inscriptions.length == 1, "Inscription not found");

        const contentType = getContentType(inscriptions[0]);
        assert.equal(contentType, "text/plain;charset=utf-8");

        const content = Buffer.concat(inscriptions[0].body);
        assert.deepStrictEqual(
            JSON.parse(content.toString('utf-8')),
            {
                p: 'brc-20',
                op: 'deploy',
                tick: 'ordi',
                max: '21000000',
                lim: '1000'
            }
        );
    });

    it("should parse image inscription", async () => {
        const electrsClient = new DefaultElectrsClient("mainnet");

        const txHex = await electrsClient.getTransactionHex("79ddcce9b4aaa4d2c3ba512a1dfd9bf2dd1f840eab98101c41bf8b801bcb3e0c");
        const tx = bitcoin.Transaction.fromHex(txHex);

        const inscriptions = parseInscriptions(tx);
        assert(inscriptions.length == 1, "Inscription not found");

        const contentType = getContentType(inscriptions[0]);
        assert.equal(contentType, "image/webp");
    });

    it("should parse custom text inscription", async () => {
        const textContent = "Hello World";
        const script = createInscriptionScript(
            Buffer.alloc(32, 0),
            createTextInscription(textContent)
        );
        const outputScript = bitcoin.script.compile(script);
        const tx = createOrdinalTransaction(outputScript);

        const inscriptions = parseInscriptions(tx);
        assert.equal(inscriptions.length, 1, "Inscription not found");
        assert.equal(inscriptions[0].body.length, 1);
        assert.equal(inscriptions[0].body[0].toString("utf-8"), textContent);
    });

    it("should parse custom inscription without content type", async () => {
        const outputScript = bitcoin.script.compile([
            Buffer.alloc(32, 0),
            bitcoin.opcodes.OP_CHECKSIG,
            bitcoin.opcodes.OP_0,
            bitcoin.opcodes.OP_IF,
            PROTOCOL_ID,
            bitcoin.opcodes.OP_0,
            Buffer.alloc(520, 0),
            bitcoin.opcodes.OP_ENDIF,
        ]);
        const tx = createOrdinalTransaction(outputScript);

        const inscriptions = parseInscriptions(tx);
        assert.equal(inscriptions.length, 1, "Inscription not found");
        assert.equal(inscriptions[0].body.length, 1);
        assert.equal(inscriptions[0].body[0].length, 520);
    });

    it("should parse custom inscription with content encoding", async () => {
        const outputScript = bitcoin.script.compile([
            Buffer.alloc(32, 0),
            bitcoin.opcodes.OP_CHECKSIG,
            bitcoin.opcodes.OP_0,
            bitcoin.opcodes.OP_IF,
            PROTOCOL_ID,
            Buffer.from([9]),
            Buffer.from(encoder.encode("gzip")),
            bitcoin.opcodes.OP_0,
            Buffer.alloc(520, 1),
            Buffer.alloc(300, 0),
            bitcoin.opcodes.OP_ENDIF,
        ]);
        const tx = createOrdinalTransaction(outputScript);

        const inscriptions = parseInscriptions(tx);
        assert.equal(inscriptions.length, 1, "Inscription not found");
        assert.equal(inscriptions[0].body.length, 2);

        const contentEncoding = getContentEncoding(inscriptions[0]);
        assert.equal(contentEncoding, "gzip");
    });
});
