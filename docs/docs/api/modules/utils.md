---
id: "utils"
title: "Module: utils"
sidebar_label: "utils"
sidebar_position: 0
custom_edit_url: null
---

## Functions

### encodeRawInput

▸ **encodeRawInput**(`tx`): `Buffer`

#### Parameters

| Name | Type |
| :------ | :------ |
| `tx` | `Transaction` |

#### Returns

`Buffer`

#### Defined in

[utils.ts:15](https://github.com/bob-collective/bob/blob/49b40f4/sdk/src/utils.ts#L15)

___

### encodeRawOutput

▸ **encodeRawOutput**(`tx`): `Buffer`

#### Parameters

| Name | Type |
| :------ | :------ |
| `tx` | `Transaction` |

#### Returns

`Buffer`

#### Defined in

[utils.ts:38](https://github.com/bob-collective/bob/blob/49b40f4/sdk/src/utils.ts#L38)

___

### encodeRawWitness

▸ **encodeRawWitness**(`tx`): `Buffer`

#### Parameters

| Name | Type |
| :------ | :------ |
| `tx` | `Transaction` |

#### Returns

`Buffer`

#### Defined in

[utils.ts:71](https://github.com/bob-collective/bob/blob/49b40f4/sdk/src/utils.ts#L71)

___

### getMerkleProof

▸ **getMerkleProof**(`block`, `txHash`, `forWitness?`): `Object`

Retrieve a Merkle proof for a Bitcoin transaction from a block's raw data.

#### Parameters

| Name | Type | Description |
| :------ | :------ | :------ |
| `block` | `Block` | The Bitcoin block containing the transaction. |
| `txHash` | `string` | The transaction hash to construct a proof for. |
| `forWitness?` | `boolean` | Set to `true` to construct a witness proof (default is `false`). |

#### Returns

`Object`

An object containing the position, proof, and root of the Merkle proof.

| Name | Type |
| :------ | :------ |
| `pos` | `any` |
| `proof` | `string` |
| `root` | `string` |

#### Defined in

[utils.ts:133](https://github.com/bob-collective/bob/blob/49b40f4/sdk/src/utils.ts#L133)
