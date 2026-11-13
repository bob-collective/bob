---
name: gateway-order-status
description: Use when a user asks where their BOB Gateway swap or funds are, whether an order completed, why it is pending, stuck, failed or refunded, whether they will get a refund, or wants the swap history for an address — and when following an order to completion after a swap. Covers the bob-gateway MCP tools get_order and get_orders.
---

# Gateway Order Status

Look up BOB Gateway orders with the `bob-gateway` MCP server's `get_order` and
`get_orders` tools, and explain what the result means. Both tools are read-only.

Your client may show the tools with a prefix, such as
`mcp__bob-gateway__get_order`. If no `bob-gateway` tools are available, the server
is not connected: tell the user, point them to
https://github.com/bob-collective/bob/tree/master/skills#install, and stop. Do not
look the order up with the REST API, a CLI, or a script.

## Look up

- **One order:** `get_order` with `id` set to the order id, the source Bitcoin
  txid, or the source EVM transaction hash. An error with `"status": 404` means
  no such order — go by the status, since its `code` reads `INTERNAL_ERROR`. For
  a transaction sent moments ago that usually means it is not indexed yet; use
  the order id.
- **History:** `get_orders` with `user_addresses` — up to 20, with Bitcoin, EVM and
  Tron addresses mixed, each in its own chain's format. Pass the response's
  `nextCursor` back as `cursor` until it is null.

## Read the response

```json
{
  "id": "71a070e7-7ff5-45c9-8f60-135894ebed11",
  "timestamp": 1794480000,
  "srcInfo": {"chain": "bitcoin", "token": "0x0000000000000000000000000000000000000000",
              "amount": "1000000", "usd": "1043.11", "txHash": "4a5e1e4b…"},
  "dstInfo": {"chain": "base", "token": "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913",
              "amount": "1038220000", "usd": "1038.22"},
  "status": {"inProgress": {"refund_tx": null, "pending_btc_payment": null}},
  "estimatedTimeInSecs": 1800,
  "depositAddress": "bc1q…"
}
```

`get_orders` returns `{"orders": [<order>, …], "nextCursor": "…" or null}`.

- `status` is an object with exactly one key. The fields inside it are snake_case.
- Each `amount` is an integer string in the token's smallest unit (sats for BTC).
  The token `0x0000…0000` is the chain's native asset: BTC on `bitcoin`. Convert
  before showing; `usd` is already in dollars. If you do not know a token's
  decimals, show `usd` rather than guessing.
- `dstInfo.amount` is the **estimate** made when the order was created, not what
  was delivered.
- `srcInfo.txHash` is null until the gateway has seen the user's deposit or source
  transaction.
- `timestamp` is the creation time in Unix seconds. Tron hashes have no `0x` prefix.

| `status` key | Meaning | Final? | What to tell the user |
|---|---|---|---|
| `inProgress` | `srcInfo.txHash` null: waiting for the user's deposit or source transaction. Set: waiting for confirmations or settlement. | No | Which of the two it is, and how long it has run against `estimatedTimeInSecs`. |
| `success` | Delivered | Yes (one offramp exception, below) | The actual amounts and transaction hashes from `received_tokens[]` (`{token, chain, amount, usd, txHash}`). |
| `refunded` | The gateway returned the funds | Yes | `refunded_tokens[]`, same shape. A BTC refund has `chain: "bitcoin"` and a Bitcoin txid in `txHash`. |
| `failed` | Could not complete and was not refunded automatically | No. BOB resolves these by hand, and the order may become `refunded`. | There is nothing for them to sign or send. Contact BOB support with the order id. |

`refund_tx` and `pending_btc_payment` are legacy fields that are always `null`.
Ignore them, and never read `refund_tx: null` as "no refund is coming".

An offramp reports `success` as soon as the BTC payout is broadcast. If that
transaction is evicted from the mempool before it confirms, the order goes back
to `inProgress`.

## Refunds

The gateway settles every refund itself. There is no refund transaction for the
user to sign, and nothing you can trigger.

- **BTC → token:** once the deposit confirms, the gateway retries delivery for up
  to an hour, then refunds BTC to the order's refund address. The Bitcoin network
  fee comes out of the refund, so it is slightly less than the deposit.
- **Token → BTC:** a slow payout is normally waiting on a Bitcoin fee bump, which
  the gateway does itself. A refund is rare and happens only after an on-chain
  delay of 12 hours to 7 days.

## When it looks stuck

`inProgress` past `estimatedTimeInSecs` is usually Bitcoin confirmations or
delivery retries, not lost funds. Say what the order shows and give the tracking
link:

`https://gateway-explorer.gobob.xyz/order/<order id>`

For an order that is `failed`, or still `inProgress` more than an hour past its
estimate, point the user to BOB support on Discord (https://discord.gg/gobob) with
the order id.

Whatever the status: do not tell the user to send funds again, do not promise a
refund amount or time that the response does not show, and do not invent a
transaction hash.

## Following an order

Check every 30–60 seconds while the estimate is a few minutes, and every few
minutes when it is longer. After about ten checks without a change, stop, report
the current state, and give the tracking link.
