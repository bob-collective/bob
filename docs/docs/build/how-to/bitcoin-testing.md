<<<<<<< HEAD
---
sidebar_position: 1
sidebar_label: Test Bitcoin Locally with Regtest 
---
||||||| parent of 1c12e27 (docs: order how to documents)
=======
---
sidebar_position: 1
---
>>>>>>> 1c12e27 (docs: order how to documents)
# Bitcoin Testing

## Starting a Local Bitcoin Development Environment

### Using Docker (Recommended)

We have a [`docker-compose.yml`](https://github.com/bob-collective/bob/blob/master/docker-compose.yml) setup script in the BOB repository.

The docker file will:

- Run the Bitcoin daemon
- Fund the "Alice" wallet
- Mine a block every ten seconds
- Start the [Esplora](https://github.com/Blockstream/esplora) backend to index the local chain and provide a [REST API](https://github.com/blockstream/esplora/blob/master/API.md)
- Start the [ord](https://github.com/ordinals/ord) ordinals indexer, block explorer, and wallet

### Using Bitcoin Core

The recommended approach to start a local Bitcoin node for testing is to use "regtest" mode. If you have downloaded Bitcoin Core from [bitcoin.org](https://bitcoin.org/en/download) use the following to start a regtest node.

```shell
bitcoind -regtest -daemon
```

To stop the node, use the following command:

```shell
bitcoin-cli -regtest stop
```

### Using Nigiri Bitcoin

For an alterantive development environment check out [Nigiri Bitcoin](https://github.com/vulpemventures/nigiri/) which also packages a Liquid daemon and an Electrum server.

## Funding a Wallet

:::info

This step is only required if you are not using the `docker-compose.yml` script.

:::

To create and fund the "Alice" wallet, use the following `bitcoin-cli` commands:

```shell
bitcoin-cli -regtest createwallet Alice
ALICE_ADDRESS=$(bitcoin-cli -regtest -rpcwallet=Alice getnewaddress)
bitcoin-cli -regtest generatetoaddress 101 ${ALICE_ADDRESS}
```

:::info

Funds from the coinbase transaction need 100 confirmations to be spendable, so make sure to mine a sufficient number of blocks. The command above mines 101 blocks, which is the minimum required for the funds to be spendable.

:::

## Transferring Funds

To create and send funds to the "Bob" wallet, use the following commands:

```shell
bitcoin-cli -regtest createwallet Bob
BOB_ADDRESS=$(bitcoin-cli -regtest -rpcwallet=Bob getnewaddress)
bitcoin-cli -regtest -rpcwallet=Alice -named sendtoaddress address=${BOB_ADDRESS} amount=$(bitcoin-cli -regtest -rpcwallet=Alice getbalance) subtractfeefromamount=true
```

To use a supported Graphical User Interface (GUI) instead, terminate the `bitcoind` process and run `bitcoin-qt` instead:

![Bitcoin Core Wallet GUI](https://bitcoin.org/img/bitcoin-core/clear-overview.png?1697494088)