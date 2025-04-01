# IStrategyWithSlippageArgs
[Git Source](https://github.com/bob-collective/bob/blob/1abe7d0a95cbaa62e47217036600733eae5f19f9/src/gateway/IStrategy.sol)

**Inherits:**
[IStrategy](/src/gateway/IStrategy.sol/interface.IStrategy.md)


## Functions
### handleGatewayMessageWithSlippageArgs


```solidity
function handleGatewayMessageWithSlippageArgs(
    IERC20 tokenSent,
    uint256 amountIn,
    address recipient,
    StrategySlippageArgs memory args
) public virtual;
```

### handleGatewayMessage


```solidity
function handleGatewayMessage(IERC20 tokenSent, uint256 amountIn, address recipient, bytes memory message) external;
```

