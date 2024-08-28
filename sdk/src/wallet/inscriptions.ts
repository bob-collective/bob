import { EsploraClient, UTXO } from "../esplora";
import { getTxInscriptions } from "../inscription";
import { OrdinalsClient, InscriptionId } from "../ordinal-api";

export async function findUtxoForInscriptionId(
    esploraClient: EsploraClient,
    ordinalsClient: OrdinalsClient,
    utxos: UTXO[],
    inscriptionId: string
): Promise<UTXO | undefined> {
    // TODO: can we get the current UTXO of the inscription from ord?
    // we can use the satpoint for this
    const { txid, index } = InscriptionId.fromString(inscriptionId)

    for (const utxo of utxos) {
        if (utxo.confirmed) {
            const inscriptionUtxo = await ordinalsClient.getInscriptionsFromOutPoint(utxo)
            if (inscriptionUtxo.inscriptions && inscriptionUtxo.inscriptions.includes(inscriptionId)) {
                return utxo;
            }
        } else if (txid == utxo.txid) {
            const inscriptions = await getTxInscriptions(esploraClient, utxo.txid);

            if (typeof inscriptions[index] !== 'undefined') {
                return utxo;
            }
        }
    }

    return undefined;
}

export async function findUtxosWithoutInscriptions(network: string, utxos: UTXO[]): Promise<UTXO[]> {
    const ordinalsClient = new OrdinalsClient(network);

    const safeUtxos: UTXO[] = [];

    // Exclude UTXOs that are uncomfirmed or have inscriptions
    await Promise.all([
        utxos.map(async (utxo) => {
            if (utxo.confirmed) {
                const inscription = await ordinalsClient.getInscriptionsFromOutPoint({
                    txid: utxo.txid,
                    vout: utxo.vout
                });

                if (inscription.inscriptions.length === 0) {
                    safeUtxos.push(utxo);
                }
            }
        })
    ]);

    return safeUtxos;
}
