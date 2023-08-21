---
sidebar_position: 2
sidebar_label: BOB Overview
---

# Building on Bitcoin

The key of BOB is to unify the compatibility of Bitcoin Rust libraries, the EVM, and a rollup. In essence:

- Interacting with BOB is the same as interacting with any EVM chain with compatibility with the same wallets and tools.
- Programming on BOB adds the option to write smart contracts with Rust and a WASM execution layer and with the EVM. Existing projects can deploy their existing Solidity/EVM code. New projects can be created in Rust, or off-chain protocols can be made available on-chain through Rust smart contracts.
- Access to Bitcoin the asset is provided via a bridge. Access to Bitcoin state is provided via a light client.
- Access to Ethereum assets and state is provided by the rollup.

## How?

BOB is based on [substrate](https://substrate.dev). Substrate is a modular and extensible blockchain framework that allows anyone to build custom blockchains. Substrate is written in Rust and thus can trivially integrate existing Rust libraries. Moreover, several modules (called pallets) already implemented for substrate enable adding runtime functionality, including an EVM, a Bitcoin bridge, DeFi protocols, governance, and many more.

If we look under the hood of substrate, we have great freedom to change the implementation of the chain.

<!-- BOB is aiming to be compatible with the Coinbase and Binance plus there’s a push to make OP Stack chains interoperable. Let’s compare how the OP Stacks works and where we would be making changes while staying compatible with the OP Stack rollup itself. -->

BOB draws inspiration from Madara (https://docs.madara.wtf/), a Starkware sequencer written in Rust/substrate utilizing the exiting STARK Rust libs. Compared to both Arbitrum (https://developer.arbitrum.io/inside-arbitrum-nitro/#geth-at-the-core) and OP Stack (https://stack.optimism.io/docs/understand/landscape/#execution), BOB utilizes Rust with a WASM execution throughout instead of modifying geth with extensions and modifications.
