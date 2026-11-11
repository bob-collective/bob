import { Block, opcodes, script, Transaction } from 'bitcoinjs-lib';
import { assert, describe, it } from 'vitest';
import { MAINNET_ESPLORA_BASE_PATH } from '../src/esplora';
import { encodeRawInput, encodeRawOutput, encodeRawWitness, getMerkleProof } from '../src/utils';

// Two inputs, two outputs; with witness, the second input's 300-byte item needs a 3-byte varint length.
function buildTx(seed: number, withWitness: boolean): Transaction {
    const tx = new Transaction();
    tx.version = 2;
    tx.locktime = seed;
    tx.addInput(Buffer.alloc(32, seed), seed, 0xfffffffd, Buffer.from([seed, seed + 1]));
    tx.addInput(Buffer.alloc(32, seed + 1), 1, 0xffffffff, Buffer.alloc(0));
    if (withWitness) {
        tx.setWitness(0, [Buffer.alloc(72, 0xaa), Buffer.alloc(33, 0x02)]);
        tx.setWitness(1, [Buffer.alloc(300, 0xbb)]);
    }
    tx.addOutput(Buffer.from('0014' + '11'.repeat(20), 'hex'), 2_100_000_000_000_000n);
    tx.addOutput(script.compile([opcodes.OP_RETURN, Buffer.alloc(20, 0x67)]), 0n);
    return tx;
}

describe('Utils Tests', () => {
    // Expected values were produced by the bitcoinjs-lib v6 implementation of these helpers.
    it('should encode raw input, output and witness vectors', () => {
        const tx = buildTx(5, true);

        assert.equal(
            encodeRawInput(tx).toString('hex'),
            '02' +
                '05'.repeat(32) +
                '05000000' +
                '020506' +
                'fdffffff' +
                '06'.repeat(32) +
                '01000000' +
                '00' +
                'ffffffff'
        );
        assert.equal(
            encodeRawOutput(tx).toString('hex'),
            '02' + '0040075af0750700' + '160014' + '11'.repeat(20) + '0000000000000000' + '166a14' + '67'.repeat(20)
        );
        assert.equal(
            encodeRawWitness(tx).toString('hex'),
            '02' + '48' + 'aa'.repeat(72) + '21' + '02'.repeat(33) + '01' + 'fd2c01' + 'bb'.repeat(300)
        );
    });

    it('should construct merkle proofs for a block with an odd number of transactions', () => {
        const block = new Block();
        block.transactions = [0, 1, 2, 3, 4].map((i) => buildTx(10 + i, i % 2 === 0));

        const txId = Buffer.from(block.transactions[4].getHash()).toString('hex');
        assert.deepEqual(getMerkleProof(block, txId), {
            pos: 4,
            proof: '8c0dc768e05f685cac769198d38b3ae423b76e33998229fd03196b565da93299f073a61f1901214f4d1f6cfc65a5d6e95a45474117178b5f1d8daac608c785eb7417a32d27f4c29ec62a22247e8b854fc5be7d29487705690fd16a3455b3f9a1',
            root: '6e11ba94874d1e4eabf6634951b6036a7d1623b7446968eeb57c7422e1e437db',
        });

        const wTxId = Buffer.from(block.transactions[2].getHash(true)).toString('hex');
        assert.deepEqual(getMerkleProof(block, wTxId, true), {
            pos: 2,
            proof: '59b83489bafc173cfa7c7493362e8d4243bacd885ae1a094a62809a6575d87c27f87a5f54162d270df8e6b2e3cdab395922e7945c7ceebe3079c9c1f0dc7d8aa12f1693df910cc5ff92b821fa16f6e700e6fc2ff59a4181c5987b6dc38f80d6a',
            root: '0853055db814372f54de3fc2e623bf6c96e5169ab4f5e5d5fc87fd7dc7dd72be',
        });
    });

    // NOTE: this is a bit flaky due to slow response times from electrs
    it('should construct witness merkle proof from block', { timeout: 20000, skip: true }, async () => {
        const hash = '000000000000000000015712838394aeb93f5d45d0e5bec197382c08b375016e';
        const response = await fetch(`${MAINNET_ESPLORA_BASE_PATH}/block/${hash}/raw`);
        const blob = await response.blob();
        const buffer = Buffer.from(await blob.arrayBuffer());
        const block = Block.fromBuffer(buffer);
        const wTxId = '6b374690d8e2dbca4187f443cddd293536400d431f43a643b263ce59c4f9a3eb';
        const merkleProof = getMerkleProof(block, wTxId, true);
        assert.deepEqual(merkleProof, {
            pos: 407,
            proof: '6034ddf453f5dd20de449b29b1221dede67ccae56f00528e0767e2ab506db31c4d2946e88f7efa3e94bb17bbd10f3f44172b59c48f2eb6bd7f67a88d149373ee4082c8b474ccf00906a1e61694fdf0b717790ac3bdf850b36afb8df107aca93b7c3c4f91ddf49c7f74244336c5833377d40760ae09dd1fba83063ace480f94cca3920a489b23f9133fc84d7987d990acc7c2569a81b547a5f65385856d90100e84878b4f305a3909a9420293cdc741109864c9338ea326449a7a303b227f2b10490bc4343355e1a391f51c42918a894c2980012cca5ffd4b56a6702abd98497802de83f5889b2ad5bd157762a58505948f32f42b9fa886c93bf30fef6144a64666843a28ef13184f9e7ac3c34b5741f58c8895a0167f496e0157e7d0a97f4041f97b8df4d8aee81d20d0d062ed3ee0f9b0afb196bdf5373712883cacdfd8349b739c0e6e41d650d05727ea5faec197bfa563d19b0150fba718ba1981aea9ef90',
            root: '7cee5e99c8f0fc25fb115b7d7d00befca61f59a8544adaf3980f52132baf61ae',
        });
    });
});
