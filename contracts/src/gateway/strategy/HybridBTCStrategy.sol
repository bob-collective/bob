// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {IStrategyWithSlippageArgs, StrategySlippageArgs} from "../IStrategy.sol";
import {Context} from "@openzeppelin/contracts/utils/Context.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

interface ITeller {
    function deposit(ERC20 depositAsset, uint256 depositAmount, uint256 minimumMint)
        external
        payable
        returns (uint256 shares);
}

contract HybridBTCStrategy is IStrategyWithSlippageArgs, Context {
    using SafeERC20 for IERC20;

    IERC20 public immutable boringVault;
    ITeller public immutable teller;

    constructor(IERC20 _boringVault, ITeller _teller) {
        boringVault = _boringVault;
        teller = _teller;
    }

    /**
     * @notice Deposits tokens into Pell Network.
     * @dev Requires that the strategy is approved to spend the incoming tokens.
     * @param tokenSent The ERC20 token to deposit.
     * @param amountIn The amount to be deposited.
     * @param recipient The address to receive the shares.
     * @param args Additional args for slippage protection.
     */
    function handleGatewayMessageWithSlippageArgs(
        IERC20 tokenSent,
        uint256 amountIn,
        address recipient,
        StrategySlippageArgs memory args
    ) public override {
        tokenSent.safeTransferFrom(_msgSender(), address(this), amountIn);
        // The token approval is made to the boringVault contract not the teller contract
        tokenSent.safeIncreaseAllowance(address(boringVault), amountIn);

        uint256 amountBefore = boringVault.balanceOf(recipient);
        uint256 shares = teller.deposit(ERC20(address(tokenSent)), amountIn, args.amountOutMin);
        IERC20(boringVault).safeTransfer(recipient, shares);
        uint256 amountAfter = boringVault.balanceOf(recipient);
        require(amountAfter > amountBefore, "Insufficient supply provided");
        uint256 amountOut = amountAfter - amountBefore;
        require(amountOut >= args.amountOutMin, "Insufficient output amount");

        emit TokenOutput(address(boringVault), amountOut);
    }
}
