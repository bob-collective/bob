---
id: "relay"
title: "Module: relay"
sidebar_label: "relay"
sidebar_position: 0
custom_edit_url: null
---

## Interfaces

- [BitcoinTxInfo](../interfaces/relay.BitcoinTxInfo.md)
- [BitcoinTxProof](../interfaces/relay.BitcoinTxProof.md)

## Functions

### getBitcoinHeaders

▸ **getBitcoinHeaders**(`electrsClient`, `startHeight`, `numBlocks`): `Promise`<`string`\>

#### Parameters

| Name | Type |
| :------ | :------ |
| `electrsClient` | [`ElectrsClient`](../interfaces/electrs.ElectrsClient.md) |
| `startHeight` | `number` |
| `numBlocks` | `number` |

#### Returns

`Promise`<`string`\>

#### Defined in

[relay.ts:120](https://github.com/bob-collective/bob/blob/49b40f4/sdk/src/relay.ts#L120)

___

### getBitcoinTxInfo

▸ **getBitcoinTxInfo**(`electrsClient`, `txId`, `forWitness?`, `forWitness?`): `Promise`<[`BitcoinTxInfo`](../interfaces/relay.BitcoinTxInfo.md)\>

Retrieves information about a Bitcoin transaction, such as version, input vector, output vector, and locktime.

#### Parameters

| Name | Type | Description |
| :------ | :------ | :------ |
| `electrsClient` | [`ElectrsClient`](../interfaces/electrs.ElectrsClient.md) | An ElectrsClient instance for interacting with the Electrum server. |
| `txId` | `string` | The ID of the Bitcoin transaction. |
| `forWitness?` | `boolean` | - |
| `forWitness?` | `boolean` | - |

#### Returns

`Promise`<[`BitcoinTxInfo`](../interfaces/relay.BitcoinTxInfo.md)\>

A promise that resolves to a BitcoinTxInfo object.

**`Example`**

```typescript
const BITCOIN_NETWORK = "regtest";
const electrsClient = new DefaultElectrsClient(BITCOIN_NETWORK);
const txId = "279121610d9575d132c95312c032116d6b8a58a3a31f69adf9736b493de96a16"; //enter the transaction id here
const info = await getBitcoinTxInfo(electrsClient, txId);
```

#### Defined in

[relay.ts:47](https://github.com/bob-collective/bob/blob/49b40f4/sdk/src/relay.ts#L47)

___

### getBitcoinTxProof

▸ **getBitcoinTxProof**(`electrsClient`, `txId`, `txProofDifficultyFactor`): `Promise`<[`BitcoinTxProof`](../interfaces/relay.BitcoinTxProof.md)\>

Retrieves a proof for a Bitcoin transaction, including the merkle proof, transaction index in the block, and Bitcoin headers.

#### Parameters

| Name | Type | Description |
| :------ | :------ | :------ |
| `electrsClient` | [`ElectrsClient`](../interfaces/electrs.ElectrsClient.md) | An ElectrsClient instance for interacting with the Electrum server. |
| `txId` | `string` | The ID of the Bitcoin transaction. |
| `txProofDifficultyFactor` | `number` | The number of block headers to retrieve for proof verification. |

#### Returns

`Promise`<[`BitcoinTxProof`](../interfaces/relay.BitcoinTxProof.md)\>

**`Example`**

```typescript
const BITCOIN_NETWORK = "regtest";
const electrsClient = new DefaultElectrsClient(BITCOIN_NETWORK);
const txId = "279121610d9575d132c95312c032116d6b8a58a3a31f69adf9736b493de96a16";//enter the transaction id here
const txProofDifficultyFactor = "1";//enter the difficulty factor
const info = await getBitcoinTxProof(electrsClient, txId, txProofDifficultyFactor);
```

#### Defined in

[relay.ts:105](https://github.com/bob-collective/bob/blob/49b40f4/sdk/src/relay.ts#L105)
