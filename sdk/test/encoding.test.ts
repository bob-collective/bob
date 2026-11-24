import { hex } from '@scure/base';
import { Transaction } from '@scure/btc-signer';
import nock from 'nock';
import { afterEach, describe, expect, it } from 'vitest';
import { EsploraClient, MAINNET_ESPLORA_BASE_PATH } from '../src/esplora';
import { ScureBitcoinSigner } from '../src/gateway/utils/bitcoin-signer';
import { getBitcoinTxInfo } from '../src/relay';
import { BtcAddressType, getInputFromUtxoAndTx } from '../src/wallet';

// Pins the byte outputs of code that used to rely on the Node `Buffer` global, so they stay identical
// in environments (browsers) that have no `Buffer`.

// https://blockstream.info/tx/10f746b824fcd844f4615daf5faa10105ef0a3ad24d583c00e350f6e981515dc (segwit, version 2)
const SEGWIT_TX_HEX =
    '02000000000104f6ffa6acd416ee27c6381598f9d1ace8f57bd5f997205fe9c10ca717a81208e20100000017160014d32ae97372f0b2549d26fa40e33f0699cfe86d55fffffffff6ffa6acd416ee27c6381598f9d1ace8f57bd5f997205fe9c10ca717a81208e20400000017160014d32ae97372f0b2549d26fa40e33f0699cfe86d55ffffffff500f4b6891bd86538937ca96508b6b34fd1eb8869e61a63448c1c7a5307c32fdbf00000000ffffffff248a0badd48bdc94ab7438cd8df43915af9e3ce334e4f3217d18564343ccbf510500000017160014d32ae97372f0b2549d26fa40e33f0699cfe86d55ffffffff07b00400000000000017a9147ecd91afdcadf6f1b9e8e026a312e4cce61e63ea872202000000000000225120173c179ec304311da634604e70958ac92f39325915ca5c0778061155c7735eb59a9d000000000000225120dca54ace6e977c3071e81ce4ee359c1c6ee003586434a29fca3632b346006e6ce80300000000000017a914ea6b832a05c6ca578baa3836f3f25553d41068a587580200000000000017a9147ecd91afdcadf6f1b9e8e026a312e4cce61e63ea87580200000000000017a9147ecd91afdcadf6f1b9e8e026a312e4cce61e63ea87a88f00000000000017a9147ecd91afdcadf6f1b9e8e026a312e4cce61e63ea870247304402200534ee640f73a3d2c03cdd67b3e55a8e9af73132bbfef97cdb6e240011e51892022075f2de763e423465659c741f9c73baccb158ae1fec5782075664383c87a79b4e012103b366c69e8237d9be7c4f1ac2a7abc6a79932fbf3de4e2f6c04797d7ef27abfe102473044022066cbcc462d3d30faebb927496ae21945d9a9cca3d26a9908809ff90bab1ad89802201877bad7327af6d2c8ebd6afe94990215285fe5dbf057604bb4a4cdabdfe1af7012103b366c69e8237d9be7c4f1ac2a7abc6a79932fbf3de4e2f6c04797d7ef27abfe10141a942ec1a6d845dbe9d0fa117cebcee4968458f400ccc0ef2f9a872aae043c068a7158637c3e372d5312f2bcaab2405438b48ceb56cf16ada3bf5d087d1d3dbf08302483045022100e5538a760f9cc8dd5d369f4dc48a4b4c858962da55c6f70b2c10fae90f052fef02204c3f5105f4c0b5c2bfb01d41a465d840e8e8863fff3eb37999c8ee2997efb30e012103b366c69e8237d9be7c4f1ac2a7abc6a79932fbf3de4e2f6c04797d7ef27abfe100000000';

// https://blockstream.info/tx/c061c23190ed3370ad5206769651eaf6fac6d87d85b5db34e30a74e0c4a6da3e (legacy, version 1)
const LEGACY_TX_HEX =
    '010000000160a941de6f9989628da24ede7730bb68990eac96da30379cec4da3cd3f1097cd010000008b4830450221009ee3ca47753ae5e4991358787ae660508222979493bd7d9b7191deff85d5e96102206a2d4b42a9d4afb3817d0e7a5b37c4d639eb16a52ad8c2d8c519f8802fff25920141049875dbe0f1f8ff9331b46630ad3fcb1c37c6516cf4b5e29d2ce0b0ae64935b40ba29e0669e9545768513a70bee36b699ca80cf37e245e24fb89907cb9a06f087ffffffff0226020000000000001976a91411dbe48cc6b617f9c6adaf4d9ed5f625b1c7cb5988acaa360b00000000001976a914a7158108fd0a615539a37122ed320784295ed1f388ac00000000';

/** Replaces the 4-byte version and locktime fields of a legacy (non-witness) transaction. */
const withVersionAndLocktime = (txHex: string, version: string, locktime: string) =>
    version + txHex.slice(8, -8) + locktime;

