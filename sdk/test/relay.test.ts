import { assert } from "chai";
import { DefaultElectrsClient } from "../src/electrs";
import { getBitcoinTxInfo, getBitcoinTxProof } from "../src/relay";

describe("Relay Tests", () => {
    it("should get tx info", async () => {
        const client = new DefaultElectrsClient();
        const txId = "2ef69769cc0ee81141c79552de6b91f372ff886216dbfa84e5497a16b0173e79";
        const txInfo = await getBitcoinTxInfo(client, txId);
        assert.deepEqual(txInfo, {
            version: '01000000',
            inputVector: '01996cf4e2f0016a1f092aaaba653c7eae5dd4b6eef1f9a2a94c64f34b2fecbd85010000006a47304402206f99da49ce586528ed8981842df30b4a5a91195fd2d83e440d4193fc16a944ec022055cfdf63a2c90638821f1b5ff1fdf77526163ae057a0d0de30a6e1d3009e7a29012102811832eef7216470f489991f1d87e36d2890755d2bbf827eb1e71804491506afffffffff',
            outputVector: '0200e9a435000000001976a914fd7e6999cd7e7114383e014b7e612a88ab6be68f88ac804a5d05000000001976a9145c1addbd0e4e78479e71fdca0555d2d44b67378e88ac',
            locktime: '00000000',
            witnessVector: undefined
        });
    });

    it("should get tx proof", async () => {
        const client = new DefaultElectrsClient();
        const txId = "2ef69769cc0ee81141c79552de6b91f372ff886216dbfa84e5497a16b0173e79";
        const txProof = await getBitcoinTxProof(client, txId, 2);
        assert.equal(txProof.txIndexInBlock, 1);
        assert.equal(Buffer.from(txProof.bitcoinHeaders, "hex").byteLength / 80, 2);
    });
});
