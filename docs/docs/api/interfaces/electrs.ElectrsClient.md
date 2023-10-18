---
id: "electrs.ElectrsClient"
title: "Interface: ElectrsClient"
sidebar_label: "electrs.ElectrsClient"
custom_edit_url: null
---

[electrs](../modules/electrs.md).ElectrsClient

## Implemented by

- [`DefaultElectrsClient`](../classes/electrs.DefaultElectrsClient.md)

## Methods

### getBlockHash

▸ **getBlockHash**(`height`): `Promise`<`string`\>

Get the block hash of the Bitcoin block at a specific height.

This function retrieves the block hash for the Bitcoin block at the given height.

#### Parameters

| Name | Type | Description |
| :------ | :------ | :------ |
| `height` | `number` | The height of the Bitcoin block. |

#### Returns

`Promise`<`string`\>

A promise that resolves to the block hash of the Bitcoin block.

**`Example`**

```typescript
const BITCOIN_NETWORK = "regtest";
const electrsClient = new DefaultElectrsClient(BITCOIN_NETWORK);
const blockHeight = 123456;
electrsClient.getBlockHash(blockHeight)
  .then((blockHash) => {
    console.log(`Block hash at height ${blockHeight}: ${blockHash}`);
  })
  .catch((error) => {
    console.error(`Error: ${error}`);
  });
```

#### Defined in

[electrs.ts:34](https://github.com/bob-collective/bob/blob/49b40f4/sdk/src/electrs.ts#L34)

___

### getBlockHeader

▸ **getBlockHeader**(`hash`): `Promise`<`string`\>

Get the raw block header, represented as a hex string, for a Bitcoin block with a given hash.

#### Parameters

| Name | Type | Description |
| :------ | :------ | :------ |
| `hash` | `string` | The hash of the Bitcoin block. |

#### Returns

`Promise`<`string`\>

A promise that resolves to the raw block header as a hex string.

**`Example`**

```typescript
const BITCOIN_NETWORK = "regtest";
const electrsClient = new DefaultElectrsClient(BITCOIN_NETWORK);
const blockHash = 'your_block_hash_here';
electrsClient.getBlockHeader(blockHash)
  .then((blockHeader) => {
    console.log(`Raw block header for block with hash ${blockHash}: ${blockHeader}`);
  })
  .catch((error) => {
    console.error(`Error: ${error}`);
  });
```

#### Defined in

[electrs.ts:56](https://github.com/bob-collective/bob/blob/49b40f4/sdk/src/electrs.ts#L56)

___

### getMerkleProof

▸ **getMerkleProof**(`txId`): `Promise`<[`MerkleProof`](electrs.MerkleProof.md)\>

Get the encoded merkle inclusion proof for a Bitcoin transaction with a given ID (txId).

#### Parameters

| Name | Type | Description |
| :------ | :------ | :------ |
| `txId` | `string` | The ID of a Bitcoin transaction. |

#### Returns

`Promise`<[`MerkleProof`](electrs.MerkleProof.md)\>

A promise that resolves to the encoded merkle inclusion proof.

**`Example`**

```typescript
const BITCOIN_NETWORK = "regtest";
const electrsClient = new DefaultElectrsClient(BITCOIN_NETWORK);
const transactionId = 'your_transaction_id_here';
electrsClient.getMerkleProof(transactionId)
  .then((merkleProof) => {
    console.log(`Merkle inclusion proof for transaction with ID ${transactionId}: ${merkleProof}`);
  })
  .catch((error) => {
    console.error(`Error: ${error}`);
  });
```

#### Defined in

[electrs.ts:100](https://github.com/bob-collective/bob/blob/49b40f4/sdk/src/electrs.ts#L100)

___

### getTransactionHex

▸ **getTransactionHex**(`txId`): `Promise`<`string`\>

Get the transaction data, represented as a hex string, for a Bitcoin transaction with a given ID (txId).

#### Parameters

| Name | Type | Description |
| :------ | :------ | :------ |
| `txId` | `string` | The ID of a Bitcoin transaction. |

#### Returns

`Promise`<`string`\>

A promise that resolves to the transaction data as a hex string.

**`Example`**

```typescript
const BITCOIN_NETWORK = "regtest";
const electrsClient = new DefaultElectrsClient(BITCOIN_NETWORK);
const transactionId = 'your_transaction_id_here';
electrsClient.getTransactionHex(transactionId)
  .then((transactionHex) => {
    console.log(`Transaction hex for transaction with ID ${transactionId}: ${transactionHex}`);
  })
  .catch((error) => {
    console.error(`Error: ${error}`);
  });
```

#### Defined in

[electrs.ts:78](https://github.com/bob-collective/bob/blob/49b40f4/sdk/src/electrs.ts#L78)
