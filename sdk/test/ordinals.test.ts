import { assert } from "chai";
import * as ecc from "tiny-secp256k1";
import * as ECPairFactory from "ecpair";
import { RemoteSigner, inscribeText } from "../src/ordinals";
import { Network, Psbt, Transaction, address, initEccLib } from "bitcoinjs-lib";
import { bitcoin } from "bitcoinjs-lib/src/networks";

const ECPair = ECPairFactory.default(ecc);
initEccLib(ecc);

class StaticSigner implements RemoteSigner {
    keyPair: ECPairFactory.ECPairInterface;

    constructor(secret: string) {
        const privateKey = Buffer.from(secret, "hex");
        this.keyPair = ECPair.fromPrivateKey(privateKey);
    }

    async getNetwork(): Promise<Network> {
        return bitcoin;
    }

    async getPublicKey(): Promise<string> {
        return this.keyPair.publicKey.toString("hex");
    }

    async sendToAddress(toAddress: string, amount: number): Promise<string> {
        const tx = new Transaction();
        tx.addOutput(address.toOutputScript(toAddress), amount);
        return tx.getId();
    }

    async getUtxoIndex(_toAddress: string, _txId: string): Promise<number> {
        return 0;
    }

    async signPsbt(inputIndex: number, psbt: Psbt): Promise<Psbt> {
        psbt.signInput(inputIndex, this.keyPair);
        return psbt;
    }
}

describe("Ordinal Tests", () => {
    it("should inscribe text", async () => {
        const secret = "fc7458de3d5616e7803fdc81d688b9642641be32fee74c4558ce680cac3d4111";
        const signer = new StaticSigner(secret);
        const toAddress = "bc1pxaneaf3w4d27hl2y93fuft2xk6m4u3wc4rafevc6slgd7f5tq2dqyfgy06";
        const tx = await inscribeText(signer, toAddress, 1, "Hello World!", 546);
        assert(tx.getId() == "5fbafffdccaecb857c2a405c4e9bb54094b10c2c3cc548222bb57d25a3f69b82");
    });
});