const esploraWithTx = (txHex: string) => {
    const client = new EsploraClient('mainnet');
    client.getTransactionHex = async () => txHex;
    return client;
};

describe('EsploraClient.getMerkleProof', () => {
    afterEach(() => {
        nock.cleanAll();
    });

    it('reverses each merkle hash to little-endian and concatenates them', async () => {
        const txId = 'b61b0172d95e266c18aea0c624db987e971a5d6d4ebc2aaed85da4642d635735';
        nock(MAINNET_ESPLORA_BASE_PATH)
            .get(`/tx/${txId}/merkle-proof`)
            .reply(200, {
                block_height: 840000,
                merkle: [
                    '00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff',
                    'a1c621cc83563a377f82ef59301f2527079f6bda7d2d04f9f5954c873f42e8ac',
                ],
                pos: 3,
            });

        const proof = await new EsploraClient('mainnet').getMerkleProof(txId);

        expect(proof).toEqual({
            blockHeight: 840000,
            merkle:
                'ffeeddccbbaa99887766554433221100ffeeddccbbaa99887766554433221100' +
                'ace8423f874c95f5f9042d7dda6b9f0727251f3059ef827f373a5683cc21c6a1',
            pos: 3,
        });
    });

    it('gives lower-case hex for an upper-case hash', async () => {
        const txId = '11'.repeat(32);
        nock(MAINNET_ESPLORA_BASE_PATH)
            .get(`/tx/${txId}/merkle-proof`)
            .reply(200, {
                block_height: 1,
                merkle: ['00112233445566778899AABBCCDDEEFF00112233445566778899AABBCCDDEEFF'],
                pos: 1,
            });

        const proof = await new EsploraClient('mainnet').getMerkleProof(txId);

        expect(proof.merkle).toBe('ffeeddccbbaa99887766554433221100ffeeddccbbaa99887766554433221100');
    });

    it('gives an empty proof for a transaction alone in its block', async () => {
        const txId = '00'.repeat(32);
        nock(MAINNET_ESPLORA_BASE_PATH)
            .get(`/tx/${txId}/merkle-proof`)
            .reply(200, { block_height: 1, merkle: [], pos: 0 });

        const proof = await new EsploraClient('mainnet').getMerkleProof(txId);

        expect(proof.merkle).toBe('');
    });
});

describe('getBitcoinTxInfo', () => {
    it('encodes version and locktime of a segwit transaction as 4-byte little-endian hex', async () => {
        const info = await getBitcoinTxInfo(esploraWithTx(SEGWIT_TX_HEX), 'unused');

        expect(info.version).toBe('02000000');
        expect(info.locktime).toBe('00000000');
        expect(info.witnessVector).toBeUndefined();
    });

    it('encodes version and locktime of a legacy transaction', async () => {
        const info = await getBitcoinTxInfo(esploraWithTx(LEGACY_TX_HEX), 'unused');

        expect(info).toEqual({
            version: '01000000',
            inputVector:
                '0160a941de6f9989628da24ede7730bb68990eac96da30379cec4da3cd3f1097cd010000008b4830450221009ee3ca47753ae5e4991358787ae660508222979493bd7d9b7191deff85d5e96102206a2d4b42a9d4afb3817d0e7a5b37c4d639eb16a52ad8c2d8c519f8802fff25920141049875dbe0f1f8ff9331b46630ad3fcb1c37c6516cf4b5e29d2ce0b0ae64935b40ba29e0669e9545768513a70bee36b699ca80cf37e245e24fb89907cb9a06f087ffffffff',
            outputVector:
                '0226020000000000001976a91411dbe48cc6b617f9c6adaf4d9ed5f625b1c7cb5988acaa360b00000000001976a914a7158108fd0a615539a37122ed320784295ed1f388ac',
            locktime: '00000000',
            witnessVector: undefined,
        });
    });

    it('keeps multi-byte and negative values byte-identical', async () => {
        const txHex = withVersionAndLocktime(LEGACY_TX_HEX, 'feffffff', '78563412');
        const info = await getBitcoinTxInfo(esploraWithTx(txHex), 'unused');

        expect(info.version).toBe('feffffff');
        expect(info.locktime).toBe('78563412');
    });

    it.each([
        ['ffffffff', 0xffffffff],
        ['00000080', 0x80000000],
    ])('encodes the unsigned locktime %s (%i) above the signed 32-bit range', async (locktime) => {
        const txHex = withVersionAndLocktime(LEGACY_TX_HEX, '01000000', locktime);
        const info = await getBitcoinTxInfo(esploraWithTx(txHex), 'unused');

        expect(info.version).toBe('01000000');
        expect(info.locktime).toBe(locktime);
    });
});

