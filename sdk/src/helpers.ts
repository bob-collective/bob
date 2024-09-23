import { EsploraClient, UTXO } from './esplora';
import { parseInscriptions } from './inscription';
import { InscriptionId, OrdinalsClient } from './ordinal-api';
import * as bitcoin from 'bitcoinjs-lib';

// Get the (encoded) inscription IDs for the address
export async function getInscriptionIds(
    esploraClient: EsploraClient,
    ordinalsClient: OrdinalsClient,
    bitcoinAddress: string
) {
    const utxos = await esploraClient.getAddressUtxos(bitcoinAddress);
    const inscriptionIds = await Promise.all(
        utxos
            .sort((a, b) => {
                // force large number if height is not available (as expected for unconfirmed utxo)
                const heightA = a.height || Number.MAX_SAFE_INTEGER;
                const heightB = b.height || Number.MAX_SAFE_INTEGER;

                return heightA - heightB;
            })
            .map((utxo) => getInscriptionIdsForUtxo(esploraClient, ordinalsClient, utxo))
    );
    return inscriptionIds.flat();
}

// Get the (encoded) inscription IDs for the UTXO
async function getInscriptionIdsForUtxo(esploraClient: EsploraClient, ordinalsClient: OrdinalsClient, utxo: UTXO) {
    if (utxo.confirmed) {
        // use ord api if the tx has been included in a block
        const outputJson = await ordinalsClient.getInscriptionsFromOutPoint(utxo);
        return outputJson.inscriptions;
    }

    const txHex = await esploraClient.getTransactionHex(utxo.txid);
    const tx = bitcoin.Transaction.fromHex(txHex);

    // FIXME: assumes inscriptions are always sent to the first output
    // which is not always the case, we should compare the satpoint
    if (utxo.vout == 0) {
        // this handles the case where we have just transferred an inscription
        // but the ordinal indexer has not yet confirmed it so we check if the
        // parent utxo has an inscription instead
        // NOTE: this won't work if the parent UTXO is not included in a block
        const parentInscriptions = await Promise.all(
            tx.ins.map(async (txInput) => {
                const txid = txInput.hash.reverse().toString('hex');
                const outputJson = await ordinalsClient.getInscriptionsFromOutPoint({ txid, vout: txInput.index });
                return outputJson.inscriptions;
            })
        );
        const inscriptionIds = parentInscriptions.flat();
        if (inscriptionIds.length > 0) {
            return inscriptionIds;
        }
    }

    // otherwise parse the inscriptions manually
    const inscriptions = parseInscriptions(tx);
    // FIXME: also assumes inscription is made on the first sat of the first output
    if (utxo.vout != 0) {
        return [];
    } else {
        // NOTE: it is possible for the inscription to be invalid if the sat is already inscribed
        return inscriptions.map((_, index) => InscriptionId.toString({ txid: utxo.txid, index }));
    }
}
