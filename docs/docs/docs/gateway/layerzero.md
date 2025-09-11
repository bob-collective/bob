---
sidebar_position: 5
---

# LayerZero

BOB Gateway's LayerZero integration enables seamless cross-chain swaps between Bitcoin and LayerZero-supported chains. All transactions route through BOB as an intermediary, leveraging LayerZero's Omnichain Fungible Token (OFT) protocol for secure cross-chain bridging.

## High-Level Developer Overview

### How It Works

The LayerZero Gateway Client routes all cross-chain transactions through BOB, acting as the hub for LayerZero connectivity:

#### Bitcoin → LayerZero Chain Flow
1. **Bitcoin → BOB**: User sends Bitcoin, receives wBTC on BOB via Gateway
2. **BOB → Target Chain**: wBTC is bridged via LayerZero OFT to destination chain
3. **Strategy Execution**: BOB strategy contract handles the LayerZero bridging automatically

#### LayerZero Chain → Bitcoin Flow
1. **Origin Chain → BOB**: User's tokens are bridged to BOB via LayerZero
2. **BOB Processing**: Offramp composer creates Bitcoin withdrawal order
3. **BOB → Bitcoin**: Standard Gateway offramp process completes the swap

### Supported Chains

The LayerZero integration supports 15+ major EVM chains including Ethereum, Base, Arbitrum, Avalanche, BSC, and more.

## Get Quote

Get a quote for cross-chain swaps via LayerZero:

```typescript
import { LayerZeroGatewayClient, parseBtc } from '@gobob/bob-sdk';
import { bob } from 'viem/chains';

const client = new LayerZeroGatewayClient(bob.id);

// Bitcoin → Base example
const onrampQuote = await client.getQuote({
    fromChain: 'bitcoin',
    fromToken: 'BTC', 
    toChain: 'base',
    toToken: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c', // wBTC
    fromUserAddress: 'bc1q...', // Bitcoin address
    toUserAddress: '0x...', // EVM address on destination chain
    amount: parseBtc("0.1"), // BTC
    l0FeeBuffer: 500 // 5% LayerZero fee buffer (optional)
});

// Base → Bitcoin example
const offrampQuote = await client.getQuote({
    fromChain: 'base',
    fromToken: '0x0555E30da8f98308EdB960aa94C0Db47230d2b9c', // wBTC
    toChain: 'bitcoin',
    toToken: 'BTC',
    fromUserAddress: '0x...', // EVM address on origin chain
    toUserAddress: 'bc1q...', // Bitcoin address
    amount: 17000 // token amount in smallest unit
});
```

## Execute Quote

Execute the cross-chain swap:

```typescript
import { createPublicClient, createWalletClient, http } from 'viem';
import { base } from 'viem/chains';

// Setup clients for the origin chain
const publicClient = createPublicClient({
    chain: base, // Origin chain
    transport: http()
});

const walletClient = createWalletClient({
    chain: base,
    transport: http(),
    account: yourAccount
});

// Execute the quote
const txHash = await client.executeQuote({
    quote,
    walletClient,
    publicClient,
    btcSigner // Only required for Bitcoin → EVM onramp
});

console.log('Transaction hash:', txHash);
```

### Architecture & Smart Contracts

#### BOB Hub Architecture

All LayerZero operations route through BOB:

- **BOB Strategy Contract**: `0x5Fd9B934c219663C7f4f432f39682be2dC42eDC7`
  - Handles Bitcoin → LayerZero bridging
  - Swaps portion of wBTC to ETH for LayerZero fees
  - Executes LayerZero `send()` with proper parameters

- **Offramp Composer**: `0xc05AA3D7BD9c61B8b94EaCC937d1F542c3E5b94a`
  - Receives LayerZero → BOB transfers
  - Creates Gateway offramp orders for Bitcoin withdrawal

#### wBTC OFT Standard

LayerZero uses standardized wBTC OFT contracts across supported chains:
- **Address**: `0x0555e30da8f98308edb960aa94c0db47230d2b9c` (most chains)
- **Exception**: Optimism uses `0xc3f854b2970f8727d28527ece33176fac67fef48`

## Error Cases & Recovery

LayerZero transactions involve multiple steps that can fail at different points, requiring manual user intervention:

### BOB → LayerZero Chain Failures

If the LayerZero bridging step fails on BOB (e.g., insufficient gas, network congestion):

- **Issue**: User's Bitcoin was swapped to wBTC on BOB, but LayerZero send failed
- **Recovery**: User must manually execute the LayerZero bridge transaction on BOB
- **Action Required**: Submit the LayerZero `send()` transaction directly on BOB with sufficient gas

### BOB → Bitcoin Offramp Failures  

If the Bitcoin withdrawal fails after LayerZero bridging completes:

#### Insufficient Fees
- **Issue**: Bitcoin network fees increased, making the withdrawal uneconomical
- **Recovery Options**:
  - **Bump Fees**: Increase the fee allocation for the withdrawal order
  - **Wait**: Monitor for lower network fees before proceeding

#### Network Congestion
- **Issue**: Bitcoin network congestion prevents timely withdrawal processing
- **Recovery Options**:
  - **Withdraw wBTC**: Cancel the withdrawal and manually withdraw wBTC from BOB
  - **Wait**: Allow additional time for network conditions to improve

### Manual Recovery Process

```typescript
// For failed Bitcoin withdrawals, users can bump fees:
const bumpTxHash = await client.bumpFee({
    orderId: offrampOrderId, // From the failed withdrawal order
    walletClient,
    publicClient
});

// Or unlock and withdraw wBTC if withdrawal is uneconomical:
const unlockTxHash = await client.unlockOrder({
    orderId: offrampOrderId,
    receiver: userEvmAddress, // Where to send the wBTC
    walletClient,
    publicClient
});
```

:::warning Manual Intervention Required
LayerZero integration may require users to complete transactions manually on BOB in certain failure scenarios. Always monitor transaction status and be prepared to assist users with recovery procedures.
:::
