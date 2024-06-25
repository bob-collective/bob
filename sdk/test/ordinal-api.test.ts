import {
    DefaultOrdinalsClient,
    InscriptionJson,
    OutputJson,
    InscriptionId,
    SatPoint,
} from '../src/ordinal-api';
import { assert, describe, it } from "vitest";

describe("Ordinal API Tests", () => {
    it("should get inscription from id", async () => {
        const client = new DefaultOrdinalsClient("mainnet");
        // Deploy ORDI - BRC20
        const inscriptionJson = await client.getInscriptionFromId({
            txid: "b61b0172d95e266c18aea0c624db987e971a5d6d4ebc2aaed85da4642d635735",
            index: 0,
        });

        const expectedInscriptionJson: InscriptionJson<InscriptionId, SatPoint> = {
            address: 'bc1pxaneaf3w4d27hl2y93fuft2xk6m4u3wc4rafevc6slgd7f5tq2dqyfgy06',
            charms: [],
            children: [],
            content_length: 94,
            content_type: 'text/plain;charset=utf-8',
            effective_content_type: "text/plain;charset=utf-8",
            fee: 4830,
            height: 779832,
            id: InscriptionId.fromString('b61b0172d95e266c18aea0c624db987e971a5d6d4ebc2aaed85da4642d635735i0'),
            number: 348020,
            next: InscriptionId.fromString('693bd98380ad6e58f83de6068c236c6eb9d629c825cc3342c2d93f24c6762c6di0'),
            parent: null,
            parents: [],
            previous: InscriptionId.fromString('4f0ff6259efa9d56b16664e6c5c9755c148818dc6bbca98f7f9166b277e4b7c0i0'),
            rune: null,
            sat: null,
            satpoint: SatPoint.fromString('b61b0172d95e266c18aea0c624db987e971a5d6d4ebc2aaed85da4642d635735:0:0'),
            timestamp: 1678248991,
            value: 10000,
        };

        assert.deepStrictEqual(inscriptionJson, expectedInscriptionJson);
    });

    it("should get inscriptions", { timeout: 10000 }, async () => {
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
            ids: [
                InscriptionId.fromString('4d8e7ad2b410eaa79e3aa703bbe5a314cc89be9a07532bfab09f3c5dffac6348i0'),
                InscriptionId.fromString('d370be1b6bf74677c82226d7a0d65743cbe3846b9216e0ad207a7b03a5230ec3i0')
            ],
            more: false,
            page_index: 0,
        };
        assert.deepStrictEqual(inscriptionsJson, expectedInscriptionsJson);
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
            runes: {},
            indexed: false,
            spent: true
        };
        assert.deepStrictEqual(outputJson, expectedOutputJson);
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
            charms: [],
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
        assert.deepStrictEqual(satJson, expectedSatJson);
    });

    it("should get inscriptions from start block", { timeout: 10000 }, async () => {
        const client = new DefaultOrdinalsClient("testnet");
        const startBlock: number = 2537138;
        const inscriptions = await client.getInscriptionsFromStartBlock(startBlock);
        // assert that inscriptions is not null or undefined
        assert.isNotNull(inscriptions);
    });
});
