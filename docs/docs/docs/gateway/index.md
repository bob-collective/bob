---
sidebar_position: 1
---

# BOB Gateway

BOB Gateway is a Bitcoin intent/RFQ-based swap protocol that allows users to swap BTC on Bitcoin for ERC20 assets on BOB. The swap mechanism is trust-minimized: BOB Gateway uses smart contracts and cross-chain BTC proofs to ensure that swaps are always executed correctly ("atomically").

:::info Gateway Overview
For a detailed technical explanation of Gateway's architecture and user flow, see the [overview](./overview.md).
:::

## Key Benefits

### Native BTC Integration
- **Direct Bitcoin Access** - No wrapping or conversion required
- **Bitcoin Security** - Inherits Bitcoin's robust security model
- **Liquidity Access** - Tap into Bitcoin's massive liquidity pool

### Simple Integration
- **Plug-and-Play Integration** - Simple EVM-level integrations with modular design
- **Easy-to-Use SDK** - Comprehensive documentation and examples
- **Multiple Languages** - Support for JavaScript, TypeScript, and more
- **Quick Setup** - Get started in under 10 minutes

### Security & Reliability
- **Bitcoin-Grade Security** - All operations secured by Bitcoin
- **Audited Contracts** - All Gateway contracts are thoroughly audited
- **Trust Minimized** - No reliance on centralized bridges
- **Solver Network** - Specialized network for Bitcoin settlement and rebalancing ensures low slippage swaps

### Featured Projects
- **Get Featured** - Successful integrations get featured on the [BOB App](https://app.gobob.xyz/en)
- **Community Support** - Access to BOB developer community
- **Marketing Support** - Promotion through BOB channels

#### Current Platforms Using BOB Gateway
- **[BOB Earn](https://app.gobob.xyz/en/earn)** - Native Bitcoin staking platform
- **Xverse Earn** - Bitcoin wallet with integrated earning features
- **Sovryn** - Bitcoin DeFi platform
- **Pell** - Bitcoin earning and staking solutions

## Use Cases

### 1-Click BTC Earn Products
Transform idle Bitcoin into earning assets with a single click. Users can stake their native BTC directly from Bitcoin wallets without wrapping or bridging, earning yield through various DeFi protocols on BOB. Available on platforms like BOB Earn, Xverse Earn, Sovryn, and Pell.

*Perfect for wallet providers and earning platforms wanting to offer Bitcoin yield.*

### BTC On/Off-Ramp
Seamlessly swap between native Bitcoin and wrapped BTC tokens across major chains. Gateway aggregates liquidity for wBTC, tBTC, solvBTC, uniBTC and other variants, providing users the best rates without multiple integrations.

*Ideal for exchanges, wallets, and DeFi protocols needing unified BTC liquidity access.*

### Cross-Chain BTC Swaps (Coming Soon)
Enable direct swaps between native Bitcoin and tokens on Ethereum, Arbitrum, Base, and other major chains. All transactions route through BOB for security and efficiency, with competitive rates powered by the solver network.

*Perfect for DEXes, aggregators, and wallets wanting to offer true cross-chain Bitcoin trading.*

### DeFi Applications
Build sophisticated financial applications using native Bitcoin as collateral. Create lending protocols where users can borrow against BTC, yield farming strategies that accept Bitcoin deposits, or liquidity pools that include native BTC pairs.

*Great for DeFi protocols wanting to tap into Bitcoin's $1T+ market cap without custody risks.*

## How It Works

BOB Gateway uses intents for peer-to-peer swaps and verifies Bitcoin transactions via on-chain light client. The system supports optional intents like staking and cross-chain routing.

## What is the Gateway SDK?

BOB Gateway SDK enables developers to seamlessly integrate native Bitcoin functionality into their applications. The SDK abstracts UTXO logic and provides React hooks for simplified development, giving your users direct access to Bitcoin's liquidity and security without needing to wrap BTC or rely on third-party bridges.

## Next Steps

- **[Overview](./overview.md)** - Complete Gateway overview
- **[Integration Guide](./integration.md)** - Get started with Gateway integration
- **[On-Chain Actions](./strategy.md)** - Learn about Gateway strategies
- **[Bitcoin Wallets](./wallets.md)** - Learn about Bitcoin wallet support

