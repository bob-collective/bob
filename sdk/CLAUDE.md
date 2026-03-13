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
├── ordinals/                  # Ordinals support
└── wallet/                    # Wallet utilities
```

### Generated vs hand-written code

Everything under `src/gateway/generated-client/` is auto-generated from the OpenAPI spec of the [bob-gateway](https://github.com/bob-collective/bob-gateway) backend. The entire directory is deleted and recreated on each codegen run. Never edit files there — put hand-written code in `client.ts`, `types/`, `utils/`, or `adapters/`.

When debugging API issues or verifying request/response shapes, refer to the gateway repo for the source of truth.

ESLint ignores `src/gateway/generated-client/**`.

## Public API

### GatewaySDK (primary class)

Instantiated with an optional base URL (defaults to mainnet). Core flow:

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

- `EsploraClient` — Bitcoin block explorer (`getFeeEstimates()`)
- `MempoolClient` — fee rate recommendations (`getRecommendedFees()`)
- `getBalance`, `estimateTxFee`, `isValidBtcAddress` — Bitcoin utilities
- `formatBtc`, `parseBtc` — satoshi ↔ BTC formatting

## Key patterns

- **USDT approval**: USDT on Ethereum requires resetting allowance to 0 before setting a new value (ERC-20 race condition). This is handled in `executeQuote` via a special ABI (`USDTApproveAbi`).
- **Best-effort registration**: After on-chain tx success, `registerTx` failures are caught and silently ignored — the order can be reconciled later.
- **Viem integration**: The SDK expects viem `PublicClient` and `WalletClient` for all EVM operations.

## Testing

Tests use **vitest** with **nock** for HTTP mocking. Each test instantiates `GatewaySDK` directly and mocks API responses. Always call `nock.cleanAll()` in `afterEach`.
