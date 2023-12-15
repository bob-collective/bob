# IRelay
[Git Source](https://github.com/bob-collective/bob/blob/6feef4b26921a0e62c67cac7076c04271613ba33/src/bridge/IRelay.sol)

Contains only the methods needed by tBTC v2. The Bitcoin relay
provides the difficulty of the previous and current epoch. One
difficulty epoch spans 2016 blocks.


## Functions
### getCurrentEpochDifficulty

Returns the difficulty of the current epoch.


```solidity
function getCurrentEpochDifficulty() external view returns (uint256);
```

### getPrevEpochDifficulty

Returns the difficulty of the previous epoch.


```solidity
function getPrevEpochDifficulty() external view returns (uint256);
```

