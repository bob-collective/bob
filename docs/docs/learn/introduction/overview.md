---
sidebar_position: 2
sidebar_label: Building on Bitcoin
---

# Building on Bitcoin

BOB positions itself as the builder stack for experimentation, real-world impact, and freedom of choice.

1. **Rust smart contracts**: Provide developers with the ability to use existing SDKs and libraries based on Rust. Extending Bitcoin with new apps (e.g., https://github.com/ordinals/ord and https://github.com/rust-nostr/nostr), libraries (e.g., https://github.com/rust-bitcoin), and SDKs (e.g., https://github.com/lightningdevkit and https://bitcoindevkit.org/) are primarily based on Rust. Existing L2 or sidechains fail to deliver on that as they are purely EVM-based or use other execution layers like Stacks and Liquid. BOB will provide builders with the ability to write smart contracts in Rust and leverage the existing ecosystem.
2. **EVM compatibility**: The EVM is not the greatest VM created. However, it benefits from a large ecosystem that is focused on improving UX via smart contract wallets, innovations in rollups, DeFi, and a maturing set of tooling. Instead of builders having to reinvent the wheel, BOB comes with the EVM built in to provide projects with access to EVM space innovations and not require them to rewrite code in a new programming language. As an added benefit, this eliminates vendor lock-in since projects can deploy on other EVM chains or spin up a BOB-compatible rollup.
3. **Rollup**: Deploying an app-specific rollup will become as simple as deploying a smart contract. With the maturing of standardized rollup stacks, the enhancements made to the rollup itself will be the key differentiators. BOB provides a Bitcoin augmentation layer enabling access to Bitcoin types and data (e.g., BRC20s, ordinals, …) and a BTC bridge to enable access to BTC. This allows builders to focus on the unique value of their application and its impact on users without having to worry about the platform risk of BOB.
4. **Bitcoin bridge**: Most Bitcoin bridging happens through centralized and trusted intermediaries. BOB comes with a better alternative built-in. The Bitcoin bridge allows anyone to provide collateral to synthetically mint BTC on BOB. A P2P swapping protocol will enable them to swap the synthetic BTC for actual BTC. At no point does an entity have to hold bridged BTC - instead a supply and demand-driven marketplace facilites users to onboard and offboard to BOB.

![values](values.png)

## BOB: A Bitcoin L2 for Builders

BOB is three things:

- A builder platform that allows anyone to create novel applications:
  - BOB supports Rust smart contracts. Many Bitcoin innovation (ord, LN, nostr, BDK) happens in Rust, a mature and well-designed language. This allows innovation for new use cases without having to rewrite logic.
  - BOB is EVM-compatible. Novel applications and mature tooling already exist on EVM chains. Innovators can build on top of these applications without having to rewrite existing logic in other programming languages.
- A novel BTC bridge that allows users and builders access to BTC and Bitcoin data (BRC20s, ordinals, …).
- A rollup that allows users and builders access to ETH, ERC20s, and Ethereum data (NFTs, ENS, …).

BOB will be the catalyst for the building on Bitcoin renaissance. The movement combines the Bitcoin core values with new avenues of thought. BOB is a Bitcoin-augmented rollup for free experimentation and innovation with real-world impact.

![BOB Components](bob-components.png)
