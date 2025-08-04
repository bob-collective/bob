---
sidebar_position: 2
---

# Integration Guide

[BOB Gateway](/docs/gateway) is a Bitcoin intent bridge that unlocks Bitcoin liquidity by reducing the number of steps to onboard users to your app, saving time and money. For example, users can go from **BTC** on Bitcoin to **staked BTC LSTs** with a single Bitcoin transaction.

Our SDK makes it possible for you to bring this UX directly into your app.

:::info Gateway Overview
For a detailed technical explanation of Gateway's architecture and user flow, see the [overview](./overview.md).
:::

## Step-by-Step

This is an example implementation of our SDK. You will need to decide how you handle asking your user to sign a partially-signed Bitcoin transaction (PSBT).

### Install the BOB SDK

Add `@gobob/bob-sdk` to your project using your preferred package manager.

```bash npm2yarn
npm install @gobob/bob-sdk
```

### Initialize the API Client

Import the `GatewaySDK` class from `@gobob/bob-sdk` and create an instance of it.

```ts title="/src/utils/gateway.ts"
import { GatewayQuoteParams, GatewaySDK } from '@gobob/bob-sdk';
import { bob } from 'viem/chains';

const gatewaySDK = new GatewaySDK(bob.id); // or bobSepolia.id
```

### Get Available Tokens

Returns an array of available output tokens for you to offer the user. Typically rendered as a drop-down menu. See [our SDK's source code](https://github.com/bob-collective/bob/blob/9c52341033af1ccbe388e64ef97a23bf6c07ccc7/sdk/src/gateway/tokens.ts#L8) for type information.

```ts
const outputTokens = await gatewaySDK.getTokens();
```

### Get a Quote

Call the `getQuote` method with your `quoteParams`. Example values shown here.

:::tip Updating the quote
We recommend rendering `quote.fee` and [its other fields](https://github.com/bob-collective/bob/blob/9c52341033af1ccbe388e64ef97a23bf6c07ccc7/sdk/src/gateway/client.ts#L15) for a clear UX. You can update the quote dynamically, such as with the `onChange` event.
:::

```ts
import { parseEther } from 'viem';
import { parseBtc } from '@gobob/bob-sdk';

const quote = await gatewaySDK.getQuote({
  fromChain: 'bitcoin',
  fromToken: 'BTC',
  fromUserAddress: 'bc1qafk4yhqvj4wep57m62dgrmutldusqde8adh20d',
  toChain: 'bob',
  toUserAddress: '0x2D2E86236a5bC1c8a5e5499C517E17Fb88Dbc18c',
  toToken: 'wBTC',
  amount: parseBtc("0.1"), // BTC
  // The amount of ETH to receive (this is subtracted from the amount)
  gasRefill: parseEther("0.00001"), // ETH
});
```

### Execute the Quote

This locks in the quote, placing a hold on the LP's funds. Internally, this creates a Bitcoin transaction that sends the quoted `amount` of BTC to the LP. This also publishes a hash of the order's parameters in the `OP_RETURN` of the transaction so the Gateway can trustlessly verify the order on BOB. Gateway will broadcast the Bitcoin transaction to the mempool; you can pass the transaction to the SDK without broadcasting from the user's wallet.

Pass the `quote` returned from the previous step.

```ts
import {
    createPublicClient,
    createWalletClient,
    http,
    zeroAddress,
    parseEther,
} from 'viem';
import { useAppKitProvider, useAppKitAccount } from '@reown/appkit/react';
import type { BitcoinConnector } from "@reown/appkit-adapter-bitcoin";
import { ReownWalletAdapter } from '@gobob/bob-sdk';

const publicClient = createPublicClient({
  chain: bob,
  transport: http(),
});

const walletClient = createWalletClient({
  chain: bob,
  transport: http(),
  account: zeroAddress, // Use connected account here
});

const { walletProvider } = useAppKitProvider<BitcoinConnector>('bip122');
const { address: btcAddress } = useAppKitAccount();

const txId = await gatewaySDK.executeQuote(quote, {
  walletClient,
  publicClient,
  btcSigner: new ReownWalletAdapter(walletProvider, btcAddress),
});
```

#### Other Wallets

The example above uses Reown AppKit for Bitcoin wallet integration. For additional wallet options and detailed integration guides, see our [Bitcoin Wallets](./wallets.md) guide which covers:

- **Reown AppKit** - Unified interface with broad wallet support (recommended)
- **sats-wagmi** - React hooks for Bitcoin wallets with support for Unisat, Leather, Xverse, and more
- **Direct integrations** - OKX Wallet, Dynamic.xyz, and other wallet-specific implementations

### Monitor the Orders

Get an array of pending and completed orders for a specific EVM address. Typically rendered in a table.

```ts
const orders = await gatewaySDK.getOrders(userAddress);
```

### Bump Fees (**Offramp Only**)

If the Bitcoin fee rate increases after an order is placed, you can then bump the transaction fee to speed up confirmation.

```ts
await gatewaySDK.bumpFeeForOfframpOrder(orderId, {
  walletClient,
  publicClient,
});
```

### Unlock Order (**Offramp Only**)

In cases where an order gets stuck or needs to be cancelled, you can unlock the order to free up the reserved liquidity. This allows the user to create a new order or cancel their intent.

```ts
await gatewaySDK.unlockOfframpOrder(orderId, receiver, {
  walletClient,
  publicClient,
});
```

:::warning Important Notes
- **Bump Fees**: Only works if the original transaction hasn't been confirmed yet. The bumped transaction will replace the original one.
- **Unlock Order**: This action is irreversible. Once unlocked, the order cannot be resumed and any reserved liquidity will be released.
:::

:::tip Best Practices
- Monitor Bitcoin mempool congestion and suggest fee bumping proactively
- Provide clear UI feedback when orders are stuck or need attention
- Always explain the implications of unlocking an order to users
:::

## Conclusion

BOB Gateway enables staking, swapping, lending, and bridging Bitcoin with a single transaction. The BOB SDK makes it possible for you to bring Gateway and Bitcoin LSTs directly to your users.

See the [Code References](#code-references) below for a deeper look at the SDK and an example implementation file.

We look forward to seeing what you Build on Bitcoin!

## Security and Trust Assumptions

The protocol requires zero trust between the market makers and users because it utilizes atomic cross-chain swaps. The verification of the Bitcoin transaction is performed cryptographically by an on-chain Bitcoin [Light Client](/docs/bob-chain/relay), making the swap trustless between both parties.

Furthermore, infrastructure run by the BOB team never has access to the market markers' tBTC, wBTC, or ETH funds stored in their smart contracts. The user interface and server manage order flow to prevent liquidity sniping and user errors (e.g. sending BTC without sufficient liquidity being available), but neither the front end or back end ever have access to users' or market makers' funds.

The code has been [audited by Pashov and Common Prefix](/docs/reference/audits#bob-gateway).

## Code References

- `bob/sdk/src/gateway/client.ts`: API client code ([GitHub](https://github.com/bob-collective/bob/blob/master/sdk/src/gateway/client.ts))
- `bob/sdk/examples/gateway.ts`: example client-side implementation ([GitHub](https://github.com/bob-collective/bob/blob/master/sdk/examples/gateway.ts))
