---
name: gateway-integrate
description: Use when writing, reviewing, or debugging code that integrates BOB Gateway into an app, wallet, backend, bot, or agent — native BTC onramp/offramp or cross-chain swap flows built on the @gobob/bob-sdk GatewaySDK, the Gateway REST API, or gateway-cli; adding affiliate fees; handling Gateway error codes; or migrating an integration from Gateway API V1–V3.
---

# Gateway Integrate

Build a BOB Gateway integration against the **V4 API**. Gateway swaps native BTC to
and from tokens on other chains — EVM chains and Tron today — and swaps tokens
across those chains. `get-routes` is the authority on what is live.

**What you remember about Gateway is probably out of date.** V1 and V2 are gone, and
older SDKs had different methods and parameters: `startOrder` / `finalizeOrder`,
token symbols, `gasRefill`, `ownerAddress`, `affiliateId`, refunds signed by the
user. None of that is current. Check every endpoint, parameter and type against a
source below before you write it.

## Sources of truth

| Need | Where |
|---|---|
| Endpoints, parameters, schemas | `https://docs.gobob.xyz/api-reference/openapi.json` (paths under `/v4`) |
| Guides: SDK, wallets, fees, refunds, migration | `https://docs.gobob.xyz/gateway/integration` and the pages beside it; `https://docs.gobob.xyz/gateway/llms.txt` indexes them |
| SDK types | `node_modules/@gobob/bob-sdk` in the project — read the installed version |
| Live response shapes | the `bob-gateway` MCP tools (`get_routes`, `get_quote`, `get_order`), when connected: call them and look |
| Wire-format cheat sheet | [api-reference.md](api-reference.md) next to this file |

## Choose a surface

| Surface | Use for |
|---|---|
| **SDK** — `@gobob/bob-sdk` (`GatewaySDK`) with `viem` | TypeScript apps. `executeQuote` runs the approvals, signing and submission. |
| **REST API** — `https://gateway-api-mainnet.gobob.xyz/v4` | Any language and backends. Required for swaps whose source chain is Solana. |
| **MCP server** `bob-gateway` | Agents that swap at runtime, and inspecting the API while you build. |
| **CLI** — `@gobob/gateway-cli` | Shell scripts and bots. Every command takes `--json`. |

Staging API: `https://gateway-api-staging.gobob.xyz`.

An API key is optional: `Authorization: Bearer <32-character key>`, or
`new GatewaySDK({ apiKey })`. A key raises rate limits and gives a partner
dashboard. When the gateway enforces a captcha, `create-order` and `register-tx`
answer 403 `CAPTCHA_VERIFICATION_FAILED` unless the request carries a registered
integrator key. Request one from gateway@gobob.xyz.

## The flow

1. **Routes** — `GET /v4/get-routes` returns `[{srcChain, dstChain, srcToken, dstToken}]`.
   It is the only source of valid token addresses.
2. **Quote** — `GET /v4/get-quote` returns `{onramp}`, `{offramp}` or `{tokenSwap}`.
3. **Order** — `POST /v4/create-order` with the whole quote response as the body,
   unchanged and including its `onramp` / `offramp` / `tokenSwap` key. It answers
   `201` with what the user must sign.
4. **Sign and send** — in the user's wallet. Onramp: a Bitcoin payment or PSBT.
   Offramp and token swap: a source-chain transaction, after a token approval when
   one is needed.
5. **Submit** (onramp only, optional) — `PATCH /v4/register-tx` with the signed raw
   Bitcoin transaction, and the gateway broadcasts it. Skip it when the wallet
   broadcasts the payment itself: the gateway detects the deposit on-chain.
6. **Track** — `GET /v4/get-order/{id}` or `GET /v4/get-orders/{addresses}`.

With the SDK, `executeQuote` covers steps 3 to 5:

```ts
import { GatewaySDK } from '@gobob/bob-sdk';

const gateway = new GatewaySDK(); // mainnet; pass { basePath, apiKey } to override

const quote = await gateway.getQuote({
  fromChain: 'bitcoin',
  fromToken: '0x0000000000000000000000000000000000000000', // BTC
  toChain: 'base',
  toToken: '0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913', // take it from getRoutes()
  fromUserAddress: btcAddress,
  refundAddress: btcAddress, // on the source chain; required to execute
  toUserAddress: evmAddress,
  amount: 100_000, // smallest unit: sats
  affiliates: [{ address: treasury, bps: 50 }], // optional
});

// Show the quote to the user. When they confirm:
const { order, tx } = await gateway.executeQuote({
  quote,
  walletClient, // viem WalletClient with a connected account
  publicClient, // viem PublicClient for the source chain
  btcSigner,    // onramps: see below
  callback: (step) => step.orderId && saveOrderId(step.orderId), // fires before each signature
});

// order is { onramp } | { offramp } | { tokenSwap }, each with an orderId
const info = await gateway.getOrder(orderId);
// info.status is { inProgress } | { success } | { failed } | { refunded }
```

