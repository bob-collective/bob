# Gateway SDK

Client for the BOB **Gateway** — trustless onramp (Bitcoin → BOB / EVM) and offramp
(EVM → Bitcoin), plus EVM cross-chain (LayerZero) transfers and strategy execution.

## Install

```shell
npm install @gobob/bob-sdk viem
# or
yarn add @gobob/bob-sdk viem
# or
pnpm add @gobob/bob-sdk viem
```

## Quick start

```typescript
import { GatewaySDK } from '@gobob/bob-sdk';
import { createPublicClient, createWalletClient, custom, http } from 'viem';
import { bob } from 'viem/chains';

const gateway = new GatewaySDK(); // mainnet by default

// 1. Quote
const quote = await gateway.getQuote({
  fromChain: 'bitcoin',
  toChain: 'bob',
  fromToken: 'BTC',
  toToken: 'WBTC',
  fromUserAddress: 'bc1q...',
  toUserAddress: '0x...',
  amount: 100_000_000, // 1 BTC in satoshis
});

// 2. Execute
const publicClient = createPublicClient({ chain: bob, transport: http() });
const walletClient = createWalletClient({ chain: bob, transport: custom(window.ethereum) });

const { order, tx } = await gateway.executeQuote({
  quote,
  walletClient,
  publicClient,
  btcSigner, // BitcoinSigner — required for onramp signing, optional for the walletless flow
});
```

## Core methods

| Method | Purpose |
|--------|---------|
| `getQuote(params)` | Fetch a quote. Onramp vs offramp inferred from `fromChain` / `toChain`. |
| `executeQuote({ quote, walletClient, publicClient, btcSigner?, callback? })` | Run the full flow for a quote (approvals → on-chain tx / BTC signing). |
| `getOrders({ userAddress, cursor?, limit? })` | Paginated orders for an EVM address → `{ orders, nextCursor? }`. |
| `getOrder(id)` | Single order by id (txId/txHash). |
| `getRoutes()` | Supported routes — chains, tokens, bridges. Source of valid `fromToken` / `toToken` addresses. |
| `getMaxSpendable(address)` | Max spendable BTC for an address. |

`getQuote` returns a discriminated union — narrow it with the generated type guards
(`instanceOfGatewayQuoteV2OneOf` = onramp, `instanceOfGatewayQuoteV3OneOf` = offramp).
Each quote exposes `inputAmount`, `feeBreakdown`, `priceImpact` (fraction, plus
optional `priceImpactUsd`), and `estimatedTimeInSecs`.

`executeQuote` returns `ExecuteQuoteResult` = `{ order, tx }` where `tx` is a Bitcoin
txid (onramp) or EVM hash (offramp). In the **walletless onramp flow** (no `btcSigner`
passed), `tx` is absent — complete the BTC payment externally using
`order.onramp.address` and `order.onramp.orderId`.

`getOrders` order objects carry status detail: `inProgress`, `failed`, `success`,
`refunded` (with token-settlement info), plus `pendingBtcPayment` (`{ txid, amount }`)
for in-flight X→BTC orders and `refundTx` for stuck ones.

## Wallet clients (`walletClient` / `publicClient`)

