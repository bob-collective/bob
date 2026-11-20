import { hex } from '@scure/base';
import { Address, Script, TEST_NETWORK } from '@scure/btc-signer';
import { assert, describe, it } from 'vitest';
import { EsploraClient } from '../src/esplora';
import { getBitcoinTxInfo, getBitcoinTxProof } from '../src/relay';
import { encodeRawOutput } from '../src/utils';

describe.skip('Relay Tests', () => {
    // doesn't do anything, but left here to build more contract tests
    it('should encode output and op return', async () => {
        const decoded = Address({ ...TEST_NETWORK, bech32: 'bcrt' }).decode(
            'bcrt1qv9pt88qqwdnjmsuzhzdy9v57qcmghj4alhdg5y'
        );
        if (decoded.type !== 'wpkh') throw new Error('Expected a P2WPKH address');
        // The witness program, as bitcoinjs-lib's `address.fromBech32(...).data` returned it.
        const output = Buffer.from(decoded.hash);
        console.log(Buffer.concat([Buffer.from([output.length]), output]).toString('hex'));
        const outs = [
            { script: output, value: 15000 },
            {
                script: Script.encode(['RETURN', hex.decode('675Ca18A04027fd50C88CcD03939E0e5C97b795f'.toLowerCase())]),
                value: 0,
            },
        ];
        console.log(encodeRawOutput({ outs }).toString('hex'));
    });

    it('should get tx info', async () => {
        const client = new EsploraClient();
        const txId = '2ef69769cc0ee81141c79552de6b91f372ff886216dbfa84e5497a16b0173e79';
        const txInfo = await getBitcoinTxInfo(client, txId);
        assert.deepEqual(txInfo, {
            version: '01000000',
            inputVector:
                '01996cf4e2f0016a1f092aaaba653c7eae5dd4b6eef1f9a2a94c64f34b2fecbd85010000006a47304402206f99da49ce586528ed8981842df30b4a5a91195fd2d83e440d4193fc16a944ec022055cfdf63a2c90638821f1b5ff1fdf77526163ae057a0d0de30a6e1d3009e7a29012102811832eef7216470f489991f1d87e36d2890755d2bbf827eb1e71804491506afffffffff',
            outputVector:
                '0200e9a435000000001976a914fd7e6999cd7e7114383e014b7e612a88ab6be68f88ac804a5d05000000001976a9145c1addbd0e4e78479e71fdca0555d2d44b67378e88ac',
            locktime: '00000000',
            witnessVector: undefined,
        });
    });

    it('should get tx proof', async () => {
        const client = new EsploraClient();
        const txId = '2ef69769cc0ee81141c79552de6b91f372ff886216dbfa84e5497a16b0173e79';
        const txProof = await getBitcoinTxProof(client, txId, 2);
        assert.equal(txProof.txIndexInBlock, 1);
        assert.equal(Buffer.from(txProof.bitcoinHeaders, 'hex').byteLength / 80, 2);
    });
});