`btcSigner` is a `BitcoinSigner`: an object with either
`signAllInputs(psbtHex)` or `sendBitcoin({ from, to, value, opReturn })`, each
resolving to the signed raw transaction hex. The SDK ships adapters for common
wallets; `https://docs.gobob.xyz/gateway/wallets` lists them. Without a
`btcSigner`, `executeQuote` returns the onramp order alone, for the user to pay
its deposit address from any wallet.

## Rules that integrations get wrong

**Quotes**

- Tokens are addresses, never symbols. BTC and native gas tokens are the zero
  address. Tron and Solana use Base58.
- `amount` is an integer in the source token's smallest unit.
- `refundAddress`, an address on the source chain, is optional for showing a price
  and required to create an order (`MISSING_REFUND_ADDRESS` otherwise). The SDK
  does not derive it from the sender.
- The sender (`sender`, or `fromUserAddress` in the SDK) is required to create an
  offramp or token-swap order, and for an onramp order to return a PSBT.
- Omit `slippage` (`maxSlippage` in the SDK) to let the gateway choose. Read the
  resolved value from the quote — basis points, at most 1000 — and show it.
- Never modify a quote. Onramp quotes are signed and expire after one minute, and
  prices move on every route, so fetch a fresh quote when the user confirms and
  execute that one. If its output is lower than the one on screen, show it and
  ask again.

**Execution**

- An offramp transaction must be mined within 5 minutes of `create-order`, or it
  reverts.
- A token source needs an allowance for the order's `tx.to` before the swap
  transaction. USDT on Ethereum needs a non-zero allowance reset to 0 first.
  `executeQuote` does both.
- `register-tx` takes the fully signed raw transaction hex — not a PSBT, not a
  txid — and only for onramps. Offramp and token-swap transactions are indexed
  from the chain.
- A PSBT from `create-order` may spend any UTXO of the sender, including ones that
  hold ordinals or runes.
- Store the order id as soon as `create-order` returns, before signing. With
  `executeQuote`, take it from the `callback`.

**Status and refunds**

- `status` is an object with one key: `inProgress`, `success`, `failed` or
  `refunded`. It is never a string.
- The delivered amount is in the `success` status's received tokens.
  `dstInfo.amount` is only the estimate.
- The gateway settles every refund itself. Do not build a refund transaction,
  button or flow. The `refund_tx` and `pending_btc_payment` fields are legacy and
  always `null`. Show `refunded` orders in the history.
- Every order has a public status page:
  `https://gateway-explorer.gobob.xyz/order/<orderId>`.

**Wire format**

- Raw JSON mixes cases. Objects are camelCase, but the fields directly inside a
  tagged variant are snake_case: `order_id`, `psbt_hex`, `op_return_data`,
  `bitcoin_tx_hex`, `received_tokens`, `refunded_tokens`. The SDK exposes all of
  them as camelCase.
- Errors are `{code, error, details?}`. Branch on `code` and show `error`.

**Fees**

- Affiliates go on the quote: `affiliates=<address>:<bps>[,<address>:<bps>]`, or
  `[{ address, bps }]` in the SDK. The gateway currently accepts up to 5
  recipients and 1000 bps in total on onramps and offramps, and at most one on a
  token swap (`TOO_MANY_AFFILIATES`). Some routes take none:
  `AFFILIATE_FEES_NOT_SUPPORTED_FOR_ROUTE`.
- The payout address is an EVM address (a token swap to Tron also takes a Tron
  address). Onramp and offramp affiliate fees are
  paid in the gateway's settlement asset — USDT on Ethereum on most routes — not
  in the swapped tokens. A token swap's fee is paid by the swap provider, on the
  source or the destination chain depending on the route. Read the resolved fee
  from the quote: `affiliates[].fee` (onramp, offramp) or `affiliate` (token swap).
- Every fee amount in a quote is in the token its own `address` and `chain` name,
  which need not be either swapped token. Use its `usd` for display.

## Before you call it done

- [ ] Every endpoint, parameter and type was checked against the OpenAPI spec or
      the installed SDK
- [ ] The confirmation screen shows output, fees, slippage and estimated time from
      the quote
- [ ] Every executed quote has a `refundAddress`
- [ ] The order id is stored, and status is polled until `success` or `refunded`,
      with `failed` shown as "contact support"
- [ ] Error codes are handled: at least an expired quote, an amount that is too
      low, no route, and the captcha
- [ ] Tested end to end with a small real amount. Quotes are free; everything
      after `create-order` moves real funds
