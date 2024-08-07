---
sidebar_position: 3
sidebar_label: "Phase 1: Optimistic BOB"
---

# Phase 1: Optimistic BOB

The first deployment of BOB uses the [OP Stack](https://stack.optimism.io), benefiting from infrastructure, marketing, and BD support. Eventually, The BOB OP-Stack deployment can be [Superchain-compatible](https://www.optimism.io/superchain) making it easy for users to move seamlessly between different Superchain rollups.

We are working on extending the functionality of the OP stack specifically for Bitcoin: (1) fees payable in tokenized BTC via [OpenGSN](https://opengsn.org/) and [account abstraction](https://ethereum.org/en/roadmap/account-abstraction/) to ensure easy on-ramp from Bitcoin, and (2) in [phase 2](/docs/learn/bob-stack/merged-mining), verify the rollup state through Bitcoin miners.
When possible, we implement smart contracts directly on the EVM to ensure OP stack updates can be directly applied to BOB.

## Why OP Stack?

There are several great options for rollups including building them from scratch or using zk rollups ([Starkware](https://starkware.co/), [zkSync](https://zksync.io/), [Polygon](https://polygon.technology/polygon-zkevm), [Scroll](https://scroll.io/), ...) and optimistic rollups ([Arbitrum](https://arbitrum.io/), [OP Stack](https://docs.optimism.io/stack/getting-started), ...). BOB's key values are experimentation, real-world impact, and freedom of choice.

In our opinion, OP Stack aligns best with these values as it allows developers to quickly deploy new protocols without having to reimplement existing protocols in new programming languages. It also gives users choice as the Superchain concept allows users to freely move between OP Stack rollups. We have further considered Arbitrum, but opted against this since, under the hood, Arbitrum uses WASM and has more incompatibilities than OP Stack (albeit the team is working very hard to mitigate those). ZK rollups are too untested for our preference and require smart contract modifications (type 3 and 4 zkEVM rollups), are slow (type 1 and 2 zkEVM), or require complete rewrites of wallet software and integrations (Starkware, ...).

BOB is built so that developers can deploy any EVM smart contract without changes.

## Adopting the OP Stack

BOB modifies the standard OP Stack:

### Governance

BOB will have its own governance to reach community decisions and enact changes. Governance will be rolled out gradually as the community grows.

If BOB chooses to be comptabile with the Superchain, then parts of BOB will be subject to OP governance with the Superchain upgrade. An OP Security Council will be capable of enacting security upgrades of the OP chain and L1 and L2 bridge contracts.

### Data Availability

A Data Availability (DA) layer stores the raw inputs to the state transition function for the execution layer. The OP Stack supports [multiple DAs but sets Ethereum as the de facto DA](https://stack.optimism.io/docs/understand/landscape/#data-availability). The OP Stack Superchain is considering a dedicated [Plasma DA chain](https://stack.optimism.io/docs/understand/explainer/#alt-data-availability-layer-plasma-protocol) to address the rising cost and limited scalability of Ethereum as a DA layer.

BOB is using Ethereum as DA, but we are investigating alternatives for future optimization:

- Bitcoin: The Celestia team has a specification for [using Bitcoin as a DA layer](https://github.com/rollkit/bitcoin-da/blob/main/spec.md) that suits BOB needs and would ensure compatibility with Celestia deployments. Using Bitcoin as a DA layer would introduce complexity around wallets as users would need to provide BTC alongside their EVM transactions to pay for DA costs.
- Celestia: If Celestia were to add an option to pay for DA cost in (bridged) Bitcoin, Celestia might become a cheaper and decent alternative to using Ethereum or Bitcoin as a DA layer.
- Merged mined DA chains: Specialized DA chains that inherit security from Bitcoin via merged mining. This would be the most secure and scalable solution but requires significant development effort.

### Sequencing

The OP Stack uses a single, centralized sequencer. BOB will use the same sequencer model for its launch but with the addition of merged mining, add functionality to verify the work of the sequencer by Bitcoin miners.

We are welcoming efforts to decentralize sequencers pushed by the OP Stack.

### Execution

OP Stack uses a fork of go-ethereum called [op-geth](https://op-geth.optimism.io/). BOB has no changes to op-geth.

Instead of changing the execution client, necessary features including proving Bitcoin state and paying transaction fees in bridged Bitcoin are added within the EVM as smart contracts. This has the advantage that:

1. The BOB OP Stack deployment stays up-to-date with improvements made by other teams improving the OP Stack without having to maintain a custom fork. In turn, this ensures that features available on other EVM chains and rollups are 1:1 applicable to BOB.
2. Technology built on top of the EVM within BOB like the merged mining security, paying in bridged Bitcoin for transaction fees, and the BTC light client can be deployed on any EVM chain. BOB is thus ready for a multi-chain and multi-rollup world.

### Settlement

The OP Stack settles on Ethereum. Initially, the OP Stack lacked fraud proofs which was a major downside of using OP Stack as users could not prove fraudulent behavior by the sequencer. On July 10th 2024, the OP-stack used by the BOB chain switched to a full fraud-proof enabled implementation, thus ensuring honest sequencer operation. For further information, please refer to the [OP stack documentation](https://docs.optimism.io/builders/notices/fp-changes).

We are working towards an approach to [add merged mining security to the rollup](./merged-mining.md) by making a PoW part of state transition. Consequently, a lack of PoW constitutes a fault by the sequencer. Thereby, Bitcoin miners validate the rollup to offset the trust in the sequencer (in both the centralized and decentralized sequencer case).
