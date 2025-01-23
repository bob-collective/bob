import * as bitcoin from 'bitcoinjs-lib';
import { EsploraClient } from './esplora';

class Rune {
    data: string;

    constructor(data: string) {
        this.data = data;
    }

    toString() {
        return `Rune: ${this.data}`;
    }
}

export function parseRune(tx: bitcoin.Transaction): Rune | undefined {
    for (const out of tx.outs) {
        try {
            const asm = bitcoin.script.toASM(out.script);

            if (asm.startsWith('OP_RETURN OP_13')) {
                const runeData = asm.split(' ')[2];
                return new Rune(runeData);
            }
        } catch (error) {
            console.error('Error parsing output script:', error);
        }
    }
}

export async function getTxRune(esploraClient: EsploraClient, txid: string) {
    const txHex = await esploraClient.getTransactionHex(txid);
    const tx = bitcoin.Transaction.fromHex(txHex);
    // A transaction may have at most one runestone.
    return parseRune(tx);
}
