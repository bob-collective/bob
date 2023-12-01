import {
    DefaultOrdinalsClient,
    InscriptionJson,
    OutputJson,
    InscriptionId,
    SatPoint,
} from '../src/ordinal-api';
import { assert } from "chai";

describe("Ordinal API Tests", () => {
    it("should get inscription from id", async () => {
        const client = new DefaultOrdinalsClient("testnet");
        const inscriptionJson = await client.getInscriptionFromId({
            txid: "74c86592f75716a14a534898913e6077fb5d7650cfc17600868964bbe2b7e512",
            index: 0,
        });

        const expectedInscriptionJson: InscriptionJson<InscriptionId, SatPoint> = {
            address: 'tb1pn2ujghy4l0la62c0e2n9q7s8a2yc4nx3kszdtdphwrsyap46ln4q3aveda',
            children: [],
            content_length: 868,
            content_type: 'text/javascript',
            genesis_fee: 395,
            genesis_height: 2537128,
            inscription_id: InscriptionId.fromString('74c86592f75716a14a534898913e6077fb5d7650cfc17600868964bbe2b7e512i0'),
            inscription_number: 560474,
            next: InscriptionId.fromString('dd90d8222da2a6f3260109b1e4d1a2c341d999fce4707b1d77e49956a51a0305i0'),
            output_value: 730,
            parent: null,
            previous: InscriptionId.fromString('332d3fae125de51de29e97cd9e80aab7c63025d5094944a3dceb117c556c41cci0'),
            rune: null,
            sat: null,
            satpoint: SatPoint.fromString('2523f1ac7594c1f45e9156588d78caa445e2dcacf5e80fdd3d5d74fa02e1c30a:171:584'),
            timestamp: 1699246476
        };

        assert.deepStrictEqual(expectedInscriptionJson, inscriptionJson);
    });

    it("should get inscriptions", async () => {
        const client = new DefaultOrdinalsClient("testnet");
        const inscriptionsJson = await client.getInscriptions();
        // assert that inscriptionsJson is not null, undefined or empty
        assert.isNotNull(inscriptionsJson);
        assert.isNotEmpty(inscriptionsJson);
    });

    it("should get inscriptions from block", async () => {
        const client = new DefaultOrdinalsClient("testnet");
        const block: number = 2537133;
        const inscriptionsJson = await client.getInscriptionsFromBlock(block);
        const expectedInscriptionsJson = {
            inscriptions: [
                InscriptionId.fromString('4d8e7ad2b410eaa79e3aa703bbe5a314cc89be9a07532bfab09f3c5dffac6348i0'),
                InscriptionId.fromString('d370be1b6bf74677c82226d7a0d65743cbe3846b9216e0ad207a7b03a5230ec3i0')
            ],
            prev: null,
            next: null,
            lowest: null,
            highest: null,
        };
        assert.deepStrictEqual(expectedInscriptionsJson, inscriptionsJson);
    });

    it("should get inscriptions from UTXO", async () => {
        const client = new DefaultOrdinalsClient("testnet");
        const outputJson = await client.getInscriptionsFromOutPoint({
            txid: "d370be1b6bf74677c82226d7a0d65743cbe3846b9216e0ad207a7b03a5230ec3",
            vout: 0
        });
        const expectedOutputJson: OutputJson = {
            value: 1967,
            script_pubkey: 'OP_PUSHNUM_1 OP_PUSHBYTES_32 24ad201633789999cbe4251018e796acb22ec5d1a6f8a1873adc6363e04d7e7d',
            address: 'tb1pyjkjq93n0zvenjlyy5gp3euk4jeza3w35mu2rpe6m33k8czd0e7s3ha8st',
            transaction: 'd370be1b6bf74677c82226d7a0d65743cbe3846b9216e0ad207a7b03a5230ec3',
            sat_ranges: null,
            inscriptions: [],
            runes: {}
        };
        assert.deepStrictEqual(expectedOutputJson, outputJson);
    });

    it("should get inscriptions from Sat", async () => {
        const client = new DefaultOrdinalsClient("testnet");
        const sat: number = 100;
        const satJson = await client.getInscriptionsFromSat(sat);
        const expectedSatJson = {
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
        assert.deepStrictEqual(expectedSatJson, satJson);
    });

    it("should get inscriptions from start block", async () => {
        const client = new DefaultOrdinalsClient("testnet");
        const startBlock: number = 2537138;
        const inscriptions = await client.getInscriptionsFromStartBlock(startBlock);
        // assert that inscriptions is not null or undefined
        assert.isNotNull(inscriptions);
    });
});
