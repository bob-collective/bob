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

## Destination Actions (Cross-Chain Calls)

LayerZero Gateway supports executing arbitrary calls on the destination chain after tokens arrive. This enables complex DeFi interactions like depositing into lending protocols, staking, or swapping tokens - all in a single cross-chain transaction.

### Using `destinationCalls`

The `destinationCalls` parameter allows you to specify a series of calls to execute on the destination chain:

```typescript
import { encodeFunctionData, erc20Abi } from 'viem';

// Example: Transfer tokens to a recipient on the destination chain
const transferCallData = encodeFunctionData({
    abi: erc20Abi,
    functionName: 'transfer',
    args: ['0xEf7Ff7Fb24797656DF41616e807AB4016AE9dCD5', BigInt(1000000)],
});

const quote = await client.getQuote({
    fromChain: 'bitcoin',
    fromToken: 'BTC',
    toChain: 'base',
    toToken: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c', // wBTC
    fromUserAddress: 'bc1q...',
    toUserAddress: '0x...',
    amount: parseBtc("0.1"),
    destinationCalls: {
        calls: [
            {
                target: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c', // wBTC token
                callData: transferCallData,
                value: BigInt(0), // Native value to send with call (if any)
            },
        ],
        leftoverRecipient: '0xEf7Ff7Fb24797656DF41616e807AB4016AE9dCD5', // Receives any leftover tokens
        gasLimit: 300000, // Optional: gas limit for destination compose execution
    },
});
```

### `destinationCalls` Structure

```typescript
interface DestinationCalls {
    calls: DestinationCall[];
    leftoverRecipient: Address;
    gasLimit?: number; // Optional, defaults to 300000
}

interface DestinationCall {
    target: Address;    // Contract address to call
    callData: Hex;      // Encoded function call data
    value: bigint;      // Native value to send with call
}
```

### How It Works

1. **Token Arrival**: Tokens arrive at the `MulticallComposer` contract on the destination chain
2. **Call Execution**: The composer executes your specified calls sequentially
3. **Leftover Handling**: Any tokens not spent by your calls are sent to `leftoverRecipient`
4. **Error Handling**: If any call reverts, all tokens are sent to `leftoverRecipient` as a safety mechanism

### Important Notes

- **Mutual Exclusivity**: You cannot provide both `destinationCalls` and `message` - use one or the other
- **Leftover Recipient**: Always specify a `leftoverRecipient` address to receive any unspent tokens
- **Gas Limit**: The `gasLimit` parameter controls gas allocated for the compose execution (default: 300000)
- **Call Validation**: Calls cannot target EOAs (externally owned accounts) or the composer contract itself
- **Token Spending**: The composer ensures calls don't spend more tokens than received

### Example: Deposit into Lending Protocol

```typescript
import { encodeFunctionData, parseAbi } from 'viem';

const wbtcAddress = '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c';
const lendingPoolAddress = '0x...'; // Your lending protocol

// Step 1: Approve lending pool to spend wBTC
const approveCallData = encodeFunctionData({
    abi: erc20Abi,
    functionName: 'approve',
    args: [lendingPoolAddress, maxUint256],
});

// Step 2: Deposit wBTC into lending pool
const lendingAbi = parseAbi([
    'function deposit(address asset, uint256 amount, address onBehalfOf, uint16 referralCode)',
]);
const depositCallData = encodeFunctionData({
    abi: lendingAbi,
    functionName: 'deposit',
    args: [wbtcAddress, amount, userAddress, 0],
});

const quote = await client.getQuote({
    // ... other params
    destinationCalls: {
        calls: [
            {
                target: wbtcAddress,
                callData: approveCallData,
                value: BigInt(0),
            },
            {
                target: lendingPoolAddress,
                callData: depositCallData,
                value: BigInt(0),
            },
        ],
        leftoverRecipient: userAddress, // Receive any leftover tokens
        gasLimit: 500000, // Higher gas limit for multiple calls
    },
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

- **BOB Strategy Contract**: `0x4572ce66cB33255B60a15e3c6cb2ef9c65A30ebC`
  - Handles Bitcoin → LayerZero bridging
  - Swaps portion of wBTC to ETH for LayerZero fees
  - Executes LayerZero `send()` with proper parameters

- **Offramp Composer**: `0xaffBF9ECC4a23adfFe887FB859654B8B780CCed0`
  - Receives LayerZero → BOB transfers
  - Creates Gateway offramp orders for Bitcoin withdrawal

#### wBTC OFT Standard

LayerZero uses standardized wBTC OFT contracts across supported chains:
- **Address**: `0x0555e30da8f98308edb960aa94c0db47230d2b9c` (most chains)
- **Exception**: Optimism uses `0xc3f854b2970f8727d28527ece33176fac67fef48`

#### MulticallComposer Contract

The `MulticallComposer` contract (`0x814347a131B08679087F9A4842d493B1e788ea7a` on all supported chains) handles destination actions on LayerZero chains:
- Receives tokens from LayerZero OFT transfers
- Executes user-specified calls on the destination chain
- Sends leftover tokens to the specified recipient
- Provides safety mechanisms to prevent token loss

**Supported Chains**: BOB, Base, Ethereum, Sonic

Reach out to the BOB Team if there is a chain which you would like us to add support for!