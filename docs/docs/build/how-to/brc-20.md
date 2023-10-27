---
sidebar_position: 4
---

# BRC-20

[BRC-20]( https://domo-2.gitbook.io/brc-20-experiment/) is a fungible token standard for Bitcoin built on top of ordinals. This standard defines three basic functions: `deploy`, `mint` and `transfer`.

To obtain the current account balance and token supplies of BRC-20 assets a __BRC-20 indexer is required__. Multiple online services provide this information. One of the most popular is called [UniSat](https://unisat.io/brc20), where you can verify account balance and asset information. This indexer is [open-sourced](https://github.com/unisat-wallet/libbrc20-indexer).

## Wallets supporting BRC-20 Assets

- [Xverse](https://www.xverse.app/download) (Browser and mobile)
- [Unisat](https://unisat.io/download ) (Browser)

## Deploy BRC-20

To create a new BRC-20 asset, create an inscription with the content defining protocol (`p`) as `brc-20`, operation (`op`) as `deploy`, asset ticker (`tick`), maximum supply (`max`), and optional limit per mint (`lim`) depending on your preference. For example:

```
{ 
  "p": "brc-20",
  "op": "deploy",
  "tick": "ordi",
  "max": "21000000",
  "lim": "1000"
}
```

:::warning
Before deploying a new asset, make sure that the ticker has not been used before. In other cases, deployment will be considered invalid.
:::

## Mint BRC-20
To mint a BRC-20 asset, create an inscription with the content defining protocol (`p`) as `brc-20`, operation (`op`) as `mint`, asset ticker (`tick`) and amount (`amt`) depending on your preference. For example:

```
{ 
  "p": "brc-20",
  "op": "mint",
  "tick": "ordi",
  "amt": "1000"
}
```

:::warning
Before doing the mint operation, check that the asset you are trying to mint is still available and that you are not minting more than the allowed maximum.
:::

## Transfer BRC-20

To transfer the BRC-20 asset, create an inscription with the content defining protocol (`p`) as `brc-20`, operation (`op`) as `transfer`, asset ticker (`tick`) and amount (`amt`) depending on your preference. For example:

```
{ 
  "p": "brc-20",
  "op": "transfer",
  "tick": "ordi",
  "amt": "500"
}
```

After the data are inscribed, send the inscription to the recipient to complete the asset transfer.

:::note 
Each transfer inscription can only be used once.
:::

Read more in [BRC-20 specification](https://domo-2.gitbook.io/brc-20-experiment/).
