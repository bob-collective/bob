---
sidebar_position: 4
---

# sats-wagmi - Reactive primitives for Bitcoin apps

sats-wagmi is a library that provides multiple BTC wallet connectors, eliminating the need for developers to integrate each one individually. The library also exports useful React hooks that mimic the standard followed in the EVM [wagmi](https://wagmi.sh/react/getting-started) library.

## Features

- BTC Wallet connectors:
  - Metamask Snap
  - Unisat
  - Leather
  - Xverse
  - Bitget
  - OKX Wallet
- BTC functionality:
  - send BTC
  - sign PSBTs
- React hooks

## Installation

To use sats-wagmi, all you need to do is install `@gobob/sats-wagmi`:

```bash npm2yarn
npm install @gobob/sats-wagmi
```

## Connect Wallet

### 1. Wrap App in Context Provider

First, wrap your React App with Context to enable sats-wagmi and React Query's reactive state and in-memory caching:

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

### 2. Display Wallet Options

After that, we will create a `WalletOptions` component that will display our connectors. This will allow users to select a wallet and connect.

Below, we render a list of connectors from useConnect. When the user selects a connector, the connect function establishes the connection

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

### 3. Display Connected Account

If an account is connected, display the wallet address.

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

## Send Transaction

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
