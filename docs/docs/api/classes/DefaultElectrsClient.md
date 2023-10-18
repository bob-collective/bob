---
id: "DefaultElectrsClient"
title: "Class: DefaultElectrsClient"
sidebar_label: "DefaultElectrsClient"
sidebar_position: 0
custom_edit_url: null
---

The `DefaultElectrsClient` class provides a client for interacting with an Esplora API
for Bitcoin network data retrieval.

## Implements

- [`ElectrsClient`](../interfaces/ElectrsClient.md)

## Constructors

### constructor

• **new DefaultElectrsClient**(`networkOrUrl?`)

Create an instance of the `DefaultElectrsClient` with the specified network or URL.
If the `networkOrUrl` parameter is omitted, it defaults to "mainnet."

#### Parameters

| Name | Type | Default value | Description |
| :------ | :------ | :------ | :------ |
| `networkOrUrl` | `string` | `"mainnet"` | The Bitcoin network (e.g., "mainnet," "testnet," "regtest") |

**`Example`**

```ts
const BITCOIN_NETWORK = "regtest";
const electrsClient = new DefaultElectrsClient(BITCOIN_NETWORK);
```

**`Example`**

```ts
// Create a client for the mainnet using the default URL.
const electrsClientMainnet = new DefaultElectrsClient();
```

#### Defined in

[electrs.ts:151](https://github.com/bob-collective/bob/blob/3b4598b/sdk/src/electrs.ts#L151)

## Properties

### basePath

• `Private` **basePath**: `string`

#### Defined in

[electrs.ts:133](https://github.com/bob-collective/bob/blob/3b4598b/sdk/src/electrs.ts#L133)
