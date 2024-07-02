---
sidebar_position: 1
---

# BOB Gateway Integration Guide

## What is BOB Gateway?

[BOB Gateway](https://docs.gobob.xyz/docs/learn/guides/bitcoin-bridge/) is a trustless, P2P bridge that enables users to bridge their BTC from Bitcoin mainnet to BOB.

You can use our SDK to add this UX directly into your app, bringing your app's functionality (lending, swapping, restaking, etc.) directly to Bitcoin hodlers.

## How Does it Work?

1. Liquidity providers (LPs) temporarily lock wrapped Bitcoin (WBTC or tBTC) in escrow smart contracts on BOB.
2. A user submits a transaction to the Bitcoin network that sends BTC to the liquidity provider's Bitcoin address and publishes the user's EVM wallet address in the `OP_RETURN` of the transaction.
3. BOB's relayer server calls an on-chain [Light Client](../examples/btc-swap/index.mdx) to trustlessly verify the Bitcoin transaction and transfer the LP's wrapped Bitcoin to the user's EVM address, including a "gratuity" amount of ETH for transaction fees on BOB.

This SDK makes it possible to do steps 2 and 3 in your application's front-end.

## Step-by-Step Integration Guide

Follow these steps to add BOB Gateway's functionality to your application.

### Install the BOB SDK

Install the `@gobob/bob-sdk` package using your preferred package manager:

```sh
# with Yarn
$ yarn add @gobob/bob-sdk

# with npm
$ npm i @gobob/bob-sdk

# with pnpm
$ pnpm add @gobob/bob-sdk

# with Bun
$ bun add @gobob/bob-sdk
```

### Initialize the API Client

Import the `GatewayApiClient` class from the `@gobob/bob-sdk` package and create an instance of it:

```ts title="/src/utils/gateway.ts"
import { GatewayApiClient } from "@gobob/bob-sdk";

const gatewayClient = new GatewayApiClient(
  "https://onramp-api-mainnet.gobob.xyz"
);
```

### Get a Quote

Call the `getQuote` method with two parameters:

- the address of the desired output token
  - WBTC: [0x03c7054bcb39f7b2e5b2c7acb37583e32d70cfa3](https://explorer.gobob.xyz/address/0x03c7054bcb39f7b2e5b2c7acb37583e32d70cfa3)
  - tBTC: [0xBBa2eF945D523C4e2608C9E1214C2Cc64D4fc2e2](https://explorer.gobob.xyz/address/0xBBa2eF945D523C4e2608C9E1214C2Cc64D4fc2e2)
- the amount of BTC the user is bridging, expressed in satoshis. For example, 1 BTC is 100000000 satoshis.

and receive two outputs:

- `onrampAddress`, the smart contract holding funds from the LP with the most competitive offer
- `bitcoinAddress`, the Bitcoin address where the user will send their BTC

```ts title="/src/utils/gateway.ts"
const { onramp_address: onrampAddress, bitcoin_address: bitcoinAddress } =
  await gatewayClient.getQuote(BOB_TBTC_V2_TOKEN_ADDRESS, amount);
```

### Create an Order

This locks in the quote a places a hold on the LP's funds. Pass `evmAddress`, the wallet address where your user would like to receive funds on BOB, and `amount`, the amount of BTC they would like to bridge expressed in satoshis.

```ts title="/src/utils/gateway.ts"
const orderId = await gatewayClient.createOrder(
  onrampAddress,
  evmAddress,
  amount
);
```

### Send BTC

Create a Bitcoin transaction that sends the quoted `amount` of BTC from the user's wallet (`fromAddress`) to the LP's `bitcoinAddress`. This also publishes the user's `evmAddress` in the `OP_RETURN` of the transaction so the Gateway knows where to send wrapped BTC.

:::tip Connecting to Bitcoin wallets
We recommend using our [sats-wagmi](./sats-wagmi.md) package to query your user's Bitcoin wallet address.
:::

```ts title="/src/utils/gateway.ts"
import { createTransfer } from "@gobob/bob-sdk";
import * as bitcoin from "bitcoinjs-lib";
import { AddressType, getAddressInfo } from "bitcoin-address-validation";
import { hex } from "@scure/base";
import { Transaction as SigTx } from "@scure/btc-signer";

const tx = await createTxWithOpReturn(
  fromAddress,
  bitcoinAddress,
  amount,
  evmAddress
);

async function createTxWithOpReturn(
  fromAddress: string,
  toAddress: string,
  amount: number,
  opReturn: string,
  fromPubKey?: string
): Promise<bitcoin.Transaction> {
  const addressType = getAddressInfo(fromAddress).type;

  // Ensure this is not the P2TR address for ordinals (we don't want to spend from it)
  if (addressType === AddressType.p2tr) {
    throw new Error(
      "Cannot transfer using Taproot (P2TR) address. Please use another address type."
    );
  }

  // We need the public key to generate the redeem and witness script to spend the scripts
  if (addressType === (AddressType.p2sh || AddressType.p2wsh)) {
    if (!fromPubKey) {
      throw new Error(
        "Public key is required to spend from the selected address type"
      );
    }
  }

  const unsignedTx = await createTransfer(
    "mainnet",
    addressType,
    fromAddress,
    toAddress,
    amount,
    fromPubKey,
    opReturn
  );
  const psbt = unsignedTx.toPSBT(0);
  const psbtHex = hex.encode(psbt);
  const signedPsbtHex = psbtHex;
  const signedTx = SigTx.fromPSBT(
    bitcoin.Psbt.fromHex(signedPsbtHex).toBuffer()
  );
  signedTx.finalize();
  const tx = bitcoin.Transaction.fromBuffer(Buffer.from(signedTx.extract()));

  return tx;
}
```

### Receive Wrapped BTC

Submit the Bitcoin transaction information as proof of transfer. This completes the process by transferring wrapped Bitcoin and ETH to the user's EVM address on BOB.

```ts
await gatewayClient.updateOrder(orderId, tx.toHex());
```

## Conclusion

Following the steps above allows users to bridge BTC from Bitcoin mainnet to BOB within your application. From there, they can connect their EVM wallet and use their wrapped BTC in your dapp.

You're always welcome to [reach out to us](../../learn/introduction/contribution.md) with questions, feedback, or ideas. We look forward to seeing what you Build on Bitcoin!

## Code References

- `/src/gateway.ts`: API client code ([GitHub](https://github.com/bob-collective/bob/blob/master/sdk/src/gateway.ts))
- `/examples/gateway.ts`: example client-side implementation ([GitHub](https://github.com/bob-collective/bob/blob/master/sdk/examples/gateway.ts))
