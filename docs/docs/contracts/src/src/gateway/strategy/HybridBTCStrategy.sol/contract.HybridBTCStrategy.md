# HybridBTCStrategy
[Git Source](https://github.com/bob-collective/bob/blob/master/src/gateway/strategy/HybridBTCStrategy.sol)

**Inherits:**
[IStrategyWithSlippageArgs](../../../gateway/IStrategy.sol/abstract.IStrategyWithSlippageArgs.md), Context


## State Variables
### boringVault

```solidity
IERC20 public immutable boringVault;
```


### teller

```solidity
ITeller public immutable teller;
```


## Functions
### constructor


```solidity
constructor(IERC20 _boringVault, ITeller _teller);
```

### handleGatewayMessageWithSlippageArgs

Deposits tokens into Veda Protocol and mints vault shares.

*Requires that the strategy is approved to spend the incoming tokens.*


```solidity
function handleGatewayMessageWithSlippageArgs(
    IERC20 tokenSent,
    uint256 amountIn,
    address recipient,
    StrategySlippageArgs memory args
) public override;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`tokenSent`|`IERC20`|The ERC20 token to deposit.|
|`amountIn`|`uint256`|The amount to be deposited.|
|`recipient`|`address`|The address to receive the shares.|
|`args`|`StrategySlippageArgs`|Additional args for slippage protection.|


