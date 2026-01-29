// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console2} from "forge-std/Script.sol";
import {IERC20Metadata} from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

// https://github.com/ethereum-optimism/optimism/blob/327f972191d858181e9cbfc03a15fcd2f4604be4/packages/contracts-bedrock/src/universal/OptimismMintableERC20Factory.sol#L19
interface IOptimismERC20Factory {
    function createOptimismMintableERC20WithDecimals(
        address _remoteToken,
        string memory _name,
        string memory _symbol,
        uint8 _decimals
    ) external returns (address);
}

contract DeployERC20 is Script {
    function run() public {
        IERC20Metadata l1_token = IERC20Metadata(vm.envAddress("L1_TOKEN"));
        console2.log("L1 Token:", address(l1_token));

        vm.createSelectFork(vm.rpcUrl("eth"));

        string memory name = l1_token.name();
        string memory symbol = l1_token.symbol();
        uint8 decimals = l1_token.decimals();

        IOptimismERC20Factory factory = IOptimismERC20Factory(
            0x4200000000000000000000000000000000000012
        );

        console2.log("Name:", name);
        console2.log("Symbol:", symbol);
        console2.log("Decimals:", decimals);

        vm.createSelectFork(vm.rpcUrl("bob"));

        vm.startBroadcast();

        address l2_token = factory.createOptimismMintableERC20WithDecimals(
            address(l1_token),
            name,
            symbol,
            decimals
        );

        vm.stopBroadcast();

        console2.log("L2 Token:", address(l2_token));
    }
}
