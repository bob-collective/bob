// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console2} from "forge-std/Script.sol";
import "../src/paymasters/OracleTokenPaymaster.sol";
import {DummyOracle} from "../src/paymasters/Oracle.sol";
// import {IERC2771} from "../lib/gsn/packages/contracts/src/interfaces/IERC2771Recipient.sol";
import {IRelayHub} from "../lib/gsn/packages/contracts/src/interfaces/IRelayHub.sol";
import {TestingWbtc} from "../src/TestingErc20.sol";

contract TestingWbtcScript is Script {
    function setUp() public {}

    function run() public {
    
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address forwarder = vm.envAddress("FORWARDER_ADDRESS");
        int tokenPrice = vm.envInt("ERC_PRICE");
        // address payable mtPaymaster = payable(vm.envAddress("MT_PAYMASTER_ADDRESS"));

        vm.startBroadcast(deployerPrivateKey);

        TestingWbtc token = new TestingWbtc("Wrapped BTC", "WBTC");
        token.setTrustedForwarder(forwarder);

        DummyOracle oracle = new DummyOracle();
        oracle.setPrice(tokenPrice);

        // OracleTokenPaymaster paymaster = OracleTokenPaymaster(mtPaymaster);
        // mtPaymaster.addOracle(token, token.decimals(), oracle);

        vm.stopBroadcast();
    }
}
