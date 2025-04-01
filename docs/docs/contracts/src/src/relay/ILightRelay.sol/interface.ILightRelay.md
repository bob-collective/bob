# ILightRelay
[Git Source](https://github.com/bob-collective/bob/blob/master/src/relay/ILightRelay.sol)


## Functions
### retarget


```solidity
function retarget(bytes memory headers) external;
```

### validateChain


```solidity
function validateChain(bytes memory headers)
    external
    view
    returns (uint256 startingHeaderTimestamp, uint256 headerCount);
```

### getBlockDifficulty


```solidity
function getBlockDifficulty(uint256 blockNumber) external view returns (uint256);
```

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

### getRelayRange


```solidity
function getRelayRange() external view returns (uint256 relayGenesis, uint256 currentEpochEnd);
```

## Events
### Genesis

```solidity
event Genesis(uint256 blockHeight);
```

### Retarget

```solidity
event Retarget(uint256 oldDifficulty, uint256 newDifficulty);
```

### ProofLengthChanged

```solidity
event ProofLengthChanged(uint256 newLength);
```

### AuthorizationRequirementChanged

```solidity
event AuthorizationRequirementChanged(bool newStatus);
```

### SubmitterAuthorized

```solidity
event SubmitterAuthorized(address submitter);
```

### SubmitterDeauthorized

```solidity
event SubmitterDeauthorized(address submitter);
```

