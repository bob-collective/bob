// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console2} from "forge-std/Script.sol";
import {BtcMarketPlace} from "../src/swap/Btc_Marketplace.sol";
import {MarketPlace} from "../src/swap/Marketplace.sol";
import {DummyRelay} from "../src/relay/DummyRelay.sol";

contract MarketplaceScript is Script {
    function setUp() public {}

    function run() public {
    
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address forwarder = vm.envAddress("FORWARDER_ADDRESS");

        vm.startBroadcast(deployerPrivateKey);
        BtcMarketPlace btcMarketplace = new BtcMarketPlace(new DummyRelay(), forwarder);
        MarketPlace marketplace = new MarketPlace(forwarder);

        vm.stopBroadcast();
    }
}
