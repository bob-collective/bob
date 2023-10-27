import { assert } from "chai";
import { DefaultElectrsClient } from "../src/electrs";

describe("Electrs Tests", () => {
    it("should get fee rate", async () => {
        const client = new DefaultElectrsClient("testnet");
        const feeRate = await client.getFeeEstimate(1);
        assert(feeRate == 1);
    });

    it("should serialize merkle proof", async () => {
        const client = new DefaultElectrsClient();
        const proof = await client.getMerkleProof("b61b0172d95e266c18aea0c624db987e971a5d6d4ebc2aaed85da4642d635735");

        assert.equal(proof.merkle, "ace8423f874c95f5f9042d7cda6b9f0727251f3059ef827f373a56831cc621a371db6dfce8daed1d809275" +
            "e0862441b3cdfd314eceea5a79ee7aeec69cc70f614082c8b474ccf00906a1e61694fdf0b717790ac3bdf850b36afb8df107ac" +
            "a93b96e7dea43442a944a6ab4f8bed0d25d3d372a836a6042375bc57fee5c5425f67a3920a489b23f9133fc84d7987d990acc7" +
            "c2569a81b547a5f65385856d90100e54ec14dd40c23c3cf1e61a2a16a03aea0e85d236942ad538262528d6748d20dc6ca7c40d" +
            "75ba7b782bc3d1302633c6def1531573c6420b99840ecffc0125f8e0f12ec4aa1d74fd5ec8d9a57c154267cb6ff0276835592c" +
            "b8500d8c3c5650e84b83e73e9094de0c2bdaa4d661a3b1adacfae0f3c0f8007ab1b2be8dbf32f073068979a263152d6c234ad0" +
            "f4b70f697168502d62ead0c0194bcf77321a85a1e127afc4477dcc3c3636a7818601d9ff43f837b15ef74d387c688fc0a45b79" +
            "aec0b6");
    });
});
