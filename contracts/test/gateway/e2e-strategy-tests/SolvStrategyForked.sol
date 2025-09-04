// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {stdStorage, StdStorage, Test, console} from "forge-std/Test.sol";

using stdStorage for StdStorage;

import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";
import {
    ISolvBTCRouterV1,
    ISolvBTCRouterV2,
    SolvBTCStrategy,
    XSolvBTCStrategy,
    SolvBTCJUPStrategy,
    SolvBTCPlusStrategy
} from "../../../src/gateway/strategy/SolvStrategy.sol";
import {StrategySlippageArgs} from "../../../src/gateway/IStrategy.sol";
import {Constants} from "./Constants.sol";
import {ForkedStrategyTemplateWbtcOft} from "./ForkedTemplate.sol";

// Command to run this contract tests with Foundry:
// BOB_PROD_PUBLIC_RPC_URL=https://rpc.gobob.xyz/ forge test --match-contract SolvStrategyForked -vv
contract SolvStrategyForked is ForkedStrategyTemplateWbtcOft {
    function setUp() public {
        super.simulateForkAndTransfer(
            21939620, address(0xa79a356B01ef805B3089b4FE67447b96c7e6DD4C), Constants.DUMMY_SENDER, 1e8
        );
    }

    function testSolvBTCStrategy() public {
        IERC20 solvBTC = IERC20(0x541FD749419CA806a8bc7da8ac23D346f2dF8B77);
        SolvBTCStrategy strategy =
            new SolvBTCStrategy(ISolvBTCRouterV2(0x56a4d805d7A292f03Ead5Be31E0fFB8f7d0E3B48), solvBTC);

        vm.startPrank(Constants.DUMMY_SENDER);
        token.approve(address(strategy), 1 * 1e8);

        strategy.handleGatewayMessageWithSlippageArgs(token, 1e8, Constants.DUMMY_RECEIVER, StrategySlippageArgs(0));
        vm.stopPrank();

        assertApproxEqAbs(solvBTC.balanceOf(Constants.DUMMY_RECEIVER), 1e18, 1e18 / 100);
    }

    function testXSolvBTCStrategy() public {
        IERC20 xSolvBTC = IERC20(0xCC0966D8418d412c599A6421b760a847eB169A8c);
        XSolvBTCStrategy strategy =
            new XSolvBTCStrategy(ISolvBTCRouterV2(0x56a4d805d7A292f03Ead5Be31E0fFB8f7d0E3B48), xSolvBTC);

        vm.startPrank(Constants.DUMMY_SENDER);
        token.approve(address(strategy), 1e8);

        strategy.handleGatewayMessageWithSlippageArgs(token, 1e8, Constants.DUMMY_RECEIVER, StrategySlippageArgs(0));
        vm.stopPrank();

        assertApproxEqAbs(xSolvBTC.balanceOf(Constants.DUMMY_RECEIVER), 1e18, 1e18 / 100);
    }

    function testSolvBTCJUPStrategy() public {
        IERC20 solvBTC = IERC20(0x541FD749419CA806a8bc7da8ac23D346f2dF8B77);
        IERC20 solvBTCJUP = IERC20(0x6b062AA7F5FC52b530Cb13967aE2E6bc0D8Dd3E4);
        bytes32 poolId = 0x6f113a39a50769de40d4f2e7e46b6a4c6d7774e2c3943ced2dbcb25e626d1d04;
        ISolvBTCRouterV1 solvBTCRouterV1 = ISolvBTCRouterV1(0xbA46FcC16B464D9787314167bDD9f1Ce28405bA1);
        ISolvBTCRouterV2 solvBTCRouterV2 = ISolvBTCRouterV2(0x56a4d805d7A292f03Ead5Be31E0fFB8f7d0E3B48);
        SolvBTCJUPStrategy strategy =
            new SolvBTCJUPStrategy(solvBTCRouterV1, solvBTCRouterV2, poolId, solvBTC, solvBTCJUP);

        vm.startPrank(Constants.DUMMY_SENDER);
        token.approve(address(strategy), 1e8);

        strategy.handleGatewayMessageWithSlippageArgs(token, 1e8, Constants.DUMMY_RECEIVER, StrategySlippageArgs(0));
        vm.stopPrank();

        assertApproxEqAbs(solvBTCJUP.balanceOf(Constants.DUMMY_RECEIVER), 1e18, 1e18 / 10);
    }

    function testSolvBTCPlusStrategy() public {
        IERC20 solvBTCPlus = IERC20(0x4Ca70811E831db42072CBa1f0d03496EF126fAad);
        SolvBTCPlusStrategy strategy =
            new SolvBTCPlusStrategy(ISolvBTCRouterV2(0x56a4d805d7A292f03Ead5Be31E0fFB8f7d0E3B48), solvBTCPlus);

        vm.startPrank(Constants.DUMMY_SENDER);
        token.approve(address(strategy), 1e8);

        strategy.handleGatewayMessageWithSlippageArgs(token, 1e8, Constants.DUMMY_RECEIVER, StrategySlippageArgs(0));
        vm.stopPrank();

        assertApproxEqAbs(solvBTCPlus.balanceOf(Constants.DUMMY_RECEIVER), 1e18, 1e18 / 100);
    }
}
