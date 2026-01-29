---
sidebar_position: 1
sidebar_label: BOB Chain
---

# BOB Chain Overview

BOB is a Hybrid Chain that combines Bitcoin's security with Ethereum's DeFi innovation.

## Architecture

Learn more about BOB's architecture:
- [Hybrid Stack](./stack-overview) - The layered architecture enabling Bitcoin and Ethereum interaction
- [Hybrid Chain](./hybrid-chain) - BOB's core chain architecture and security model
- [Transactions](./transactions) - How transactions work on BOB
- [Hybrid Nodes](./hybrid-nodes) - Technical infrastructure providers securing the network

## Key Features

- **Hybrid Zero-Knowledge Rollup**: BOB has upgraded to [Kailua](https://risczero.com/op-kailua), making it a hybrid zk rollup. By default, state proposals are made optimistically. When competing proposals arise, they're settled by validity proofs. Proposers can also submit validity proofs on-demand for fast withdrawals.
- **Fast Bitcoin Finality**: All transactions are finalized by Bitcoin. This is achieved through Bitcoin checkpointing and BTC staking via Babylon's [BSN](https://docs.babylonlabs.io/guides/overview/bitcoin_secured_networks/) architecture.
- **EVM Compatibility**: Deploy and interact with EVM smart contracts. BOB uses the OP Stack with [minimal modifications](https://op-geth.optimism.io/) to the EVM - 100% compatibale with Base, Optimism and all other [Superchains](https://docs.optimism.io/superchain/superchain-explainer).
- **Low Fees**: Significantly lower transaction costs than Bitcoin or Ethereum.
- **Native BTC Support**: Use BTC directly in DeFi - with a single transaction through the power of [Gateway](https://docs.gobob.xyz/gateway) and with full Bitcoin-security by [BitVM](/docs/bitvm).

## Use Cases

- **DeFi**: Lending, borrowing, trading, and yield farming with BTC, stablecoins, ETH, and other assets.
- **NFTs**: Mint and trade NFTs with Bitcoin security.
- **Payments**: Fast, low-cost payments using BTC and stablecoins.
- **Staking**: Earn yield on BTC and other assets.

## Network Information

### Mainnet

- **Mainnet Chain ID**: 60808
- **Mainnet RPC**: https://rpc.gobob.xyz/
- **Explorer**: https://explorer.gobob.xyz/
- **App**: https://app.gobob.xyz/

### Testnet

- **Testnet Chain ID**: 808813
- **Testnet RPC**: https://bob-sepolia.rpc.gobob.xyz
- **Testnet Explorer**: https://bob-sepolia.explorer.gobob.xyz/
- **App**: https://bob-sepolia.gobob.xyz/
