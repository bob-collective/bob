# FullRelayWithVerify
[Git Source](https://github.com/bob-collective/bob/blob/master/src/relay/FullRelayWithVerify.sol)

**Inherits:**
[FullRelay](../../relay/FullRelay.sol/contract.FullRelay.md)


## Functions
### constructor

Gives a starting point for the relay

*We don't check this AT ALL really. Don't use relays with bad genesis*


```solidity
constructor(bytes memory _genesisHeader, uint256 _height, bytes32 _periodStart)
    FullRelay(_genesisHeader, _height, _periodStart);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_genesisHeader`|`bytes`|   The starting header|
|`_height`|`uint256`|          The starting height|
|`_periodStart`|`bytes32`|     The hash of the first header in the genesis epoch|


### verifyProof

Verifies an SPV proof of a tx by checking that the tx is valid with respect
to a header and the header is valid with respect to the chain


```solidity
function verifyProof(bytes calldata _header, bytes calldata _proof, bytes32 _txId, uint256 _index, uint8 _numConfs)
    external
    view;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_header`|`bytes`|        The header containing the merkleroot committing to the tx|
|`_proof`|`bytes`|         The merkle proof intermediate nodes|
|`_txId`|`bytes32`|          The transaction id to verify|
|`_index`|`uint256`|         The index of the tx in the merkle tree's leaves|
|`_numConfs`|`uint8`|      Number of confirmations required|


### verifyHeaderHash

Verifies that a given block hash is part of the chain and is sufficiently deep


```solidity
function verifyHeaderHash(bytes32 _headerHash, uint8 _numConfs) public view;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_headerHash`|`bytes32`| The header hash to verify|
|`_numConfs`|`uint8`|   Number of confirmations required|


### _getConfs

Finds the number of headers on top of the argument

*Bounded to 6400 gas (8 looksups) max*


```solidity
function _getConfs(bytes32 _headerHash) internal view virtual returns (uint8);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`_headerHash`|`bytes32`| The LE double-sha2 header hash|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`uint8`|The number of headers on top|


