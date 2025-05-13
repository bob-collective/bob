// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

interface IStrategy {
    event TokenOutput(address tokenReceived, uint256 amountOut);

    function handleGatewayMessage(IERC20 tokenSent, uint256 amountIn, address recipient, bytes memory message)
        external;
}

abstract contract IStrategyWithSlippageArgs is IStrategy {
    function handleGatewayMessageWithSlippageArgs(
        IERC20 tokenSent,
        uint256 amountIn,
        address recipient,
        StrategySlippageArgs memory args
    ) public virtual;

    function handleGatewayMessage(IERC20 tokenSent, uint256 amountIn, address recipient, bytes memory message)
        external
    {
        StrategySlippageArgs memory args = abi.decode(message, (StrategySlippageArgs));

        handleGatewayMessageWithSlippageArgs(tokenSent, amountIn, recipient, args);
    }
}

struct StrategySlippageArgs {
    uint256 amountOutMin;
}

abstract contract IStrategyWithSwapArgs is IStrategy {
    function handleGatewayMessageWithSwapArgs(
        IERC20 tokenSent,
        uint256 amountIn,
        address recipient,
        StrategySwapArgs memory args
    ) public virtual;

    function handleGatewayMessage(IERC20 tokenSent, uint256 amountIn, address recipient, bytes memory message)
        external
    {
        StrategySwapArgs memory args = abi.decode(message, (StrategySwapArgs));

        handleGatewayMessageWithSwapArgs(tokenSent, amountIn, recipient, args);
    }
}

struct StrategySwapArgs {
    address tokenOut;
    bytes routingData;
}
