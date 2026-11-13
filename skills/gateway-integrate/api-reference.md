# Gateway V4 wire format

A cheat sheet for the raw REST API. The OpenAPI spec at
`https://docs.gobob.xyz/api-reference/openapi.json` is authoritative; when the two
disagree, the spec wins.

Base URL: `https://gateway-api-mainnet.gobob.xyz` (staging:
`https://gateway-api-staging.gobob.xyz`). Optional header on every request:
`Authorization: Bearer <32-character API key>`.

## Endpoints

| Call | Purpose |
|---|---|
| `GET /v4/get-routes` | All supported routes |
| `GET /v4/get-quote` | Quote a swap |
| `POST /v4/create-order` | Create an order from a quote; answers `201` |
| `PATCH /v4/register-tx` | Onramp only: broadcast a signed Bitcoin transaction through the gateway |
| `GET /v4/get-order/{id}` | One order by order id, source Bitcoin txid, or source EVM transaction hash |
| `GET /v4/get-orders/{addresses}` | Orders for up to 20 comma-separated addresses; query `cursor`, `limit` |
| `GET /v4/get-max-spendable/{address}` | Most BTC (sats) a Bitcoin address can send after network fees |

## get-quote query parameters

| Parameter | Required | Notes |
|---|---|---|
| `srcChain`, `dstChain` | yes | Chain names as `get-routes` returns them, e.g. `bitcoin`, `ethereum`, `base`, `bob`, `tron` |
| `srcToken`, `dstToken` | yes | Token addresses in the chain's own format; the zero address for BTC and native gas tokens |
| `amount` | yes | Integer string, smallest unit of the source token |
| `recipient` | yes | Address on the destination chain |
| `sender` | no | Address on the source chain. Needed to create offramp and token-swap orders, and for an onramp PSBT |
| `refundAddress` | no | Address on the source chain. Needed to create any order |
| `slippage` | no | Basis points, at most 1000. Omit it and the gateway chooses |
| `affiliates` | no | `<address>:<bps>` pairs, comma-separated |

## Quote response

Exactly one of three keys. Every amount is
`{"amount": "<integer string>", "address": "<token>", "chain": "<chain>", "usd": "<string or null>"}`.

```jsonc
{ "onramp": {
    "inputAmount": {…}, "outputAmount": {…},
    "fees": {…}, "executionFees": {…},
    "feeBreakdown": { "solverFee": {…}, "protocolFee": {…}, "affiliateFee": {…},
                      "executionFee": {…}, "layerzeroFee": {…} },
    "sender": "bc1q… or null", "recipient": "0x…",
    "dstChain": "base", "dstToken": "0x…",
    "slippage": "50",                    // string, basis points
    "affiliates": [ { "address": "0x…", "fee": {…} } ],
    "estimatedTimeInSecs": 483,
    "priceImpact": "-0.0004",            // fraction; may be null
    "priceImpactUsd": "-0.03",
    "signedQuoteData": "…"               // the signature; send it back untouched
} }

{ "offramp": {
    "srcChain": "ethereum", "tokenAddress": "0x…",
    "inputAmount": {…}, "outputAmount": {…},   // output is in sats
    "totalFeeUsd": "1.23",
    "feeBreakdown": { "solverFee": {…}, "inclusionFee": {…}, "protocolFee": {…},
                      "affiliateFee": {…}, "fastestFeeRate": "12" },   // inclusionFee is in BTC
    "sender": "0x… or null", "recipient": "bc1q…",
    "slippage": 50,                      // number, basis points
    "affiliates": [ … ],
    "txTo": "0x…",                       // the contract the order transaction calls
    "estimatedTimeInSecs": 600,
    "priceImpact": "-0.0010", "priceImpactUsd": "-0.25",
    "refundAddress": "0x… or null"
} }

{ "tokenSwap": {
    "srcChain": "base", "dstChain": "ethereum",
    "inputAmount": {…}, "outputAmount": {…},
    "fees": {…},                         // placeholder, always zero
    "sender": "0x… or null", "recipient": "0x…",
    "slippage": 50,
    "txTo": "0x…",
    "affiliate": { "address": "0x…", "bps": 50 },   // or null; at most one per token swap
    "estimatedTimeInSecs": 60,
    "priceImpact": "-0.0010", "priceImpactUsd": "-0.25",
    "refundAddress": "0x… or null"
} }
```

Each amount is in the token its own `address` and `chain` name. Fee lines are
often in USDT on Ethereum whatever the route, so do not assume a swapped token.

Other fields exist; pass the whole object back to `create-order` without reading
meaning into them.

## create-order response

