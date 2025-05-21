// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {stdStorage, StdStorage, Test, console} from "forge-std/Test.sol";

using stdStorage for StdStorage;

import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";
import {IceCreamSwapStrategy} from "../../../src/gateway/strategy/IceCreamSwapStrategy.sol";
import {StrategySwapArgs} from "../../../src/gateway/IStrategy.sol";
import {Constants} from "./Constants.sol";
import {ForkedStrategyTemplateWbtc} from "./ForkedTemplate.sol";

// Using a contract for decoding the routing data so that we can use the same calldata slicing that the aggregator does
contract CalldataDecoder {
    // function signature must be the same as the aggregator guard
    function IceCreamSwap()
        public
        pure
        returns (
            uint256 id,
            address executor,
            uint256 amountIn,
            uint256 minAmountOut,
            address firstTokenReceiver,
            address recipient,
            address tokenIn,
            address tokenOut
        )
    {
        assembly {
            id := shr(240, calldataload(4))
            executor := shr(96, calldataload(6))
            amountIn := shr(128, calldataload(26))
            minAmountOut := shr(128, calldataload(42))
            firstTokenReceiver := shr(96, calldataload(59))
            recipient := shr(96, calldataload(79))
            tokenIn := shr(96, calldataload(99))
            tokenOut := shr(96, calldataload(119))
        }
    }
}

