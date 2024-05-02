---
sidebar_position: 3
---
import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

# Local Development

## EVM and Bitcoin

### EVM Setup

If you are developing on the BOB network, you need to install a wallet and a development environment.

#### Install a wallet

- [Metamask](https://metamask.io/)

You can also use other EVM wallets. BOB has a special Snap though so that you can use MetaMask for both the BOB chain and Bitcoin.

#### Install a development environment

<Tabs>
  <TabItem value="foundry" label="Foundry (based on Rust)">
    Quickstart:

```shell
# Install foundry
curl -L https://foundry.paradigm.xyz | bash
# Init a new project
forge init hello_bob
cd hello_bob
# Build the project
forge build

# Start a local chain
anvil
```

Check the docs for more information: [Foundry](https://book.getfoundry.sh/getting-started/installation).
  </TabItem>
  <TabItem value="hardhat" label="Hardhat (based on TypeScript)">
    Quickstart:

```shell
# Install hardhat
yarn add --dev hardhat
# Init a new project
npx hardhat init
```

Check the docs for more information: [Hardhat](https://hardhat.org/getting-started/).
  </TabItem>
</Tabs>

### Bitcoin

#### Install

Install Bitcoin Core `25.0` or higher so that you have access to the `bitcoind` and `bitcoin-cli` commands:

- MacOS: `brew install bitcoin`
- Ubuntu: `sudo add-apt-repository ppa:bitcoin/bitcoin && sudo apt-get update && sudo apt-get install bitcoind`
- Arch: `yay bitcoin-core`
- Download a binary: https://bitcoin.org/en/download

#### Start Bitcoin Core

```shell
# Start a regtest node
bitcoind -regtest
# Create a wallet named Alice
bitcoin-cli -regtest createwallet alice
# Get a new address for Alice
ADDRESS=$(bitcoin-cli -regtest -rpcwallet=alice getnewaddress)
# Mine 101 blocks to the address
bitcoin-cli -regtest -rpcwallet=alice generatetoaddress 101 $ADDRESS
```

#### Wallet

You can use `bitcoin-cli` to manage the wallet. However, if you prefer you a UI, you can use Bitcoin QT.

Start Bitcoin QT (for wallet functionality):

```shell
bitcoin-qt -regtest
```

## RPCs and APIs

### Esplora

Very useful for Bitcoin blockchain indexing and querying.

- Fork of electrs (Electrum Server in Rust)
- Indexes data using Bitcoin Core
- Provides REST API

The good:

- Can be run locally

API documentation: [Esplora](https://github.com/Blockstream/esplora/blob/master/API.md)

Quickstart:

```shell
git clone https://github.com/Blockstream/esplora && cd esplora
npm install
export API_URL=http://localhost:3000/ # or https://blockstream.info/api/ if you don't have a local API server
(see more config options below)
npm run dev-server
```

### Ord

If you are working with ordinals on a deeper level or in a local environment with Bitcoin regtest, this is a useful tool.

- Indexer, block-explorer and wallet
- Uses Bitcoin Core for signing and indexing
- Provides JSON-API

The good:

- Shows ordinals and inscriptions
- Can be run locally

The bad:

- Slow to index mainnet

Get started: https://docs.ordinals.com/

### UniSat

If you are working with ordinals or BRC20 tokens on Bitcoin testnet or mainnet, the hosted service is easier to use than using `ord`.

- Hosted service
- Provides REST API

The good:

- BRC20 indexing
- Swagger supports code-gen
- Additional APIs (inscription, swap, marketplace)

The bad:

- Only mainnet and testnet (hosted)

Get started: https://docs.unisat.io/dev/unisat-developer-service

## More Tools

### BOB SDK

- [sats-wagmi](/docs/build/bob-sdk/sats-wagmi): Learn how to use the sats-wagmi React hooks library to connect your app with Bitcoin wallets like UniSat, Leather, Xverse, and MetaMask.
- [Bitcoin Light Client](/docs/build/bob-sdk/relay): Learn how to interact with Bitcoin by proving transaction inclusion on BOB.
- [Use MetaMask for Bitcoin](/docs/build/bob-sdk/metamask-snap): Learn how to use MetaMask for Bitcoin on BOB.
- [Use tBTC, wBTC, or other ERC20 Tokens as Fee Tokens on BOB](/docs/build/bob-sdk/bridged-btc-gas-fee): Learn how to use tBTC and wBTC as fee tokens on BOB.

### Third-Party Tools on BOB

- [Wallets](/docs/build/tools/wallets)
- [APIs](/docs/build/tools/api)
- [Rust zkVM](/docs/build/tools/rust-zkvm)
- [Oracles](/docs/build/tools/oracles)

### Collection of Bitcoin Tools

[Lopp Development Tool List](https://www.lopp.net/bitcoin-information/developer-tools.html)