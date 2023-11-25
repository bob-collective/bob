# IRelay
[Git Source](https://github.com/bob-collective/bob/blob/cebdda1540fcce89f17d600bd2a84828c8c85ba6/src/bridge/IRelay.sol)

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

### difficultyCheckEnabled

Returns true iff difficulty check should be performed.


```solidity
function difficultyCheckEnabled() external view returns (bool);
```

