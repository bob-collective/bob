import * as bitcoin from 'bitcoinjs-lib';
import { EsploraClient } from './esplora';
// This implementation is based on ord 0.17.1, *use this as a reference only
import { Runestone } from 'runestone-js';

export function parseRunestone(tx: bitcoin.Transaction): Runestone | undefined {
    for (const out of tx.outs) {
        if (out.script[0] === bitcoin.opcodes.OP_RETURN && out.script[1] === bitcoin.opcodes.OP_13) {
            // skip OP_RETURN OP_13 OP_PUSHBYTES
            const pureOutputBuffer = out.script.subarray(3);
            return Runestone.dechiper(pureOutputBuffer);
        }
    }
}

export async function getTxRunestone(esploraClient: EsploraClient, txid: string) {
    const txHex = await esploraClient.getTransactionHex(txid);
    const tx = bitcoin.Transaction.fromHex(txHex);
    return parseRunestone(tx);
}
