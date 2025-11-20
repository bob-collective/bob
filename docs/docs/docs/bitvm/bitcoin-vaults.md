---
sidebar_position: 3
sidebar_label: Bitcoin Vaults
---

# Bitcoin Vaults and Liquidation Engine

## Overview

Bitcoin vaults enable native BTC as collateral for lending and stablecoins on DeFi chains like Ethereum, Base, BOB, and Solana. Using BitVM technology, these vaults provide trust-minimized access to DeFi while maintaining Bitcoin's security guarantees.

## How Bitcoin Vaults Work

Bitcoin vaults leverage BitVM to create trust-minimized bridges that allow users to:

1. **Lock native BTC** in a BitVM-style vault on Bitcoin
2. **Mint an NFT** on the DeFi chain representing the locked BTC value
3. **Use NFT as collateral** to borrow stablecoins (USDT/USDC) or mint CDP-style stablecoins
4. **Withdraw BTC** at any time by generating a ZK proof that the loan was repaid

### Key Advantages over Traditional Wrappers

Unlike fungible BTC wrappers (wBTC, cbBTC), Bitcoin vaults ensure:

- **Enforced withdrawals**: Depositors can withdraw via cryptographic proof without requiring bridge operator cooperation
- **No UTXO mixing**: Users receive precisely the BTC they deposited from their original UTXO
- **Trust minimization**: No third-party trust assumptions under normal operations

## BOB Bitcoin Vault Liquidation Engine

The BOB Liquidation Engine addresses critical limitations in basic Bitcoin vault implementations:

### Traditional Vault Limitations

1. **Delayed liquidations**: Cannot trigger until loan repayment deadline expires
2. **All-or-nothing liquidation**: Entire BTC amount must be liquidated
3. **Static liquidator set**: Predefined liquidators limit available capital during market stress
4. **Non-atomic process**: Creates inefficiencies and risks for lenders and borrowers

### Liquidation Engine Solutions

#### Base Engine Features

**Open Liquidator Set**
- Removes predefined liquidator restrictions
- Anyone can participate in liquidations
- Eliminates trust assumptions about liquidator availability and liquidity

**Multi-party Liquidations**
- Multiple liquidators can collaborate on single liquidations
- Reduces individual capital requirements
- Improves market resilience during high-volume liquidation events

**Bridge Integration**
- Routes liquidations through Bitcoin bridge deposit addresses
- Compatible with existing bridges (wBTC, LBTC, BitVM bridges)
- No modifications required to bridge infrastructure

#### Optional Extensions

**Partial Liquidations**
- Liquidate only to safe LTV ratio, not entire position
- Depositor opt-in with bridge trust for non-liquidated portion
- Significantly reduces collateral loss for borrowers

**Fast Liquidations**
- Reduces settlement from days to 1-6 Bitcoin blocks (10-60 minutes)
- Trust model options:
  - n-of-n consensus: All operators must agree (strongest security)
  - m-of-n consensus: Majority/supermajority agreement
  - Emulated covenants: Spending restrictions prevent operator theft
- Automatic BitVM fallback if consensus fails

**Atomic Liquidations**
- Single-transaction liquidation when bridges support pre-deposits
- Eliminates settlement risk for liquidators
- Enables flash loan liquidations
- Requires deep bridge integration

## Architecture

### Components

1. **Bitcoin Vault**
   - BitVM implementation with BTC deposit input
   - Multiple outputs: depositor and bridge addresses
   - Fraud-proof challenge mechanism

2. **Liquidation Engine Smart Contract**
   - Tracks liquidator contributions and owed collateral
   - Manages bridged BTC distribution
   - Deployed on DeFi chain (can serve multiple chains)

3. **Operators**
   - Facilitate BTC withdrawals to bridges during liquidations
   - Receive fees from liquidation profits
   - Can reuse existing BitVM bridge operator sets

4. **Bridge Integration**
   - Accepts vault deposits at predefined addresses
   - Mints wrapped BTC to liquidation engine
   - Handles BTC return for partial liquidations

### Liquidation Flow

1. **Vault Setup**: Depositor and operators create vault with outputs to depositor and bridge
2. **Loan Liquidation**: Liquidators repay loan via smart contract using own capital
3. **Bitcoin Withdrawal**: Operators assert liquidation and submit to Bitcoin vault
4. **Bridge Minting**: BTC deposited to bridge, wrapped tokens minted to engine
5. **Distribution**: Liquidators claim bridged BTC proportional to contributions
6. **Operator Fees**: Operators receive percentage of liquidated BTC

## Trust Models

| Component | Basic Vault | Base Engine | With Extensions |
|-----------|-------------|-------------|-----------------|
| **Depositor Withdrawal** | Trust minimized | Trust minimized | Trust minimized (+ optional bridge trust) |
| **Liquidator Access** | Static set only | Anyone | Anyone |
| **Liquidation Speed** | Multiple days | Multiple days | Minutes to instant |
| **Partial Liquidation** | Not possible | Not possible | Available (opt-in) |
| **Capital Requirements** | Single liquidator | Distributed | Distributed |

## Use Cases

### Lending Protocols
- Native BTC collateral without wrapping
- Efficient liquidations during market volatility
- Reduced bad debt through fast liquidations

### CDP Stablecoins
- BTC-backed stablecoin minting
- Partial liquidations preserve user capital
- Open liquidator participation ensures stability

### Cross-chain DeFi
- Use BTC collateral on Ethereum, Base, BOB, Solana
- Maintain Bitcoin security while accessing DeFi yields
- Bridge-agnostic design enables wide compatibility

## Implementation Status

:::info Development Phase
The Bitcoin Vault Liquidation Engine is currently in research and development, with mainnet deployment planned as part of BOB's Phase 2 BitVM integration.
:::

### Current Progress
- Liquidation engine design finalized
- Basic Bitcoin vault tested on mainnet (Babylon experiment)
- Integration with lending protocols in development

### Upcoming Milestones
- BitVM3 optimizations for improved capital efficiency
- Bridge partner integrations for atomic liquidations
- Mainnet deployment with select lending protocols

## Technical Resources

- [Liquidation Engine Whitepaper](/liquidation-engine.pdf)
- [BitVM Documentation](./index.mdx)
- [Hybrid Nodes Overview](../bob-chain/hybrid-nodes.md)

## Security Considerations

### Depositor Safety
- Maintain self-custody throughout lending process
- Cryptographic proof ensures withdrawal rights
- Original UTXO return prevents tainted coin concerns

### Protocol Security
- 1-of-n BitVM security model
- Fraud proofs protect against malicious operators
- Economic incentives align all participants

### Market Stability
- Open liquidations prevent liquidity crises
- Partial liquidations reduce cascade effects
- Fast settlement minimizes price impact risks