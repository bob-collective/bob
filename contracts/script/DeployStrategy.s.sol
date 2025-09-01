// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import "forge-std/Script.sol";

import {SolvBTCJUPStrategy, ISolvBTCRouterV2, ISolvBTCRouterV1} from "../src/gateway/strategy/SolvStrategy.sol";
import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";

contract DeployStrategyScript is Script {
    address public deployer;

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");

        vm.startBroadcast(deployerPrivateKey);

        ISolvBTCRouterV2 solvBTCRouterV2 = ISolvBTCRouterV2(0x56a4d805d7A292f03Ead5Be31E0fFB8f7d0E3B48);
        ISolvBTCRouterV1 solvBTCRouterV1 = ISolvBTCRouterV1(0xbA46FcC16B464D9787314167bDD9f1Ce28405bA1);
        IERC20 solvBTC = IERC20(0x541FD749419CA806a8bc7da8ac23D346f2dF8B77);
        IERC20 solvBTCJUP = IERC20(0x6b062AA7F5FC52b530Cb13967aE2E6bc0D8Dd3E4);
        bytes32 poolId = 0x6f113a39a50769de40d4f2e7e46b6a4c6d7774e2c3943ced2dbcb25e626d1d04;

        new SolvBTCJUPStrategy(solvBTCRouterV1, solvBTCRouterV2, poolId, solvBTC, solvBTCJUP);

        vm.stopBroadcast();
    }
}
