---
sidebar_position: 5
---

# Ordinals

Ordinals are a system for tracking and transferring satoshis, Bitcoin's smallest units and attaching data to satoshis. When such attachment of data to satoshi happens an __inscription__ is created. Inscription content is entirely on-chain, stored in taproot script-path spend scripts forever. These scripts receive witness discounts, making inscription content storage relatively economical.


## How to create an inscription
Inscriptions are created using taproot script. That means you have to use a P2TR type of address to be able to create the inscription. 

### Wallets that support taproot addresses

- [Sparrow wallet](https://sparrowwallet.com/) (Desktop)
- [Xverse](https://www.xverse.app/download) (Browser and mobile)
- [Unisat](https://unisat.io/download ) (Browser)

### Inscribing data
There are several ways to create inscriptions, the following two are advised:
- [Inscribe the Planet](https://inscribetheplanet.com/) __[RECOMMENDED]__ service which is free to use and supports file upload, direct BRC-20 deployments and minting.
- [Default way using the `ord` client](https://docs.ordinals.com/guides/inscriptions.html) (Requires full Bitcoin node and network reindexing)

:::note
Even though the inscribing process requires only 1 satoshi to which data are inscribed, there are associated fees to store the data based on inscription size and padding - an amount of satoshis included in the UTXO to pay for future inscriptions transfers.
:::

## How to transfer inscription
To transfer the inscription, the satoshi to which the data were inscribed has to be transferred. To do that, simply select the inscription in your wallet and send it to the recipient's address.
:::tip
Make sure that the recipient of the transfer provided a valid P2TR address.
:::


## How to explore ordinal data
Even though inscription data are stored on the blockchain, they are not directly accessible by calling a Bitcoin node. To be able to work with inscriptions, an ordinals indexer has to be run. There is a main [open-source indexer implementation](https://github.com/casey/ord) which contains an ordinals explorer web interface and an API deployed at [ordinals.com](https://ordinals.com/).