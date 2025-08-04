---
sidebar_position: 4
---

# Bitcoin Wallets

BOB Gateway requires Bitcoin wallet integration to allow users to sign Bitcoin transactions. This guide covers the different wallet integration options available for your application.

## Wallet Integration Options

### Reown AppKit (Recommended)

Reown AppKit provides a unified interface for Bitcoin wallet connections with broad wallet support.

**Installation:**
```bash npm2yarn
npm install @reown/appkit @reown/appkit-adapter-bitcoin
```

**Usage with Gateway SDK:**
```ts
import { useAppKitProvider, useAppKitAccount } from '@reown/appkit/react';
import type { BitcoinConnector } from "@reown/appkit-adapter-bitcoin";
import { ReownWalletAdapter } from '@gobob/bob-sdk';

const { walletProvider } = useAppKitProvider<BitcoinConnector>('bip122');
const { address: btcAddress } = useAppKitAccount();

const txId = await gatewaySDK.executeQuote(quote, {
  walletClient,
  publicClient,
  btcSigner: new ReownWalletAdapter(walletProvider, btcAddress),
});
```

### sats-wagmi

sats-wagmi is a library with Bitcoin wallet connectors that abstracts wallet integration complexity. It provides React hooks similar to the EVM [wagmi](https://wagmi.sh/react/getting-started) library.

**Supported Wallets:**
- Metamask Snap
- Unisat
- Leather
- Xverse
- Bitget
- OKX Wallet

**Features:**
- React hooks for wallet management
- PSBT signing
- BTC transaction sending

**Installation:**
```bash npm2yarn
npm install @gobob/sats-wagmi
```

**Usage with Gateway SDK:**
```ts
import { Network, XverseConnector } from '@gobob/sats-wagmi';

const btcSigner = new XverseConnector(Network.mainnet);

const txId = await gatewaySDK.executeQuote(quote, {
  walletClient,
  publicClient,
  btcSigner,
});
```

### Other Wallet Options

#### OKX Wallet Direct Integration

For OKX wallet, you can use their direct send API:

```ts
import { OkxWalletAdapter } from '@gobob/bob-sdk';

const txId = await gatewaySDK.executeQuote(quote, {
  walletClient,
  publicClient,
  btcSigner: new OkxWalletAdapter(window.okxwallet.bitcoin),
});
```

#### Dynamic.xyz

Dynamic provides Bitcoin wallet support. See their [PSBT signing guide](https://docs.dynamic.xyz/wallets/using-wallets/bitcoin/sign-a-psbt) for implementation details.


## UI Integration Guide

sats-wagmi provides the most comprehensive React integration for Bitcoin wallets.

### Installation

```bash npm2yarn
npm install @gobob/sats-wagmi
```

### Connect Wallet

#### 1. Wrap App in Context Provider

To start, we will need to wrap our React App with Context so that our application is aware of sats-wagmi & React Query's reactive state and in-memory caching:

```tsx
 // 1. Import modules
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { SatsWagmiConfig } from "@gobob/sats-wagmi";

// 2. Set up a React Query client.
const queryClient = new QueryClient()

function App() {
  // 3. Wrap app with sats-wagmi and React Query context.
  return (
    <QueryClientProvider client={queryClient}>
      <SatsWagmiConfig network="testnet" queryClient={queryClient}>
        {/** ... */} 
      </SatsWagmiConfig>
    </QueryClientProvider> 
  )
}
```

#### 2. Display Wallet Options

After that, we will create a `WalletOptions` component that will display our connectors. This will allow users to select a wallet and connect.

Below, we are rendering a list of `connectors` retrieved from `useConnect`. When the user clicks on a connector, the `connect` function will connect the users' wallet.

```tsx
import * as React from 'react'
import { useConnect, SatsConnector } from "@gobob/sats-wagmi";

export function WalletOptions() {
  const { connectors, connect } = useConnect()

  return connectors.map((connector) => (
    <button key={connector.name} onClick={() => connect({ connector })}>
      {connector.name}
    </button>
  ))
}
```

#### 3. Display Connected Account

Lastly, if an account is connected, we want to show the connected address.

We are utilizing `useAccount` to extract the account and `useDisconnect` to show a "Disconnect" button so a user can disconnect their wallet.

```tsx
import { useAccount, useDisconnect } from "@gobob/sats-wagmi";

function Account() {
  const { address } = useAccount()
  const { disconnect } = useDisconnect()

  return (
    <div>
      <p>Address: {address}</p>
      <button onClick={() => disconnect()}>Disconnect</button>
    </div>
  );
}
```

### Send Transaction

Create your `SendTransaction` component that will contain the send transaction logic.

```tsx
import type { FormEvent } from 'react';
import { type Hex, parseUnits } from 'viem';
import { useSendTransaction } from "@gobob/sats-wagmi";

function SendTransaction() {
  const { data: hash, error, isPending, sendTransaction } = useSendTransaction();

  async function submit(e: FormEvent<HTMLFormElement>) {
    e.preventDefault();
    const formData = new FormData(e.target as HTMLFormElement);
    const to = formData.get('address') as Hex;
    const value = formData.get('value') as string;

    sendTransaction({ to, value: parseUnits(value, 8) });
  }

  const { isLoading: isConfirming, isSuccess: isConfirmed } =
    useWaitForTransactionReceipt({
      hash,
    })

  return (
    <div>
      <h2>Send Transaction</h2>
      <form onSubmit={submit}>
        <input required name='address' placeholder='Address' />
        <input required name='value' placeholder='Amount (BTC)' step='0.00000001' type='number' />
        <button disabled={isPending} type='submit'>
          {isPending ? 'Confirming...' : 'Send'}
        </button>
      </form>
      {hash && <div>Transaction Hash: {hash}</div>}
      {isConfirming && 'Waiting for confirmation...'}
      {isConfirmed && 'Transaction confirmed.'}
      {error && <div>Error: {error.message}</div>}
    </div>
  );
}
```

### Gateway Integration Hook

sats-wagmi provides a convenient hook for Gateway transactions:

```tsx title="useSendGatewayTransaction.tsx"
import { useSendGatewayTransaction } from '@gobob/sats-wagmi';

function GatewayExample() {
  const {
    mutate: sendGatewayTransaction,
    data: txHash,
    error,
    isPending
  } = useSendGatewayTransaction();

  const handleGatewayTransaction = async () => {
    const quote = await gatewaySDK.getQuote({
      fromChain: 'bitcoin',
      fromToken: 'BTC',
      fromUserAddress: 'bc1q...',
      toChain: 'bob',
      toUserAddress: '0x...',
      toToken: 'wBTC',
      amount: parseBtc("0.1"),
      gasRefill: parseEther("0.00001"),
    });

    sendGatewayTransaction({
      gatewaySDK,
      quote,
      quoteParams: {
        fromUserAddress: 'bc1q...',
        // ... other params
      }
    });
  };

  return (
    <div>
      <button 
        onClick={handleGatewayTransaction}
        disabled={isPending}
      >
        {isPending ? 'Processing...' : 'Send Gateway Transaction'}
      </button>
      {txHash && <p>Transaction: {txHash}</p>}
      {error && <p>Error: {error.message}</p>}
    </div>
  );
}
```
