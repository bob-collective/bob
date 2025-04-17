---
sidebar_position: 2
---

import AddToWallet from '@site/src/components/AddToWallet';

# BOB Networks

This guide provides detailed information about the BOB network, its configuration, and how to connect to it.

## Network Information

### Mainnet Details
- **Network Name:** `BOB Mainnet`
- **Chain ID:** `60808`
- **Currency Symbol:** `ETH`
- **RPC URL:** [`https://rpc.gobob.xyz/`](https://rpc.gobob.xyz/)
- **Block Explorer URL:** [`https://explorer.gobob.xyz/`](https://explorer.gobob.xyz/)

### Testnet Details
- **Network Name:** `BOB Testnet`
- **Chain ID:** `60808`
- **Currency Symbol:** `ETH`
- **RPC URL:** `https://testnet.rpc.gobob.xyz/`
- **Block Explorer URL:** `https://testnet.explorer.gobob.xyz/`

## Connect to BOB

### Quick Connect

<AddToWallet />

### Manual Connection
1. Open your wallet → Click **Network Selector** → **Add Network**
2. Enter the network details from above
3. Click **Save** and switch to **BOB Mainnet**

## Network Features

### Gas Fees

- Gas is paid in ETH
- Gas fees are dynamic and based on network utilization
- Gas optimization tips:
  - Use off-peak hours
  - Monitor gas prices
  - Consider batch transactions

### Transaction Speeds

- Block time: ~2 seconds
- Finality: Bitcoin finality
- Transaction limits: Based on gas limits

### Network Status

- Check the BOB Sequencer status at [BOB Sequencer Status](https://conduit-bob.checkly-dashboards.com/)
- Monitor network status at [BOB Explorer](https://explorer.gobob.xyz/)

## Troubleshooting

### Common Issues

#### Connection Problems

- Verify RPC URL is correct
- Check network status
- Try alternative RPC endpoints:
  - `https://bob-mainnet.public.blastapi.io`
  - `https://bob-mainnet.publicnode.com`

#### Transaction Issues

- Ensure sufficient ETH for gas
- Check network congestion
- Verify transaction parameters
- Try increasing gas limit