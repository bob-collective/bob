import * as bitcoinjsLib from "bitcoinjs-lib";

const encoder = new TextEncoder();

function toXOnly(pubkey: Buffer) {
    return pubkey.subarray(1, 33);
}

interface Inscription {
    contentType: Buffer;
    content: Buffer;
}

/**
 * Create a basic text inscription.
 */
export function createTextInscription(text: string): Inscription {
    const contentType = Buffer.from(encoder.encode("text/plain;charset=utf-8"));
    const content = Buffer.from(encoder.encode(text));
    return { contentType, content };
}

function createInscriptionScript(xOnlyPublicKey: Buffer, inscription: Inscription) {
    const protocolId = Buffer.from(encoder.encode("ord"));
    return [
        xOnlyPublicKey,
        bitcoinjsLib.opcodes.OP_CHECKSIG,
        bitcoinjsLib.opcodes.OP_0,
        bitcoinjsLib.opcodes.OP_IF,
        protocolId,
        1,
        1,
        inscription.contentType,
        bitcoinjsLib.opcodes.OP_0,
        inscription.content,
        bitcoinjsLib.opcodes.OP_ENDIF,
    ];
}

export interface CommitTxData {
    script: (number | Buffer)[];
    scriptTaproot: bitcoinjsLib.payments.Payment;
    outputScript: Buffer;
    tapLeafScript: {
        leafVersion: number;
        script: Buffer;
        controlBlock: Buffer;
    }
}

/**
 * Create the commit tx of the input public key and inscription data.
 * @dev Requires caller to initialize ECC lib.
 */
export function createCommitTxData(
    network: bitcoinjsLib.Network,
    publicKey: Buffer,
    inscription: Inscription
): CommitTxData {
    const xOnlyPublicKey = toXOnly(publicKey);
    const script = createInscriptionScript(xOnlyPublicKey, inscription);

    const outputScript = bitcoinjsLib.script.compile(script);

    const scriptTree = {
        output: outputScript,
        redeemVersion: 192, // 0xc0
    };

    const scriptTaproot = bitcoinjsLib.payments.p2tr({
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
        script,
        scriptTaproot,
        outputScript,
        tapLeafScript,
    };
}

export interface CommitTxResult {
    txId: string;
    sendUtxoIndex: number;
    sendAmount: number;
}
