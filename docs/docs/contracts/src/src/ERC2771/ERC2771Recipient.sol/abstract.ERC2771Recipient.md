# ERC2771Recipient
[Git Source](https://github.com/bob-collective/bob/blob/master/src/ERC2771/ERC2771Recipient.sol)

**Inherits:**
[IERC2771Recipient](../../ERC2771/IERC2771Recipient.sol/abstract.IERC2771Recipient.md)

Note that this contract was called `BaseRelayRecipient` in the previous revision of the GSN.

A base contract to be inherited by any contract that want to receive relayed transactions.

A subclass must use `_msgSender()` instead of `msg.sender`.


## State Variables
### _trustedForwarder

```solidity
address private _trustedForwarder;
```


## Functions
### getTrustedForwarder

:warning: **Warning** :warning: The Forwarder can have a full control over your Recipient. Only trust verified Forwarder.

Method is not a required method to allow Recipients to trust multiple Forwarders. Not recommended yet.


```solidity
function getTrustedForwarder() public view virtual returns (address forwarder);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`forwarder`|`address`|The address of the Forwarder contract that is being used.|


### _setTrustedForwarder


```solidity
function _setTrustedForwarder(address _forwarder) internal;
```

### isTrustedForwarder

:warning: **Warning** :warning: The Forwarder can have a full control over your Recipient. Only trust verified Forwarder.


```solidity
function isTrustedForwarder(address forwarder) public view virtual override returns (bool);
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
function _msgSender() internal view virtual override returns (address ret);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`ret`|`address`|sender The real sender of this call. For a call that came through the Forwarder the real sender is extracted from the last 20 bytes of the `msg.data`. Otherwise simply returns `msg.sender`.|


### _msgData

Use this method in the contract instead of `msg.data` when difference matters (hashing, signature, etc.)


```solidity
function _msgData() internal view virtual override returns (bytes calldata ret);
```
**Returns**

|Name|Type|Description|
|----|----|-----------|
|`ret`|`bytes`|data The real `msg.data` of this call. For a call that came through the Forwarder, the real sender address was appended as the last 20 bytes of the `msg.data` - so this method will strip those 20 bytes off. Otherwise (if the call was made directly and not through the forwarder) simply returns `msg.data`.|


