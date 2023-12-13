# RelayUtils
[Git Source](https://github.com/bob-collective/bob/blob/67a580a9eab42424f3435ce488e8ec16222a7a9b/src/relay/LightRelay.sol)


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

