# RelayUtils
[Git Source](https://github.com/bob-collective/bob/blob/9c01c7cfc14aec17ca362111dc5fef421675ccf1/src/relay/LightRelay.sol)


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


