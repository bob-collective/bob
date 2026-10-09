# SDK — @gobob/bob-sdk

## Build & Test

```shell
pnpm install
pnpm run build        # tsc → dist/
pnpm run test         # vitest
pnpm run lint         # eslint
pnpm run format       # prettier
pnpm run codegen      # regenerate OpenAPI client (requires OPEN_API_SPEC_URL env)
```

## Architecture

```
sdk/src/
├── gateway/
│   ├── client.ts              # GatewayApiClient (exported as GatewaySDK) — main entry point
│   ├── generated-client/      # OpenAPI-generated — DO NOT EDIT (regenerated entirely by `pnpm run codegen`)
│   ├── adapters/              # Wallet adapters implementing BitcoinSigner (OKX, Reown)
│   ├── types/                 # Hand-written types (BitcoinSigner, GetQuoteParams, StrategyParams)
│   └── utils/                 # Chain resolution, BTC formatting
├── esplora.ts                 # Bitcoin block explorer client
├── mempool.ts                 # Mempool fee estimation
└── wallet/                    # Wallet utilities
```

### Generated vs hand-written code

Everything under `src/gateway/generated-client/` is auto-generated from the OpenAPI spec of the [bob-gateway](https://github.com/bob-collective/bob-gateway) backend. The entire directory is deleted and recreated on each codegen run. Never edit files there — put hand-written code in `client.ts`, `types/`, `utils/`, or `adapters/`.

When debugging API issues or verifying request/response shapes, refer to the gateway repo for the source of truth.

ESLint ignores `src/gateway/generated-client/**`.

### Error bodies skip the generated deserialization

Successful responses pass through the generated `…FromJSON` deserializers (`JSONApiResponse`, in
`generated-client/apis/`), so everywhere else in the SDK sees camelCase. Error responses do not: the
middleware in `client.ts` hands `response.json()` straight to `GatewayError.fromResponse`, so
`parseDetails` receives the body exactly as the Gateway sent it — snake_case, like `required_bps` or
`src_chain`. Reading those by their camelCase names returns `undefined` with no type error, because
the parameter is typed as the shape the function *returns*.

Map them with the **per-variant** helper the `code` switch already selects —
`GatewayErrorDetailsV3OneOfFromJSON(raw)` — never the union-level `GatewayErrorDetailsV3FromJSON(raw)`.
The names differ only by `OneOf`, but the union one guesses the variant from the body's shape,
ignores `code`, and returns `{}` for every real error body.

Those helpers copy fields through with no fallback, while `DetailsFor` still declares each one
required. A response omitting a field therefore leaves `undefined` behind a type that says
otherwise.

## Public API

### GatewaySDK (primary class)

Instantiated with optional `basePath` and `apiKey` arguments (defaults to mainnet). Core flow:

1. **`getQuote(params)`** — fetch a quote (returns a discriminated union: onramp | offramp | layerZero)
2. **`executeQuote({ quote, walletClient, publicClient, btcSigner? })`** — execute the full transaction flow for a quote
3. **`getOrders(address)`** — list all orders for an EVM address
4. **`getRoutes()`** — list supported token/chain routes
5. **`getMaxSpendable(address)`** — max spendable BTC for an address

### Quote type discrimination

Quotes are a union type. Use generated type guards to narrow:
- `instanceOfGatewayQuoteOneOf(quote)` → onramp (BTC → token)
- `instanceOfGatewayQuoteOneOf1(quote)` → offramp (token → BTC)
- `instanceOfGatewayQuoteOneOf2(quote)` → layerZero (EVM cross-chain)

### BitcoinSigner interface

Two mutually exclusive signing patterns — adapters implement one:
- `sendBitcoin(params)` — high-level: wallet broadcasts the tx (e.g. OKX)
- `signAllInputs(psbtHex)` — low-level: sign a PSBT and return hex (e.g. Reown)

If no `btcSigner` is provided to `executeQuote`, it returns the order for external/manual payment.

### Other exports

- `EsploraClient` — Bitcoin block explorer (`getFeeEstimates()`, `getBalance()`)
- `MempoolClient` — fee rate recommendations (`getRecommendedFees()`, `estimateTxTime()`)
- `getBalance`, `estimateTxFee` — standalone Bitcoin wallet utilities (`wallet/utxo.ts`)
- `isValidBtcAddress` — Bitcoin address validation
- `formatBtc`, `parseBtc` — satoshi ↔ BTC formatting

## Key patterns

- **USDT approval**: USDT on Ethereum requires resetting allowance to 0 before setting a new value (ERC-20 race condition). This is handled in `executeQuote` via a special ABI (`USDTApproveAbi`).
- **Best-effort registration**: After on-chain tx success, `registerTx` failures are caught and silently ignored — the order can be reconciled later.
- **Viem integration**: The SDK expects viem `PublicClient` and `WalletClient` for all EVM operations.

## Testing

Tests use **vitest** with **nock** for HTTP mocking. Each test instantiates `GatewaySDK` directly and mocks API responses. Always call `nock.cleanAll()` in `afterEach`.
