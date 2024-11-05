---
sidebar_position: 3
sidebar_label: Hybrid L2 Roadmap
---

# Hybrid L2 Roadmap

The [BOB Hybrid L2](https://docs.gobob.xyz/whitepaper.pdf) will launch in multiple phases, improving security with each release.

### Phase 1: Bootstrapping as an Ethereum L2

BOB first launched as an Ethereum L2 built with the [OP stack](https://docs.optimism.io/), operating a native Ethereum
bridge, and supporting multiple 3rd-party Bitcoin bridges. This allows dApps on BOB to build on top of best-in-class developer tooling and new developments like sub-second transaction speeds and [fee token abstraction](/docs/learn/builder-guides/bridged-btc-gas-fee/index.md).

BOB tracks the state of Bitcoin with a BTC light client, verifying block headers and accepting transaction inclusion proofs. **This enables smart contracts on BOB to trustlessly use Bitcoin state in real time**, such as [BOB Gateway](/docs/learn/user-guides/bob-gateway/index.md).

### Phase 2: Bitcoin "Soft" Finality

In Phase 2, BOB will add Bitcoin finality to the Ethereum L2 setup. Once per epoch (one or more BOB blocks), the Sequencer requests a sign-off by participants of the Bitcoin finality protocol, who fully validate the BOB chain state. Using BitVM, we can then construct a trust-minimized Bitcoin bridge secured by this Bitcoin "soft" finality protocol. In other words, to attack the Bitcoin bridge one would need to corrupt the majority of Bitcoin finality protocol participants ([hash rate](https://gobob.xyz/optimine) or BTC stake). The Ethereum bridge will remain secured by Ethereum.

**This Bitcoin "soft" finality can be used to accelerate withdrawals of the Ethereum bridge, reducing the delay from 7 days to a few minutes or hours.**

### Phase 3: Full Bitcoin Security

The final step is inheriting Bitcoin security by posting BOB's state transition proofs on Bitcoin. In the absence of a Bitcoin fork enabling on-chain zk-verifiers, BOB will have to leverage optimistic verification via BitVM. Achieving optimistic rollups on Bitcoin without additional trust assumptions requires using the Bitcoin mainchain as a data availability layer. The associated costs are onerous and pose a challenge in terms of economics. As a result, to complete the transition to Phase 3, BOB must reach sufficient scale in terms of active users such that incurring additional data availability fees does not increase transaction fees beyond that of competing Ethereum L2s. Alternative data availability layers can be considered as a trade-off between cost and security, as they introduce additional trust assumptions beyond that of Bitcoin.

### Further Reading

To learn more about the background, motivations, and technical details of the BOB Hybrid L2, read the [BOB Hybrid L2 Vision Paper](https://docs.gobob.xyz/whitepaper.pdf).
