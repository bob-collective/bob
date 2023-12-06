---
sidebar_position: 3
sidebar_label: Inscribe and Transfer Ordinals
---

# Ordinals

[Ordinals](https://docs.ordinals.com/) are a system for tracking and transferring satoshis, Bitcoin's smallest units and attaching data to satoshis. When such attachment of data to satoshi happens an __inscription__ is created. Inscription content is entirely on-chain, stored in taproot script-path spend scripts forever. These scripts receive witness discounts, making inscription content storage relatively economical.

## How to create an inscription
Inscriptions are created using taproot script. That means you have to use a P2TR type of address to be able to create the inscription. 

### Wallets that support taproot addresses

- [`ord` wallet](https://docs.ordinals.com/guides/inscriptions.html) (Desktop, full Bitcoin node requirement)
- [Sparrow wallet](https://sparrowwallet.com/) (Desktop)
- [Xverse](https://www.xverse.app/download) (Browser and mobile)
- [Unisat](https://unisat.io/download) (Browser)
- [MetaMask](https://metamask.io/) [btcsnap](https://github.com/bob-collective/btcsnap) (Browser)

### Inscribing data
If you're developing an app with Javascript or Typescript you can use the BOB SDK to create the commit and reveal transactions required to inscribe data to an ordinal. Refer to the `inscribeText` function and provide a custom `RemoteSigner` implementation for your wallet. The test in [`sdk/test/ordinals.test.ts`](https://github.com/bob-collective/bob/blob/master/sdk/test/ordinals.test.ts) provides an example implementation using [`bitcoinjs-lib`](https://github.com/bitcoinjs/bitcoinjs-lib) and [`tiny-secp256k1`](https://github.com/bitcoinjs/tiny-secp256k1).

#### Using an external service
There are other ways to create inscriptions, the following two are advised:
- [Inscribe the Planet](https://inscribetheplanet.com/) __[RECOMMENDED]__ service which is free to use and supports file upload, direct BRC-20 deployments and minting.
- [Default way using the `ord` client](https://docs.ordinals.com/guides/inscriptions.html) (Requires full Bitcoin node and network reindexing)

:::note
Even though the inscribing process requires only 1 satoshi to which data are inscribed, there are associated fees to store the data based on inscription size and padding - an amount of satoshis included in the UTXO to pay for future inscriptions transfers.
:::

## How to transfer an inscription
To transfer the inscription, the satoshi to which the data were inscribed has to be transferred. To do that, simply select the inscription in your wallet and send it to the recipient's address.

:::tip
Make sure that the recipient of the transfer provided a valid P2TR address.
:::

## How to explore ordinal data
Even though inscription data are stored on the blockchain, they are not directly accessible by calling a Bitcoin node. To be able to work with inscriptions, an ordinals indexer has to be run. There is a main [open-source indexer implementation](https://github.com/casey/ord) which contains an ordinals explorer web interface and an API deployed at [ordinals.com](https://ordinals.com/).

BOB provides a free public API for accessing Ordinals data on both Bitcoin mainnet and testnet. We provide OpenAPI documentation for both APIs at https://ord-mainnet.gobob.xyz/docs and https://ord-testnet.gobob.xyz/docs respectively.
As of October 2023 the API is in beta and is subject to change. We will do our best to keep the API stable and backwards compatible. If you have any questions or suggestions, please join our Discord server at https://discord.com/invite/interlay and let us know.
