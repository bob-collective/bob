---
sidebar_position: 2
sidebar_label: "BOB Gateway: Bitcoin Intents"
---

# BOB Gateway: Bitcoin Intents

## Introduction

Bitcoin users can easily onboard to the BOB Hybrid L2 without previously holding any Ethereum assets. This page explains the structure of _BOB Gateway_, an intent-based bridge that coordinates peer-to-peer swaps between users and liquidity providers (LPs).

Cross-chain transfers are secured by verifying Bitcoin transaction proofs with an [on-chain light client](/learn/builder-guides/relay), avoiding the need for an oracle. Optional intents, such as staking, lending, and swapping a small amount of ETH for transaction fees can all be accomplished while only requiring a single Bitcoin transaction from the user.

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
1. The user sends BTC to the liquidity provider's Bitcoin address. A hash of the user's order is included in the `OP_RETURN` of their transaction, including data such as the recipient's EVM address on BOB.
1. The relayer trustlessly verifies the user's Bitcoin transaction by submitting a Merkle proof to an on-chain [Light Client](/learn/builder-guides/relay.md), granting the relayer permission to withdraw the LP's wrapped Bitcoin without needing to use an oracle.
1. Gateway sends the LP's wrapped Bitcoin to the user's EVM address. If the user requested a Bitcoin LST/LRT, that token is minted using the LP's wrapped Bitcoin _before_ it is sent to the user.

## Architecture

![architecture](./architecture.png)

### User Flow

1. A user requests to swap BTC for wrapped BTC (e.g. WBTC, tBTC, or FBTC) or staked BTC (e.g. SolvBTC.BBN, uniBTC)
1. The user gets a "quote" of which Gateway smart contract is an available route (i.e. which LP they can swap with)
1. The user creates an "order" with the relayer to reserve the LP's liquidity
1. The user creates a Bitcoin transaction, updating the order with their txid
1. The relayer monitors the Bitcoin chain and sends the LP's wrapped BTC to the user after the txid seen on Bitcoin. Specifically, the relayer submits a Merkle proof of the Bitcoin tx to an [on-chain light client](/learn/builder-guides/relay) for trustless verification of the user's intent.

### Liquidity Provider (LP) Flow

1. An LP asks the relayer to deploy a new Gateway contract, which functions as an escrow for their funds. This is permissioned at the moment because BOB pays the transaction fees.
2. The LP deposits wrapped Bitcoin (e.g. WBTC, tBTC, FBTC) in their Gateway contract.
3. The LP can only withdraw their funds or update their swap fees after a delay so that the relayer has time to finish open orders. The relayer will not accept new orders during this delay until reset.

### Example Complex Order Flows

#### Swapping BTC for WBTC and gas

1. BOB creates a Gateway contract from the Registry
2. LP deposits WBTC into the Gateway contract
3. User request a quote from the off-chain relayer to swap BTC for WBTC and to add ETH for gas fees
4. Off-chain relayer provides the quote
5. Users places an order with the relayer
6. User sends BTC to the LPs Bitcoin address
7. Off-chain relayer monitors the Bitcoin chain for the transaction (user to LP)
8. Off-chain relayer sends the merkle proof to the Gateway contract which:

- Unlocks the WBTC from the Gateway contract
- Swaps a small portion of the WBTC to ETH and sends it to the user
- Sends the remaining WBTC to the user

![swap](./swap.png)

#### Staking BTC for SolvBTC.BBN and swap for gas

All steps are the same as the swap flow, except for step 8 when the BTC proof is submitted:

- Unlocks the WBTC from the Gateway contract
- Swaps a small portion of the WBTC to ETH and sends it to the user
- Sends the remaining WBTC to the Solv contracts to stake WBTC for SolvBTC. Stakes the solvBTC for solvBTC.BBN. Sends the SolvBTC.BBN to the user.

![staking](./Staking-Flow.png)
