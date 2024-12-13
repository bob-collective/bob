---
sidebar_position: 2
sidebar_label: "BOB Gateway: Bitcoin Intents"
---

# BOB Gateway: Bitcoin Intents

## Introduction

Bitcoin users can easily onboard to the BOB Hybrid L2 without previously holding any Ethereum assets. This page explains the structure of _BOB Gateway_, an intent-based bridge that coordinates peer-to-peer swaps between users and liquidity providers (LPs).

Cross-chain transfers are secured by verifying Bitcoin transaction proofs with an [on-chain light client](/learn/builder-guides/relay), avoiding the need for an oracle. Optional intents, such as staking, lending, and swapping a small amount of ETH for gas can all be accomplished while only requiring a single Bitcoin transaction from the user.

## Next Steps for Gateway

We are interested in working closely with builders looking to connect their smart contracts as new _strategies_ for BOB Gateway users.

These are some of the features we're working on for Gateway's next upgrade, with new ideas being added frequently.

- "Unwrap" your wrapped BTC on BOB by withdrawing back to BTC on Bitcoin mainnet
- "Unstake" your BTC LST/LRT back to BTC on Bitcoin mainnet
- Swapping from BTC to any ERC20 through a DEX (already possible, not fully implemented yet)

:::info Integrate BOB Gateway Into your App
[See our Builder Guide](/learn/builder-guides/gateway) to learn more about our SDK and extending BOB Gateway's functionality to bring Bitcoiners one transaction away from your use-case.
:::

## How Gateway Works

1. Liquidity providers (LPs) temporarily lock wrapped Bitcoin (WBTC or tBTC) in escrow smart contracts on BOB.
1. A user makes a request to the off-chain relayer to reserve some of the available liquidity.
1. The user sends BTC to the liquidity provider's Bitcoin address. A hash of the user's order is included in the `OP_RETURN` of their transaction, including data such as the recipient's EVM address on BOB and their desired strategy (i.e. their intent).
1. The relayer submits a Merkle proof of the user's Bitcoin transaction to an on-chain [Light Client](/learn/builder-guides/relay), granting the relayer permission to withdraw the LP's wrapped Bitcoin. **Encoding the user's order in their Bitcoin transaction makes it possible to trustlessly verify and complete the user's intent without using an oracle.**
1. Gateway sends the LP's wrapped Bitcoin to the user's EVM address. If the user requested a Bitcoin LST/LRT, that token is minted using the LP's wrapped Bitcoin _before_ it is sent to the user.

## Architecture

<img
src={require("./architecture.png").default}
style={{ width: "100%", maxWidth: "100%", height: "auto" }}
alt="architecture"
/>

### User Flow

1. A user requests to swap BTC for wrapped BTC (e.g. WBTC, tBTC, or FBTC) or staked BTC (e.g. SolvBTC.BBN, uniBTC).
1. The user gets a "quote" of which Gateway smart contract is an available route (i.e. which LP they can swap with).
1. The user creates an "order" with the relayer to reserve the LP's liquidity.
1. The user creates a Bitcoin transaction, updating the order with their txid. Gateway's SDK creates a hash of the user's intent, which is included in the `OP_RETURN` of the transaction. This hash includes metadata such as the user's EVM address, which strategy they are using (i.e. their intent), and how many sats to convert to ETH for gas, among other data. By including a deterministic hash of the user's order, we lay the groundwork for making Gateway trustless in the future by decentralizing the relayer set.
1. The relayer monitors the Bitcoin chain and sends the LP's wrapped BTC to the user after the txid is seen on Bitcoin. Specifically, the relayer submits a Merkle proof of the user's Bitcoin tx to an [on-chain light client](/learn/builder-guides/relay) for trustless verification of the user's intent. It accomplishes this by requiring that the recipient's EVM address, sequence of smart contract calls, and other order data exactly match the hash in the `OP_RETURN` of the user's Bitcoin transaction.

### Liquidity Provider (LP) Flow

1. An LP asks the relayer to deploy a new Gateway contract, which functions as an escrow for their funds. This is permissioned at the moment because BOB pays the transaction's gas. See the [Trust Assumptions](#trust-assumptions) section below for more information.
2. The LP deposits wrapped Bitcoin (e.g. WBTC, tBTC, FBTC) in their Gateway contract.
3. The LP can only withdraw their funds or update their swap fees after a delay so that the relayer has time to finish open orders. The relayer will not accept new orders during this delay until reset.

## Trust Assumptions

While development for BOB Gateway is advancing rapidly, there are still some training wheels in place that introduce additional trust assumptions to the architecture. These are going to be addressed and replaced with trustless or trust-minimized equivalents as the product matures.

- Liquidity providers (LPs) receive BTC directly from users on Bitcoin mainnet. As a result, the off-chain relayer cannot interfere with funds received by the LPs.
- The relayer is currently necessary to complete the user's intent (e.g. by submitting a Merkle proof of the BTC transaction to the on-chain light client, unlocking the LP's wrapped BTC). Since users cannot submit the proofs themselves yet, user funds are "stuck" whenever the relayer is offline. Users will be able to submit their own BTC proofs after we transition Gateway use a full Bitcoin light client and add functionality to protect LPs from potentially malicious strategy contracts.
- At the moment there is only one relayer. In addition to its other functionality, this relayer pays gas on the user's behalf so that the user doesn't need to make a transaction on BOB to complete the bridging process. Since a malicious actor could create a strategy designed to take advantage of the relayer (e.g. spend all of the funds available for gas fees), the process of adding new strategies to Gateway as well as decentralizing the relayer role are restricted by the BOB team until Gateway is upgraded as described above.