describe('ScureBitcoinSigner', () => {
    // BIP84 test vector: https://github.com/bitcoin/bips/blob/master/bip-0084.mediawiki#test-vectors
    const MNEMONIC = 'abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about';
    const WIF = 'KyZpNDKnfs94vbrwhJneDi77V6jF64PWPF8x5cdJb8ifgg2DUc9d';
    const ADDRESS = 'bc1qcr8te4kr609gcawutmrza0j4xv80jy8z306fyu';
    const PRIVATE_KEY_HEX = '4604b4b710fe91f584fff084e1a9159fe4f8408fff380596a604948474ce4fa3';

    it('derives the BIP84 vector from a seed phrase', async () => {
        const signer = await ScureBitcoinSigner.fromSeedPhrase(MNEMONIC, "m/84'/0'/0'/0/0");

        expect(await signer.getP2WPKHAddress()).toBe(ADDRESS);
    });

    it('decodes the same key from WIF, hex, 0x-hex and upper-case hex', async () => {
        for (const signer of [
            ScureBitcoinSigner.fromWIF(WIF),
            ScureBitcoinSigner.fromKey(WIF),
            ScureBitcoinSigner.fromKey(PRIVATE_KEY_HEX),
            ScureBitcoinSigner.fromKey(`0x${PRIVATE_KEY_HEX}`),
            ScureBitcoinSigner.fromKey(PRIVATE_KEY_HEX.toUpperCase()),
            new ScureBitcoinSigner(PRIVATE_KEY_HEX),
            new ScureBitcoinSigner(`0x${PRIVATE_KEY_HEX}`),
        ]) {
            expect(await signer.getP2WPKHAddress()).toBe(ADDRESS);
        }
    });

    it('rejects a key that is neither hex nor WIF', () => {
        expect(() => ScureBitcoinSigner.fromKey('not a key')).toThrow('Invalid private key');
    });
});

describe('getInputFromUtxoAndTx', () => {
    const utxo = (txid: string, vout: number) => ({ txid, vout, value: 0, confirmed: true });
    const PUBLIC_KEY = '03b366c69e8237d9be7c4f1ac2a7abc6a79932fbf3de4e2f6c04797d7ef27abfe1';

    const encode = (input: object) =>
        Object.fromEntries(
            Object.entries(input).map(([key, value]) => [
                key,
                value instanceof Uint8Array
                    ? hex.encode(value)
                    : key === 'witnessUtxo'
                      ? { script: hex.encode(value.script), amount: value.amount }
                      : value,
            ])
        );

    it('adds the p2sh-p2wpkh redeem script and both utxo forms for a segwit transaction', () => {
        const txid = '10f746b824fcd844f4615daf5faa10105ef0a3ad24d583c00e350f6e981515dc';
        const input = getInputFromUtxoAndTx(
            'mainnet',
            utxo(txid, 0),
            Transaction.fromRaw(hex.decode(SEGWIT_TX_HEX), { allowUnknownOutputs: true }),
            BtcAddressType.p2sh,
            PUBLIC_KEY
        );

        expect(encode(input)).toEqual({
            txid,
            index: 0,
            type: 'sh',
            script: 'a9147ecd91afdcadf6f1b9e8e026a312e4cce61e63ea87',
            address: '3DFVKuT9Ft4rWpysAZ1bHpg55EBy1HVPcr',
            hash: '7ecd91afdcadf6f1b9e8e026a312e4cce61e63ea',
            redeemScript: '0014d32ae97372f0b2549d26fa40e33f0699cfe86d55',
            witnessUtxo: { script: 'a9147ecd91afdcadf6f1b9e8e026a312e4cce61e63ea87', amount: 1200n },
            nonWitnessUtxo: SEGWIT_TX_HEX,
        });
    });

    it('adds the taproot internal key derived from the public key', () => {
        const txid = '10f746b824fcd844f4615daf5faa10105ef0a3ad24d583c00e350f6e981515dc';
        const input = getInputFromUtxoAndTx(
            'mainnet',
            utxo(txid, 1),
            Transaction.fromRaw(hex.decode(SEGWIT_TX_HEX), { allowUnknownOutputs: true }),
            BtcAddressType.p2tr,
            PUBLIC_KEY
        );

        expect(input.txid).toBe(txid);
        expect(hex.encode((input as { tapInternalKey: Uint8Array }).tapInternalKey)).toBe(PUBLIC_KEY.slice(2));
        expect(hex.encode(input.nonWitnessUtxo!)).toBe(SEGWIT_TX_HEX);
    });

    it('gives only the full previous transaction for a legacy transaction', () => {
        const txid = 'c061c23190ed3370ad5206769651eaf6fac6d87d85b5db34e30a74e0c4a6da3e';
        const input = getInputFromUtxoAndTx(
            'mainnet',
            utxo(txid, 0),
            Transaction.fromRaw(hex.decode(LEGACY_TX_HEX), { allowUnknownOutputs: true }),
            BtcAddressType.p2pkh
        );

        expect(encode(input)).toEqual({ txid, index: 0, nonWitnessUtxo: LEGACY_TX_HEX });
    });
});
