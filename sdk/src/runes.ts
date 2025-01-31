import * as bitcoin from 'bitcoinjs-lib';
import { EsploraClient } from './esplora';
import { Cenotaph, RunestoneSpec, tryDecodeRunestone } from '@magiceden-oss/runestone-lib';

export function parseRunestone(tx: bitcoin.Transaction): RunestoneSpec | Cenotaph | null {
    const rune = tryDecodeRunestone({
        vout: tx.outs.map((out) => ({
            scriptPubKey: {
                hex: out.script.toString('hex'),
            },
        })),
    });

    return rune;
}

export async function getTxRunestone(esploraClient: EsploraClient, txid: string) {
    const txHex = await esploraClient.getTransactionHex(txid);
    const tx = bitcoin.Transaction.fromHex(txHex);
    return parseRunestone(tx);
}
