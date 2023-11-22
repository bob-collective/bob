# ILightRelay
[Git Source](https://github.com/bob-collective/bob/blob/d9da9844231f0238dc8154200871bc3b4af31769/src/relay/LightRelay.sol)

[//]: # (**Inherits:**)

[//]: # ([IRelay]&#40;/docs/docs/contracts/src/src/bridge/IRelay.sol/interface.IRelay.md&#41;)


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

### getEpochDifficulty


```solidity
function getEpochDifficulty(uint256 epochNumber) external view returns (uint256);
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

