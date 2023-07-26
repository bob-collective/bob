---
sidebar_position: 4
---



# Roadmap

- EVM compatibility
  - Store state in EVM-compatible format, i.e., data-type and encoding
  - Support interacting with the chain via ethers.js
  - Support etherscan
  - Support for EVM-wallets (MetaMask, WalletConnect, Ledger, ...)
  - Support for Account Abstraction tooling (Safe, ...)
  - Support custodians (Fireblocks, ...)
- Developer tooling
  - Support EVM builder tools (hardhat, foundry, ...)
  - Add foundry support for Rust smart contracts
  - Allow forking of mainnet for testing
  - Cross VM calls (Safe interacts with Rust contract, ...)
  - Builder documentation, guides, and examples
  - Block explorer support
  - GraphQL support (squid or The Graph)
  - Hackathon templates
- OP Stack compatibility
  - Data availability layer support/relayer
  - Sequencing implmentation
  - Execution layer
  - Settlement layer
- Bitcoin compatibility
  - `rust-bitcoin` support (blockers are SCALE encoding, decoding and pallet-contracts storage layout)
  - support all transaction types in the btc realy (blocked: taproot)
  - Simplify tx proof submission by automating coinbase tx submission
- Scale Bridge
  - Vault staking/nomination
  - Multi-sig vaults
  - TSS vaults
  - Dynamic bridge insurance
  - UI/UX improvements for users
  - UI/UX improvements for vaults
- Custody models
  - Centralized BTC version
  - Customized custody
- DeFi extensions
  - Flash swaps
- External stats
  - DeFiLlama
  - Coingecko
  - Coinmarketcap
- Analytics
  - Accounts
  - Transactions
  - Volume
  - TVL
  - Projects
- Security
  - Increased bug bounty program
  - Monitoring
  - Circuit-breakers
  - 24/7 team availability
  - Emergency procedures
  - Blue/red team

