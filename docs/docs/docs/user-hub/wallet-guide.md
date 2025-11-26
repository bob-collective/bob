---
sidebar_position: 3
---
import AddToWallet from '@site/src/components/AddToWallet';
import AddBobToken from '@site/src/components/AddBobToken';

# Wallets

This guide lists EVM-compatible wallets that are supported on the BOB network. Any EVM wallet supports the BOB network and this list is not complete but serves as a good guide for a few wallets you might find useful.

Each wallet is categorized by type and includes setup instructions and key features.

## Connecting to BOB

### Automatic Connection (Recommended)

Use the "Add to Wallet" button below to automatically add BOB to your wallet:

<AddToWallet />

### Manual Connection

1. Open your wallet settings
2. Add network with these details:
   - Network Name: `BOB Mainnet` or `BOB Sepolia`
   - Chain ID: `60808` (Mainnet) or `808813` (Sepolia)
   - Currency Symbol: `ETH`
   - RPC URL: See network details above
   - Block Explorer: See network details above

## Add the BOB Token

Use the buttons below to add the official BOB token (18 decimals) on each supported network. The component will switch your wallet to the right chain, add it if needed, and register the token contract automatically.

<AddBobToken />

### Token Addresses

| Network  | Contract | Explorer |
| --- | --- | --- |
| BOB | `0xb0bd54846a92b214c04a63b26ad7dc5e19a60808` | [BOB Explorer](https://explorer.gobob.xyz/address/0xb0bd54846a92b214c04a63b26ad7dc5e19a60808) |
| Ethereum | `0xC9746F73cC33a36c2cD55b8aEFD732586946Cedd` | [Etherscan](https://etherscan.io/address/0xC9746F73cC33a36c2cD55b8aEFD732586946Cedd) |
| BNB Smart Chain | `0x52B5fB4B0F6572B8C44d0251Cc224513ac5eB7E7` | [BscScan](https://bscscan.com/address/0x52B5fB4B0F6572B8C44d0251Cc224513ac5eB7E7) |

BOB uses Chainlink CCIP to bridge liquidity between these deployments. You can move the token with:

- [CCIP Transporter](https://app.transporter.io/?token=BOB)
- [BOB Gateway Bridge](https://app.gobob.xyz/en/bridge)

## Browser Wallets

### MetaMask
- **Type**: Browser Extension
- **Security**: Medium (password + recovery phrase)
- **Setup**: [Install MetaMask](https://metamask.io/)
- **BOB Support**: Full support, works with all BOB features

### Rabby Wallet
- **Type**: Browser Extension
- **Security**: Medium (password + recovery phrase)
- **Setup**: [Install Rabby](https://rabby.io/)
- **BOB Support**: Full support, works with all BOB features

### Rainbow
- **Type**: Browser Extension
- **Security**: Medium (password + recovery phrase)
- **Setup**: [Install Rainbow](https://rainbow.me/)
- **BOB Support**: Full support, works with all BOB features

### Coinbase Wallet
- **Type**: Browser Extension & Mobile App
- **Security**: Medium (password + recovery phrase)
- **Setup**: [Install Coinbase Wallet](https://www.coinbase.com/wallet)
- **BOB Support**: Full support, works with all BOB features

## Mobile Wallets

### OKX Wallet
- **Type**: Mobile App
- **Security**: Medium (password + recovery phrase)
- **Setup**: [Download OKX](https://www.okx.com/web3)
- **BOB Support**: Full support, works with all BOB features

### Binance Wallet
- **Type**: Mobile App
- **Security**: Medium (password + recovery phrase)
- **Setup**: [Download Binance](https://www.binance.com/en/wallet)
- **BOB Support**: Full support, works with all BOB features

## Hardware Wallets

### Ledger
- **Type**: Hardware Device
- **Security**: High (private keys never leave device)
- **Setup**: [Get Ledger](https://www.ledger.com/)
- **BOB Support**: Full support, works with all BOB features

### Trezor
- **Type**: Hardware Device
- **Security**: High (private keys never leave device)
- **Setup**: [Get Trezor](https://trezor.io/)
- **BOB Support**: Full support, works with all BOB features

## Institutional Wallets

### Fireblocks
- **Type**: Enterprise Platform
- **Security**: High (institutional-grade security)
- **Setup**: [Contact Fireblocks](https://www.fireblocks.com/)
- **BOB Support**: Full support, works with all BOB features

## Multi-Signature Wallets

### Safe (formerly Gnosis Safe)
- **Type**: Smart Contract Wallet
- **Security**: High (requires multiple approvals)
- **Setup**: [Create Safe Account](https://safe.gobob.xyz/welcome/accounts)
- **BOB Support**: Full support, works with all BOB features
