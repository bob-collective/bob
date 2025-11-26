# Account Abstraction

BOB supports multiple account abstraction solutions, including the latest EIP-7702 standard introduced with Ethereum's Pectra upgrade. This enables enhanced user experiences with gasless transactions, batch operations, and seamless wallet integrations.

## EIP-7702 Support

With the latest OP stack update, BOB now supports [EIP-7702](https://eips.ethereum.org/EIPS/eip-7702), a groundbreaking Ethereum standard that enables externally owned accounts (EOAs) to execute smart contract functionality temporarily. This provides [superior UX](https://blog.thirdweb.com/eip-7702/) compared to traditional account abstraction methods.

### Key Benefits

EIP-7702 on BOB enables:

- **Batch Transactions**: Execute multiple operations in a single atomic transaction (token approvals, swaps, transfers)
- **Gas Sponsorship**: Allow third parties to pay gas fees for your transactions
- **Temporary Delegation**: Grant scoped permissions without exposing private keys
- **Enhanced Security**: Trustless execution without requiring smart contract deployment
- **Backward Compatibility**: Full compatibility with existing ERC-4337 infrastructure

### How It Works

EIP-7702 introduces a new transaction type (0x04) that allows EOAs to:

1. Create an authorization list specifying which smart contract to delegate execution to
2. Temporarily execute smart contract logic from their own address
3. Return to normal EOA behavior after transaction completion

### Implementation on BOB

BOB's EIP-7702 implementation provides:

- **Native Support**: Integrated into BOB's OP stack upgrade
- **Bitcoin Wallet Integration**: Use EIP-7702 features with Bitcoin wallets through BOB Gateway
- **DeFi Protocol Support**: Enhanced interactions with lending, staking, and trading protocols
- **Cross-Chain Operations**: Seamless integration with BOB's interoperability layer

### Getting Started

To use EIP-7702 on BOB:

1. **Connect a Compatible Wallet**: Use any EOA wallet (MetaMask, Rabby, etc.)
2. **Interact with DApps**: Look for applications offering batch operations or sponsored transactions
3. **Bitcoin Integration**: Use BOB Gateway to control EVM operations from Bitcoin wallets

:::info EIP-7702 Resources
For technical implementation details and code examples:
- [EIP-7702 Official Specification](https://eips.ethereum.org/EIPS/eip-7702)
- [QuickNode Implementation Guide](https://www.quicknode.com/guides/ethereum-development/smart-contracts/eip-7702-smart-accounts)
- [Thirdweb EIP-7702 Guide](https://blog.thirdweb.com/eip-7702/)
:::

## Safe Wallet

[Safe Wallet](https://safe.gobob.xyz/welcome) (formerly known as Gnosis Safe multisig) is a smart contract wallet. Featuring 130+ ecosystem apps and the ability to create your own modules and guard, Safe is the most trusted decentralized custody protocol and collective asset management platform.

## BTC Connect

Made by [Particle Network](https://particle.network/), BTC Connect enables users to control ERC-4337 smart accounts (i.e. smart contract wallets) on EVM-chains with their native Bitcoin wallets (e.g. UniSat). Interactions with products deployed on BOB can be conducted via Bitcoin wallet signatures using their pre-built modal. Learn more in their [docs](https://docs.particle.network/developers/btc-connect).
