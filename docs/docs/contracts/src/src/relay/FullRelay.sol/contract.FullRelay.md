# FullRelay
[Git Source](https://github.com/bob-collective/bob/blob/master/src/relay/FullRelay.sol)

**Inherits:**
[IFullRelay](../../relay/FullRelayInterfaces.sol/interface.IFullRelay.md)

**Author:**
Distributed Crafts (https://www.gobob.xyz/)
Forked from https://github.com/summa-tx/relays
Changes made:
1. dependency changes
- changed summa-tx/bitcoin-spv to keep-network/bitcoin-spv-sol
- remove SafeMath
2. test changes
- fixed some tests that were written incorrectly in the summa repo
- ported Truffle javascript tests to Foundry solidity
- new tests added
3. solidity compiler version upgraded to 0.8.17
4. OnDemandSPV was gutted and only the verification part was kept


## State Variables
### HEIGHT_INTERVAL

```solidity
uint32 public constant HEIGHT_INTERVAL = 4;
```


### relayGenesis

```solidity
bytes32 internal relayGenesis;
```


### bestKnownDigest

```solidity
bytes32 internal bestKnownDigest;
```


### lastReorgCommonAncestor

```solidity
bytes32 internal lastReorgCommonAncestor;
```


### previousBlock

```solidity
mapping(bytes32 => bytes32) internal previousBlock;
```


### blockHeight

```solidity
mapping(bytes32 => uint256) internal blockHeight;
```


### currentEpochDiff

```solidity
uint256 internal currentEpochDiff;
```


### prevEpochDiff

```solidity
uint256 internal prevEpochDiff;
```


## Functions
### constructor

Gives a starting point for the relay

*We don't check this AT ALL really. Don't use relays with bad genesis*


```solidity
constructor(bytes memory _genesisHeader, uint256 _height, bytes32 _periodStart);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_genesisHeader`|`bytes`|   The starting header|
|`_height`|`uint256`|          The starting height|
|`_periodStart`|`bytes32`|     The hash of the first header in the genesis epoch|


### isHeaderValidLength

Checks whether the header is 80 bytes long


```solidity
function isHeaderValidLength(bytes memory _header) internal pure returns (bool);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_header`|`bytes`|   The header for which the length is checked|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bool`|True if the header's length is 80 bytes, and false otherwise|


### isHeaderChainValidLength

Checks whether the header chain's length is a multiple of 80 bytes


```solidity
function isHeaderChainValidLength(bytes memory _headerChain) internal pure returns (bool);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_headerChain`|`bytes`|   The header chain for which the length is checked|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bool`|True if the header chain's length is a multiple of 80 bytes, and false otherwise|


### isMerkleArrayValidLength

Checks whether the merkle proof array's length is a multiple of 32 bytes


```solidity
function isMerkleArrayValidLength(bytes memory _merkleProofArray) internal pure returns (bool);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_merkleProofArray`|`bytes`|   The merkle proof array for which the length is checked|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bool`|True if the merkle proof array's length is a multiple of 32 bytes, and false otherwise|


### getCurrentEpochDifficulty

Getter for currentEpochDiff

*This is updated when a new heavist header has a new diff*


```solidity
function getCurrentEpochDifficulty() external view returns (uint256);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`uint256`|The difficulty of the bestKnownDigest|


### getPrevEpochDifficulty

Getter for prevEpochDiff

*This is updated when a difficulty change is accepted*


```solidity
function getPrevEpochDifficulty() external view returns (uint256);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`uint256`|The difficulty of the previous epoch|


### getRelayGenesis

Getter for relayGenesis

*This is an initialization parameter*


```solidity
function getRelayGenesis() public view returns (bytes32);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bytes32`|The hash of the first block of the relay|


### getBestKnownDigest

Getter for bestKnownDigest

*This updated only by calling markNewHeaviest*


```solidity
function getBestKnownDigest() public view returns (bytes32);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bytes32`|The hash of the best marked chain tip|


### getLastReorgCommonAncestor

Getter for relayGenesis

*This is updated only by calling markNewHeaviest*


```solidity
function getLastReorgCommonAncestor() public view returns (bytes32);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bytes32`|The hash of the shared ancestor of the most recent fork|


### findHeight

Finds the height of a header by its digest

*Will fail if the header is unknown*


```solidity
function findHeight(bytes32 _digest) external view returns (uint256);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_digest`|`bytes32`| The header digest to search for|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`uint256`|The height of the header, or error if unknown|


### findAncestor

Finds an ancestor for a block by its digest

*Will fail if the header is unknown*


```solidity
function findAncestor(bytes32 _digest, uint256 _offset) external view returns (bytes32);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_digest`|`bytes32`| The header digest to search for|
|`_offset`|`uint256`||

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bytes32`|The height of the header, or error if unknown|


### isAncestor

Checks if a digest is an ancestor of the current one

*Limit the amount of lookups (and thus gas usage) with _limit*


```solidity
function isAncestor(bytes32 _ancestor, bytes32 _descendant, uint256 _limit) external view returns (bool);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_ancestor`|`bytes32`|   The prospective ancestor|
|`_descendant`|`bytes32`| The descendant to check|
|`_limit`|`uint256`|      The maximum number of blocks to check|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bool`|true if ancestor is at most limit blocks lower than descendant, otherwise false|


### addHeaders

Adds headers to storage after validating

*We check integrity and consistency of the header chain*


```solidity
function addHeaders(bytes calldata _anchor, bytes calldata _headers) external returns (bool);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_anchor`|`bytes`|    The header immediately preceeding the new chain|
|`_headers`|`bytes`|   A tightly-packed list of 80-byte Bitcoin headers|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bool`|True if successfully written, error otherwise|


### addHeadersWithRetarget

Adds headers to storage, performs additional validation of retarget

*Checks the retarget, the heights, and the linkage*


```solidity
function addHeadersWithRetarget(
    bytes calldata _oldPeriodStartHeader,
    bytes calldata _oldPeriodEndHeader,
    bytes calldata _headers
) external returns (bool);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_oldPeriodStartHeader`|`bytes`|The first header in the difficulty period being closed|
|`_oldPeriodEndHeader`|`bytes`|  The last header in the difficulty period being closed|
|`_headers`|`bytes`|             A tightly-packed list of 80-byte Bitcoin headers|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bool`|True if successfully written, error otherwise|


### markNewHeaviest

Gives a starting point for the relay

*We don't check this AT ALL really. Don't use relays with bad genesis*


```solidity
function markNewHeaviest(bytes32 _ancestor, bytes calldata _currentBest, bytes calldata _newBest, uint256 _limit)
    external
    returns (bool);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_ancestor`|`bytes32`|        The digest of the most recent common ancestor|
|`_currentBest`|`bytes`|     The 80-byte header referenced by bestKnownDigest|
|`_newBest`|`bytes`|         The 80-byte header to mark as the new best|
|`_limit`|`uint256`|           Limit the amount of traversal of the chain|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bool`|True if successfully updates bestKnownDigest, error otherwise|


### _addHeaders

Adds headers to storage after validating

*We check integrity and consistency of the header chain*


```solidity
function _addHeaders(bytes memory _anchor, bytes memory _headers, bool _internal) internal returns (bool);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_anchor`|`bytes`|    The header immediately preceeding the new chain|
|`_headers`|`bytes`|   A tightly-packed list of new 80-byte Bitcoin headers to record|
|`_internal`|`bool`|  True if called internally from addHeadersWithRetarget, false otherwise|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bool`|True if successfully written, error otherwise|


### _addHeadersWithRetarget

Extract basic info

Adds headers to storage, performs additional validation of retarget

*Checks the retarget, the heights, and the linkage*


```solidity
function _addHeadersWithRetarget(bytes memory _oldStart, bytes memory _oldEnd, bytes memory _headers)
    internal
    returns (bool);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_oldStart`|`bytes`|            The first header in the difficulty period being closed|
|`_oldEnd`|`bytes`|              The last header in the difficulty period being closed|
|`_headers`|`bytes`|             A tightly-packed list of 80-byte Bitcoin headers|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bool`|True if successfully written, error otherwise|


### _findHeight

Finds the height of a header by its digest

*Will fail if the header is unknown*


```solidity
function _findHeight(bytes32 _digest) internal view returns (uint256);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_digest`|`bytes32`| The header digest to search for|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`uint256`|The height of the header|


### _findAncestor

Finds an ancestor for a block by its digest

*Will fail if the header is unknown*


```solidity
function _findAncestor(bytes32 _digest, uint256 _offset) internal view returns (bytes32);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_digest`|`bytes32`| The header digest to search for|
|`_offset`|`uint256`||

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bytes32`|The height of the header, or error if unknown|


### _isAncestor

Checks if a digest is an ancestor of the current one

*Limit the amount of lookups (and thus gas usage) with _limit*


```solidity
function _isAncestor(bytes32 _ancestor, bytes32 _descendant, uint256 _limit) internal view virtual returns (bool);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_ancestor`|`bytes32`|   The prospective ancestor|
|`_descendant`|`bytes32`| The descendant to check|
|`_limit`|`uint256`|      The maximum number of blocks to check|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bool`|true if ancestor is at most limit blocks lower than descendant, otherwise false|


### _markNewHeaviest

Marks the new best-known chain tip


```solidity
function _markNewHeaviest(bytes32 _ancestor, bytes memory _current, bytes memory _new, uint256 _limit)
    internal
    returns (bool);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_ancestor`|`bytes32`|        The digest of the most recent common ancestor|
|`_current`|`bytes`|         The 80-byte header referenced by bestKnownDigest|
|`_new`|`bytes`|             The 80-byte header to mark as the new best|
|`_limit`|`uint256`|           Limit the amount of traversal of the chain|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bool`|True if successfully updates bestKnownDigest, error otherwise|


### _isMostRecentAncestor

Checks if a digest is an ancestor of the current one

*Limit the amount of lookups (and thus gas usage) with _limit*


```solidity
function _isMostRecentAncestor(bytes32 _ancestor, bytes32 _left, bytes32 _right, uint256 _limit)
    internal
    view
    returns (bool);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_ancestor`|`bytes32`|   The prospective shared ancestor|
|`_left`|`bytes32`|       A chain tip|
|`_right`|`bytes32`|      A chain tip|
|`_limit`|`uint256`|      The maximum number of blocks to check|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bool`|true if it is the most recent common ancestor within _limit, false otherwise|


### _heaviestFromAncestor

Decides which header is heaviest from the ancestor

*Does not support reorgs above 2017 blocks (:*


```solidity
function _heaviestFromAncestor(bytes32 _ancestor, bytes memory _left, bytes memory _right)
    internal
    view
    returns (bytes32);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_ancestor`|`bytes32`|   The prospective shared ancestor|
|`_left`|`bytes`|       A chain tip|
|`_right`|`bytes`|      A chain tip|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bytes32`|true if it is the most recent common ancestor within _limit, false otherwise|


