---
sidebar_position: 4
---

# Roadmap and Contribution

## What do we need to build?

The core things that need building is:

- Adding the base EVM compatibility layer from frontier. Then, frontier would need to be extended to be compatible with OP Stack. First focus needs to be on the op-geth execution engine and then on the settlement/fraud proof contracts.
- Adding Rust smart contracts. Ink is well supported in substrate and offers a developer experience very close to core Rust as it wraps the smart contracts inside a macro to handle its integration into a runtime (https://use.ink/basics/contract-template). From there, contracts need to be provided access to the existing runtime functions like the Bitcoin light client, AMM, lending, bridge, …. For a great dev experience, hackathon templates and a well-documented SDK needs to be created for devs to get started with a new contract as simply as possible. Setting up a new project that can itneract with the Bitcoin light client should take less than 20 minutes.
- Improving the core Bitcoin bridge. We already have the most decentralized Bitcoin bridge and it works as in we have Vaults providing collateral and support for LST and other yield bearing assets as collateral. However, the bridge needs improvements by separating out the asset minting backed by collateral and the cross-chain operation of bridging in and out of Bitcoin.

## What is already done?

We already have key parts of the runtime built and live:

- Basic DeFi functions with a Uniswap v2, Curve v1, and Compound v2 are natively available in the runtime.
- Most decentralized Bitcoin bridge is live since August 2022. This includes a Bitcoin light client that is capable of trustless transaction inclusion proofs of Bitcoin transaction.
- Access to native USDT, USDC (soon), and cross-chain assets via Wormhole (soon).
- Fully decentralized governance with upgradable runtimes. 

## How can you contribute?

- EVM compatibility
  - Store state in EVM-compatible format, i.e., data-type and encoding
  - Support interacting with the chain via ethers.js
  - Support etherscan
  - Support for EVM-wallets (MetaMask, WalletConnect, Ledger, ...)
  - Support for Account Abstraction tooling (Safe, ...)
  - Support custodians (Fireblocks, ...)
- Developer tooling
  - Add foundry support for Rust smart contracts
  - Allow forking of mainnet for testing (chopsticks integration)
  - Cross VM calls (Gnosis Safe multi-sig interacts with Rust contract, ...)
  - Builder documentation, guides, and examples
  - Block explorer support
  - GraphQL support (like squid or The Graph)
  - Hackathon templates
- OP Stack compatibility
  - Data availability layer support/relayer
  - Sequencing implementation
  - Execution layer
  - Settlement layer
- Bitcoin compatibility
  - `rust-bitcoin` support (blockers are SCALE encoding, decoding and pallet-contracts storage layout)
  - support all transaction types in the btc realy (missing taproot)
  - Simplify tx proof submission by automating coinbase tx submission
- Scale Bridge
  - Redesign bridge protocol
  - UI/UX improvements for users
  - UI/UX improvements for vaults
- Custody models
  - Centralized BTC version
  - Customized custody
- DeFi extensions
  - Flash swaps
- Analytics
  - Accounts
  - Transactions
  - Volume
  - TVL
  - Projects
- External stats
  - DeFiLlama
  - Coingecko
  - Coinmarketcap
- Security
  - More external contributors and reviewers
  - Bug bounty program
  - Monitoring
  - Circuit-breakers
  - Emergency procedures
  - Blue/red team
