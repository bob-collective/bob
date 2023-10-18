---
id: "relay.BitcoinTxProof"
title: "Interface: BitcoinTxProof"
sidebar_label: "relay.BitcoinTxProof"
custom_edit_url: null
---

[relay](../modules/relay.md).BitcoinTxProof

Represents a Bitcoin transaction proof, including the merkle proof, transaction index in a block, and Bitcoin headers.

## Properties

### bitcoinHeaders

• **bitcoinHeaders**: `string`

Concatenated Bitcoin headers for proof verification.

#### Defined in

[relay.ts:87](https://github.com/bob-collective/bob/blob/49b40f4/sdk/src/relay.ts#L87)

___

### merkleProof

• **merkleProof**: `string`

The merkle proof for the Bitcoin transaction.

#### Defined in

[relay.ts:79](https://github.com/bob-collective/bob/blob/49b40f4/sdk/src/relay.ts#L79)

___

### txIndexInBlock

• **txIndexInBlock**: `number`

The index of the transaction in the block.

#### Defined in

[relay.ts:83](https://github.com/bob-collective/bob/blob/49b40f4/sdk/src/relay.ts#L83)
