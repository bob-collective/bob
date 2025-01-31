---
sidebar_position: 2
sidebar_label: Stack Overview
---

# Stack Overview

## Introduction

This page outlines the _BOB Stack_, the infrastructure choices we made on our pragmatic path to building a [Hybrid L2](/learn/introduction/roadmap).

Each part plays a unique role in powering BOB's vision: combining the security and liquidity of Bitcoin with the innovation and activity of Ethereum's vibrant DeFi ecosystem.

### Rollup Layer

_Bitcoin security, native Ethereum bridge. Outlook: BitVM_

A traditional Layer 2 settles its transactions or "rolls up" to some Layer 1, inheriting its security. A major motivation for building a Hybrid L2 is to inherit security from _multiple_ Layer 1 networks.

BOB launched on the [OP stack](https://docs.optimism.io/) to be able to roll up to Ethereum for 1-click onboarding of users, assets, and liquidity. This unlocks cutting-edge upgrades to the network, such as the upcoming ZK improvements by [RiscZero](https://www.risczero.com/).

We are currently prototyping [the next phase of our roadmap](/learn/introduction/roadmap/#-phase-2-bitcoin-soft-finality), inheriting finality and security from Bitcoin. This includes a modification to BOB's sequencer to request a sign-off from Bitcoin finality protocol participants to validate the chain state before rolling up to Ethereum.

[BOB's ultimate goal](/learn/introduction/roadmap/#phase-3-full-bitcoin-security) is to settle directly on Bitcoin. While we believe in full-state validation via ZK rollups in the long run, it is very unlikely that Bitcoin will support a ZK verifier in the near future. As such, the best possible design at the moment is an optimistic rollup that leverages on-chain fraud-proofs via mechanisms like [BitVM](/learn/introduction/bitvm/).

### EVM Core

_Smart Contracts, Best-in-Class Infrastructure & Dev Tools_

[Intent-based bridging secured by SPV](/learn/introduction/gateway), [smart accounts](/learn/reference/tools/account-abstraction/) controlled by Bitcoin wallets, and [unilateral exit back to Bitcoin](/learn/introduction/bitvm/), Ethereum, and other L1s - BOB is singularly-focused on bringing this UX and security to Bitcoin DeFi by providing all of these tools to teams building on BOB.

At the core, BOB leverages the [Ethereum Virtual Machine (EVM)](https://ethereum.org/en/developers/docs/evm/) to enable the creation and execution of smart contracts developed using the [Solidity smart contract programming language](https://soliditylang.org/). As always, this choice was motivated by our mission to bring Ethereum's expressive and ergonomic ecosystem of tooling to the next generation of developers looking to build on Bitcoin.

By using the same EVM as Ethereum, teams building on BOB have access to familiar and modern tooling:

- [Wallets](/learn/reference/tools/wallets/), including [Account Abstraction](/learn/reference/tools/account-abstraction/), [Social Login](/learn/reference/tools/social-login/), and [our SDK for Bitcoin wallets](/learn/builder-guides/sats-wagmi/)
- [Data Analytics](/learn/reference/tools/data-indexers/) and [Node Providers](/learn/reference/tools/node-providers/)
- [Cross-chain Messaging](/learn/reference/tools/cross-chain/) and [Oracles](/learn/reference/tools/oracles/)

### BOB SDK

_A smart contract developer's universal toolkit for all things building on Bitcoin_

Similar to [OpenZeppelin](https://www.openzeppelin.com/) and other great Solidity libraries, BOB provides a powerful SDK for all things building on Bitcoin.

This includes a wide range of Solidity contracts that can be used to interact with Bitcoin including core, [Ordinals](https://docs.ordinals.com/), [BRC20s](https://brc20.gitbook.io/brc20/overview/introduction), [Runes](https://rodarmor.com/blog/runes/), and [Lightning](https://lightning.network/), as well as improved inscription APIs and tools for a unified BTC and EVM wallet experience (e.g., manage Ordinals in your MetaMask wallet via [Snaps](https://metamask.io/snaps/)). Plus the ability to leverage [Account Abstraction](https://ethereum.org/en/roadmap/account-abstraction/) with bridged BTC.

### Rust zkVM

_ZK tooling and support for Bitcoin's Rust libraries._

The majority of Bitcoin’s stack and applications built around it are implemented in Rust, including core [SDKs](https://github.com/rust-bitcoin/rust-bitcoin), [Lightning](https://github.com/lightningdevkit/rust-lightning), and [Ordinals](https://github.com/ordinals/ord). BOB can support Bitcoin’s Rust libraries, most notably via the [RISC Zero zkVM](https://dev.risczero.com/api/zkvm/) that allows off-chain execution of Rust programs while using [ZK proofs](https://ethereum.org/en/zero-knowledge-proofs/) to verify correct execution in EVM smart contracts. In the future, we see this as an avenue for [ZK rollups](https://vitalik.ca/general/2021/01/05/rollup.html) directly on Bitcoin where BOB itself can be proven in the zkVM and verified by Bitcoin consensus.

### Bitcoin Bridges

_Trustless P2P swaps, threshold cryptography, and intents-based bridging. Outlook: BitVM Bridge_

BOB provides trustless access for smart contracts to read Bitcoin block and transactional data via a [BTC light client](https://blog.threshold.network/blockchain-relays-101/). Trustlessly reading Bitcoin data within EVM smart contract logic unlocks a broad design space, such as P2P native BTC swaps, Ordinal auctions, and hashrate tokenization.

[BOB Bridge](/learn/user-guides/onboard-to-bob/bob-gateway/) is our production bridge built on that same light client verification. As of this writing, 30,000 users have traded a total of 75 BTC in a trust-minimized, peer-to-peer way.

[BOB Stake](/learn/user-guides/bob-stake/) extends this idea by executing users' "intents" to stake, restake, or lend their BTC automatically during the bridge process. There are [more than a dozen options available](https://app.gobob.xyz/en/stake) at the moment, some offering five different sources of yield. All options are BTC-denominated and have no risk of impermanent loss.

We built the [BOB Gateway SDK](/learn/builder-guides/gateway) to make it possible for any frontend to offer its users these options by plugging into our infrastructure.

BOB also supports a range of Bitcoin bridges, both decentralized and institutional. Through a native ETH L1/L2 bridge, BOB has access to market leader [WBTC](https://wbtc.network/) and the more secure, threshold-based version, [tBTC v1](https://threshold.network/). BOB will support advanced bridge models including opt-in collateralization (see [iBTC](https://www.interlay.io/)).

Lastly, [BOB has prototyped](https://blog.gobob.xyz/posts/bob-announces-trust-minimized-bitcoin-bridge-prototype-powered-by-bitvm) a two-way light-client BTC bridge powered by [BitVM](/learn/introduction/bitvm/) in partnership with Fiamma. A bridged version of BTC secured by BitVM is a massive improvement in BTC safety because it relies on a 1-of-N trust assumption for operation and unlocks unilateral exit back to Bitcoin.

## Conclusion

BOB is committed to improving the security and UX for Bitcoin DeFi at every layer: inheriting security and finality from Bitcoin, building a better UX for self-custody when staking BTC, and giving all users' unilateral exit to bridge their assets back to their original L1s. Our Hybrid L2 has already made inroads on all of these goals, with major upgrades to each of them in line of sight.
