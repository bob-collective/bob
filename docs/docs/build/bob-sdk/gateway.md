---
sidebar_position: 1
---

# BOB Gateway

## What is BOB Gateway?

[BOB Gateway](https://docs.gobob.xyz/docs/learn/guides/bitcoin-bridge/) is a trustless, P2P bridge that enables users to bridge their BTC from Bitcoin mainnet to BOB.

You can use our SDK to build this UX directly into your app, bringing your app's functionality (lending, swapping, restaking, etc.) directly to Bitcoin hodlers.

## Features

<!-- - BTC Wallet connectors:
  - Metamask Snap
  - Unisat
  - Leather
  - Xverse
  - Bitget (soon)
- BTC functionality:
  - send bitcoin
  - inscribe (text and images)
  - send inscription
  - sign input (psbt)
- React hooks -->

## Installation

To use sats-wagmi, all you need to do is install the
`@gobob/sats-wagmi`:

```sh
# with Yarn
$ yarn add @gobob/sats-wagmi

# with npm
$ npm i @gobob/sats-wagmi

# with pnpm
$ pnpm add @gobob/sats-wagmi

# with Bun
$ bun add @gobob/sats-wagmi
```

## Usage

### Connector

```ts
import { MMSnapConnector } from "./connectors";

const mmSnap = new MMSnapConnector(network);

mmSnap.connect();
```

### React Hooks

1. Wrap your application with the `SatsWagmiConfig` provided by **@gobob/sats-wagmi**.

```tsx
import { SatsWagmiConfig } from "@gobob/sats-wagmi";

// Do this at the root of your application
function App({ children }) {
  return <SatsWagmiConfig network="testnet">{children}</SatsWagmiConfig>;
}
```

2. Now start by connecting:

```tsx
import { useConnect, SatsConnector } from "@gobob/sats-wagmi";

function Example() {
  const { connectors, connect } = useConnect();

  const handleConnect = (connector: SatsConnector) => {
    connect({
      connector,
    });
  };

  return (
    <div>
      {connectors.map((connector) => (
        <button key={connector.name} onClick={() => handleConnect(connector)}>
          {connector.name}
        </button>
      ))}
    </div>
  );
}
```

3. Once connected, you should be able to use the connector utility and have access to the connected BTC account:

```tsx
import { useConnect, SatsConnector } from "@gobob/sats-wagmi";

function Example() {
  const { address, connector } = useSatsAccount();

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
