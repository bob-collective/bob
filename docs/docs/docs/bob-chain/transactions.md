---
sidebar_position: 4
sidebar_label: Transactions
---

# Transactions

BOB inherits its transaction mechanics from the [OP Stack](https://docs.optimism.io/stack/transactions), providing users with fast, cost-effective transactions while maintaining the security guarantees of Ethereum.

## Transaction Fees

BOB's transaction fees are calculated differently from Ethereum to account for the cost of posting transaction data to Ethereum for data availability.

### Fee Components

BOB transaction fees include:
- **L1 data fee**: Cost of posting transaction data to Ethereum (the largest component)
- **L2 execution fee**: Cost of executing the transaction on BOB (typically much smaller)

### Fee Calculation

BOB uses the [Isthmus upgrade](https://docs.optimism.io/stack/transactions/fees) fee mechanism which calculates total fees as:

```
operatorFee = operatorFeeConstant + operatorFeeScalar * gasUsed / 1e6
totalFee = operatorFee + gasUsed * (baseFee + priorityFee) + l1Fee
```

Where:
- **operatorFee**: Fixed operator cost plus variable component based on gas usage
- **gasUsed**: Amount of gas consumed by the transaction
- **baseFee**: Dynamic base fee (EIP-1559)
- **priorityFee**: Optional tip for transaction prioritization
- **l1Fee**: Cost of posting transaction data to Ethereum

### EIP-1559 Support

BOB fully supports [EIP-1559](https://eips.ethereum.org/EIPS/eip-1559) fee mechanism, including:
- **Base fee**: Dynamic fee that adjusts based on network congestion
- **Priority fee**: Optional tip to prioritize transaction inclusion
- **Blob transactions**: Uses Ethereum's blob storage for more cost-effective data availability

### Onchain Costs

BOB's actual onchain costs and fee statistics can be tracked on [L2Beat](https://l2beat.com/scaling/projects/bob#onchain-costs), which shows:
- Total annual costs for data posting to Ethereum
- Average cost per transaction
- Data posted to Ethereum over time

### Cost Efficiency

BOB transactions are typically much cheaper than Ethereum transactions because:
- **Batch processing**: Multiple transactions share the cost of L1 data posting
- **Optimized data compression**: Transaction data is compressed before posting to Ethereum
- **Blob transactions**: Uses Ethereum's blob storage for more cost-effective data availability

For detailed fee calculations and mechanisms, see the [OP Stack transaction fees documentation](https://docs.optimism.io/stack/transactions/fees).

## Transaction Finality

BOB transactions follow a three-stage finality process that inherits Ethereum's security guarantees:

### Finality Stages

1. **Unsafe** (0-1 minute): Transaction is included in a BOB block but not yet posted to Ethereum
2. **Safe** (1-60 minutes): Transaction data is posted to Ethereum and included in an Ethereum block (BOB posts data every hour)
3. **Finalized** (15-74 minutes): Ethereum block containing the transaction is finalized (2 epochs + 14 minutes after safe)

### Kailua Enhancement
With BOB's [hybrid zk rollup](/docs/bob-chain/hybrid-chain) powered by Kailua, state proposals can be finalized instantly using validity proofs. While this service is not yet exposed to users, the technical capability exists to enable **on-demand fast withdrawals** in the future, potentially reducing withdrawal times from the current 4-day period to as little as 1 hour.

### Finality Guarantees
- **Unsafe transactions**: Can be reorganized by the sequencer
- **Safe transactions**: Protected from sequencer reorgs, require Ethereum reorg to reverse
- **Finalized transactions**: Irreversible under normal Ethereum consensus conditions

### Common Misconception: 7-Day Finality

A common misconception is that transactions on BOB take 7 days to finalize. **This is incorrect.** Transactions on BOB become finalized when their data is included in a finalized Ethereum block, typically between 15-74 minutes after submission.

This misconception often arises due to BOB's Standard Bridge, which includes a 4-day delay on _withdrawals_ of ETH and ERC-20 tokens. Withdrawing tokens from BOB to Ethereum using the Standard Bridge requires a minimum 4-day wait. This delay affects only withdrawals through the Standard Bridge and does not impact transaction finality on BOB itself.

**Important:** A transaction is finalized before the state proposal that references the transaction is finalized. Transaction finality depends on Ethereum's consensus mechanism, not on the OP Stack's fault proof system.

For comprehensive details on transaction finality, see the [OP Stack transaction finality documentation](https://docs.optimism.io/stack/transactions/transaction-finality).

## Transaction Flow

1. **Submission**: User submits transaction to BOB network
2. **Sequencing**: BOB sequencer includes transaction in a block
3. **Data Availability**: Transaction data posted to Ethereum via blob transactions
4. **Finalization**: Transaction becomes finalized when included in finalized Ethereum block

## Withdrawals

BOB supports two withdrawal mechanisms:

### Standard Bridge Withdrawals
- **4-day delay**: Standard OP Stack bridge withdrawals require 4-day challenge period
- **High security**: Inherits Ethereum's security guarantees
- **Universal support**: Works with all ERC-20 tokens and ETH

### Fast Withdrawals (Kailua)
- **Future feature**: Technical capability exists for 1-hour finality via validity proofs
- **Not yet available**: Service not currently exposed to users
- **Cost optimization**: Will allow users to pay for proofs only when speed is needed

This hybrid approach will eventually give users the flexibility to choose between cost-optimized standard withdrawals (4-day delay) and speed-optimized fast withdrawals (1-hour finality) based on their specific needs. 