The request body is the whole quote response, key included. The response has the
same key as the quote, and the fields directly under it are snake_case.

```jsonc
{ "onramp": {
    "order_id": "f81d4fae-7dec-11d0-a765-00a0c91e6bf6",
    "address": "bc1q…",                  // pay inputAmount.amount sats here
    "op_return_data": null,              // when set, the payment must carry it in an OP_RETURN
    "psbt_hex": "70736274…"              // only when the quote had a sender
} }

{ "offramp":   { "order_id": "…", "tx": <tx> } }
{ "tokenSwap": { "order_id": "…", "tx": <tx> } }
```

`<tx>` is tagged by `type`:

```jsonc
{ "type": "evm",    "to": "0x…", "data": "0x…", "value": "0", "chain": "ethereum" }
{ "type": "tron",   "to": "T…",  "data": "0x…", "value": "0", "chain": "tron", "feeLimit": "…" }  // feeLimit in sun
{ "type": "solana", "chainId": 0, "transaction": "<base64>" }
```

The Solana `transaction` is a base64 serialized unsigned `VersionedTransaction`:
decode it, sign it, re-serialize it and broadcast it. Do not sign the base64 string.

## register-tx

```jsonc
// PATCH /v4/register-tx
{ "onramp": { "order_id": "…", "bitcoin_tx_hex": "<fully signed raw transaction hex>" } }
// → { "onramp": { "txid": "…" } }
```

## Order

```jsonc
{
  "id": "71a070e7-7ff5-45c9-8f60-135894ebed11",
  "timestamp": 1794480000,                       // creation, Unix seconds
  "srcInfo": { "chain": "bitcoin", "token": "0x0000…", "amount": "1000000",
               "usd": "1043.11", "txHash": "4a5e…" },   // txHash null until seen
  "dstInfo": { "chain": "base", "token": "0x8335…", "amount": "1038220000",
               "usd": "1038.22" },                      // amount is the estimate
  "status": <status>,
  "estimatedTimeInSecs": 1800,
  "depositAddress": "bc1q…"                      // onramps
}
```

`<status>` has exactly one key:

```jsonc
{ "inProgress": { "refund_tx": null, "pending_btc_payment": null } }
{ "success":    { "received_tokens": [ <transfer> ] } }
{ "refunded":   { "refunded_tokens": [ <transfer> ] } }
{ "failed":     { "refund_tx": null } }
```

`<transfer>` is `{"token", "chain", "amount", "usd", "txHash"}`. `refund_tx` and
`pending_btc_payment` are legacy fields and always `null`. `get-orders` answers
`{"orders": [ <order> ], "nextCursor": "… or null"}`.

## Errors

Non-2xx responses carry `{"code": "…", "error": "<message>", "details": {…}}`
(`details` is optional).

| HTTP | Codes |
|---|---|
| 400 | `INVALID_REQUEST` (also an expired or tampered onramp quote, and an expired order), `MISSING_REFUND_ADDRESS`, `INVALID_ORDER_ARGS`, `INVALID_AFFILIATE_FEE`, `SLIPPAGE_TOO_LOW`, `SLIPPAGE_TOO_HIGH`, `INSUFFICIENT_CONFIRMED_FUNDS`, `INVALID_DESTINATION_CHAIN_ID` |
| 404 | `NO_ROUTE`, `ORDER_NOT_FOUND`. An unknown id on `get-order` answers 404 with code `INTERNAL_ERROR` and message `Not Found`: go by the status |
| 409 | `DUPLICATE_ORDER` |
| 422 | `INSUFFICIENT_AMOUNT`, `QUOTE_AMOUNT_TOO_LOW`, `INSUFFICIENT_SWAP_AMOUNT`, `INSUFFICIENT_SOLVER_BALANCE`, `INSUFFICIENT_PAYMENT_AMOUNT`, `EXCEEDED_LIMIT`, `UNABLE_TO_COVER_FEES` |
| 429 | `PER_ACCOUNT_LIMIT_EXCEEDED`, `GLOBAL_LIMIT_EXCEEDED` |
| 503 | `DISABLED_CHAIN` |
| 500 | `SIMULATION_FAILED`, `GAS_ESTIMATE_FAILED`, `INTERNAL_ERROR` |

Further codes exist — among them `TOO_MANY_AFFILIATES`,
`AFFILIATE_FEES_NOT_SUPPORTED_FOR_ROUTE`, `NON_COMPLIANT_ADDRESSES`,
`BUNGEE_NO_ROUTE`, `SERVICE_UNAVAILABLE` and `INVALID_CURSOR`. Take the HTTP status
of any code not in this table from the response, and treat an unknown code as a
failure to show the user.
