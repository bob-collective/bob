// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {stdStorage, StdStorage, Test, console} from "forge-std/Test.sol";

using stdStorage for StdStorage;

import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";
import {ITeller, HybridBTCStrategy} from "../../../src/gateway/strategy/HybridBTCStrategy.sol";
import {StrategySlippageArgs} from "../../../src/gateway/IStrategy.sol";
import {Constants} from "./Constants.sol";
import {ForkedStrategyTemplateWbtc} from "./ForkedTemplate.sol";

// Command to run this contract tests with Foundry:
// BOB_PROD_PUBLIC_RPC_URL=https://rpc.gobob.xyz/ forge test --match-contract HybridBTCStrategyForkedWbtc -vv
contract HybridBTCStrategyForkedWbtc is ForkedStrategyTemplateWbtc {
    event TokenOutput(address tokenReceived, uint256 amountOut);

    uint256 public amountIn = 100000000; // 1 btc

    // Veda contracts
    ITeller teller = ITeller(0x19ab8c9896728d3A2AE8677711bc852C706616d3);
    IERC20 boringVault = IERC20(0x9998e05030Aee3Af9AD3df35A34F5C51e1628779);

    HybridBTCStrategy hybridBTCStrategy;

    function setUp() public {
        // transferring from the uniswap pool address to the sender
        super.simulateForkAndTransfer(
            14000000, address(0x4A1dF9716147b785f3f82019f36f248Ac15DC308), Constants.DUMMY_SENDER, amountIn
        );

        // Deploy the strategy
        hybridBTCStrategy = new HybridBTCStrategy(boringVault, teller);
    }

    function testDepositToVault() public {
        // Initial wbtc balance of the sender
        uint256 initialWbtcBalance = token.balanceOf(Constants.DUMMY_SENDER);

        vm.startPrank(Constants.DUMMY_SENDER);
        token.approve(address(hybridBTCStrategy), amountIn);
        vm.expectEmit(address(hybridBTCStrategy));
        emit TokenOutput(address(boringVault), amountIn);
        hybridBTCStrategy.handleGatewayMessageWithSlippageArgs(
            token,
            amountIn,
            Constants.DUMMY_RECEIVER,
            StrategySlippageArgs(0) // No slippage allowed
        );
        vm.stopPrank();

        // The recipient should have the amountIn in boringVault shares
        assertEq(IERC20(boringVault).balanceOf(Constants.DUMMY_RECEIVER), amountIn);
        // The strategy should have no balance of the boringVault since it has been transferred to the recipient
        assertEq(IERC20(boringVault).balanceOf(address(hybridBTCStrategy)), 0);
        // The sender should have the initialWbtcBalance - amountIn in wbtc
        assertEq(token.balanceOf(Constants.DUMMY_SENDER), initialWbtcBalance - amountIn);
    }
}
