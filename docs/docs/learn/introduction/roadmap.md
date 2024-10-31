---
sidebar_position: 3
sidebar_label: Rollup Roadmap
---

# Rollup Roadmap

The BOB L2 will launch in multiple phases, improving security with each release.

### Phase 1: Bootstrapping as an ETH L2

_Bootstrapping as an Optimistic Rollup on Ethereum_
BOB first launched as an Ethereum L2 built with the [OP stack](https://docs.optimism.io/), operating a native Ethereum
bridge, and supporting multiple 3rd party Bitcoin bridges.

BOB tracks the state of Bitcoin with a BTC light client, verifying block headers and accepting transaction inclusion proofs. This enables smart contracts on BOB to trustlessly use Bitcoin state, such as [BOB Gateway](/docs/learn/user-guides/bob-gateway/index.md).

### Phase 2: Bitcoin "Soft" Finality

_Hybrid rollup: Ethereum settlement plus Bitcoin security._

In Phase 2, BOB will add Bitcoin finality to the Ethereum L2 setup. Once per epoch (one or more BOB blocks), the Sequencer requests a sign-off by participants of the Bitcoin finality protocol, who fully validate the BOB chain state. Using BitVM, we can then construct a trust-minimized Bitcoin bridge secured by this Bitcoin "soft" finality protocol. In other words, to attack the Bitcoin bridge one would need to corrupt the majority of Bitcoin finality protocol participants (hash rate or BTC stake). The Ethereum bridge will remain secured by Ethereum.

Thereby, Bitcoin "soft" finality can be used to accelerate withdrawals of the Ethereum bridge, reducing the delay from 7 days to a few minutes/hours.

### Phase 3: Full Bitcoin Security

_BitVM rollup: Bitcoin settlement & on-chain fraud proofs, ETH L1/L2 bridge and BTC fraud-proof handling_

<!--The first step to achieving BOB's transition to an optimistic rollup settling on Bitcoin is to design a BOB light client in BitVM, i.e., as an off-chain program committed to by N operators such that on-chain challenges can be executed. Once completed, the next step is to [use BitVM to create a 2-way bridge](https://github.com/BitVM/BitVM/blob/main/docs/sidechain_bridges.md) between Bitcoin and BOB using bi-directional light clients. From there, the final step is to expand the BitVM program from light client and bridge to fully validate the BOB rollup state / accept (ZK) fraud proofs.-->
<!---->
<!--Thereby it may be necessary to introduce ZK compression to BOB, most likely as part of the fraud proof mechanism to ensure more efficient equivocations in case of sequencer failure. This upgrade will occur in collaboration with ZK infrastructure providers, such as [RISC Zero](https://www.risczero.com/news/altlayer-zkfraudproofs).-->
<!---->
<!--BOB's vision is to ensure that Bitcoin assets bridged to the rollup are secured by Bitcoin, wheres ETH assets are secured by Ethereum.-->

The final step is inheriting Bitcoin security as described in Section 4.1. In the absence of a Bitcoin fork enabling on-chain zk-verifiers, BOB will have to leverage optimistic verification via BitVM. Achieving optimistic rollups on Bitcoin without additional trust assumptions, requires using the Bitcoin mainchain as data availability layer. The associated costs are onerous and pose a challenge in terms of economics. As a result, to complete the transition to Phase 3, BOB must reach sufficient scale in terms of active users such that incurring additional data availability fees does not increase transaction fees beyond that of competing Ethereum L2s. Alternative data availability layers can be considered as a trade-off between cost and security, as they introduce additional trust assumptions beyond that of Bitcoin.

![BitVM BOB](bitvm-bob.png)
