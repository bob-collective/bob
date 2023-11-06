import {
    DefaultOrdinalsClient,
    Ordinal,
    InscriptionsData,
    InscriptionUTXO,
    InscriptionDataFromId, InscriptionId,
} from '../src/ordinal-indexer/indexer';
import {assert} from "chai";


describe("Indexer Tests", () => {
    it("should get inscriptions from id", async () => {
        const client = new DefaultOrdinalsClient("testnet");
        const inscriptionId: InscriptionId = "74c86592f75716a14a534898913e6077fb5d7650cfc17600868964bbe2b7e512i0" as InscriptionId;
        const inscriptions: InscriptionDataFromId = await client.getInscriptionFromId(inscriptionId);

        const expectedInscription: InscriptionDataFromId = {
            address: 'tb1pfwdp9xswfxcd4f33mxgqul6f56gt7q9v5gjmrtgs4xs3ezxks03swrsr2d',
            children: [],
            content_length: 868,
            content_type: 'text/javascript',
            genesis_fee: 395,
            genesis_height: 2537128,
            inscription_id: '74c86592f75716a14a534898913e6077fb5d7650cfc17600868964bbe2b7e512i0',
            inscription_number: 560474,
            next: 'dd90d8222da2a6f3260109b1e4d1a2c341d999fce4707b1d77e49956a51a0305i0',
            output_value: 546,
            parent: null,
            previous: '332d3fae125de51de29e97cd9e80aab7c63025d5094944a3dceb117c556c41cci0',
            rune: null,
            sat: null,
            satpoint: '74c86592f75716a14a534898913e6077fb5d7650cfc17600868964bbe2b7e512:0:0',
            timestamp: 1699246476
        };

        assert.deepStrictEqual(expectedInscription, inscriptions);
    });

    it("should get inscriptions", async () => {
        const client = new DefaultOrdinalsClient("testnet");
        const inscriptions: InscriptionsData = await client.getInscriptions();
        // Assert that inscriptions is not null or undefined
        assert.isNotNull(inscriptions);
    });

    it("should get inscriptions from block", async () => {
        const client = new DefaultOrdinalsClient("testnet");
        const block: number = 2537133;
        const inscriptions: InscriptionsData = await client.getInscriptionsFromBlock(block);
        const expectedInscription: InscriptionsData = {
            inscriptions: [
                '4d8e7ad2b410eaa79e3aa703bbe5a314cc89be9a07532bfab09f3c5dffac6348i0',
                'd370be1b6bf74677c82226d7a0d65743cbe3846b9216e0ad207a7b03a5230ec3i0'
            ],
            prev: null,
            next: null,
            lowest: null,
            highest: null,
        };
        assert.deepStrictEqual(expectedInscription, inscriptions);
    });

    it("should get inscriptions from UTXO", async () => {
        const client = new DefaultOrdinalsClient("testnet");
        const utxo: string = "d370be1b6bf74677c82226d7a0d65743cbe3846b9216e0ad207a7b03a5230ec3:0";
        const inscriptions: InscriptionUTXO = await client.getInscriptionFromUTXO(utxo);
        const expectedInscription: InscriptionUTXO = {
            value: 1967,
            script_pubkey: "OP_PUSHNUM_1 OP_PUSHBYTES_32 24ad201633789999cbe4251018e796acb22ec5d1a6f8a1873adc6363e04d7e7d",
            address: 'tb1pyjkjq93n0zvenjlyy5gp3euk4jeza3w35mu2rpe6m33k8czd0e7s3ha8st',
            transaction: 'd370be1b6bf74677c82226d7a0d65743cbe3846b9216e0ad207a7b03a5230ec3',
            sat_ranges: null,
            inscriptions: ['d370be1b6bf74677c82226d7a0d65743cbe3846b9216e0ad207a7b03a5230ec3i0'],
            runes: {}
        };
        assert.deepStrictEqual(expectedInscription, inscriptions);
    });

    it("should get inscriptions from Sat", async () => {
        const client = new DefaultOrdinalsClient("testnet");
        const sat: number = 100;
        const ordinal: Ordinal = await client.getInscriptionsFromSat(sat);
        const expectedOrdinal: Ordinal = {
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
        assert.deepStrictEqual(expectedOrdinal, ordinal);
    });

    it("should get inscriptions from start block", async () => {
        const client = new DefaultOrdinalsClient("testnet");
        const startBlock: number = 2537138;
        const inscriptions: InscriptionsData = await client.getInscriptionsFromStartBlock(startBlock);
        // Assert that inscriptions is not null or undefined
        assert.isNotNull(inscriptions);
    });
});
