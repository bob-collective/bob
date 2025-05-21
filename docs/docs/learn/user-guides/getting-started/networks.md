---
sidebar_position: 2
sidebar_label: Networks
---

import AddToWallet from '@site/src/components/AddToWallet';

# Networks

BOB offers two networks for different use cases:

## BOB Mainnet

BOB Mainnet is the production network where real value is at stake. Use this network for:

- Deploying production applications
- Trading with real assets
- Participating in DeFi protocols
- Staking BTC

### Network Details

- **Chain ID**: `60808`
- **RPC URL**: `https://rpc.gobob.xyz/`
- **Block Explorer**: `https://explorer.gobob.xyz/`
- **Currency Symbol and Gas Token**: `ETH`

### Alternative RPC URLs

If the main RPC URL is not working, try these alternatives:

- `https://bob.gateway.tenderly.co`
- `https://bob.drpc.org`

## BOB Sepolia (Testnet)

BOB Sepolia is the test network where you can experiment without risking real assets. Use this network for:

- Testing smart contracts
- Developing applications
- Learning how to use BOB
- Testing integrations

### Network Details

- **Chain ID**: `808813`
- **RPC URL**: `https://bob-sepolia.rpc.gobob.xyz`
- **Block Explorer**: `https://bob-sepolia.explorer.gobob.xyz/`
- **Currency Symbol and Gas Token**: `Sepolia ETH`

## Connecting to BOB

### Automatic Connection (Recommended)

Use the "Add to Wallet" button below to automatically add BOB to your wallet:

<AddToWallet />

### Manual Connection

1. Open your wallet settings
2. Add network with these details:
   - Network Name: `BOB Mainnet` or `BOB Sepolia`
   - Chain ID: `60808` (Mainnet) or `11155111` (Sepolia)
   - Currency Symbol: `ETH`
   - RPC URL: See network details above
   - Block Explorer: See network details above

## Getting Testnet ETH

To get testnet ETH for BOB Sepolia:

1. Get Sepolia ETH from a faucet:
   - [Alchemy Sepolia Faucet](https://sepoliafaucet.com/)
   - [Infura Sepolia Faucet](https://www.infura.io/faucet/sepolia)
2. Bridge your Sepolia ETH to BOB Sepolia using the [BOB Bridge](https://app.gobob.xyz/bridge)

## Need Help?

If you have any questions about connecting to BOB networks, please reach out to us on [Discord](https://discord.gg/gobob) or [Telegram](https://t.me/+CyIcLW2nfaFlNDc1). 