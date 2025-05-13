// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {stdStorage, StdStorage, Test, console} from "forge-std/Test.sol";

using stdStorage for StdStorage;

import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";
import {
    IceCreamSwapStrategy, IIceCreamSwapAggregatorGuard
} from "../../../src/gateway/strategy/IceCreamSwapStrategy.sol";
import {StrategySwapArgs} from "../../../src/gateway/IStrategy.sol";
import {Constants} from "./Constants.sol";
import {ForkedStrategyTemplateWbtc} from "./ForkedTemplate.sol";

interface SingletonFactory {
    function deploy(bytes memory _initCode, bytes32 salt) external returns (address payable);
}

// Command to run this contract tests with Foundry:
// BOB_PROD_PUBLIC_RPC_URL=https://rpc.gobob.xyz/ forge test --match-contract IceCreamSwapStrategyForkedWbtc -vv
contract IceCreamSwapStrategyForkedWbtc is ForkedStrategyTemplateWbtc {
    event TokenOutput(address tokenReceived, uint256 amountOut);

    SingletonFactory internal singletonFactory = SingletonFactory(0xce0042B868300000d44A59004Da54A005ffdcf9f);
    bytes32 internal deployerSalt = bytes32(uint256(1));

    uint256 public amountIn = 100000; 

    IIceCreamSwapAggregatorGuard iceCreamSwapAggregatorGuard =
        IIceCreamSwapAggregatorGuard(0xC87De04e2EC1F4282dFF2933A2D58199f688fC3d);

    IceCreamSwapStrategy iceCreamSwapStrategy;

    // Test data
    // IceCreamSwap API call for wbtc -> usdc.e swap: 
    // https://aggregator.icecreamswap.com/60808?src=0x03C7054BCB39f7b2e5B2c7AcB37583e32D70Cfa3&dst=0xe75D0fB2C24A55cA1e3F96781a2bCC7bdba058F0&amount=100000&from=0x2344D9B3CCBdecc934Bba644deF132F7d21A7566&slippage=0.5&convenienceFee=0
    bytes swapCalldata =
        hex"3f0bde25d502e578184bc88eb48485bba23a37b5509578d2ae38000000000000000000000000000186a0000000000000000000000000057650ee08e578184bc88eb48485bba23a37b5509578d2ae3805e9ab742646956ef3ab0c043f3ddf813ab8432403c7054bcb39f7b2e5b2c7acb37583e32d70cfa3e75d0fb2c24a55ca1e3f96781a2bcc7bdba058f06407fec527abad1aafdb9a3b5a2171800c21a2fe2ece0e4b20ab662a9fc222a391b3ccb9ebf0485d05d032ac25d322df992303dca074ee7392c117b92bfd1fc5e25a8f55c2e849492ad7966ea8a0dd9e060402060506530307010b510301ff";

    function setUp() public {
        // transferring from the uniswap pool address to the sender
        super.simulateForkAndTransfer(
            16900000, address(0x4A1dF9716147b785f3f82019f36f248Ac15DC308), Constants.DUMMY_SENDER, amountIn*2
        );

        // Using Create2 to deploy the strategy so we can precompute the address for use in API calls
        // address is: 0x2344D9B3CCBdecc934Bba644deF132F7d21A7566
        iceCreamSwapStrategy = IceCreamSwapStrategy(
            singletonFactory.deploy(
                abi.encodePacked(type(IceCreamSwapStrategy).creationCode, abi.encode(iceCreamSwapAggregatorGuard)),
                deployerSalt
            )
        );

        console.log("iceCreamSwapStrategy", address(iceCreamSwapStrategy));
    }

    function testSwap() public {
        // Initial wbtc balance of the sender
        uint256 initialWbtcBalance = token.balanceOf(Constants.DUMMY_SENDER);

        console.log("initialWbtcBalance", initialWbtcBalance);

        console.log(address(iceCreamSwapAggregatorGuard).code.length);

        console.log("out initial balance", IERC20(0xe75D0fB2C24A55cA1e3F96781a2bCC7bdba058F0).balanceOf(0x05e9ab742646956Ef3Ab0C043F3ddf813ab84324));

        vm.startPrank(Constants.DUMMY_SENDER);
        token.approve(address(iceCreamSwapStrategy), amountIn*2);
        // vm.expectEmit(address(hybridBTCStrategy));
        // emit TokenOutput(address(boringVault), amountIn);
        iceCreamSwapStrategy.handleGatewayMessageWithSwapArgs(
            token,
            amountIn,
            Constants.DUMMY_RECEIVER,
            StrategySwapArgs({tokenOut: address(0), amountOutMin: 0, routingData: swapCalldata})
        );
        vm.stopPrank();

        // // The recipient should have the amountIn in boringVault shares
        // assertEq(IERC20(boringVault).balanceOf(Constants.DUMMY_RECEIVER), amountIn);
        // // The strategy should have no balance of the boringVault since it has been transferred to the recipient
        // assertEq(IERC20(boringVault).balanceOf(address(hybridBTCStrategy)), 0);
        // // The sender should have the initialWbtcBalance - amountIn in wbtc
        // assertEq(token.balanceOf(Constants.DUMMY_SENDER), initialWbtcBalance - amountIn);
    }
}


// 95403654

// 101822274