---
sidebar_position: 1
---

# Overview

## Introduction

Bitcoin users can easily onboard to the BOB Hybrid Chain without previously holding any Ethereum assets. This page explains the technology behind the [BOB BTC Bridge](https://app.gobob.xyz/en?type=deposit&network=bitcoin&receive=WBTC) and [BOB Earn](https://app.gobob.xyz/en/stake): _BOB Gateway_ is an intent-based bridge that coordinates peer-to-peer swaps between users and liquidity providers (LPs).

Cross-chain transfers are secured by verifying Bitcoin transaction proofs with an [on-chain light client](/docs/bob-chain/relay), avoiding the need for an oracle. Optional intents, such as staking, lending, and swapping tokens can all be accomplished while only requiring a single Bitcoin transaction from the user.

## Bridge or Stake BTC on BOB

Gateway powers the [BOB BTC Bridge](https://app.gobob.xyz/en?type=deposit&network=bitcoin&receive=WBTC) and [BOB Earn](https://app.gobob.xyz/en/stake).

:::info Bridge or Stake BTC on BOB
Check our user guides to learn how to swap your BTC to wrapped BTC or staked BTC.
- [Bridge BTC to BOB](/docs/user-hub/onboard-to-bob/bob-gateway): Receive wBTC or tBTC on BOB.
- [Stake BTC on BOB](/docs/user-hub/stake-btc): Receive xSolvBTC, uniBTC, or other staked BTC on BOB.
:::

## Gateway for Builders

We are interested in working closely with builders looking to connect their smart contracts as new _strategies_ for BOB Gateway users.

These are some of the features we're working on for Gateway's next upgrade, with new ideas being added frequently.

- "Unwrap" your wrapped BTC on BOB by withdrawing back to BTC on Bitcoin mainnet
- "Unstake" your BTC LST/LRT back to BTC on Bitcoin mainnet
- Swapping from BTC to any ERC20 through a DEX (already possible, not fully implemented yet)

:::info Integrate BOB Gateway Into your App
[See our Builder Guide](/docs/gateway): Learn more about our SDK and extending BOB Gateway's functionality to bring Bitcoiners one transaction away from your use-case.
:::

## How Gateway (Onramp) Works

1. Liquidity providers (LPs) temporarily lock wrapped Bitcoin (WBTC or tBTC) in escrow smart contracts on BOB.
1. A user makes a request to the off-chain relayer to reserve some of the available liquidity.
1. The user sends BTC to the liquidity provider's Bitcoin address. A hash of the user's order is included in the `OP_RETURN` of their transaction, including data such as the recipient's EVM address on BOB and their desired strategy (i.e. their intent).
1. The relayer submits a Merkle proof of the user's Bitcoin transaction to an on-chain [Light Client](/docs/bob-chain/relay), granting the relayer permission to withdraw the LP's wrapped Bitcoin. **Encoding the user's order in their Bitcoin transaction makes it possible to trustlessly verify and complete the user's intent without using an oracle.**
1. Gateway sends the LP's wrapped Bitcoin to the user's EVM address. If the user requested a Bitcoin LST/LRT, that token is minted using the LP's wrapped Bitcoin _before_ it is sent to the user.

## Gateway (Onramp) Architecture

<img
src={require("./architecture.png").default}
style={{ width: "100%", maxWidth: "100%", height: "auto" }}
alt="architecture"
/>

### Gateway (Onramp) User Flow

1. A user requests to swap BTC for wrapped BTC (e.g. WBTC, tBTC, or FBTC) or staked BTC (e.g. xSolvBTC, uniBTC).
1. The user gets a "quote" of which Gateway smart contract is an available route (i.e. which LP they can swap with).
1. The user creates an "order" with the relayer to reserve the LP's liquidity.
1. The user creates a Bitcoin transaction, updating the order with their txid. Gateway's SDK creates a hash of the user's intent, which is included in the `OP_RETURN` of the transaction. This hash includes metadata such as the user's EVM address, which strategy they are using (i.e. their intent), and how many sats to convert to ETH for gas, among other data. By including a deterministic hash of the user's order, we lay the groundwork for making Gateway trustless in the future by decentralizing the relayer set.
1. The relayer monitors the Bitcoin chain and sends the LP's wrapped BTC to the user after the txid is seen on Bitcoin. Specifically, the relayer submits a Merkle proof of the user's Bitcoin tx to an [on-chain light client](/docs/bob-chain/relay) for trustless verification of the user's intent. It accomplishes this by requiring that the recipient's EVM address, sequence of smart contract calls, and other order data exactly match the hash in the `OP_RETURN` of the user's Bitcoin transaction.

### Gateway (Onramp) Liquidity Provider (LP) Flow

1. An LP asks the relayer to deploy a new Gateway contract, which functions as an escrow for their funds. This is permissioned at the moment because BOB pays the transaction's gas. See the [Trust Assumptions](#trust-assumptions) section below for more information.
1. The LP deposits wrapped Bitcoin (e.g. WBTC, tBTC, FBTC) in their Gateway contract.
1. The LP can only withdraw their funds or update their swap fees after a delay so that the relayer has time to finish open orders. The relayer will not accept new orders during this delay until reset.

## How Gateway (Offramp) Works

1. Liquidity providers (LPs) register their solver and fund them with native Bitcoin (BTC).
1. A user requests a quote for swapping wrapped Bitcoin (e.g., tBTC, WBTC) to native Bitcoin.
1. The user submits an "offramp order" to the `OfframpRegistry` smart contract, locking their wrapped Bitcoin and specifying their Bitcoin address.
1. A solver (LP) monitors open orders, accepts one, and broadcasts a Bitcoin transaction to the user's specified address. The transaction includes metadata (e.g., order ID) in the `OP_RETURN` field to enable trustless verification.
1. The solver notifies the relayer, which then submits a Merkle proof of the Bitcoin transaction to the on-chain `OfframpRegistry`. This proof unlocks the wrapped Bitcoin, transferring it to the solver as reimbursement.
1. If the order is not fulfilled, users can bump transaction fees to incentivize LPs or unlock their locked assets after a claim delay period.

### Gateway (Offramp) Architecture
<img
src={require("./offramp_architecture.png").default}
style={{ width: "100%", maxWidth: "100%", height: "auto" }}
alt="architecture"
/>

### Gateway (Offramp) User Flow

1. A user requests to swap wrapped Bitcoin (e.g., tBTC, WBTC) for native Bitcoin (BTC).
1. The user receives an "offramp quote", including estimated fees and liquidity availability.
1. The user creates an offramp order by locking wrapped Bitcoin in the `OfframpRegistry` contract.
1. A liquidity provider (solver) accepts the order, sends the Bitcoin transaction to the user’s Bitcoin address, and attaches the order metadata in `OP_RETURN`.
1. If the order is not accepted within a reasonable time, the user can either bump transaction fees or unlock their funds.

### Gateway (Offramp) Liquidity Provider (LP) Flow

1. An LP registers their solver address in `OfframpRegistry` and configures it to accept user orders.
1. The LP runs an `offramp-solver` binary and funds it with Bitcoin (BTC) to fulfill user requests.
1. The solver continuously scans for open user orders and matches those that meet acceptable fee thresholds.
1. Upon finding a matching order, the solver broadcasts a Bitcoin transaction to the user’s address, embedding the necessary metadata.
1. After confirmation, the solver notifies the relayer via API. The relayer submits a Merkle proof to `OfframpRegistry`, unlocking the user's locked wrapped Bitcoin and transferring it to the solver as settlement.

## Adoption and Fees

You can find the current liquidity, LPs, and usage of Gateway on the [BOB Gateway Dune dashboard](https://dune.com/bob_collective/gateway).

LPs can configure fees. Currently, most LPs use the following fee structure:

- less than 0.01 BTC: 20 bps
- 0.01 - 0.1 BTC: 10 bps
- 0.1 - 1 BTC: 7.5 bps
- more than 1 BTC: 5 bps

Frontends can also charge additional fees on top. The BOB bridge and staking page apply a 0% fee on top of the LP fee. By default, LPs are chosen based on available liquidity and the lowest fee.

For an up-to-date info on fees, please see the quotes provided on the BOB bridge and staking pages or use the SDK if you are a builder.

## Trust Assumptions

While development for BOB Gateway is advancing rapidly, there are still some training wheels in place that introduce additional trust assumptions to the architecture. These are going to be addressed and replaced with trustless or trust-minimized equivalents as the product matures.

- Liquidity providers (LPs) receive BTC directly from users on Bitcoin mainnet. As a result, the off-chain relayer cannot interfere with funds received by the LPs.
- The relayer is currently necessary to complete the user's intent (e.g. by submitting a Merkle proof of the BTC transaction to the on-chain light client, unlocking the LP's wrapped BTC). Since users cannot submit the proofs themselves yet, user funds are "stuck" whenever the relayer is offline. Users will be able to submit their own BTC proofs after we transition Gateway use a full Bitcoin light client and add functionality to protect LPs from potentially malicious strategy contracts.
- Another reason that Gateway does not currently accept proof submissions directly from users is to prevent "liquidity sniping." If there were no relayer (i.e. users could reserve liquidity directly from the Gateway LPs for free before the user sends their BTC), it would be possible to reserve all available liquidity, blocking Gateway's operation.
- At the moment there is only one relayer. In addition to its other functionality, this relayer pays gas on the user's behalf so that the user doesn't need to make a transaction on BOB to complete the bridging process. Since a malicious actor could create a strategy designed to take advantage of the relayer (e.g. spend all of the funds available for gas fees), the process of adding new strategies to Gateway as well as decentralizing the relayer role are restricted by the BOB team until Gateway is upgraded as described above.

### Security of the Light Client

Gateway currently uses a "light" Bitcoin light client, which only checks that the submitted Bitcoin block headers meet the current or previous difficulty target. Later, we will transition to a "full" Bitcoin light client and submit every Bitcoin block header to the relay.

The following calculation shows that **it would take ~$12,000,000 of hashrate (excluding the opportunity cost of not mining) to attack the relay** at the time of this writing in December, 2024.

Let's review the calculation given in the [Bitcoin Wiki](https://en.bitcoin.it/wiki/Difficulty) to compute the hashrate:

```
hashrate = difficulty * 2**32 / 600 (60 * 10 = 10 minutes)
hashrate = ~157 (GH/s) = (22012.4941572 * 2**32 / 600) / 10**9 (example)
hashrate = ~743 (EH/s) = (103919634711492.2 * 2**32 / 600) / 10**18 (time of writing, Dec 2024)
```

The `LightRelay` requires that the proof is included at the _current_ or _previous_ difficulty so we can assume the attacker has 2016 \* 2 blocks to brute-force a valid chain of `proofLength`.
This is possible since, due to SPV assumptions, we can not verify the transactions references by the block header are valid, only that sufficient PoW has accumulated on the chain.

Let's assume the attacker can generate 6 blocks (with some invalid transactions) within two difficulty adjustment period, 2016 \* 2 blocks (four weeks).

```
hashrate * 6/(2016*2)
743 (EH/s) * 6/(2016*2) = 1.105 EH/s (~1,105,654 TH/s)
```

So we need ~0.142% of the current hashrate to build six blocks in two weeks.

If we estimate it would cost $11 per TH/s (excluding electricity and other setup costs) then the total cost of that hashrate would be $12,162,194.
This excludes the opportunity cost from actually mining on Bitcoin mainnet, receiving fees and block rewards.

**Therefore, given that the value protected by the relay is less than $12m, protocols secured by the relay are "economically safe".**
