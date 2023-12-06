# IRelay
[Git Source](https://github.com/bob-collective/bob/blob/dae01a70f25bbe8256dca739e9a4468ec9c8194f/src/bridge/IRelay.sol)

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

