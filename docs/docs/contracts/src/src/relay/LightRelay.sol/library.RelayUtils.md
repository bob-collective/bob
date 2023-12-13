# RelayUtils
[Git Source](https://github.com/bob-collective/bob/blob/d9c9196f0c99ad631c4c8411f2d25decea2e634f/src/relay/LightRelay.sol)


## Functions
### extractTimestampAt

Extract the timestamp of the header at the given position.

*Assumes that the specified position contains a valid header.
Performs no validation whatsoever.*


```solidity
function extractTimestampAt(bytes memory headers, uint256 at) internal pure returns (uint32);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`headers`|`bytes`|Byte array containing the header of interest.|
|`at`|`uint256`|The start of the header in the array.|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`uint32`|The timestamp of the header.|


