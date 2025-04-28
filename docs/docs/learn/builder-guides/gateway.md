---
sidebar_position: 2
---

# Integrate BOB Gateway in Your App

BOB Gateway is a Bitcoin intent bridge that unlocks Bitcoin liquidity by reducing the number of steps needed to onboard users to your app, saving time and money.
For example, users can go from BTC on Bitcoin to staked BTC LSTs with a single Bitcoin transaction via the onramp.
Users can also easily move from BOB back to Bitcoin using the offramp.

Our SDK makes it possible for you to bring this UX directly into your app.

## How Gateway Works
### Gateway-Onramp (Bitcoin -> BOB)
1. Liquidity providers (LPs) temporarily lock wrapped Bitcoin (WBTC or tBTC) in escrow smart contracts on BOB. 
2. A user makes a request for wrapped or staked Bitcoin (e.g. WBTC, tBTC, or a Bitcoin LST/LRT). 
3. The user sends BTC to the liquidity provider's Bitcoin address. A hash of the user's order is included in the `OP_RETURN` of the transaction. 
4. Gateway finalizes the transaction. After trustlessly verifying the user's Bitcoin transaction with an on-chain [Light Client](/learn/builder-guides/relay.md), Gateway sends the LP's wrapped Bitcoin to the user's EVM address. If the user requested a Bitcoin LST/LRT, that token is minted using the LP's wrapped Bitcoin before it is sent to the user.

This SDK exposes helper functions for steps 2, 3, and 4 to be used in your application's front end.

### Gateway-Offramp (BOB → Bitcoin)

1. Users lock their wrapped Bitcoin (WBTC or tBTC) into a smart contract on BOB.
2. Liquidity Providers (LPs) accept the user's order directly through the smart contract.
3. The LP sends Bitcoin to the user's Bitcoin address, including a unique order hash in the transaction’s `OP_RETURN`.
4. The Gateway finalizes the transaction by trustlessly verifying, using an on-chain [Light Client](/learn/builder-guides/relay.md), that the LP’s Bitcoin transfer matches the user's address and that the `OP_RETURN` matches the order data — unlocking the user's wrapped Bitcoin on BOB to the LP.

:::info Learn More
Discover the architecture of BOB Gateway and how it simplifies Bitcoin transactions by visiting our [BOB Gateway introduction page](../introduction/gateway).
:::

## Step-by-Step Integration Guide

This is an example implementation of our SDK. You will need to decide how you handle asking your user to sign a partially-signed Bitcoin transaction (PSBT). We recommend using our [sats-wagmi](/learn/builder-guides/sats-wagmi.md) package to connect to your users' wallets.

### Install the BOB SDK

Add `@gobob/bob-sdk` to your project using your preferred package manager.

```bash npm2yarn
npm install @gobob/bob-sdk
```

### Initialize the API Client

Import the `GatewaySDK` class from `@gobob/bob-sdk` and create an instance of it.

```ts title="/src/utils/gateway.ts"
import { GatewayQuoteParams, GatewaySDK } from '@gobob/bob-sdk';

const gatewaySDK = new GatewaySDK('bob'); // or "signet"
```

### Get Available Tokens

