---
sidebar_position: 2
sidebar_label: "BitVM"
---

# BitVM: A New Paradigm for Bitcoin

## Introduction

Building on Bitcoin is fun once again!

BitVM plays a crucial role in BOB's [Hybrid L2 Roadmap](/learn/introduction/roadmap) by [enabling Bitcoin finality](https://blog.gobob.xyz/posts/bob-integrates-with-babylon-to-become-a-bitcoin-secured-network-bringing-bitcoin-finality-to-the-hybrid-l2) for our rollup. BitVM also makes it safer for BTC holders to participate in DeFi on BOB by [minimizing the trust needed to bridge](https://blog.gobob.xyz/posts/bob-announces-trust-minimized-bitcoin-bridge-prototype-powered-by-bitvm). From the network infrastructure to the tokens in your wallet, BitVM unlocks huge security advancements for Bitcoin DeFi.

Read on to learn how BitVM enables Bitcoin rollups and a new type of BTC bridge with a "1-of-N" trust assumption - all without a soft fork to Bitcoin.

## Why BitVM Matters

Before we dive into technical details, let's introduce a few products that BitVM makes possible on Bitcoin.

BitVM enables:

- Vastly improved bridge design: users can deposit and withdraw BTC from an L2 without trusting multi-sig custodians
  - [BOB is developing a trust-minimized BitVM bridge with Fiamma](https://blog.gobob.xyz/posts/bob-announces-trust-minimized-bitcoin-bridge-prototype-powered-by-bitvm)
- Bitcoin rollups that verify the Layer 2 state directly on Bitcoin
  - [BOB will receive Bitcoin finality from Babylon](https://blog.gobob.xyz/posts/bob-integrates-with-babylon-to-become-a-bitcoin-secured-network-bringing-bitcoin-finality-to-the-hybrid-l2)
- Significant decreases in the trust assumptions that Bitcoin users make when interacting with off-chain protocols
- An entirely new design space for applications built directly on Bitcoin

## BitVM Non-Technical Summary

_BitVM_ is a way to expand the range of what is possible to do on Bitcoin. It does this by performing computation off-chain that is held accountable by an on-chain fraud proving mechanism. Since Bitcoin Script is very limited in what it can do, running more advanced programs on Bitcoin requires a way to verify that the results of off-chain calculations are correct.

Existing bridges typically rely on centralized entities—such as Wrapped Bitcoin (WBTC) and Coinbase Wrapped Bitcoin (cbBTC)—or semi-trusted networks like tBTC, where security depends on the honesty of the majority of participants. In contrast, BitVM bridges introduce a superior security model: BTC deposits cannot be stolen as long as there is a single honest and online node in the network, and this node can be the depositor themself.

BitVM verifies off-chain computation similarly to optimistic rollups:

1. An _operator_ performs a computation off-chain,
1. then makes an on-chain claim about the results of that computation.
1. If the claim is incorrect, a _verifier_ will invalidate it directly on the Bitcoin blockchain.

In practice, this would look like a collection of operators depositing their collateral and committing to manage a pool of BTC according to the rules of a specific, pre-defined computer program (a bridge, for example).

**If an operator attempts to withdraw BTC in a way that violates those rules, any verifier can prevent the withdrawal and slash the dishonest operator's collateral for the malicious attempt.** The verifier does this by submitting a specific kind of transaction on Bitcoin that will be explained in a moment.

This reveals two important aspects of the setup:

1. An operator only needs to provide enough collateral to cover the cost of disproving plus some extra amount to deter them from cheating and being slashed. This capital efficiency is exactly why BitVM is much better than other bridges, including collateralized bridges.
2. The verifier can challenge a dishonest transaction directly to the blockchain, slashing the operator and preventing the malicious withdrawal from happening. In other words, one honest verifier can prevent an infinite number of operators from cheating.

This setup is called _optimistic_ because normal operation is assumed to be correct, allowing compute to happen at web2 prices while also maintaining a strong mechanism of on-chain accountability to prevent a dishonest claim from any operators.

## BitVM Technical Summary

1. Compress a program into a SNARK verifier implemented in Bitcoin Script. With Groth16, this will be about 1GB in size.
1. Split the verifier into discrete sub-program chunks that are a maximum of 4MB each. This makes it possible for an entire specific sub-program chunk could be included in a Bitcoin transaction in the event of a challenge.
1. During the setup step, each operator commits to the program by depositing collateral.
1. Each operator continually runs the original program (e.g. a bridge) off-chain to determine the correct outputs of each input (e.g. how much BTC to withdraw and who to send it to during a bridge peg-out).
1. The operator uses their own funds to cover the program's output (e.g. sends their own BTC to a withdrawing user), then requests a reimbursement by withdrawing funds from BitVM (i.e. the set of funds being managed by the operators).
1. If challenged, the operator must reveal all of the intermediary program results (i.e. each of the inputs and outputs of the sub-program chunks). If the operator is cheating, at least one of the sub-program results will be fake.
1. **Anyone can disprove the operator by executing that specific sub-program in a Bitcoin transaction, showing that the operator's attempted withdrawal was invalid.**
1. The bad operator is kicked out and cannot access the BitVM funds.

:::tip Practical and Permissionless
Anyone can challenge a dishonest operator with only three Bitcoin transactions.
:::

## BitVM Bridge Summary

One of the most exciting applications of BitVM is a BTC [light-client](/learn/builder-guides/relay) bridge on Bitcoin. Specifically, BitVM reduces the trust assumptions needed for a safe _peg-out_ when users want to withdraw or unwrap their BTC back to Bitcoin.

1. Operators pay BTC to the withdrawing user from their own funds and then reclaim the BTC from BitVM.
1. BitVM checks that there is a correct peg-out on Bitcoin matching the unwrap transaction on the L2.
1. If all is correct, the operator gets the BTC refunded.
1. If the operator has committed fraud (e.g. by sending BTC to their wallet instead of the user's), the refund transaction is prevented and the operator is slashed and kicked out.

Under correct operation, the bridging process completes in less than one hour in each direction. This is much faster than existing Ethereum L1 or L2 bridges to Bitcoin.

## Common Misconceptions

- "BitVM" is not a virtual machine (VM) like the Ethereum Virtual Machine (EVM) or the Java Virtual Machine (JVM); it is not an execution environment for arbitrary programs. Rather, each instance of BitVM is a way to use existing tools in Bitcoin to create on-chain fraud proofs for off-chain execution of a specific, pre-determined program (e.g. a bridge or rollup).
- In its current design, a specific BitVM instance only supports deposits and withdrawals of a fixed amount defined at setup, such as exactly 0.1 BTC. A user can only `PegOut` (i.e. withdraw) an amount that exactly matches a previous `PegIn`. A potential workaround is to create several instances of BitVM for the same purpose (e.g. bridge), each corresponding to a different `PegIn` size (e.g. 0.1 BTC, 1 BTC, 10 BTC).
- The 4GB program is never put on-chain. That would require 1,000 separate transactions, each the size of an entire Bitcoin block! Instead, the setup process includes the creation of a Taproot tree that represents the relationship between the inputs and outputs of all the intermediary sub-programs. The last Tapleaf in the tree is used for the `Payout` transaction, enabling the operator to claim the funds if the program execution is not challenged.
- Operators pay out using their own money, then are _reimbursed_ by spending the UTXO from a previous `PegIn` after the challenge period. These settings (challenge period duration, operator capital) are what constrain the throughput of the bridge(/other collateralized program)
