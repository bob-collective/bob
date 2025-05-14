// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {IStrategyWithSwapArgs, StrategySwapArgs} from "../IStrategy.sol";
import {Context} from "@openzeppelin/contracts/utils/Context.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

contract IceCreamSwapStrategy is IStrategyWithSwapArgs, Context {
    using SafeERC20 for IERC20;

    /// @notice IceCreamSwap aggregator guard address
    address public immutable iceCreamSwapAggregatorGuard;

    constructor(address _iceCreamSwapAggregatorGuard) {
        iceCreamSwapAggregatorGuard = _iceCreamSwapAggregatorGuard;
    }

    /**
     * @notice Swaps the incoming token to the tokenOut address using the IceCreamSwap aggregator guard.
     * @dev Requires that the strategy is approved to spend the incoming tokens.
     * @param tokenSent The ERC20 token to deposit.
     * @param amountIn The amount to be deposited.
     * @param recipient The address to receive the shares.
     * @param args Additional args containing the output token and the calldata for the swap.
     */
    function handleGatewayMessageWithSwapArgs(
        IERC20 tokenSent,
        uint256 amountIn,
        address recipient,
        StrategySwapArgs memory args
    ) public override {
        tokenSent.safeTransferFrom(_msgSender(), address(this), amountIn);

        tokenSent.safeIncreaseAllowance(iceCreamSwapAggregatorGuard, amountIn);

        uint256 initialTokenOutBalance = IERC20(args.tokenOut).balanceOf(recipient);

        // Using a low level call because the aggregator reads the calldata directly instead of from function arguments
        (bool success, bytes memory returnOrRevertData) = iceCreamSwapAggregatorGuard.call(args.routingData);

        // If the call was successful and the return data is a uint256 (which is 32 bytes) then the swap was successful
        if (success && returnOrRevertData.length == 32) {
            // The IceCreamSwap call will fail if the amountOut is less than the minAmountOut set in the args.routingData
            uint256 amountOut = abi.decode(returnOrRevertData, (uint256));
            // check the tokenOut balance of the recipient address has increased by amountOut
            require(IERC20(args.tokenOut).balanceOf(recipient) >= initialTokenOutBalance + amountOut, "Swap failed");

            emit TokenOutput(args.tokenOut, amountOut);
        } else {
            // The calling contract (gateway) will handle the revert
            revert("Swap failed");
        }
    }
}