All EVM-side work in `executeQuote` / `executeStrategy` is done through two injected
clients that **follow the viem [`PublicClient`](https://viem.sh/docs/clients/public)
and [`WalletClient`](https://viem.sh/docs/clients/wallet) interfaces**:

```typescript
walletClient: WalletClient<Transport, Chain, Account>; // a connected account is required
publicClient: PublicClient<Transport>;
```

`publicClient` reads chain state and simulates; `walletClient` signs and broadcasts.
On a standard EVM chain you create both with viem (`createPublicClient` /
`createWalletClient`) and inject them directly.

The SDK only calls the subset listed below — anything that satisfies these call
signatures is accepted, regardless of the underlying chain. That is the seam used
for non-EVM chains like Tron.

### `publicClient` — required surface

| Member | Used for |
|--------|----------|
| `readContract({ address, abi, functionName, args? })` | ERC-20 `allowance`, OFT `approvalRequired`. |
| `simulateContract({ account, address, abi, functionName, args })` → `{ request }` | Build the validated `approve` / reset request before writing. |
| `waitForTransactionReceipt({ hash, retryCount })` | Block on approval / send receipts. |

### `walletClient` — required surface

| Member | Used for |
|--------|----------|
| `account` (`Account` with `.address`) | Sender address; `executeQuote` throws if absent for offramp. |
| `writeContract(request)` → `Hash` | Send the `approve` / allowance-reset tx produced by `simulateContract`. |
| `sendTransaction({ account, to, data, value })` → `Hash` | Broadcast the gateway order transaction. |

## Tron support

To run an onramp/offramp against Tron you must **inject a `walletClient` and a
`publicClient` that conform to the viem interface above** — adapters that wrap
TronWeb (or your Tron signer of choice) and expose viem-shaped methods. The SDK
detects Tron token addresses with `isValidTronAddress` / `tronAddressToHex` and
otherwise drives the exact same code path.

Your adapters must implement at least the following — argument and return shapes
must match viem:

```typescript
import type { Abi, Account, Address, Hash, Hex } from 'viem';

// --- publicClient ---
interface TronPublicClient {
  readContract(args: {
    address: Address;
    abi: Abi;
    functionName: string;
    args?: readonly unknown[];
  }): Promise<unknown>;

  simulateContract(args: {
    account: Address;
    address: Address;
    abi: Abi;
    functionName: string;
    args: readonly unknown[];
  }): Promise<{ request: unknown }>; // `request` is passed verbatim to writeContract

  waitForTransactionReceipt(args: {
    hash: Hash;
    retryCount?: number;
  }): Promise<{ status: 'success' | 'reverted' } & Record<string, unknown>>;
}

// --- walletClient ---
interface TronWalletClient {
  account: Account; // must carry `.address`

  // Consumes the `request` returned by publicClient.simulateContract
  writeContract(request: unknown): Promise<Hash>;

  sendTransaction(args: {
    account: Address;
    to: Address;
    data: Hex;
    value: bigint;
  }): Promise<Hash>;
}
```

Notes for the adapter author:

- **`account.address` is mandatory.** Offramp throws `walletClient is required for
  offramp order` if `walletClient.account` is missing.
- **`simulateContract` → `writeContract` is a pair.** The object you return as
  `request` is handed straight back to `writeContract`; keep whatever your signer
  needs inside it.
- **Approvals reuse the ERC-20 `approve` / `allowance` ABI** (and the USDT
  zero-then-set reset). Map these to the TRC-20 equivalents so amounts and the
  spender (`order.offramp.tx.to`) line up.
- **`waitForTransactionReceipt` must actually block** until the tx is mined and
  honor `retryCount` (the SDK uses `RETRY_COUNT = 8`); resolving early breaks the
  approve → send ordering.
- **`value` is a `bigint`** of the native amount (from `order.offramp.tx.value`),
  `data` is `0x`-prefixed hex.

## Bitcoin signing (`btcSigner`)

`executeQuote` accepts an optional `BitcoinSigner` with two mutually exclusive
patterns (an adapter implements one):

```typescript
interface BitcoinSigner {
  // High-level: wallet builds + broadcasts (e.g. OKX). Returns txid/hex.
  sendBitcoin?(params: {
    from: string | null | undefined;
    to: string;
    value: string;
    opReturn?: string;
    isSignet?: boolean;
  }): Promise<string>;

  // Low-level: sign a PSBT, return signed hex (e.g. Reown).
  signAllInputs?(psbtHex: string): Promise<string>;
}
```

### Custom adapter

```typescript
import type { BitcoinSigner } from '@gobob/bob-sdk';

class CustomWalletAdapter implements BitcoinSigner {
  async sendBitcoin(params: {
    from: string;
    to: string;
    value: string;
    opReturn?: string;
  }): Promise<string> {
    // build, sign, broadcast — return txid/signed hex
  }

  async signAllInputs(psbtHex: string): Promise<string> {
    // sign PSBT, return signed hex
  }
}
```

## Affiliate fees

`getQuote` accepts an `affiliates` array to split fees across multiple recipients per
quote. Each entry is `{ address, bps }` (1 bps = 0.01%); `bps` must be > 0:

```typescript
const quote = await gateway.getQuote({
  // ...other params
  affiliates: [
    { address: '0xPartnerA', bps: 50 }, // 0.50%
    { address: '0xPartnerB', bps: 25 }, // 0.25%
  ],
});
```

Routes that don't support affiliate fees return `AFFILIATE_FEES_NOT_SUPPORTED_FOR_ROUTE`.

## Errors

API failures throw a typed `GatewayError` (`isGatewayError`, `GatewayErrorCode`,
and per-code detail types — `NoRouteDetails`, `ExceededLimitDetails`, …). Non-JSON
HTTP errors are wrapped via `GatewayError.fromText`.
