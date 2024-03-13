// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console2} from "forge-std/Script.sol";
import {LightRelay} from "../src/relay/LightRelay.sol";

contract RelayScript is Script {
    function setUp() public {}

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");

        bytes memory genesisHeader = vm.envBytes("GENESIS_HEADER");
        uint256 genesisHeight = vm.envUint("GENESIS_HEIGHT");
        bytes memory retargetHeaders = vm.envBytes("RETARGET_HEADERS");

        vm.startBroadcast(deployerPrivateKey);
        LightRelay relay = new LightRelay();

        // Initialize relay at the given block
        relay.genesis(genesisHeader, genesisHeight, 1);

        // submit retarget
        relay.retarget(retargetHeaders);

        vm.stopBroadcast();
    }
}
