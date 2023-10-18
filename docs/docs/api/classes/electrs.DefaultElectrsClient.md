---
id: "electrs.DefaultElectrsClient"
title: "Class: DefaultElectrsClient"
sidebar_label: "electrs.DefaultElectrsClient"
custom_edit_url: null
---

[electrs](../modules/electrs.md).DefaultElectrsClient

## Implements

- [`ElectrsClient`](../interfaces/electrs.ElectrsClient.md)

## Constructors

### constructor

• **new DefaultElectrsClient**(`networkOrUrl?`)

#### Parameters

| Name | Type | Default value |
| :------ | :------ | :------ |
| `networkOrUrl` | `string` | `"mainnet"` |

#### Defined in

[electrs.ts:111](https://github.com/bob-collective/bob/blob/49b40f4/sdk/src/electrs.ts#L111)

## Properties

### basePath

• `Private` **basePath**: `string`

#### Defined in

[electrs.ts:109](https://github.com/bob-collective/bob/blob/49b40f4/sdk/src/electrs.ts#L109)

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

#### Implementation of

[ElectrsClient](../interfaces/electrs.ElectrsClient.md).[getBlockHash](../interfaces/electrs.ElectrsClient.md#getblockhash)

#### Defined in

[electrs.ts:127](https://github.com/bob-collective/bob/blob/49b40f4/sdk/src/electrs.ts#L127)

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

#### Implementation of

[ElectrsClient](../interfaces/electrs.ElectrsClient.md).[getBlockHeader](../interfaces/electrs.ElectrsClient.md#getblockheader)

#### Defined in

[electrs.ts:131](https://github.com/bob-collective/bob/blob/49b40f4/sdk/src/electrs.ts#L131)

___

### getJson

▸ **getJson**<`T`\>(`url`): `Promise`<`T`\>

#### Type parameters

| Name |
| :------ |
| `T` |

#### Parameters

| Name | Type |
| :------ | :------ |
| `url` | `string` |

#### Returns

`Promise`<`T`\>

#### Defined in

[electrs.ts:152](https://github.com/bob-collective/bob/blob/49b40f4/sdk/src/electrs.ts#L152)

___

### getMerkleProof

▸ **getMerkleProof**(`txId`): `Promise`<[`MerkleProof`](../interfaces/electrs.MerkleProof.md)\>

Get the encoded merkle inclusion proof for a Bitcoin transaction with a given ID (txId).

#### Parameters

| Name | Type | Description |
| :------ | :------ | :------ |
| `txId` | `string` | The ID of a Bitcoin transaction. |

#### Returns

`Promise`<[`MerkleProof`](../interfaces/electrs.MerkleProof.md)\>

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

#### Implementation of

[ElectrsClient](../interfaces/electrs.ElectrsClient.md).[getMerkleProof](../interfaces/electrs.ElectrsClient.md#getmerkleproof)

#### Defined in

[electrs.ts:139](https://github.com/bob-collective/bob/blob/49b40f4/sdk/src/electrs.ts#L139)

___

### getText

▸ **getText**(`url`): `Promise`<`string`\>

#### Parameters

| Name | Type |
| :------ | :------ |
| `url` | `string` |

#### Returns

`Promise`<`string`\>

#### Defined in

[electrs.ts:160](https://github.com/bob-collective/bob/blob/49b40f4/sdk/src/electrs.ts#L160)

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

#### Implementation of

[ElectrsClient](../interfaces/electrs.ElectrsClient.md).[getTransactionHex](../interfaces/electrs.ElectrsClient.md#gettransactionhex)

#### Defined in

[electrs.ts:135](https://github.com/bob-collective/bob/blob/49b40f4/sdk/src/electrs.ts#L135)
