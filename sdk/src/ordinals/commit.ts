import * as bitcoin from "bitcoinjs-lib";

const encoder = new TextEncoder();

// individual data pushes may not be larger than 520 bytes
export const MAX_CHUNK_SIZE = 520;

export interface Inscription {
    contentType: Buffer; // MIME type
    content: Buffer;
}

/**
 * Create a basic text inscription.
 */
export function createTextInscription(text: string): Inscription {
    return createInscription(
        "text/plain;charset=utf-8",
        Buffer.from(encoder.encode(text))
    );
}

/**
 * Create an inscription.
 */
export function createInscription(contentType: string, content: Buffer): Inscription {
    return {
        // e.g. `image/png`
        contentType: Buffer.from(encoder.encode(contentType)),
        content
    };
}

export function chunkContent(data: Buffer) {
    const body: Buffer[] = [];
    let start = 0;
    while (start < data.length) {
        body.push(data.subarray(start, start + MAX_CHUNK_SIZE));
        start += MAX_CHUNK_SIZE;
    }
    return body;
}

export function createInscriptionScript(xOnlyPublicKey: Buffer, inscription: Inscription) {
    const protocolId = Buffer.from(encoder.encode("ord"));
    return [
        xOnlyPublicKey,
        bitcoin.opcodes.OP_CHECKSIG,
        bitcoin.opcodes.OP_0,
        bitcoin.opcodes.OP_IF,
        protocolId,
        1,
        1, // NOTE: Buffer.from([1]) is replaced here https://github.com/bitcoinjs/bitcoinjs-lib/blob/master/src/script.js#L53
        inscription.contentType,
        bitcoin.opcodes.OP_0,
        ...chunkContent(inscription.content),
        bitcoin.opcodes.OP_ENDIF,
    ];
}

export interface CommitTxData {
    scriptTaproot: bitcoin.payments.Payment;
    tapLeafScript: {
        leafVersion: number;
        script: Buffer;
        controlBlock: Buffer;
    }
}

function toXOnly(pubkey: Buffer) {
    return pubkey.subarray(1, 33);
}

/**
 * Create the commit tx of the input public key and inscription data.
 * @dev Requires caller to initialize ECC lib.
 */
export function createCommitTxData(
    network: bitcoin.Network,
    publicKey: Buffer,
    inscription: Inscription
): CommitTxData {
    const xOnlyPublicKey = toXOnly(publicKey);
    const script = createInscriptionScript(xOnlyPublicKey, inscription);

    const outputScript = bitcoin.script.compile(script);

    const scriptTree = {
        output: outputScript,
        redeemVersion: 192, // 0xc0
    };

    const scriptTaproot = bitcoin.payments.p2tr({
        internalPubkey: xOnlyPublicKey,
        scriptTree,
        redeem: scriptTree,
        network,
    });

    const cblock = scriptTaproot.witness?.[scriptTaproot.witness.length - 1];

    const tapLeafScript = {
        leafVersion: scriptTaproot.redeemVersion!,
        script: outputScript,
        controlBlock: cblock,
    };

    return {
        scriptTaproot,
        tapLeafScript,
    };
}

