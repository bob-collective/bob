import * as bitcoin from 'bitcoinjs-lib';
import { Inscription } from '../inscription';

export interface CommitTxData {
    scriptTaproot: bitcoin.payments.Payment;
    tapLeafScript: {
        leafVersion: number;
        script: Buffer;
        controlBlock: Buffer;
    };
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
    const script = inscription.toScript(xOnlyPublicKey);

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

    const cblock = scriptTaproot.witness?.at(-1);

    const tapLeafScript = {
        leafVersion: scriptTaproot.redeemVersion!,
        script: outputScript,
        controlBlock: cblock || Buffer.alloc(0),
    };

    return {
        scriptTaproot,
        tapLeafScript,
    };
}
