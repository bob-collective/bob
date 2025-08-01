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
import {IBedrockVault, BedrockStrategy} from "../../../src/gateway/strategy/BedrockStrategy.sol";
import {XSolvBTCStrategy, ISolvBTCRouterV2} from "../../../src/gateway/strategy/SolvStrategy.sol";
import {StrategySlippageArgs} from "../../../src/gateway/IStrategy.sol";
import {Constants} from "./Constants.sol";
import {ForkedStrategyTemplateTbtc, ForkedStrategyTemplateWbtc} from "./ForkedTemplate.sol";

// Command to run this contract tests with Foundry:
// BOB_PROD_PUBLIC_RPC_URL=https://rpc.gobob.xyz/ forge test --match-contract PellStrategyForked -vv
contract PellStrategyForked is ForkedStrategyTemplateTbtc {
    function setUp() public {
        super.simulateForkAndTransfer(
            6077077, address(0xa79a356B01ef805B3089b4FE67447b96c7e6DD4C), Constants.DUMMY_SENDER, 1 ether
        );
    }

    function testPellStrategy() public {
        PellStrategy pellStrategy = new PellStrategy(
            IPellStrategyManager(0x00B67E4805138325ce871D5E27DC15f994681bC1),
            IPellStrategy(0x0a5e1Fe85BE84430c6eb482512046A04b25D2484)
        );

        vm.startPrank(Constants.DUMMY_SENDER);
        token.approve(address(pellStrategy), 1 ether);

        pellStrategy.handleGatewayMessageWithSlippageArgs(
            token,
            1 ether, // Amount: 1 TBTC
            Constants.DUMMY_RECEIVER,
            StrategySlippageArgs(0) // No slippage allowed
        );
        vm.stopPrank();

        assertEq(pellStrategy.stakerStrategyShares(Constants.DUMMY_RECEIVER), 1 ether);
    }
}

// Command to run this contract tests with Foundry:
// BOB_PROD_PUBLIC_RPC_URL=https://rpc.gobob.xyz/ forge test --match-contract PellBedRockStrategyForked -vv
contract PellBedRockStrategyForked is ForkedStrategyTemplateWbtc {
    function setUp() public {
        super.simulateForkAndTransfer(
            19911846, address(0x508A838922a93096C1Eb23FE21D8938BBd653Db6), Constants.DUMMY_SENDER, 1e8
        );
    }

    function testPellBedrockStrategy() public {
        IBedrockVault vault = IBedrockVault(0x2ac98DB41Cbd3172CB7B8FD8A8Ab3b91cFe45dCf);
        BedrockStrategy bedrockStrategy = new BedrockStrategy(vault);

        PellStrategy pellStrategy = new PellStrategy(
            IPellStrategyManager(0x00B67E4805138325ce871D5E27DC15f994681bC1),
            IPellStrategy(0x631ae97e24f9F30150d31d958d37915975F12ed8)
        );

        PellBedrockStrategy pellBedrockstrategy = new PellBedrockStrategy(bedrockStrategy, pellStrategy);

        vm.startPrank(Constants.DUMMY_SENDER);
        token.approve(address(pellBedrockstrategy), 1e8);

        pellBedrockstrategy.handleGatewayMessageWithSlippageArgs(
            token, 1e8, Constants.DUMMY_RECEIVER, StrategySlippageArgs(0)
        );
        vm.stopPrank();

        assertEq(pellStrategy.stakerStrategyShares(Constants.DUMMY_RECEIVER), 1e8);
    }
}

// Command to run this contract tests with Foundry:
// BOB_PROD_PUBLIC_RPC_URL=https://rpc.gobob.xyz/ forge test --match-contract PellBedRockXSolvBTCStrategyForked -vv
contract PellBedRockXSolvBTCStrategyForked is ForkedStrategyTemplateWbtc {
    function setUp() public {
        super.simulateForkAndTransfer(
            19911846, address(0x508A838922a93096C1Eb23FE21D8938BBd653Db6), Constants.DUMMY_SENDER, 1e8
        );
    }

    function testPellXSolvBTCStrategy() public {
        IERC20 xSolvBTC = IERC20(0xCC0966D8418d412c599A6421b760a847eB169A8c);
        XSolvBTCStrategy xSolvBTCStrategy =
            new XSolvBTCStrategy(ISolvBTCRouterV2(0x56a4d805d7A292f03Ead5Be31E0fFB8f7d0E3B48), xSolvBTC);

        PellStrategy pellStrategy = new PellStrategy(
            IPellStrategyManager(0x00B67E4805138325ce871D5E27DC15f994681bC1),
            IPellStrategy(0x6f0AfADE16BFD2E7f5515634f2D0E3cd03C845Ef)
        );

        PellXSolvBTCStrategy strategy = new PellXSolvBTCStrategy(xSolvBTCStrategy, pellStrategy);

        vm.startPrank(Constants.DUMMY_SENDER);
        token.approve(address(strategy), 1e8);

        strategy.handleGatewayMessageWithSlippageArgs(
            token,
            1e8, // Amount: 1 WBTC
            Constants.DUMMY_RECEIVER,
            StrategySlippageArgs(0) // No slippage allowed
        );
        vm.stopPrank();

        assertApproxEqAbs(pellStrategy.stakerStrategyShares(Constants.DUMMY_RECEIVER), 1e18, 1e18 / 100);
    }
}
