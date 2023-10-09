// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.17;

import {Test, console2} from "forge-std/Test.sol";
import {LightRelay} from "../src/relay/LightRelay.sol";

contract LightRelayTest is Test {
    LightRelay public relay;

    struct Header { 
        bytes32 digest;
        bytes data;
        uint256 height;
    }

    Header genesis;
    bytes32 periodStart;

    Header[3] headers;

    constructor() public {
        genesis = Header({
            digest: hex"00000000000000000021ce9ccc1691e25066eb388be7821a4906dbee4b611546",
            data: hex"00000020db62962b5989325f30f357762ae456b2ec340432278e14000000000000000000d1dd4e30908c361dfeabfb1e560281c1a270bde3c8719dbda7c848005317594440bf615c886f2e17bd6b082d",
            height: 562621
        });
        periodStart = hex"5204b3afd5c0dc010de8eeb28925b97d4b38a16b1d020f000000000000000000";

        headers[0] = Header({
            digest: hex"0000000000000000000f498a3b8394685a70ba0b0d8cb27850b1f499a380b5b8",
            data: hex"000000204615614beedb06491a82e78b38eb6650e29116cc9cce21000000000000000000b034884fc285ff1acc861af67be0d87f5a610daa459d75a58503a01febcc287a34c0615c886f2e17046e7325",
            height: 562622
        });
        headers[1] = Header({
            digest: hex"0000000000000000002066539e823973bbd8174973a5af1634ab56f685a949f5",
            data: hex"00000020b8b580a399f4b15078b28c0d0bba705a6894833b8a490f000000000000000000b16c32aa36d3b70749e7febbb9e733321530cc9a390ccb62dfb78e3955859d4c44c0615c886f2e1744ea7cc4",
            height: 562623
        });
        headers[2] = Header({
            digest: hex"00000000000000000014c9dc88648827978218b292dbdf0aa92aaf28aef9ff8b",
            data: hex"00000020f549a985f656ab3416afa5734917d8bb7339829e536620000000000000000000af9c9fe22494c39cf382b5c8dcef91f079ad84cb9838387aaa17948fbf25753430c2615c886f2e170a654758",
            height: 562624
        });
    }


    function setUp() public {
        relay = new LightRelay();
        relay.genesis(
            genesis.data,
            genesis.height,
            1
        );
    }

    function test_Retarget() public {
        // relay.addHeaders(genesis.data, abi.encodePacked(
        //     headers[0].data,
        //     headers[1].data,
        //     headers[2].data
        // ));
        // assertEq(relay.getBestKnownDigest(), headers[2].digest);
    }
}
