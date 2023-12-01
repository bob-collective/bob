---
sidebar_position: 1
---
# BOB on the OP Stack

The first deployment of BOB will be using the [OP Stack](https://stack.optimism.io) and seeks to be compatible with the Superchain to allow users to easily transfer assets from Ethereum and other OP Stack rollups such as Optimism and Base in addition to the easy access to Bitcoin that comes with the BOB stack itself.

Below we are giving a description of how BOB is adopting the OP Stack layers and where changes are made.

## Goverance

BOB will have its own governance to reach community decisions and enact changes.

The BOB OP Stack rollup will be partly subject to OP governance with the Superchain upgrade. A OP Security Council will be capable of enacting security upgrades of the OP chain and L1 and L2 bridge contracts.

## Data Availability

A Data Availability (DA) layer stores the raw inputs to the state transition function for the execution layer. The OP Stack supports [multiple DAs but sets Ethereum as the de-facto DA](https://stack.optimism.io/docs/understand/landscape/#data-availability). The OP Stack Superchain is considering a dedicated [Plasma DA chain](https://stack.optimism.io/docs/understand/explainer/#alt-data-availability-layer-plasma-protocol) to address rising cost and limited scalability of Ethereum as a DA layer.

BOB will follow the OP Stack standard to retain Superchain compatibility.

However, we will lobby to offer alterantive DA layers that are in our opinion more closely aligned with Bitcoin:

- Bitcoin: The Celestia team has a specification for [using Bitcoin as a DA layer](https://github.com/rollkit/bitcoin-da/blob/main/spec.md) that suits BOB needs and would ensure compatibility with Celestia deployments. Using Bitcoin as a DA layer would introduce complexity around wallets as users would need to provide BTC alongside their EVM transactions to pay for DA cost.
- Celestia: If Celestia were to add an option to pay for DA cost in (bridged) Bitcoin, Celestia might become a cheaper and decent alterantive to using Ethereum or Bitcoin as a DA layer.

## Sequencing

The OP Stack uses a single, centralized sequencer. BOB will use the same sequencer model for its launch but with the addition of merged mining, add functionality to verify the work of the sequencer by Bitcoin miners.

We are welcoming efforts to decentralize sequencers pushed by the OP Stack.

## Execution

OP Stack uses a fork of go-ethereum called op-geth. BOB has no changes to op-geth.

Instead of changing the execution client, necessary features including proving Bitcoin state and paying transaction fees in bridged Bitcoin are added within the EVM as smart contracts. This has the advantage that:

1. The BOB OP Stack deployment stays up-to-date with improvements made by other teams improving the OP Stack without haivng to maintain a custom fork. In turn, this ensures that features available on other EVM chains and rollups are 1:1 applicable to BOB.
2. Technology built on top of the EVM within BOB like the merged mining security, paying in bridged Bitcoin for transaction fees, and the BTC light client can be deployed on any EVM chain. BOB is thus ready for a multi-chain and multi-rollup world.

## Settlement

The OP Stack settles on Ethereum. Currently, the OP Stack is lacking fraud proofs which is a major downside of using OP Stack as users have no possiblity to prove fraudulent behavior by the sequencer.

However, OP Stack has a candidate for fraud proofs in testing and is actively working on increasing efficiency of fraud proofs using ZK.

In stage 1 of Bitcoin security, apps deployed on BOB can verify the Bitcoin merged mining security of BOB to react to issues with the sequencer.

In stage 2 of Bitcoin security, BOB will include merged mining security within the fraud proofs. Based on the verdict of miners, settlement on Ethereum can be prevented.