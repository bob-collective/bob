// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console2} from "forge-std/Script.sol";
import "../src/relay/LightRelay.sol";

contract BridgeScript is Script {
    function setUp() public {}

    function run() public {
        bytes memory data =
            hex"04000000473ed7b7ef2fce828c318fd5e5868344a5356c9e93b6040400000000000000004409cae5b7b2f8f18ea55f558c9bfa7c5f4778a1a53172a48fc57e172d0ed3d264c5eb56c3a40618af9bc1c7";
        uint256 height = 403200;

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