Returns an array of available output tokens for you to offer the user. Typically rendered as a drop-down menu. See [our SDK's source code](https://github.com/bob-collective/bob/blob/9c52341033af1ccbe388e64ef97a23bf6c07ccc7/sdk/src/gateway/tokens.ts#L8) for type information.

```ts
const outputTokens = await gatewaySDK.getTokens();
```

## Gateway-Onramp Methods

The following methods onboard users from Bitcoin to BOB through a simple and secure flow.

### Get a Quote

Call the `getQuote` method with your `quoteParams`. Example values shown here.

:::tip Updating the quote
We recommend rendering `quote.fee` and [its other fields](https://github.com/bob-collective/bob/blob/9c52341033af1ccbe388e64ef97a23bf6c07ccc7/sdk/src/gateway/client.ts#L15) for a clear UX. You can update the quote dynamically, such as with the `onChange` event.
:::

```ts
const quoteParams: GatewayQuoteParams = {
  fromToken: 'BTC',
  fromChain: 'Bitcoin',
  fromUserAddress: 'bc1qafk4yhqvj4wep57m62dgrmutldusqde8adh20d',
  toChain: 'BOB',
  toUserAddress: '0x2D2E86236a5bC1c8a5e5499C517E17Fb88Dbc18c',
  toToken: 'tBTC', // or e.g. "SolvBTC"
  amount: 10000000, // 0.1 BTC
  gasRefill: 10000, // 0.0001 BTC. The amount of BTC to swap for ETH for tx fees.
};

const quote = await gatewaySDK.getQuote(quoteParams);
```

#### Get available staking or lending contracts

The SDK will handle automatically when the `toToken` has a fungible ERC20 token, but sometimes there is no representation. In that case we can list the available integrations and specify that in the quote.

```ts
const strategies = await gatewaySDK.getStrategies();
const strategy = strategies.find(
  (contract) => contract.integration.name === 'pell-wbtc',
)!;
const quoteParamsStaking: GatewayQuoteParams = {
  ...quoteParams,
  toChain: strategy.chain.chainId,
  toToken: strategy.inputToken.symbol, // "wbtc"
  strategyAddress: strategy.address,
};
```

### Start the Order

This locks in the quote, placing a hold on the LP's funds. Pass the same `quoteParams` as before and the `quote` returned from the previous step.

Returns a `uuid` for the order and `psbtBase64`, a partially-signed Bitcoin transaction (PSBT) the user must sign.

```ts
const { uuid, psbtBase64 } = await gatewaySDK.startOrder(quote, quoteParams);
```

### Sign the Bitcoin Transaction

Create a Bitcoin transaction that sends the quoted `amount` of BTC to the LP's `bitcoinAddress`. This also publishes a hash of the order's parameters in the `OP_RETURN` of the transaction so the Gateway can trustlessly verify the order on BOB.

<Tabs>
<TabItem value="sats-wagmi" label="sats-wagmi (Recommended)">

Please follow the [guide here](/learn/builder-guides/sats-wagmi.md) to install and use sats-wagmi. In this example, we sign the `psbtBase64` using sats-wagmi which abstracts the complex wallet logic for multiple connectors (including OKX, UniSat and Xverse).

It is also possible to directly use the `useSendGatewayTransaction` hook, example below.

```tsx
const { uuid, psbtBase64 } = await gatewaySDK.startOrder(quote, quoteParams);
const bitcoinTxHex = await connector.signAllInputs(psbtBase64!);
await gatewaySDK.finalizeOrder(uuid, bitcoinTxHex);
```

</TabItem>
<TabItem value="send-okx" label="Send (OKX)">

Please refer to the [OKX docs](https://www.okx.com/web3/build/docs/sdks/chains/bitcoin/introduce) for more information.
In this example, instead of signing the `psbtBase64` we instead use the in-built wallet methods to directly send the BTC.

```ts
const { uuid, bitcoinAddress, satoshis, opReturnHash } =
  await gatewaySDK.startOrder(quote, quoteParams);
const { txhash } = await window.okxwallet.bitcoin.send({
  from: quoteParams.fromUserAddress,
  to: bitcoinAddress,
  value: satoshis.toString(),
  memo: opReturnHash,
});
await gatewaySDK.finalizeOrder(uuid, txhash);
```

</TabItem>

<TabItem value="dynamic" label="Dynamic">

Please refer to the Dynamic guide on [PSBT signing](https://docs.dynamic.xyz/wallets/using-wallets/bitcoin/sign-a-psbt).

</TabItem>

<TabItem value="particle" label="Particle">

Please refer to the Particle guide on [BTC Connect](https://developers.particle.network/guides/integrations/partners/bob#connecting-bitcoin-wallets-to-bob-using-btc-connect).

</TabItem>

</Tabs>

### Finalize the Order

Submit the Bitcoin transaction as proof of transfer. This completes the process by transferring wrapped Bitcoin and ETH to the user's EVM address on BOB.

Gateway can broadcast the Bitcoin transaction to the mempool; you can pass the transaction to the SDK without broadcasting from the user's wallet.

```ts
// NOTE: Gateway broadcasts the transaction
await gatewaySDK.finalizeOrder(uuid, tx.hex);
```

### Monitor the User's Orders

Get an array of pending and completed orders for a specific EVM address. Typically rendered in a table.

```ts
const orders = await gatewaySDK.getOrders(userAddress);
```

### Example - sats-wagmi

<Tabs>
<TabItem value="sats-wagmi-app" label="Gateway.tsx">

```js reference title="Gateway.tsx"
https://github.com/bob-collective/sats-wagmi/blob/ae876d96bb2e54e5a24e0f3e1aaa6799565169e4/playgrounds/vite-react/src/Gateway.tsx#L1-L37
```

</TabItem>
<TabItem value="sats-wagmi-hook" label="useSendGatewayTransaction.tsx">

```js reference title="useSendGatewayTransaction.tsx"
https://github.com/bob-collective/sats-wagmi/blob/ae876d96bb2e54e5a24e0f3e1aaa6799565169e4/packages/sats-wagmi/src/hooks/useSendGatewayTransaction.tsx#L28-L69
```

</TabItem>
</Tabs>

## Gateway-Offramp Methods

The following methods allow users to transfer wrapped Bitcoin from BOB to Bitcoin through a seamless and secure process.

### Get a Quote and Start an Order

Call the `createOfframpOrder` method with your `quoteParams`. Example values are shown below:

```ts
const quoteParams: GatewayQuoteParams = {
  bitcoinUserAddress: 'tb1qcwcjsc0mltyt293877552grdktjhnvnnqyv83c', // user address to receive bitcoin on
  fromUserAddress: '0x2D2E86236a5bC1c8a5e5499C517E17Fb88Dbc18c', 
  toToken: 'bobBTC', // wrapped bitcoin token
  amount: 4000000, // Amount should be in token decimals
};

const quote: OfframpCreateOrderParams = await gatewaySDK.createOfframpOrder(quoteParams);
```

Next, sign and send the transaction using the received quote details along with the ABI.
The example below shows how to do this using Viem.  
See more documentation on creating a [Viem client](https://viem.sh/docs/clients/wallet).

Example: 

```ts
import { createWalletClient, http, privateKeyToAccount } from 'viem';
import { bobSepolia } from 'viem/chains';

// create wallet client
const PRIVATE_KEY = '0xYOUR_PRIVATE_KEY'; // Make sure it has the 0x prefix
const account = privateKeyToAccount(PRIVATE_KEY);
const walletClient = createWalletClient({
    account,
    chain: bobSepolia,
    transport: http(), // or custom(yourProvider) if needed
});

// Send the transaction using the received quote
const txHash = await walletClient.writeContract({
    address: quote.quote.registryAddress as `0x${string}`, // The registryAddress from the quote
    abi: quote.offrampABI,
    functionName: quote.offrampFunctionName,
    args: quote.offrampArgs,
});

console.log('Transaction Hash:', txHash);
```

### Monitor the User's Orders

Get an array of pending and completed orders for a specific EVM address.

```ts
const orders: OfframpOrderDetails = await gatewaySDK.getOfframpOrders(userEvmAddress);
```

### Bump Fees for User Order

When monitoring user orders, if `shouldFeesBeBumped` is set to `true`, it indicates that the transaction fees need to be increased.  
This can happen if the fee rate increased between the time the original quote was created and when the Liquidity Provider (LP) was processing the order.

Example:
```ts
const bumpFee: OfframpBumpFeeParams[] = await gatewaySDK.bumpFeeForOfframpOrder(orderId);

// Next, create and send the updated transaction via Viem
```

### Unlock User Order

When monitoring an offramp order, if `canOrderBeUnlocked` returns `true`, it means the order can now be unlocked.  
This happens when the order is still `Active`, or `Accepted` but not processed within the claim delay period.  
Unlocking refunds the user's funds.

Example:

```ts
const orderUnlock: OfframpUnlockFundsParams[] = await gatewaySDK.unlockOfframpOrder(orderId, receiverEvmAddress);

// Next, create and send the unlock transaction via Viem
```


## Conclusion

BOB Gateway enables staking, swapping, lending, and bridging Bitcoin with a single transaction. The BOB SDK makes it possible for you to bring Gateway and Bitcoin LSTs directly to your users.

See the [Code References](#code-references) below for a deeper look at the SDK and an example implementation file.

We look forward to seeing what you Build on Bitcoin!

## Security and Trust Assumptions

The protocol requires zero trust between the market makers and users because it utilizes atomic cross-chain swaps. The verification of the Bitcoin transaction is performed cryptographically by an on-chain Bitcoin [Light Client](/learn/builder-guides/relay.md), making the swap trustless between both parties.

Furthermore, infrastructure run by the BOB team never has access to the market markers' tBTC, wBTC, or ETH funds stored in their smart contracts. The user interface and server manage order flow to prevent liquidity sniping and user errors (e.g. sending BTC without sufficient liquidity being available), but neither the front end or back end ever have access to users' or market makers' funds.

The code has been [audited by Pashov and Common Prefix](/learn/reference/audits#bob-gateway).

## Code References

- `bob/sdk/src/gateway/client.ts`: API client code ([GitHub](https://github.com/bob-collective/bob/blob/master/sdk/src/gateway/client.ts))
- `bob/sdk/examples/gateway.ts`: example client-side implementation ([GitHub](https://github.com/bob-collective/bob/blob/master/sdk/examples/gateway.ts))
