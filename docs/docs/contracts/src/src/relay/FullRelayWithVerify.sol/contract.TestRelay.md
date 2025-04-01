# TestRelay
[Git Source](https://github.com/bob-collective/bob/blob/1abe7d0a95cbaa62e47217036600733eae5f19f9/src/relay/FullRelayWithVerify.sol)

**Inherits:**
[FullRelayWithVerify](/src/relay/FullRelayWithVerify.sol/contract.FullRelayWithVerify.md)


## State Variables
### isAncestorOverride

```solidity
bool isAncestorOverride;
```


### isAncestorOverrideRes

```solidity
bool isAncestorOverrideRes;
```


## Functions
### constructor

Gives a starting point for the relay

*We don't check this AT ALL really. Don't use relays with bad genesis*


```solidity
constructor(bytes memory _genesisHeader, uint256 _height, bytes32 _periodStart)
    FullRelayWithVerify(_genesisHeader, _height, _periodStart);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_genesisHeader`|`bytes`|   The starting header|
|`_height`|`uint256`|          The starting height|
|`_periodStart`|`bytes32`|     The hash of the first header in the genesis epoch|


### heaviestFromAncestor


```solidity
function heaviestFromAncestor(bytes32 _ancestor, bytes calldata _left, bytes calldata _right)
    external
    view
    returns (bytes32);
```

### isMostRecentAncestor


```solidity
function isMostRecentAncestor(bytes32 _ancestor, bytes32 _left, bytes32 _right, uint256 _limit)
    external
    view
    returns (bool);
```

### setAncestorOverride


```solidity
function setAncestorOverride(bool _isAncestorOverride, bool _isAncestorOverrideRes) public;
```

### _isAncestor


```solidity
function _isAncestor(bytes32 _ancestor, bytes32 _descendant, uint256 _limit)
    internal
    view
    virtual
    override
    returns (bool);
```

### _getConfs


```solidity
function _getConfs(bytes32) internal view virtual override returns (uint8);
```

