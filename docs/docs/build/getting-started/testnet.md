---
sidebar_position: 3
---

# Testnet

## Start Building

BOB is fully EVM compatible so you can use any tools you are used to from Ethereum.

## Fluffy BOB

We are hosting a public devnet for BOB using [conduit](https://conduit.xyz/).

- Published testnet: https://app.conduit.xyz/published/view/fluffy-bob-7mjgi9pmtg

This page also contains a button you can click to add the BOB testnet to your wallet. For convenience, the most useful links are copied below.

- **L2 RPC** https://l2-fluffy-bob-7mjgi9pmtg.t.conduit.xyz
- **L2 WS** wss://l2-fluffy-bob-7mjgi9pmtg.t.conduit.xyz
- **L2 Block Explorer** https://explorerl2-fluffy-bob-7mjgi9pmtg.t.conduit.xyz
- **Chain ID** `901`
- **Faucet** https://app.conduit.xyz/published/view/fluffy-bob-7mjgi9pmtg - use the L2 faucet box.
- **Ordinals API** https://ord-testnet.gobob.xyz/docs/
Sometimes the faucet fails, in which case you can use curl to directly call the API:

```sh
curl -XPOST -i https://faucetl2-fluffy-bob-7mjgi9pmtg.t.conduit.xyz/drip/[address]
```
