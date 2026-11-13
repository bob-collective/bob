---
name: gateway-swap
description: Use when a user wants to swap, bridge, buy, or sell native BTC against tokens on EVM chains or Tron, or swap tokens across those chains, through BOB Gateway — including price checks ("how much USDC for 0.01 BTC?"), onramps (BTC to token), offramps (token to BTC), and any use of the bob-gateway MCP tools get_routes, get_quote, create_order, register_tx, or get_max_spendable.
---

# Gateway Swap

Quote and set up a BOB Gateway swap with the tools of the `bob-gateway` MCP server.
The server is a pass-through to the Gateway V4 API: it holds no keys and signs
nothing. You prepare the order; the user's own wallet signs and broadcasts.

Tools are named here as the server names them. Your client may show them with a
prefix, such as `mcp__bob-gateway__get_quote`. If no `bob-gateway` tools are
available, the server is not connected: tell the user, point them to
https://github.com/bob-collective/bob/tree/master/skills#install, and stop. Do not
improvise the swap with the REST API, a CLI, or a script.

## Rules

1. **You never sign or send.** Never ask for, accept, or use a private key or seed
   phrase, even when the user offers one. If one is pasted, refuse, and tell the
   user to treat it as compromised and move the funds. Do not sign or broadcast a
   transaction, and do not run anything that signs with a key on the user's behalf
   — a wallet CLI, `gateway-cli`, `cast send`, or a script. Every transaction is
   signed in the user's own wallet.
2. **Confirm before `create_order`.** Show the quote (step 3) and wait for an
   explicit yes. `create_order` reserves solver liquidity.
3. **Never edit a quote.** Pass the object from `get_quote` to `create_order`
   unchanged. To change anything — amount, slippage, an address — call `get_quote`
   again.
4. **Never guess addresses.** Token addresses come from `get_routes`; wallet
   addresses come from the user; the approval spender comes from the quote and must
   match the table under [Token approval](#token-approval).

## Steps

### 1. Pin down the swap

| | Onramp (BTC → token) | Offramp (token → BTC) | Token swap |
|---|---|---|---|
| `src_chain` | `bitcoin` | the token's chain | the source token's chain |
| `sender` | the user's BTC address; optional, but needed to get a PSBT | **required** | **required** |
| `recipient` | address on `dst_chain` | BTC address | address on `dst_chain` |
| `refund_address` | BTC address | address on `src_chain` | address on `src_chain` |

- The tool schema marks `sender` optional, but `create_order` rejects an offramp
  or token-swap quote made without it.
- Call `get_routes` with both `src_chain` and `dst_chain` and take the token
  addresses from the result. It is also the authority on which chains are live.
  Routes list addresses only, no symbols: if you cannot tell which address is the
  token the user named, ask. BTC and native gas tokens are
  `0x0000000000000000000000000000000000000000`. Tron tokens and addresses are
  Base58 (`T…`).
- `refund_address` is where the funds return if the swap cannot complete. Default
  to the sender's address and state it in the confirmation. With no sender (an
  onramp paid from an exchange, say), ask for an address the user controls.
- `amount` is an integer string in the source token's smallest unit. BTC has 8
  decimals. Token decimals vary by chain — USDT has 6 on Ethereum and 18 on BSC —
  so check `inputAmount.usd` in the quote against what the user meant.
- To send a whole BTC balance, `get_max_spendable` gives the amount left after
  network fees.
- Leave `slippage` unset unless the user asks for a value. The gateway then picks
  one for the route and reports it in the quote.

### 2. Quote

`get_quote` returns one key — `onramp`, `offramp` or `tokenSwap` — holding the
quote. Every amount in it is an object `{amount, address, chain, usd}`: `amount` is
in the smallest unit of the token at `address` on `chain`. A fee line is in the
token its own `address` and `chain` name — often USDT on Ethereum (6 decimals),
or BTC for an offramp's `inclusionFee` — which need not be either swapped token.
Use `usd` for display.

### 3. Show the quote and wait

