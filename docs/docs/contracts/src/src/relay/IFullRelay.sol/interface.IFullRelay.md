# IFullRelay
[Git Source](https://github.com/bob-collective/bob/blob/master/src/relay/IFullRelay.sol)


## Functions
### getCurrentEpochDifficulty


```solidity
function getCurrentEpochDifficulty() external view returns (uint256);
```

### getPrevEpochDifficulty


```solidity
function getPrevEpochDifficulty() external view returns (uint256);
```

### getRelayGenesis


```solidity
function getRelayGenesis() external view returns (bytes32);
```

### getBestKnownDigest


```solidity
function getBestKnownDigest() external view returns (bytes32);
```

### getLastReorgCommonAncestor


```solidity
function getLastReorgCommonAncestor() external view returns (bytes32);
```

### findHeight


```solidity
function findHeight(bytes32 _digest) external view returns (uint256);
```

### findAncestor


```solidity
function findAncestor(bytes32 _digest, uint256 _offset) external view returns (bytes32);
```

### isAncestor


```solidity
function isAncestor(bytes32 _ancestor, bytes32 _descendant, uint256 _limit) external view returns (bool);
```

### addHeaders


```solidity
function addHeaders(bytes calldata _anchor, bytes calldata _headers) external returns (bool);
```

### addHeadersWithRetarget


```solidity
function addHeadersWithRetarget(
    bytes calldata _oldPeriodStartHeader,
    bytes calldata _oldPeriodEndHeader,
    bytes calldata _headers
) external returns (bool);
```

### markNewHeaviest


```solidity
function markNewHeaviest(bytes32 _ancestor, bytes calldata _currentBest, bytes calldata _newBest, uint256 _limit)
    external
    returns (bool);
```

## Events
### Extension

```solidity
event Extension(bytes32 indexed _first, bytes32 indexed _last);
```

### NewTip

```solidity
event NewTip(bytes32 indexed _from, bytes32 indexed _to, bytes32 indexed _gcd);
```

