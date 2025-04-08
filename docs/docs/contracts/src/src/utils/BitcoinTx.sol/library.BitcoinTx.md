# BitcoinTx
[Git Source](https://github.com/bob-collective/bob/blob/master/src/utils/BitcoinTx.sol)

Allows to reference Bitcoin raw transaction in Solidity.

*See https://developer.bitcoin.org/reference/transactions.html#raw-transaction-format*


## Functions
### validateProof

Validates the SPV proof of the Bitcoin transaction using a light relay contract.
Reverts in case the validation or proof verification fail.


```solidity
function validateProof(ILightRelay relay, uint256 txProofDifficultyFactor, Info memory txInfo, Proof memory proof)
    internal
    view
    returns (bytes32 txHash);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`relay`|`ILightRelay`|Bitcoin light relay providing the current Bitcoin network difficulty.|
|`txProofDifficultyFactor`|`uint256`|The number of confirmations required on the Bitcoin chain.|
|`txInfo`|`Info`|Bitcoin transaction data.|
|`proof`|`Proof`|Bitcoin proof data.|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`txHash`|`bytes32`|Proven 32-byte transaction hash.|


### validateProof

Validates the SPV proof of the Bitcoin transaction using a full relay contract.
Reverts in case the validation or proof verification fail.


```solidity
function validateProof(IFullRelayWithVerify relay, uint256 minConfirmations, Info memory txInfo, Proof memory proof)
    internal
    view
    returns (bytes32 txHash);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`relay`|`IFullRelayWithVerify`|Bitcoin full relay contract.|
|`minConfirmations`|`uint256`|The minimumnumber of confirmations required on the Bitcoin chain stored in the full relay.|
|`txInfo`|`Info`|Bitcoin transaction data.|
|`proof`|`Proof`|Bitcoin proof data.|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`txHash`|`bytes32`|Proven 32-byte transaction hash.|


### verifySPVProof

Verifies an SPV proof of a Bitcoin transaction.


```solidity
function verifySPVProof(bytes32 txHash, Proof memory proof) internal view;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`txHash`|`bytes32`|The hash of the transaction to verify.|
|`proof`|`Proof`|The proof.|


### computeTxHash

Validates Bitcoin transaction input and output vectors then computes the hash.


```solidity
function computeTxHash(Info memory txInfo) internal view returns (bytes32 txHash);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`txInfo`|`Info`|Bitcoin transaction data.|

**Returns**

|Name|Type|Description|
|----|----|-----------|
|`txHash`|`bytes32`|32-byte transaction hash.|


### evaluateProofDifficulty

Evaluates the given Bitcoin proof difficulty against the actual
Bitcoin chain difficulty provided by the light relay oracle.
Reverts in case the evaluation fails.


```solidity
function evaluateProofDifficulty(ILightRelay relay, uint256 txProofDifficultyFactor, bytes memory bitcoinHeaders)
    internal
    view;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`relay`|`ILightRelay`|Bitcoin light relay providing the current Bitcoin network difficulty.|
|`txProofDifficultyFactor`|`uint256`|The number of confirmations required on the Bitcoin chain.|
|`bitcoinHeaders`|`bytes`|Bitcoin headers chain being part of the SPV proof. Used to extract the observed proof difficulty.|


### verifyHeader

Validates the header using the full relay contract by checking it against the chain stored in the full relay.


```solidity
function verifyHeader(IFullRelayWithVerify relay, uint256 minConfirmations, bytes memory bitcoinHeader) internal view;
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`relay`|`IFullRelayWithVerify`|Bitcoin full relay contract.|
|`minConfirmations`|`uint256`|The minimum number of confirmations required on the Bitcoin chain stored in the full relay.|
|`bitcoinHeader`|`bytes`|Bitcoin header to verify.|


### processTxOutputs

Processes all outputs from the transaction.


```solidity
function processTxOutputs(bytes memory txOutputVector, bytes32 scriptPubKeyHash)
    internal
    pure
    returns (TxOutputsInfo memory resultInfo);
```
**Parameters**

|Name|Type|Description|
|----|----|-----------|
|`txOutputVector`|`bytes`|Bitcoin transaction output vector. This function assumes vector's structure is valid so it must be validated using e.g. `BTCUtils.validateVout` function before it is passed here.|
|`scriptPubKeyHash`|`bytes32`|Expected Bitcoin scriptPubKey keccak256 hash.|


### extractEvmAddressFromOutput


```solidity
function extractEvmAddressFromOutput(bytes memory _output, uint256 _at) internal pure returns (address evmAddress);
```

### extractHashFromOutput


```solidity
function extractHashFromOutput(bytes memory _output, uint256 _at) internal pure returns (bytes32 outputHash);
```

### reverseEndianness


```solidity
function reverseEndianness(bytes32 b) internal pure returns (bytes32 txHash);
```

### ensureTxInputSpendsUtxo


```solidity
function ensureTxInputSpendsUtxo(bytes memory _vin, BitcoinTx.UTXO memory utxo) internal pure;
```

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


## Structs
### Info
Represents Bitcoin transaction data.


```solidity
struct Info {
    bytes4 version;
    bytes inputVector;
    bytes outputVector;
    bytes4 locktime;
}
```

### Proof
Represents data needed to perform a Bitcoin SPV proof.


```solidity
struct Proof {
    bytes merkleProof;
    uint256 txIndexInBlock;
    bytes bitcoinHeaders;
    bytes32 coinbasePreimage;
    bytes coinbaseProof;
}
```

### UTXO
Represents info about an unspent transaction output.


```solidity
struct UTXO {
    bytes32 txHash;
    uint32 txOutputIndex;
    uint64 txOutputValue;
}
```

### TxOutputsProcessingInfo
Represents temporary information needed during the processing of
the Bitcoin transaction outputs. This structure is an internal one
and should not be exported outside of the transaction processing code.

*Allows to mitigate "stack too deep" errors on EVM.*


```solidity
struct TxOutputsProcessingInfo {
    uint256 outputStartingIndex;
    uint256 outputsCount;
}
```

### TxOutputsInfo
Represents an outcome of the Bitcoin transaction
outputs processing.


```solidity
struct TxOutputsInfo {
    uint64 value;
    address evmAddress;
    bytes32 hash;
}
```

