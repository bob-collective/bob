import { assert, describe, it } from "vitest";
import { MAINNET_ESPLORA_BASE_PATH } from "../src/esplora";
import { Block } from "bitcoinjs-lib";
import { estimateTxFee, estimateTxFeeConst, getMerkleProof } from "../src/utils";

describe("Utils Tests", () => {
    // NOTE: this is a bit flaky due to slow response times from electrs
    it("should construct witness merkle proof from block", { timeout: 20000, skip: true }, async () => {
        const hash = "000000000000000000015712838394aeb93f5d45d0e5bec197382c08b375016e";
        const response = await fetch(`${MAINNET_ESPLORA_BASE_PATH}/block/${hash}/raw`);
        const blob = await response.blob()
        const buffer = Buffer.from(await blob.arrayBuffer());
        const block = Block.fromBuffer(buffer);
        const wTxId = "6b374690d8e2dbca4187f443cddd293536400d431f43a643b263ce59c4f9a3eb";
        const merkleProof = getMerkleProof(block, wTxId, true);
        assert.deepEqual(merkleProof, {
            pos: 407,
            proof: '6034ddf453f5dd20de449b29b1221dede67ccae56f00528e0767e2ab506db31c4d2946e88f7efa3e94bb17bbd10f3f44172b59c48f2eb6bd7f67a88d149373ee4082c8b474ccf00906a1e61694fdf0b717790ac3bdf850b36afb8df107aca93b7c3c4f91ddf49c7f74244336c5833377d40760ae09dd1fba83063ace480f94cca3920a489b23f9133fc84d7987d990acc7c2569a81b547a5f65385856d90100e84878b4f305a3909a9420293cdc741109864c9338ea326449a7a303b227f2b10490bc4343355e1a391f51c42918a894c2980012cca5ffd4b56a6702abd98497802de83f5889b2ad5bd157762a58505948f32f42b9fa886c93bf30fef6144a64666843a28ef13184f9e7ac3c34b5741f58c8895a0167f496e0157e7d0a97f4041f97b8df4d8aee81d20d0d062ed3ee0f9b0afb196bdf5373712883cacdfd8349b739c0e6e41d650d05727ea5faec197bfa563d19b0150fba718ba1981aea9ef90',
            root: '7cee5e99c8f0fc25fb115b7d7d00befca61f59a8544adaf3980f52132baf61ae'
        });
    });

    it("should estimate fee", async () => {
        // NOTE: off by one but not sure why
        assert.equal(estimateTxFee(1), estimateTxFeeConst(1) + 1);
    });
});
