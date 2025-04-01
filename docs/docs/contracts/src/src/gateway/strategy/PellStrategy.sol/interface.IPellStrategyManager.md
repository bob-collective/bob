# IPellStrategyManager
[Git Source](https://github.com/bob-collective/bob/blob/1abe7d0a95cbaa62e47217036600733eae5f19f9/src/gateway/strategy/PellStrategy.sol)

*Pell ABI for their strategy manager.*


## Functions
### depositIntoStrategyWithStaker


```solidity
function depositIntoStrategyWithStaker(address staker, IPellStrategy strategy, IERC20 token, uint256 amount)
    external
    returns (uint256 shares);
```

### stakerStrategyShares


```solidity
function stakerStrategyShares(address staker, IPellStrategy strategy) external view returns (uint256);
```

