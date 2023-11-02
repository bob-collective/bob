import {
    DefaultOrdinalsClient,
    Ordinal,
    InscriptionsData,
    InscriptionUTXO,
    InscriptionDataFromId, InscriptionId,
} from '../src/ordinals/indexer';
import {assert} from "chai";



describe("Indexer Tests", () => {
    it.skip("should get inscriptions from id", async () => {

        const client = new DefaultOrdinalsClient("regtest");
        let id64: InscriptionId = "d48ddc093055faa940c2a44220f77c38526b3e0f79d686a7340ba5b990a440a0i0" as InscriptionId;
        const inscriptions: InscriptionDataFromId = await client.getInscriptionFromId(id64);

        const expected_inscription: InscriptionDataFromId = {
            address: 'bcrt1pjw626gy4m7hx5htenxgfplg4lugejs7xjh3kwjfxuapz7hujaguqausksf',
            children: [],
            content_length: 94,
            content_type: 'application/json',
            genesis_fee: 3402,
            genesis_height: 227,
            inscription_id: 'd48ddc093055faa940c2a44220f77c38526b3e0f79d686a7340ba5b990a440a0i0',
            inscription_number: 3,
            next: null,
            output_value: 10000,
            parent: null,
            previous: '605a758a2ea7d31a820930e193df9460d19741090ad8b6d6ad92542715e1ceb8i0',
            rune: null,
            sat: null,
            satpoint: 'd48ddc093055faa940c2a44220f77c38526b3e0f79d686a7340ba5b990a440a0:0:0',
            timestamp: 1698903875
        };

        assert.deepStrictEqual(expected_inscription, inscriptions);
    });

    it.skip("should get inscriptions", async () => {
        console.log("hello");
        const client = new DefaultOrdinalsClient("regtest");
        const inscriptions: InscriptionsData = await client.getInscriptions();
        // Assert that inscriptions is not null or undefined
        assert.isNotNull(inscriptions);
    });

    it.skip("should get inscriptions from block", async () => {
        const client = new DefaultOrdinalsClient("regtest");
        let block: number = 227;
        const inscriptions: InscriptionsData = await client.getInscriptionsFromBlock(block);
        const expected_inscription: InscriptionsData = {
            inscriptions: ['d48ddc093055faa940c2a44220f77c38526b3e0f79d686a7340ba5b990a440a0i0'],
            prev: null,
            next: null,
            lowest: null,
            highest: null,
        };
        assert.deepStrictEqual(expected_inscription, inscriptions);
    });

    it.skip("should get inscriptions from UTXO", async () => {
        const client = new DefaultOrdinalsClient("regtest");
        let utxo: string = "81188c296a0e5d9bba5c070a44bba1dbd1534e4c93fa5542b882fc402b6c5b69:0";
        const inscriptions: InscriptionUTXO = await client.getInscriptionFromUTXO(utxo);
        const expected_inscription: InscriptionUTXO = {
            value: 13402,
            script_pubkey: 'OP_PUSHNUM_1 OP_PUSHBYTES_32 23981d827eceaa029542d7b10f85f7e8d4f0732dbd25ce8be9235e47ae902d66',
            address: 'bcrt1pywvpmqn7e64q992z67cslp0har20quedh5juazlfyd0y0t5s94nqkmdwl2',
            transaction: '81188c296a0e5d9bba5c070a44bba1dbd1534e4c93fa5542b882fc402b6c5b69',
            sat_ranges: null,
            inscriptions: [],
            runes: {}
        };
        assert.deepStrictEqual(expected_inscription, inscriptions);
    });

    it.skip("should get inscriptions from Sat", async () => {
        const client = new DefaultOrdinalsClient("regtest");
        let sat: number = 100;
        const ordinal: Ordinal = await client.getInscriptionsFromSat(sat);
        const expected_ordinal: Ordinal = {
            number: 100,
            decimal: '0.100',
            degree: '0°0′0″100‴',
            name: 'nvtdijuwxht',
            block: 0,
            cycle: 0,
            epoch: 0,
            period: 0,
            offset: 100,
            rarity: 'common',
            percentile: '0.0000000000047619047671428594%',
            satpoint: null,
            timestamp: 1296688602,
            inscriptions: []
        };
        assert.deepStrictEqual(expected_ordinal, ordinal);
    });
});
