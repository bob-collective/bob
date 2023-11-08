// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console2} from "forge-std/Script.sol";
import {PimlicoERC20Paymaster, IERC20, IEntryPoint} from "../src/paymasters/AccountAbstraction/AATokenPaymaster.sol";
import {DummyOracle} from "../src/paymasters/Oracle.sol";


contract AATokenPaymasterScript is Script {
    function setUp() public {}

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        IERC20 token = IERC20(vm.envAddress("ERC_ADDRESS"));
        IEntryPoint entrypoint = IEntryPoint(
            vm.envAddress("ENTRYPOINT_ADDRESS")
        );
        address owner = vm.envAddress("OWNER_ADDRESS");

        vm.startBroadcast(deployerPrivateKey);

        DummyOracle ethOracle = new DummyOracle();
        ethOracle.setPrice(189100000000); // 1 eth = 1891usd
        DummyOracle usdcOracle = new DummyOracle();
        usdcOracle.setPrice(100000000); // 1 usdc = 1 usd
        PimlicoERC20Paymaster paymaster = new PimlicoERC20Paymaster(
            token,
            entrypoint,
            usdcOracle,
            ethOracle,
            owner,
            18
        );
        paymaster.updatePrice();
        entrypoint.depositTo{value: 1 ether}(address(paymaster));
        vm.stopBroadcast();
    }
}