// Command to run this contract tests with Foundry:
// BOB_PROD_PUBLIC_RPC_URL=https://rpc.gobob.xyz/ forge test --match-contract IceCreamSwapStrategyForkedWbtc -vv
contract IceCreamSwapStrategyForkedWbtc is ForkedStrategyTemplateWbtc {
    event TokenOutput(address tokenReceived, uint256 amountOut);

    IERC20 wbtc = IERC20(Constants.WBTC_ADDRESS);
    address iceCreamSwapAggregatorGuard = 0xC87De04e2EC1F4282dFF2933A2D58199f688fC3d;

    IceCreamSwapStrategy iceCreamSwapStrategy;

    address sender = makeAddr("sender");
    address recipient = makeAddr("recipient");

    function decodeSwapCalldata(bytes memory swapCalldata)
        internal
        returns (uint256, address, uint256, uint256, address, address, address, address)
    {
        CalldataDecoder calldataDecoder = new CalldataDecoder();
        (, bytes memory returnData) = address(calldataDecoder).call(swapCalldata);
        (
            uint256 id,
            address executor,
            uint256 amountIn,
            uint256 minAmountOut,
            address firstTokenReceiver,
            address _recipient,
            address tokenIn,
            address tokenOut
        ) = abi.decode(returnData, (uint256, address, uint256, uint256, address, address, address, address));

        return (id, executor, amountIn, minAmountOut, firstTokenReceiver, _recipient, tokenIn, tokenOut);
    }

    function setUp() public {
        // transferring from the uniswap pool address to the sender
        // We use a fork that is close in time to when the api calls are made to ensure accurate price/liquidity data
        super.simulateForkAndTransfer(17099000, address(0x4A1dF9716147b785f3f82019f36f248Ac15DC308), sender, 1000000000);

        iceCreamSwapStrategy = new IceCreamSwapStrategy(iceCreamSwapAggregatorGuard);
    }

    function testSwap() public {
        uint256 amountIn = 10000; // 0.0001 wbtc
        IERC20 lbtc = IERC20(0xA45d4121b3D47719FF57a947A9d961539Ba33204);

        // IceCreamSwap api call for time when test was written for wbtc -> usdc.e swap:
        // https://aggregator.icecreamswap.com/60808?src=0x03C7054BCB39f7b2e5B2c7AcB37583e32D70Cfa3&dst=0xA45d4121b3D47719FF57a947A9d961539Ba33204&amount=10000&from=0x006217c47ffA5Eb3F3c92247ffFE22AD998242c5&slippage=0.5&convenienceFee=0
        // tx.data from api response
        bytes memory swapCalldata =
            hex"3f0bde25d919e578184bc88eb48485bba23a37b5509578d2ae3800000000000000000000000000002710000000000000000000000000000026df06e578184bc88eb48485bba23a37b5509578d2ae38006217c47ffa5eb3f3c92247fffe22ad998242c503c7054bcb39f7b2e5b2c7acb37583e32d70cfa3a45d4121b3d47719ff57a947a9d961539ba3320445bfa70e8f387da47ffe3dec447bc16f0b9a62612bfd1fc5e25a8f55c2e849492ad7966ea8a0dd9e0604025303050045510301ff";
        // toAmount from api response
        uint256 amountOut = 9999;

        (,, uint256 _amountIn, uint256 _minAmountOut,, address _recipient, address _tokenIn, address _tokenOut) =
            decodeSwapCalldata(swapCalldata);

        assertEq(_tokenIn, address(wbtc), "Token in is not wbtc");
        assertEq(_tokenOut, address(lbtc), "Token out is not lbtc");
        assertEq(_recipient, recipient, "Recipient is not the expected recipient");
        assertEq(_amountIn, amountIn, "Amount in is not the expected amount");
        assertLt(_minAmountOut, amountOut, "Amount out is less than the minimum");

        uint256 initialSenderWbtcBalance = wbtc.balanceOf(sender);
        uint256 initialRecipientLbtcBalance = lbtc.balanceOf(recipient);

        vm.startPrank(sender);
        wbtc.approve(address(iceCreamSwapStrategy), amountIn);
        vm.expectEmit(address(iceCreamSwapStrategy));
        emit TokenOutput(address(lbtc), amountOut);
        iceCreamSwapStrategy.handleGatewayMessageWithSwapArgs(
            wbtc, amountIn, recipient, StrategySwapArgs({tokenOut: address(lbtc), routingData: swapCalldata})
        );
        vm.stopPrank();

        // assert that the recipient has received more than the minAmountOut
        assertGe(lbtc.balanceOf(recipient), initialRecipientLbtcBalance + _minAmountOut);
        assertEq(lbtc.balanceOf(address(iceCreamSwapStrategy)), 0);
        assertEq(wbtc.balanceOf(address(iceCreamSwapStrategy)), 0);
        assertEq(wbtc.balanceOf(sender), initialSenderWbtcBalance - amountIn);
    }

    function testSwapMultiHop() public {
        // wbtc -> usdc.e
        uint256 amountIn = 10000000; // 0.1 wbtc
        IERC20 usdc = IERC20(0xe75D0fB2C24A55cA1e3F96781a2bCC7bdba058F0);

        // https://aggregator.icecreamswap.com/60808?src=0x03C7054BCB39f7b2e5B2c7AcB37583e32D70Cfa3&dst=0xe75D0fB2C24A55cA1e3F96781a2bCC7bdba058F0&amount=10000000&from=0x006217c47ffA5Eb3F3c92247ffFE22AD998242c5&slippage=0.5&convenienceFee=0
        bytes memory swapCalldata =
            hex"3f0bde25ac37e578184bc88eb48485bba23a37b5509578d2ae3800000000000000000000000000989680000000000000000000000002577bea0a0ce578184bc88eb48485bba23a37b5509578d2ae38006217c47ffa5eb3f3c92247fffe22ad998242c503c7054bcb39f7b2e5b2c7acb37583e32d70cfa3e75d0fb2c24a55ca1e3f96781a2bcc7bdba058f06407fec527abad1aafdb9a3b5a2171800c21a2fe2ece0e4b20ab662a9fc222a391b3ccb9ebf0485d05d032ac25d322df992303dca074ee7392c117b937760492b899398a2b3b86aa9490679cd9d63b2de112389471d577f7bc45c03c7c37f70abca1cc9302f57b2e4d310f3cf432fafa8d0d780ee920467c42000000000000000000000000000000000000062bfd1fc5e25a8f55c2e849492ad7966ea8a0dd9e0e0402a1470605060e0402bace06070606080206090a53030b011c510301ff";
        uint256 amountOut = 10113771396;

        (,,, uint256 _minAmountOut,,,,) = decodeSwapCalldata(swapCalldata);

        assertLt(_minAmountOut, amountOut, "Amount out is less than the minimum");

        uint256 initialRecipientUsdcBalance = usdc.balanceOf(recipient);

        vm.startPrank(sender);
        wbtc.approve(address(iceCreamSwapStrategy), amountIn);
        iceCreamSwapStrategy.handleGatewayMessageWithSwapArgs(
            wbtc, amountIn, recipient, StrategySwapArgs({tokenOut: address(usdc), routingData: swapCalldata})
        );
        vm.stopPrank();

        assertGe(usdc.balanceOf(recipient), initialRecipientUsdcBalance + _minAmountOut);
    }

    function testSwapNoCalldata() public {
        uint256 amountIn = 10000; // 0.0001 wbtc
        IERC20 lbtc = IERC20(0xA45d4121b3D47719FF57a947A9d961539Ba33204);

        vm.startPrank(sender);
        wbtc.approve(address(iceCreamSwapStrategy), amountIn);
        vm.expectRevert("Swap failed");
        iceCreamSwapStrategy.handleGatewayMessageWithSwapArgs(
            wbtc, amountIn, recipient, StrategySwapArgs({tokenOut: address(lbtc), routingData: ""})
        );
        vm.stopPrank();
    }

    function testSwapSlippageExceeded() public {
        // wbtc -> usdc.e
        uint256 amountIn = 10000000; // 0.1 wbtc
        IERC20 usdc = IERC20(0xe75D0fB2C24A55cA1e3F96781a2bCC7bdba058F0);

        // https://aggregator.icecreamswap.com/60808?src=0x03C7054BCB39f7b2e5B2c7AcB37583e32D70Cfa3&dst=0xe75D0fB2C24A55cA1e3F96781a2bCC7bdba058F0&amount=10000000&from=0x006217c47ffA5Eb3F3c92247ffFE22AD998242c5&slippage=0.5&convenienceFee=0
        bytes memory swapCalldata =
            hex"3f0bde25ac37e578184bc88eb48485bba23a37b5509578d2ae3800000000000000000000000000989680000000000000000000000002577bea0a0ce578184bc88eb48485bba23a37b5509578d2ae38006217c47ffa5eb3f3c92247fffe22ad998242c503c7054bcb39f7b2e5b2c7acb37583e32d70cfa3e75d0fb2c24a55ca1e3f96781a2bcc7bdba058f06407fec527abad1aafdb9a3b5a2171800c21a2fe2ece0e4b20ab662a9fc222a391b3ccb9ebf0485d05d032ac25d322df992303dca074ee7392c117b937760492b899398a2b3b86aa9490679cd9d63b2de112389471d577f7bc45c03c7c37f70abca1cc9302f57b2e4d310f3cf432fafa8d0d780ee920467c42000000000000000000000000000000000000062bfd1fc5e25a8f55c2e849492ad7966ea8a0dd9e0e0402a1470605060e0402bace06070606080206090a53030b011c510301ff";
        uint256 amountOut = 10113771396;

        (,,, uint256 _minAmountOut,,,,) = decodeSwapCalldata(swapCalldata);

        assertLt(_minAmountOut, amountOut, "Amount out is less than the minimum");

        // before swap is submitted, another large swap is made which causes the market price to increase
        // so that the swap will revert due to slippage
        // https://aggregator.icecreamswap.com/60808?src=0x03C7054BCB39f7b2e5B2c7AcB37583e32D70Cfa3&dst=0xe75D0fB2C24A55cA1e3F96781a2bCC7bdba058F0&amount=30000000&from=0x006217c47ffA5Eb3F3c92247ffFE22AD998242c5&slippage=10&convenienceFee=0
        bytes memory largeSwapCalldata =
            hex"3f0bde25afafe578184bc88eb48485bba23a37b5509578d2ae3800000000000000000000000001c9c380000000000000000000000006231bb2a50ce578184bc88eb48485bba23a37b5509578d2ae38006217c47ffa5eb3f3c92247fffe22ad998242c503c7054bcb39f7b2e5b2c7acb37583e32d70cfa3e75d0fb2c24a55ca1e3f96781a2bcc7bdba058f0e112389471d577f7bc45c03c7c37f70abca1cc9302f57b2e4d310f3cf432fafa8d0d780ee920467c42000000000000000000000000000000000000066407fec527abad1aafdb9a3b5a2171800c21a2fe2ece0e4b20ab662a9fc222a391b3ccb9ebf0485d05d032ac25d322df992303dca074ee7392c117b937760492b899398a2b3b86aa9490679cd9d63b2d2bfd1fc5e25a8f55c2e849492ad7966ea8a0dd9e0e04026b5b0605060e0702a7d0060809060702060a0953030b0142510301ff";
        uint256 largeAmountIn = 30000000;

        vm.startPrank(sender);
        wbtc.approve(address(iceCreamSwapStrategy), largeAmountIn);
        iceCreamSwapStrategy.handleGatewayMessageWithSwapArgs(
            wbtc, largeAmountIn, recipient, StrategySwapArgs({tokenOut: address(usdc), routingData: largeSwapCalldata})
        );

        // now try the swap, which will revert due to slippage exceeding 0.5%
        wbtc.approve(address(iceCreamSwapStrategy), amountIn);
        vm.expectRevert("Swap failed");
        iceCreamSwapStrategy.handleGatewayMessageWithSwapArgs(
            wbtc, amountIn, recipient, StrategySwapArgs({tokenOut: address(usdc), routingData: swapCalldata})
        );
        vm.stopPrank();
    }
}
