---
sidebar_position: 4
---

# sats-wagmi - Reactive primitives for Bitcoin apps

sats-wagmi is a library with a handful of BTC wallet connectors, leaving aside the need of the developer to integrate each one individually. The library also exports useful React hooks that mimic the standard followed in the EVM [wagmi](https://wagmi.sh/react/getting-started) library.

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

### 2. Display Wallet Options

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

### 3. Display Connected Account

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

## Send Transaction

Create your `SendTransaction` component that will contain the send transaction logic.

```tsx
import { useAccount } from "@gobob/sats-wagmi";

function SendTransaction() {
  const { address, connector } = useAccount()

  const handleTransfer = () => {
    connector?.sendToAddress(
      "tb1p9gl248kp19jgennea98e2tv8acfrvfv0yws2tc5j6u72e84caapsh2hexs",
      100000000
    );
  };

  return (
    <div>
      <p>Address: {address}</p>
      <button onClick={handleTransfer}>Transfer 1 BTC</button>
    </div>
  );
}
```