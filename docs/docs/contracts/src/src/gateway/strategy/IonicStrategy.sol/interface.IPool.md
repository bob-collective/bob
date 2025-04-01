# IPool
[Git Source](https://github.com/bob-collective/bob/blob/1abe7d0a95cbaa62e47217036600733eae5f19f9/src/gateway/strategy/IonicStrategy.sol)

*Interface for the Ionic Finance Pool, allowing entry and exit from markets.*


## Functions
### enterMarkets


```solidity
function enterMarkets(address[] memory cTokens) external returns (uint256[] memory);
```

### exitMarket


```solidity
function exitMarket(address cTokenAddress) external returns (uint256);
```

