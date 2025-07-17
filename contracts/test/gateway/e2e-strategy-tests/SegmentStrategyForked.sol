// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {stdStorage, StdStorage, Test, console} from "forge-std/Test.sol";

using stdStorage for StdStorage;

import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";
import {
    IPellStrategyManager,
    IPellStrategy,
    PellStrategy,
    PellBedrockStrategy,
    PellXSolvBTCStrategy
} from "../../../src/gateway/strategy/PellStrategy.sol";
import {
    ISeBep20,
    SegmentStrategy,
    SegmentBedrockStrategy,
    SegmentXSolvBTCStrategy
} from "../../../src/gateway/strategy/SegmentStrategy.sol";
import {IBedrockVault, BedrockStrategy} from "../../../src/gateway/strategy/BedrockStrategy.sol";
import {XSolvBTCStrategy, ISolvBTCRouterV2} from "../../../src/gateway/strategy/SolvStrategy.sol";
import {StrategySlippageArgs} from "../../../src/gateway/IStrategy.sol";
import {Constants} from "./Constants.sol";
import {ForkedStrategyTemplateTbtc, ForkedStrategyTemplateWbtc} from "./ForkedTemplate.sol";

// Command to run this contract tests with Foundry:
// BOB_PROD_PUBLIC_RPC_URL=https://rpc.gobob.xyz/ forge test --match-contract SegmentStrategyForked -vv
contract SegmentStrategyForked is ForkedStrategyTemplateTbtc {
    function setUp() public {
        super.simulateForkAndTransfer(
            5607192, address(0xa79a356B01ef805B3089b4FE67447b96c7e6DD4C), Constants.DUMMY_SENDER, 1 ether
        );
    }

    function testSegmentStrategy() public {
        ISeBep20 seBep20 = ISeBep20(0xD30288EA9873f376016A0250433b7eA375676077);
        SegmentStrategy strategy = new SegmentStrategy(seBep20);

        vm.startPrank(Constants.DUMMY_SENDER);
        token.approve(address(strategy), 1 ether);

        strategy.handleGatewayMessageWithSlippageArgs(token, 1 ether, Constants.DUMMY_RECEIVER, StrategySlippageArgs(0));
        vm.stopPrank();

        uint256 userSeBalance = IERC20(address(seBep20)).balanceOf(Constants.DUMMY_RECEIVER);
        assertGt(userSeBalance, 0, "User has seTokens");
        assertEq(token.balanceOf(Constants.DUMMY_RECEIVER), 0, "User has no TBTC tokens");

        vm.prank(Constants.DUMMY_RECEIVER);
        seBep20.redeem(userSeBalance);

        assertGt(token.balanceOf(Constants.DUMMY_RECEIVER), 0, "User received TBTC tokens after redeem");
    }
}

// Command to run this contract tests with Foundry:
// BOB_PROD_PUBLIC_RPC_URL=https://rpc.gobob.xyz/ forge test --match-contract SegmentBedrockAndXSolvBTCStrategyForked -vv
contract SegmentBedrockAndXSolvBTCStrategyForked is ForkedStrategyTemplateWbtc {
    function setUp() public {
        super.simulateForkAndTransfer(
            19911846, address(0x508A838922a93096C1Eb23FE21D8938BBd653Db6), Constants.DUMMY_SENDER, 1e8
        );
    }

    function testSegmentBedrockStrategy() public {
        IBedrockVault vault = IBedrockVault(0x2ac98DB41Cbd3172CB7B8FD8A8Ab3b91cFe45dCf);
        BedrockStrategy bedrockStrategy = new BedrockStrategy(vault);
        ISeBep20 seBep20 = ISeBep20(0x7848F0775EebaBbF55cB74490ce6D3673E68773A);
        SegmentStrategy segmentStrategy = new SegmentStrategy(seBep20);
        SegmentBedrockStrategy strategy = new SegmentBedrockStrategy(bedrockStrategy, segmentStrategy);

        vm.startPrank(Constants.DUMMY_SENDER);
        token.approve(address(strategy), 1e8);

        strategy.handleGatewayMessageWithSlippageArgs(token, 1e8, Constants.DUMMY_RECEIVER, StrategySlippageArgs(0));
        vm.stopPrank();

        uint256 userSeBalance = IERC20(address(seBep20)).balanceOf(Constants.DUMMY_RECEIVER);
        assertGt(userSeBalance, 0, "User has seTokens");
        assertEq(token.balanceOf(Constants.DUMMY_RECEIVER), 0, "User has no WBTC tokens");

        IERC20 uniBTC = IERC20(vault.uniBTC());
        assertEq(uniBTC.balanceOf(Constants.DUMMY_RECEIVER), 0, "User has no uniBTC tokens");

        vm.prank(Constants.DUMMY_RECEIVER);
        seBep20.redeem(userSeBalance);

        userSeBalance = IERC20(address(seBep20)).balanceOf(Constants.DUMMY_RECEIVER);
        assertEq(userSeBalance, 0, "User has redeemed");

        assertGt(uniBTC.balanceOf(Constants.DUMMY_RECEIVER), 0, "User increase in uniBTC Balance");
    }

    function testSegmentXSolvBTCStrategy() public {
        IERC20 xSolvBTC = IERC20(0xCC0966D8418d412c599A6421b760a847eB169A8c);
        XSolvBTCStrategy xSolvBTCStrategy =
            new XSolvBTCStrategy(ISolvBTCRouterV2(0x56a4d805d7A292f03Ead5Be31E0fFB8f7d0E3B48), xSolvBTC);
        ISeBep20 seBep20 = ISeBep20(0x5EF2B8fbCc8aea2A9Dbe2729F0acf33E073Fa43e);
        SegmentStrategy segmentStrategy = new SegmentStrategy(seBep20);
        SegmentXSolvBTCStrategy strategy = new SegmentXSolvBTCStrategy(xSolvBTCStrategy, segmentStrategy);

        vm.startPrank(Constants.DUMMY_SENDER);
        token.approve(address(strategy), 1e8);

        strategy.handleGatewayMessageWithSlippageArgs(token, 1e8, Constants.DUMMY_RECEIVER, StrategySlippageArgs(0));
        vm.stopPrank();

        uint256 userSeBalance = IERC20(address(seBep20)).balanceOf(Constants.DUMMY_RECEIVER);
        assertGt(userSeBalance, 0, "User has seTokens");
        assertEq(token.balanceOf(Constants.DUMMY_RECEIVER), 0, "User has no WBTC tokens");

        vm.prank(Constants.DUMMY_RECEIVER);
        seBep20.redeem(userSeBalance);

        userSeBalance = IERC20(address(seBep20)).balanceOf(Constants.DUMMY_RECEIVER);
        assertEq(userSeBalance, 0, "User has redeemed");
        assertGt(xSolvBTC.balanceOf(Constants.DUMMY_RECEIVER), 0, "User still has xSolvBTC tokens");
    }
}
