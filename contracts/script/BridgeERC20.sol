// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console2} from "forge-std/Script.sol";
import {IERC20Metadata} from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

// https://github.com/ethereum-optimism/optimism/blob/4465688458251351b048c6f0d5cf9e2dd14bd515/packages/contracts-bedrock/interfaces/universal/IStandardBridge.sol#L31
interface IStandardBridge {
    function bridgeERC20(
        address _localToken,
        address _remoteToken,
        uint256 _amount,
        uint32 _minGasLimit,
        bytes memory _extraData
    ) external;
}

contract BridgeERC20 is Script {
    address constant L1_STANDARD_BRIDGE =
        0x3F6cE1b36e5120BBc59D0cFe8A5aC8b6464ac1f7;

    function run() public {
        vm.startBroadcast();

        address l1_token = vm.envAddress("L1_TOKEN");
        address l2_token = vm.envAddress("L2_TOKEN");
        uint amount = 100;
        IERC20(l1_token).approve(L1_STANDARD_BRIDGE, amount);
        IStandardBridge(L1_STANDARD_BRIDGE).bridgeERC20(
            l1_token,
            l2_token,
            amount,
            200000,
            ""
        );

        vm.stopBroadcast();
    }
}
