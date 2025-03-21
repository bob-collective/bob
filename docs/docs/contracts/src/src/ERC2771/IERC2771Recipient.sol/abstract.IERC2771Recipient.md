# IERC2771Recipient
[Git Source](https://github.com/bob-collective/bob/blob/master/src/ERC2771/IERC2771Recipient.sol)

A contract must implement this interface in order to support relayed transaction.

It is recommended that your contract inherits from the ERC2771Recipient contract.


## Functions
### isTrustedForwarder

:warning: **Warning** :warning: The Forwarder can have a full control over your Recipient. Only trust verified Forwarder.


```solidity
function isTrustedForwarder(address forwarder) public view virtual returns (bool);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`forwarder`|`address`|The address of the Forwarder contract that is being used.|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bool`|isTrustedForwarder `true` if the Forwarder is trusted to forward relayed transactions by this Recipient.|


### _msgSender

Use this method the contract anywhere instead of msg.sender to support relayed transactions.


```solidity
function _msgSender() internal view virtual returns (address);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`address`|sender The real sender of this call. For a call that came through the Forwarder the real sender is extracted from the last 20 bytes of the `msg.data`. Otherwise simply returns `msg.sender`.|


### _msgData

Use this method in the contract instead of `msg.data` when difference matters (hashing, signature, etc.)


```solidity
function _msgData() internal view virtual returns (bytes calldata);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`<none>`|`bytes`|data The real `msg.data` of this call. For a call that came through the Forwarder, the real sender address was appended as the last 20 bytes of the `msg.data` - so this method will strip those 20 bytes off. Otherwise (if the call was made directly and not through the forwarder) simply returns `msg.data`.|


