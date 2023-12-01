---
sidebar_position: 1
---
# Technical Vision

We see BOB as a collective undertaking to scale Bitcoin the right way: inheriting security from Bitcoin while providing usable decentralization for builders today without waiting for hardforks.

The technical vision outlines the endgame for BOB and describes how it differs from the first deployments. The discrepency between endgame and current state forms opportunities to contribute and push the BOB stack further.

## Bringing Bitcoin Security to Rollups on Any Chain

We belive that rollups and sidechains should receive their consensus security from Bitcoin rather than relying on other L1s.

### Ideal World: Verifiably Secure through Zero Knowledge Proofs

To inherit full Bitcoin security, Bitcoin miners need to validate a BOB rollup state transition as part of Bitcoin consensus such that the rollups state transition function becomes part of the Bitcoin state transition function. No current Bitcoin sidechain achieves this level of security as this will require a hard fork of Bitcoin.

In our opinion, non-interactive zero-knowledge proofs (NIZKP) are the ideal candidate to achieve full Bitcoin security as the verification of the proof is much simpler than its creation. The creation of the proof can be done by the rollup nodes. Bitcoin would need to add an OP code to verify ZK proofs or there would need to be great improvements in the efficiency of [BitVM](https://bitvm.org) to be able to verify ZK proofs.

However, adding a ZK verification OP code and being able to verify ZK proofs in BitVM will likely take years as there is still heavy changes around ZK proofs and no "gold standard" has yet emerged.

### Hardfork-free Bitcoin Security through Merged Mining

As we expect that verifying ZK proofs is still a long way to go, the next best thing to inhert Bitcoin security for BOB rollups is to allow miners to verify rollup state transitions. Miners can opt-in to verify rollups and if all miners were to merge mine a rollup, it would have equivalent PoW security as Bitcoin itself.

The process of merged mining allows a Bitcoin miner to:

1. Run a full node of the rollup to verify blocks produced by rollup sequencers (or become a sequencer). This removes trust from the sequencer.
2. Fetch the latest block hash and include the block hash into the coinbase transaction of a Bitcoin block.
3. Submit Bitcoin blocks that meet the difficulty target of the BOB rollup to the rollup.

When using Ethereum rollups, Ethereum becomes a co-processor to Bitcoin where the computation happens on the EVM rollup but the verification is done by the miner.

We see a staged approach to make use of the rollup PoW:

- Stage 1: Dapps deployed on an EVM rollup can check the submitted PoW to the rollup and customize their interpretation. For example, a ordinals P2P exchange might want to pause their platform if insufficient PoW is attached to the rollup, Other dapps might ignore the PoW alltogether, making the model quite flexible.
- Stage 2: Merged minig becomes a condition for a valid state transition of the rollup. Assuming that a rollup is launched on an L1 with smart contracts like Ethereum, the sequencer needs to ensure that at certain intervals a sufficient PoW is added. The rollup would be paused and its state could be invalidated through including the PoW as part of the fraud proofs (in optimistic rollups) and validity proofs (in zk rollups). Requiring PoW as part of a valid state transition for the rollup ensures that the state transition of the rollup on say Ethereum cannot settle without the explicit consent from Bitcoin miners.
- Stage 3: Staking sequencers on Bitcoin and proving incorrect behavior through BitVM, one-time signatures, and other techniques ensures that sequencers are ecnomically incentivized on Bitcoin to correctly produce L2 blocks.
- Stage 4: In the final stage, the rollup transitions to a fully zk-verified rollup that can then be verified by Bitcoin consensus without merged mining. We expect this to take about five years from now as it will require zk technology to mature and Bitcoin made capable of verifying zk proofs.

:::info
We will share a technical paper on optimistic sequenced merged mining detailing the technical protocol soon.
:::

:::note BOB Launch PLan
BOB launches as an [optimistic rollup using the OP Stack](op-stack) which may seem counterintuitive to the above goal of eventually being a ZK rollup. However, we see having full EVM compatibility and tooling a worthwhile trade-off to adopting zkEVM rollups. Moreover, we see extremely promising progress around abstracting the entire EVM execution into higher level zkVM like Risc Zero. Executing entire rollup blocks in a zkVM requires no changes to the EVM while still allowing validity proof production that then can eventually be used for a Bitcoin verified ZK rollup.

We further will launch the merged mining option shortly after BOB will go live in the stage 1 option described above.
:::

## Usable Decentralization

Centralization is plaguing development on Bitcoin today. Due to Bitcoin's limited programmability, many applications building on Bitcoin, like the majority of Lightning wallets, are centralized as it allows for a better UX than their dencentralized counterparts.

Lack of decentalized appliocations with great UX is a major issue that can be resolved by (1) allowing more epxressive smart contracts and piggy-backing off of the developments made on Ethereum and other L1 chains, and (2) ensuring that the EVM rollups are still secured by Bitcoin.

### Adoption Through UX

- Readily available assets for Bitcoin users by native bridged (rollups) to Ethereum
- Unified UX of Bitcoin with BOB-enhanced rollups will win out
- Mass adoption will need privacy 

## Multi-Chain and Multi-Rollup Future

- BOB is not a single rollup: different rollups can make different trade-offs for various use cases. Examples: very fast block times with limited contracting for payments, large blocks for storing ordinals, ...
- BOB is an enhancer of existing sidechains and rollups and can be added to existing EVM chains

## Off-chain Computation before On-chain Computation

- Even in the EVM, some computations/programs might still be too complex
- Complex programs like a BRC20 or Ordinals co-processor should be operated off-chain and its correct execution proven on-chain
- Simple programs can be kept on-chain