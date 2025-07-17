// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {stdStorage, StdStorage, Test, console} from "forge-std/Test.sol";

using stdStorage for StdStorage;

import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";
import {ISolvBTCRouterV2, SolvBTCStrategy, XSolvBTCStrategy} from "../../../src/gateway/strategy/SolvStrategy.sol";
import {StrategySlippageArgs} from "../../../src/gateway/IStrategy.sol";
import {Constants} from "./Constants.sol";
import {ForkedStrategyTemplateWbtc} from "./ForkedTemplate.sol";

// Command to run this contract tests with Foundry:
// BOB_PROD_PUBLIC_RPC_URL=https://rpc.gobob.xyz/ forge test --match-contract SolvStrategyForked -vv
contract SolvStrategyForked is ForkedStrategyTemplateWbtc {
    function setUp() public {
        super.simulateForkAndTransfer(
            19911846, address(0x508A838922a93096C1Eb23FE21D8938BBd653Db6), Constants.DUMMY_SENDER, 1e8
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
}
