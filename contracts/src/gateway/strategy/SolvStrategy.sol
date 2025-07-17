// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {IStrategyWithSlippageArgs, StrategySlippageArgs} from "../IStrategy.sol";

import {Context} from "@openzeppelin/contracts/utils/Context.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

/**
 * @dev Solv ABI for their router v2.
 */
interface ISolvBTCRouterV2 {
    /**
     * @notice Deposit currency and get targetToken
     * @param targetToken_ The target ERC20 token address to receive
     * @param currency_ The currency address to deposit
     * @param currencyAmount_ The amount of currency to deposit
     * @param minimumTargetTokenAmount_ The minimum acceptable return amount of target token
     * @param expireTime_ The expire time
     * @return targetTokenAmount_ The targetToken amount to receive
     */
    function deposit(
        address targetToken_,
        address currency_,
        uint256 currencyAmount_,
        uint256 minimumTargetTokenAmount_,
        uint64 expireTime_
    ) external returns (uint256 targetTokenAmount_);
}

/**
 * @title Strategy contract for SolvBTC.
 */
contract SolvBTCStrategy is IStrategyWithSlippageArgs, Context {
    using SafeERC20 for IERC20;

    ISolvBTCRouterV2 public immutable solvBTCRouter;
    IERC20 public immutable solvBTC;

    uint256 public constant ORDER_TIME_LIMIT = 1 days;

    constructor(ISolvBTCRouterV2 _solvBTCRouter, IERC20 _solvBTC) {
        solvBTCRouter = _solvBTCRouter;
        solvBTC = _solvBTC;
    }

    /**
     * @notice Deposits tokens into Solv.
     * @dev Requires that the strategy is approved to spend the incoming tokens.
     * @param tokenSent The ERC20 token to deposit.
     * @param amountIn The amount to be deposited.
     * @param recipient The address to receive SolvBTC.
     * @param args Additional args for slippage protection.
     */
    function handleGatewayMessageWithSlippageArgs(
        IERC20 tokenSent,
        uint256 amountIn,
        address recipient,
        StrategySlippageArgs memory args
    ) public override {
        tokenSent.safeTransferFrom(_msgSender(), address(this), amountIn);
        tokenSent.safeIncreaseAllowance(address(solvBTCRouter), amountIn);

        uint256 solvBTCAmount = solvBTCRouter.deposit(
            address(solvBTC),
            address(tokenSent),
            amountIn,
            args.amountOutMin,
            uint64(block.timestamp + ORDER_TIME_LIMIT)
        );

        solvBTC.safeTransfer(recipient, solvBTCAmount);

        emit TokenOutput(address(solvBTC), solvBTCAmount);
    }
}

contract XSolvBTCStrategy is IStrategyWithSlippageArgs, Context {
    using SafeERC20 for IERC20;

    ISolvBTCRouterV2 public immutable solvBTCRouter;
    IERC20 public immutable xSolvBTC;

    uint256 public constant ORDER_TIME_LIMIT = 1 days;

    constructor(ISolvBTCRouterV2 _solvBTCRouter, IERC20 _xSolvBTC) {
        solvBTCRouter = _solvBTCRouter;
        xSolvBTC = _xSolvBTC;
    }

    /**
     * @notice Deposits tokens into Solv.
     * @dev Requires that the strategy is approved to spend the incoming tokens.
     * @param tokenSent The ERC20 token to deposit.
     * @param amountIn The amount to be deposited.
     * @param recipient The address to receive e.g. SolvBTC.BBN.
     * @param args Additional args for slippage protection.
     */
    function handleGatewayMessageWithSlippageArgs(
        IERC20 tokenSent,
        uint256 amountIn,
        address recipient,
        StrategySlippageArgs memory args
    ) public override {
        tokenSent.safeTransferFrom(_msgSender(), address(this), amountIn);
        tokenSent.safeIncreaseAllowance(address(solvBTCRouter), amountIn);

        uint256 xSolvBTCAmount = solvBTCRouter.deposit(
            address(xSolvBTC),
            address(tokenSent),
            amountIn,
            args.amountOutMin,
            uint64(block.timestamp + ORDER_TIME_LIMIT)
        );

        xSolvBTC.safeTransfer(recipient, xSolvBTCAmount);

        emit TokenOutput(address(xSolvBTC), xSolvBTCAmount);
    }
}
