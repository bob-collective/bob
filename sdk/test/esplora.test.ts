import { assert, describe, it } from 'vitest';
import { EsploraClient, Transaction, Block } from '../src/esplora';

describe('Esplora Tests', () => {
    it('should get block height', async () => {
        const client = new EsploraClient('testnet');
        const height = await client.getLatestHeight();
        assert(height > 0);
    });

    it('should get block', async () => {
        const client = new EsploraClient('testnet');
        const block = await client.getBlock('000000000933ea01ad0ee984209779baaec3ced90fa3f408719526f8d77f4943');
        const expectedBlock: Block = {
            id: '000000000933ea01ad0ee984209779baaec3ced90fa3f408719526f8d77f4943',
            height: 0,
            version: 1,
            timestamp: 1296688602,
            tx_count: 1,
            size: 285,
            weight: 1140,
            merkle_root: '4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b',
            previousblockhash: null,
            mediantime: 1296688602,
            nonce: 414098458,
            bits: 486604799,
            difficulty: 1,
        };

        for (const key in expectedBlock) {
            assert.equal(block[key], expectedBlock[key]);
        }
    });

    it('should get block hash', async () => {
        const client = new EsploraClient('testnet');
        const blockHash = await client.getBlockHash(0);
        assert.equal(blockHash, '000000000933ea01ad0ee984209779baaec3ced90fa3f408719526f8d77f4943');
    });

    it('should get block header', async () => {
        const client = new EsploraClient('testnet');
        const blockHeader = await client.getBlockHeader(
            '000000000933ea01ad0ee984209779baaec3ced90fa3f408719526f8d77f4943'
        );
        assert.equal(
            blockHeader,
            '0100000000000000000000000000000000000000000000000000000000000000000000003ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323a9fb8aa4b1e5e4adae5494dffff001d1aa4ae18'
        );
    });

    it('should get block header at certain height', async () => {
        const client = new EsploraClient('testnet');
        const blockHeader = await client.getBlockHeaderAt(10);
        assert.equal(
            blockHeader,
            '010000001e93aa99c8ff9749037d74a2207f299502fa81d56a4ea2ad5330ff50000000002ec2266c3249ce2e079059e0aec01a2d8d8306a468ad3f18f06051f2c3b1645435e9494dffff001d008918cf'
        );
    });

    it('should get transaction', async () => {
        const client = new EsploraClient('testnet');
        const tx = await client.getTransaction('4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b');
        const expectedTransaction: Transaction = {
            txid: '4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b',
            version: 1,
            locktime: 0,
            vin: [
                {
                    txid: '0000000000000000000000000000000000000000000000000000000000000000',
                    vout: 4294967295,
                    prevout: null,
                    scriptsig:
                        '04ffff001d0104455468652054696d65732030332f4a616e2f32303039204368616e63656c6c6f72206f6e206272696e6b206f66207365636f6e64206261696c6f757420666f722062616e6b73',
                    scriptsig_asm:
                        'OP_PUSHBYTES_4 ffff001d OP_PUSHBYTES_1 04 OP_PUSHBYTES_69 5468652054696d65732030332f4a616e2f32303039204368616e63656c6c6f72206f6e206272696e6b206f66207365636f6e64206261696c6f757420666f722062616e6b73',
                    is_coinbase: true,
                    sequence: 4294967295,
                },
            ],
            vout: [
                {
                    scriptpubkey:
                        '4104678afdb0fe5548271967f1a67130b7105cd6a828e03909a67962e0ea1f61deb649f6bc3f4cef38c4f35504e51ec112de5c384df7ba0b8d578a4c702b6bf11d5fac',
                    scriptpubkey_asm:
                        'OP_PUSHBYTES_65 04678afdb0fe5548271967f1a67130b7105cd6a828e03909a67962e0ea1f61deb649f6bc3f4cef38c4f35504e51ec112de5c384df7ba0b8d578a4c702b6bf11d5f OP_CHECKSIG',
                    scriptpubkey_type: 'p2pk',
                    value: 5000000000,
                },
            ],
            size: 204,
            weight: 816,
            fee: 0,
            status: {
                confirmed: true,
                block_height: 0,
                block_hash: '000000000933ea01ad0ee984209779baaec3ced90fa3f408719526f8d77f4943',
                block_time: 1296688602,
            },
        };
        assert.deepEqual(tx, expectedTransaction);
    });

    it('should get tx hex', async () => {
        const client = new EsploraClient('testnet');
        const txHex = await client.getTransactionHex(
            '4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b'
        );
        assert.equal(
            txHex,
            '01000000010000000000000000000000000000000000000000000000000000000000000000ffffffff4d04ffff001d0104455468652054696d65732030332f4a616e2f32303039204368616e63656c6c6f72206f6e206272696e6b206f66207365636f6e64206261696c6f757420666f722062616e6b73ffffffff0100f2052a01000000434104678afdb0fe5548271967f1a67130b7105cd6a828e03909a67962e0ea1f61deb649f6bc3f4cef38c4f35504e51ec112de5c384df7ba0b8d578a4c702b6bf11d5fac00000000'
        );
    });

    it('should serialize merkle proof', async () => {
        const client = new EsploraClient();
        const proof = await client.getMerkleProof('b61b0172d95e266c18aea0c624db987e971a5d6d4ebc2aaed85da4642d635735');

        assert.equal(
            proof.merkle,
            'ace8423f874c95f5f9042d7cda6b9f0727251f3059ef827f373a56831cc621a371db6dfce8daed1d809275' +
                'e0862441b3cdfd314eceea5a79ee7aeec69cc70f614082c8b474ccf00906a1e61694fdf0b717790ac3bdf850b36afb8df107ac' +
                'a93b96e7dea43442a944a6ab4f8bed0d25d3d372a836a6042375bc57fee5c5425f67a3920a489b23f9133fc84d7987d990acc7' +
                'c2569a81b547a5f65385856d90100e54ec14dd40c23c3cf1e61a2a16a03aea0e85d236942ad538262528d6748d20dc6ca7c40d' +
                '75ba7b782bc3d1302633c6def1531573c6420b99840ecffc0125f8e0f12ec4aa1d74fd5ec8d9a57c154267cb6ff0276835592c' +
                'b8500d8c3c5650e84b83e73e9094de0c2bdaa4d661a3b1adacfae0f3c0f8007ab1b2be8dbf32f073068979a263152d6c234ad0' +
                'f4b70f697168502d62ead0c0194bcf77321a85a1e127afc4477dcc3c3636a7818601d9ff43f837b15ef74d387c688fc0a45b79' +
                'aec0b6'
        );
    });

    it('should get fee rate', async () => {
        const client = new EsploraClient('mainnet');
        const feeRate = await client.getFeeEstimate(1);
        assert(feeRate > 0);
    });

    it('should get balance', async () => {
        const client = new EsploraClient('testnet');
        const balance = await client.getBalance('tb1qjhekcm565spvr0epqu5nvd9mhgwaafg6d0n2yw');
        assert.deepEqual(balance, {
            chain: 727499862,
            mempool: 0,
            total: 727499862,
        });
    });
});
