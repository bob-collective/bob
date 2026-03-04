import { assert, describe, it } from 'vitest';
import { EsploraClient, Transaction, Block } from '../src/esplora';

describe('Esplora Tests', () => {
    it('should get block height', async () => {
        const client = new EsploraClient('testnet4');
        const height = await client.getLatestHeight();
        assert(height > 0);
    });

    it('should get block', async () => {
        const client = new EsploraClient('testnet4');
        const block = await client.getBlock('00000000da84f2bafbbc53dee25a72ae507ff4914b867c565be350b0da8bf043');
        const expectedBlock: Block = {
            id: '00000000da84f2bafbbc53dee25a72ae507ff4914b867c565be350b0da8bf043',
            height: 0,
            version: 1,
            timestamp: 1714777860,
            tx_count: 1,
            size: 261,
            weight: 1044,
            merkle_root: '7aa0a7ae1e223414cb807e40cd57e667b718e42aaf9306db9102fe28912b7b4e',
            previousblockhash: null,
            mediantime: 1714777860,
            nonce: 393743547,
            bits: 486604799,
            difficulty: 1,
        };

        for (const key in expectedBlock) {
            assert.equal(
                block[key as keyof Block],
                expectedBlock[key as keyof Block],
                `Mismatch in block field: ${key}`
            );
        }
    });

    it('should get block hash', async () => {
        const client = new EsploraClient('testnet4');
        const blockHash = await client.getBlockHash(0);
        assert.equal(blockHash, '00000000da84f2bafbbc53dee25a72ae507ff4914b867c565be350b0da8bf043');
    });

    it('should get block header', async () => {
        const client = new EsploraClient('testnet4');
        const blockHeader = await client.getBlockHeader(
            '00000000da84f2bafbbc53dee25a72ae507ff4914b867c565be350b0da8bf043'
        );
        assert.equal(
            blockHeader,
            '0100000000000000000000000000000000000000000000000000000000000000000000004e7b2b9128fe0291db0693af2ae418b767e657cd407e80cb1434221eaea7a07a046f3566ffff001dbb0c7817'
        );
    });

    it('should get block header at certain height', async () => {
        const client = new EsploraClient('testnet4');
        const blockHeader = await client.getBlockHeaderAt(10);
        assert.equal(
            blockHeader,
            '00000020596cf1fc7312d155129d921d792a69d933c7101cd043dd5a5f8bcfb200000000265cfcc10ce1f8ba85fbe9ceb1ecc897ed81b2e4aa5ffe628ec20627abcbfbd49ed73866ffff001da3e98ead'
        );
    });

    it('should get transaction', async () => {
        const client = new EsploraClient('testnet4');
        const tx = await client.getTransaction('e7e45a66c587d8c653e0c6b824bdc768f9633f54f2723d96858daebde3938351');
        const expectedTransaction: Transaction = {
            txid: 'e7e45a66c587d8c653e0c6b824bdc768f9633f54f2723d96858daebde3938351',
            version: 1,
            locktime: 0,
            vin: [
                {
                    txid: 'bcd6d38fdfc02e018463913fbe12ba6a8eee3bb7869ffa3735ef63e433786f95',
                    vout: 21,
                    prevout: {
                        scriptpubkey: '001403a11ef572d484a988edf0b2976d7bc6306af37f',
                        scriptpubkey_asm: 'OP_0 OP_PUSHBYTES_20 03a11ef572d484a988edf0b2976d7bc6306af37f',
                        scriptpubkey_type: 'v0_p2wpkh',
                        scriptpubkey_address: 'tb1qqws3aatj6jz2nz8d7zefwmtmcccx4umlc5ygr7',
                        value: 10000000,
                    },
                    scriptsig: '',
                    scriptsig_asm: '',
                    witness: [
                        '3044022003fb4f0637dd445100a4ba725315539f86a2ea7732406eb72ab6da6ede1d44cd022043f43f637c68b0856e35f5bd6318f6fcdbff5a564d92606adc0a9548b249fbd901',
                        '020b3f09dd02d7321a19bfa739954264dd27b2209c54d28144698aa97fccc44d31',
                    ],
                    is_coinbase: false,
                    sequence: 4294967293,
                },
            ],
            vout: [
                {
                    scriptpubkey: '001403a11ef572d484a988edf0b2976d7bc6306af37f',
                    scriptpubkey_asm: 'OP_0 OP_PUSHBYTES_20 03a11ef572d484a988edf0b2976d7bc6306af37f',
                    scriptpubkey_type: 'v0_p2wpkh',
                    scriptpubkey_address: 'tb1qqws3aatj6jz2nz8d7zefwmtmcccx4umlc5ygr7',
                    value: 9918608,
                },
                {
                    scriptpubkey: '0014be7aff68a9f804d040d12c07a7fa2aa923e51b91',
                    scriptpubkey_asm: 'OP_0 OP_PUSHBYTES_20 be7aff68a9f804d040d12c07a7fa2aa923e51b91',
                    scriptpubkey_type: 'v0_p2wpkh',
                    scriptpubkey_address: 'tb1qhea0769flqzdqsx39sr607324y372xu35c733j',
                    value: 1093,
                },
            ],
            size: 222,
            weight: 561,
            fee: 80299,
            status: {
                confirmed: true,
                block_height: 124786,
                block_hash: '0000000000000002f4f2cf38e076d29aa0b5378024dd96834b66eefc1df7ca0f',
                block_time: 1772630858,
            },
        };
        assert.deepEqual(tx, expectedTransaction);
    });

    it('should get tx hex', async () => {
        const client = new EsploraClient('testnet4');
        const txHex = await client.getTransactionHex(
            'e7e45a66c587d8c653e0c6b824bdc768f9633f54f2723d96858daebde3938351'
        );
        assert.equal(
            txHex,
            '01000000000101956f7833e463ef3537fa9f86b73bee8e6aba12be3f916384012ec0df8fd3d6bc1500000000fdffffff02905897000000000016001403a11ef572d484a988edf0b2976d7bc6306af37f4504000000000000160014be7aff68a9f804d040d12c07a7fa2aa923e51b9102473044022003fb4f0637dd445100a4ba725315539f86a2ea7732406eb72ab6da6ede1d44cd022043f43f637c68b0856e35f5bd6318f6fcdbff5a564d92606adc0a9548b249fbd90121020b3f09dd02d7321a19bfa739954264dd27b2209c54d28144698aa97fccc44d3100000000'
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
        const client = new EsploraClient('testnet4');
        const balance = await client.getBalance('tb1qmmxta87spaeulfzkd9x2sz99km6r6q2qft2mzd');
        assert.deepEqual(balance, {
            confirmed: 1093,
            unconfirmed: 0,
            total: 1093,
        });
    });
});
