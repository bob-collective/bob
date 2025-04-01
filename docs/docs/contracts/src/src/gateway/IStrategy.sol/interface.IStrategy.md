# IStrategy
[Git Source](https://github.com/bob-collective/bob/blob/1abe7d0a95cbaa62e47217036600733eae5f19f9/src/gateway/IStrategy.sol)


## Functions
### handleGatewayMessage


```solidity
function handleGatewayMessage(IERC20 tokenSent, uint256 amountIn, address recipient, bytes memory message) external;
```

## Events
### TokenOutput

```solidity
event TokenOutput(address tokenReceived, uint256 amountOut);
```

