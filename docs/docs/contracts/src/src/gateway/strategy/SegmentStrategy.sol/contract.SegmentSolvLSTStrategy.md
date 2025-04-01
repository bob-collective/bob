# SegmentSolvLSTStrategy
[Git Source](https://github.com/bob-collective/bob/blob/1abe7d0a95cbaa62e47217036600733eae5f19f9/src/gateway/strategy/SegmentStrategy.sol)

**Inherits:**
[IStrategyWithSlippageArgs](/src/gateway/IStrategy.sol/abstract.IStrategyWithSlippageArgs.md), Context


## State Variables
### solvLSTStrategy

```solidity
SolvLSTStrategy public immutable solvLSTStrategy;
```


### segmentStrategy

```solidity
SegmentStrategy public immutable segmentStrategy;
```


## Functions
### constructor


```solidity
constructor(SolvLSTStrategy _solvLSTStrategy, SegmentStrategy _segmentStrategy);
```

### handleGatewayMessageWithSlippageArgs


```solidity
function handleGatewayMessageWithSlippageArgs(
    IERC20 tokenSent,
    uint256 amount,
    address recipient,
    StrategySlippageArgs memory args
) public override;
```

