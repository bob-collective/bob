---
sidebar_position: 1
---
# Bitcoin Testing

The recommended approach to start a local Bitcoin node for testing is to use "regtest" mode. If you have downloaded Bitcoin Core from [bitcoin.org](https://bitcoin.org/en/download) use `bitcoind -regtest` to start a regtest node. To create and fund the "Alice" wallet, use the following `bitcoin-cli` commands:

```shell
bitcoin-cli -regtest createwallet Alice
ALICE_ADDRESS=$(bitcoin-cli -regtest -rpcwallet=Alice getnewaddress)
bitcoin-cli -regtest generatetoaddress 101 ${ALICE_ADDRESS}
```

:::info

Funds from the coinbase transaction need 100 confirmations to be spendable, so make sure to mine a sufficient number of blocks.

:::

We also have a [`docker-compose.yml`](https://github.com/bob-collective/bob/blob/master/docker-compose.yml) setup script in the BOB repository to run the Bitcoin daemon, fund the "Alice" wallet and mine a block every ten seconds. This will also run the [Esplora](https://github.com/Blockstream/esplora) backend to index the local chain and provide a REST API. For a more complete development environment check out [Nigiri Bitcoin](https://github.com/vulpemventures/nigiri/) which also packages a Liquid daemon and an Electrum server.

To create and send funds to the "Bob" wallet, use the following commands:

```shell
bitcoin-cli -regtest createwallet Bob
BOB_ADDRESS=$(bitcoin-cli -regtest -rpcwallet=Bob getnewaddress)
bitcoin-cli -regtest -rpcwallet=Alice -named sendtoaddress address=${BOB_ADDRESS} amount=$(bitcoin-cli -regtest -rpcwallet=Alice getbalance) subtractfeefromamount=true
```

To use a supported Graphical User Interface (GUI) instead, terminate the `bitcoind` process and run `bitcoin-qt` instead:

![Bitcoin Core Wallet GUI](https://bitcoin.org/img/bitcoin-core/clear-overview.png?1697494088)