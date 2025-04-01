# IFullRelayWithVerify
[Git Source](https://github.com/bob-collective/bob/blob/1abe7d0a95cbaa62e47217036600733eae5f19f9/src/relay/IFullRelayWithVerify.sol)

**Inherits:**
[IFullRelay](/src/relay/IFullRelay.sol/interface.IFullRelay.md)


## Functions
### verifyProof


```solidity
function verifyProof(bytes calldata _header, bytes calldata _proof, bytes32 _txId, uint256 _index, uint8 _numConfs)
    external
    view;
```

