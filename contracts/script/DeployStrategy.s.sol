// // SPDX-License-Identifier: MIT
// pragma solidity ^0.8.24;

// import "forge-std/Script.sol";

// import {SolvBTCJUPStrategy, SolvBTCPlusStrategy, ISolvBTCRouterV2} from "../src/gateway/strategy/SolvStrategy.sol";
// import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";

// contract DeployStrategyScript is Script {
//     address public deployer;

//     function run() public {
//         uint256 deployerPrivateKey = vm.deriveKey(vm.envString("MNEMONIC"), 0);

//         vm.startBroadcast(deployerPrivateKey);

//         ISolvBTCRouterV2 solvBTCRouter = ISolvBTCRouterV2(0x56a4d805d7A292f03Ead5Be31E0fFB8f7d0E3B48);
//         IERC20 solvBTCJUP = IERC20(0x6b062AA7F5FC52b530Cb13967aE2E6bc0D8Dd3E4);
//         IERC20 solvBTCPlus = IERC20(0x4Ca70811E831db42072CBa1f0d03496EF126fAad);

//         new SolvBTCJUPStrategy(solvBTCRouter, solvBTCJUP);
//         new SolvBTCPlusStrategy(solvBTCRouter, solvBTCPlus);

//         vm.stopBroadcast();
//     }
// }