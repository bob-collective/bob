---
sidebar_position: 3
sidebar_label: 'Phase 1: Optimistic BOB'
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

### Settlement & Fraud Proofs

The OP Stack settles on Ethereum. Initially, the OP Stack lacked fraud proofs which was a major downside of using OP Stack as users could not prove fraudulent behavior by the sequencer. On July 10th 2024, the OP-stack used by the BOB chain switches to a full fraud-proof enabled implementation, thus ensuring honest sequencer operation. 

:::info Important Notice for Bridges and Users
**ALL** withdrawals that are not finalized before the Fault Proofs upgrade executes will need to be reproven after the upgrade is complete.

  *   Reproving simply requires that you execute the withdrawal proving flow again.
  *   Withdrawals prior to the Fault Proofs upgrade must wait a 7-day challenge period before finalization. As a result, any withdrawal initiated less than 7 days before the upgrade cannot be finalized before the upgrade is executed. You may want to consider waiting until after the upgrade is complete to begin a withdrawal during this 7-day window.
:::


#### Overview of Changes

If you are operating a custom bridge, review this section for changes you need to make. If you are using Optimism SDK or Viem for your bridging, you can [skip to the next section](#for-bridges-and-centralized-exchanges).

The `L2OutputOracle` is being entirely removed and replaced by the `OptimismPortal` and `DisputeGameFactory`. The `L2OutputOracle` smart contract is currently used by the trusted Proposer role to store L2 state output proposals.
Presently, developers use these outputs to prove that their withdrawals actually happened on L2. But with fault proofs, developers will have to change how their client software proves withdrawals in the first step of the two-step withdrawal process.

##### `L2OutputOracle`

The `OptimismPortal` is changing slightly with Fault Proof Mainnet because it now points to the `DisputeGameFactory` contract instead of the `L2OutputOracle` contract.
Most notable change for developers is that withdrawals will have to be proven against the `rootClaim` of some dispute game rather than referencing an output in the `L2OutputOracle` contract.

##### `OptimismPortal`

The `OptimismPortal` smart contract is the low-level contract on L1 used to execute deposits and withdrawals.
Previously, developers would look at the `L2OutputOracle` to find the exact next output that included their withdrawal.
Now, developers must look at the `OptimismPortal` contract to determine the `respectedGameType` and then use this information to query the `DisputeGameFactory` for a list of recent `DisputeGame` contracts with the correct game type.

##### `DisputeGameFactory`

It is recommended that developers search for a reasonable number of recent games, say 100 games, and pick the first proposal with a sufficient block number.
Developers should then verify this proposal locally as the default game type will allow for permissionless proposals and there is no longer a strong guarantee that proposals will be valid.

#### For Bridges and Centralized Exchanges

Enable the fault proofs impacts bridges, centralized exchanges, and custom solutions that use withdrawals.

:::info NOTE
  Withdrawals that haven't finalized before the upgrade occurs will be unable to be finalized post-upgrade without reproving. This means these withdrawals will need to go through a new 7-day period. The time accrued before the upgrade will not count.
  This means the withdrawal time could be as long as 13-14 days during the upgrade window. (If you submit it \~6 days before the upgrade, then must re-prove after the upgrade, you will have to wait a new seven days.)
:::

  ##### Update Logic to Support Fault Proofs

:::info NOTE

Most teams use the Optimism SDK or Viem to handle this logic under the hood and will simply need to update their software version ahead of the upgrade. However, any project performing withdrawals programmatically will need to update their client code (see [`OptimismPortal`](#optimismportal) for details).

:::

  *   **Option 1: Optimism SDK Update.** If you use OptimismSDK for bridging, simply update to version 3.2.0 or higher.
      The Optimism SDK changes do not break the API and require no changes other than updating to the correct software version to support the new `OptimismPortal` logic. The Optimism SDK will automatically begin to use the new logic once it detects that the FPM update has gone live.
  *   **Option 2: Viem Update.** Update to the latest version of Viem (version of >=2.9.0). Viem will automatically begin to use the new logic once it detects that the FPM update has gone live.

  ##### Update Withdrawal Monitor

  The Withdrawal Monitor service is an important part of the two-step withdrawal system that checks if anyone has been able to prove withdrawals that do not actually appear on L2.
  The Withdrawal Monitor is now slightly slower at startup time but is more reliable, simpler, and compatible with more infrastructure to more easily support any OP Stack chain.
  Changes to the Withdrawal Monitor are entirely backwards compatible.

  *   **Option 1: Withdrawal from source.** If building or using withdrawal-monitor from source, make sure that you are using the latest develop branch. All withdrawal monitor changes are fully backwards compatible.
  *   **Option 2: Docker image.** If using docker, use the latest chain-mon docker image.

  ##### Update Dispute Monitor

  The Dispute Monitor service detects when invalid outputs have been proposed. Given the large number of changes to the output proposal system, the current Fault Monitor is being replaced in favor of a new Dispute Monitor purposely built for the fault proof monitor system.

:::info NOTE
    Any team running the current Fault Monitor will see the monitor stop functioning when the Fault Proof Monitor upgrade goes live. These teams should run the new service and update their alerting accordingly.
:::

#### Next Steps

*   See the [Fault Proofs Explainer](https://docs.optimism.io/stack/protocol/fault-proofs/explainer) to learn more about the three main components of the Fault Proof System.
*   You can also read more about [Cannon FPVM](https://docs.optimism.io/stack/protocol/fault-proofs/cannon) and [Mips.sol](https://docs.optimism.io/stack/protocol/fault-proofs/mips), the onchain smart contract implementation of Cannon.


### Settlement & Merged mining

We are working towards an approach to add merged mining security to the rollup by making a PoW part of state transition. Consequently, a lack of PoW constitutes a fault by the sequencer. Thereby, Bitcoin miners validate the rollup to offset the trust in the sequencer (in both the centralized and decentralized sequencer case).