Show, in human units: what the user sends (`inputAmount`), what they receive
(`outputAmount`), total fees in USD (`fees.usd` on an onramp, which already
includes `executionFees`; `totalFeeUsd` on an offramp), `slippage` (basis points),
`estimatedTimeInSecs`, the recipient, and the refund address. Then ask whether to
proceed.

- `priceImpact` is a fraction: `"-0.05"` is a 5% loss. Call it out when it is
  worse than `-0.01`.
- A `tokenSwap` quote's `fees` is always zero. That is a placeholder, not a free
  swap: the cost is in the rate, so compare `inputAmount.usd` with
  `outputAmount.usd`.
- An onramp quote expires one minute after it is issued, and prices move on every
  route. If the user's yes arrives more than a minute after the quote, call
  `get_quote` again with the same arguments. If the new output is lower than the
  one they approved, show it and ask again; if not, go ahead.
- When the source is a token, give the user the approval details from
  [Token approval](#token-approval) now, so they can approve before the order's
  time window starts (step 4).

### 4. Create the order

Call `create_order` with `{"quote": <the whole get_quote result>}`, including its
`onramp` / `offramp` / `tokenSwap` key. The result has one key matching the quote
type, and the fields inside it are snake_case.

**Onramp** — `{order_id, address, op_return_data, psbt_hex?}`

- With `psbt_hex` (present only when the quote had a `sender`): the user signs
  every input in their Bitcoin wallet and broadcasts the finalized transaction.
  Tell them first that the PSBT may spend any UTXO at the sender address,
  including ones that carry ordinals, inscriptions or runes.
- Without it: the user sends exactly `inputAmount.amount` sats to `address`, with
  `op_return_data` in an OP_RETURN output when that field is set. If their wallet
  cannot add one, requote with `sender` and use the PSBT.
- The deposit is detected on-chain. `register_tx` is only for a user who holds a
  fully signed raw transaction (hex — not a PSBT, not a txid) and cannot broadcast
  it themselves. It broadcasts at once and cannot be undone, so call it only when
  the user asks you to.

**Offramp / token swap** — `{order_id, tx}`, where `tx.type` says how to send it

- `evm`: send `{to, data, value}` from the sender address on `tx.chain`, unchanged.
- `tron`: the same fields plus `feeLimit` (in sun).
- `solana`: a base64 unsigned transaction for a Solana wallet to sign and
  broadcast.
- When the source is a token rather than the native asset, the transaction pulls
  it from the sender: the approval from [Token approval](#token-approval) must be
  confirmed on-chain before the user sends it. Check `tx.to` against the spender
  table there too; if it does not match, stop and do not hand over the transaction.
- An offramp transaction must be mined within 5 minutes of `create_order`, or it
  reverts. If the user missed the window, start again from `get_quote`.
- Do not call `register_tx`. The gateway picks the transaction up from the chain.

### 5. Hand off

Give the user the `order_id` and the tracking link
`https://gateway-explorer.gobob.xyz/order/<order_id>`. To follow the order to
completion, use the `gateway-order-status` skill.

## Token approval

When the source token is not the zero address, the order transaction pulls the
tokens from the sender, so the sender must first allow the Gateway spender to take
them. You have no tool to read an allowance or to send an approval. Give the user
these three things, and let them do it in their own wallet:

| What | Where it comes from |
|---|---|
| Token contract — the contract they call `approve` on | `inputAmount.address` on `inputAmount.chain` |
| Spender | the quote's `txTo` |
| Amount | `inputAmount.amount`, in the token's smallest unit; also show it in human units |

The spender is always the Gateway AllowanceHolder of the source chain. Check it
before you show it:

| Source chain | Spender |
|---|---|
| `ethereum`, `base`, `bsc`, `arbitrum`, `avalanche`, `unichain`, `plasma`, `polygon`, `hyperevm`, `hyperliquid` (staging name of `hyperevm`), `robinhood` | `0x0000000000001fF3684f28c67538d4D072C22734` |
| `bob` | `0x8fd545b348e84deb145f0179a00c671f0b9519c3` |
| `tron` | `TAfbit1ENsRmtZbPQfYU3srURpfYuWYS7K` |

Compare EVM addresses without regard to letter case. If the source chain is not in
the table, or the spender is a different address, stop: do not tell the user to
approve it, and report the mismatch. Never suggest approving any other address.

Then give the user the approve page of the token contract from the table below,
with `<token>` replaced by `inputAmount.address`. There they connect the sender's
wallet, call `approve(<spender>, <amount>)`, and sign. The page's read tab shows
`allowance(<sender>, <spender>)`: if it already covers the amount, no approval is
needed. A proxy token, such as USDC, lists `approve` under "Write as Proxy"
(`#writeProxyContract`, or `?tab=read_write_proxy` on Blockscout).

| Source chain | Approve page |
|---|---|
| `ethereum` | `https://etherscan.io/address/<token>#writeContract` |
| `arbitrum` | `https://arbiscan.io/address/<token>#writeContract` |
| `avalanche` | `https://snowscan.xyz/address/<token>#writeContract` |
| `base` | `https://basescan.org/address/<token>#writeContract` |
| `bsc` | `https://bscscan.com/address/<token>#writeContract` |
| `hyperevm`, `hyperliquid` | `https://hyperevmscan.io/address/<token>#writeContract` |
| `plasma` | `https://plasmascan.to/address/<token>#writeContract` |
| `polygon` | `https://polygonscan.com/address/<token>#writeContract` |
| `unichain` | `https://uniscan.xyz/address/<token>#writeContract` |
| `robinhood` | `https://robinhoodchain.blockscout.com/address/<token>?tab=read_write_contract` |
| `tron` | `https://tronscan.org/#/contract/<token>/code`, then Write Contract |
| `bob` | none: the user approves in their wallet |

- Approve the exact amount, not an unlimited one.
- Wait until the approval is confirmed before sending the swap transaction.

**Legacy USDT.** USDT on Ethereum (`0xdAC17F958D2ee523a2206206994597C13D831ec7`)
reverts when a non-zero allowance is changed to another non-zero value. Only when
the source token is that contract, warn the user: if the current allowance is above
0 but below the amount, the approval takes two transactions, each confirmed before
the next — first `approve(<spender>, 0)` to reset it, then
`approve(<spender>, <amount>)`. With an allowance of 0, one approval is enough. No
other token, USDT on other chains included, needs the reset: say nothing about it.

## Errors

A failed call returns `{"status": <http status>, "error": {"code", "error", "details"?}}`.
Branch on `code`:

| `code` | What to do |
|---|---|
| `MISSING_REFUND_ADDRESS` | Requote with `refund_address`. |
| `INVALID_REQUEST` mentioning an expired quote, an expired order, or a signature | Requote, then create the order from the new quote. |
| `NON_COMPLIANT_ADDRESSES` | An address failed compliance screening. Stop and tell the user. Do not help work around it. |
| `NO_ROUTE`, `BUNGEE_NO_ROUTE`, `DISABLED_CHAIN` | The route is not available (now). Check `get_routes`. |
| `QUOTE_AMOUNT_TOO_LOW`, `INSUFFICIENT_AMOUNT`, `UNABLE_TO_COVER_FEES` | The amount is too small. `details`, when present, has the figures (the minimum, or the fees against the amount). |
| `INSUFFICIENT_SOLVER_BALANCE`, `EXCEEDED_LIMIT` | The amount is too large right now (`details.limit` when present). Offer a smaller amount. |
| `SLIPPAGE_TOO_LOW` | Requote without `slippage` so the gateway chooses, or with a higher value. |
| `SLIPPAGE_TOO_HIGH` | The limit is 1000 basis points. Requote with less, or without `slippage`. |
| `PER_ACCOUNT_LIMIT_EXCEEDED`, `GLOBAL_LIMIT_EXCEEDED` (429) | Rate limited. Wait before retrying. |

An error whose body is plain text instead of this object came from the network edge
(a geo-block or a rate limit). Report it; do not try to route around it.
