// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {IStrategyWithSlippageArgs, StrategySlippageArgs} from "../IStrategy.sol";

import {Context} from "@openzeppelin/contracts/utils/Context.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

/**
 * @dev Solv ABI for their router v1.
 */
interface ISolvBTCRouterV1 {
    /**
     * Subscribe with payment currency (i.e. WBTC) and receive SolvBTC.
     * @param poolId SolvBTC fund ID.
     * @param currencyAmount Amount of currency to be paid.
     * @return shareValue Amount of SolvBTC to be received after subscription.
     */
    function createSubscription(bytes32 poolId, uint256 currencyAmount) external returns (uint256 shareValue);
}

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

        // Expiry time must be in the future, so we use 1 second from now since the deposit will happen atomically
        uint256 solvBTCAmount = solvBTCRouter.deposit(
            address(solvBTC), address(tokenSent), amountIn, args.amountOutMin, uint64(block.timestamp + 1)
        );

        solvBTC.safeTransfer(recipient, solvBTCAmount);

        emit TokenOutput(_msgSender(), address(solvBTC), solvBTCAmount);
    }
}

contract XSolvBTCStrategy is IStrategyWithSlippageArgs, Context {
    using SafeERC20 for IERC20;

    ISolvBTCRouterV2 public immutable solvBTCRouter;
    IERC20 public immutable xSolvBTC;

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

        // Expiry time must be in the future, so we use 1 second from now since the deposit will happen atomically
        uint256 xSolvBTCAmount = solvBTCRouter.deposit(
            address(xSolvBTC), address(tokenSent), amountIn, args.amountOutMin, uint64(block.timestamp + 1)
        );

        xSolvBTC.safeTransfer(recipient, xSolvBTCAmount);

        emit TokenOutput(_msgSender(), address(xSolvBTC), xSolvBTCAmount);
    }
}

contract SolvBTCJUPStrategy is IStrategyWithSlippageArgs, Context {
    using SafeERC20 for IERC20;

    ISolvBTCRouterV1 public immutable solvBTCRouterV1;
    ISolvBTCRouterV2 public immutable solvBTCRouterV2;
    bytes32 public immutable poolId;
    IERC20 public immutable solvBTC;
    IERC20 public immutable solvBTCJUP;

    constructor(
        ISolvBTCRouterV1 _solvBTCRouterV1,
        ISolvBTCRouterV2 _solvBTCRouterV2,
        bytes32 _poolId,
        IERC20 _solvBTC,
        IERC20 _solvBTCJUP
    ) {
        solvBTCRouterV1 = _solvBTCRouterV1;
        solvBTCRouterV2 = _solvBTCRouterV2;
        poolId = _poolId;
        solvBTC = _solvBTC;
        solvBTCJUP = _solvBTCJUP;
    }

    function handleGatewayMessageWithSlippageArgs(
        IERC20 tokenSent,
        uint256 amountIn,
        address recipient,
        StrategySlippageArgs memory args
    ) public override {
        tokenSent.safeTransferFrom(_msgSender(), address(this), amountIn);
        tokenSent.safeIncreaseAllowance(address(solvBTCRouterV2), amountIn);

        // Expiry time must be in the future, so we use 1 second from now since the deposit will happen atomically
        // deposit input token into SolvBTC
        uint256 solvBTCAmount = solvBTCRouterV2.deposit(
            address(solvBTC), address(tokenSent), amountIn, args.amountOutMin, uint64(block.timestamp + 1)
        );

        solvBTC.safeIncreaseAllowance(address(solvBTCRouterV1), solvBTCAmount);

        // now deposit SolvBTC for SolvBTC.JUP
        uint256 solvBTCJUPAmount = solvBTCRouterV1.createSubscription(poolId, solvBTCAmount);

        solvBTCJUP.safeTransfer(recipient, solvBTCJUPAmount);

        emit TokenOutput(_msgSender(), address(solvBTCJUP), solvBTCJUPAmount);
    }
}

contract SolvBTCPlusStrategy is IStrategyWithSlippageArgs, Context {
    using SafeERC20 for IERC20;

    ISolvBTCRouterV2 public immutable solvBTCRouter;
    IERC20 public immutable solvBTCPlus;

    constructor(ISolvBTCRouterV2 _solvBTCRouter, IERC20 _solvBTCPlus) {
        solvBTCRouter = _solvBTCRouter;
        solvBTCPlus = _solvBTCPlus;
    }

    function handleGatewayMessageWithSlippageArgs(
        IERC20 tokenSent,
        uint256 amountIn,
        address recipient,
        StrategySlippageArgs memory args
    ) public override {
        tokenSent.safeTransferFrom(_msgSender(), address(this), amountIn);
        tokenSent.safeIncreaseAllowance(address(solvBTCRouter), amountIn);

        // Expiry time must be in the future, so we use 1 second from now since the deposit will happen atomically
        uint256 solvBTCPlusAmount = solvBTCRouter.deposit(
            address(solvBTCPlus), address(tokenSent), amountIn, args.amountOutMin, uint64(block.timestamp + 1)
        );

        solvBTCPlus.safeTransfer(recipient, solvBTCPlusAmount);

        emit TokenOutput(_msgSender(), address(solvBTCPlus), solvBTCPlusAmount);
    }
}
