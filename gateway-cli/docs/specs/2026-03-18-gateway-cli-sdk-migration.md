# Gateway CLI: Move to Bob Monorepo & Rewrite on SDK

## Context

`@gobob/gateway-cli` is a CLI for bridging Bitcoin to/from EVM chains via BOB Gateway. It currently lives in a standalone repo (`bob-gateway-cli`) with its own REST API client, signer modules, and polling logic — reimplementing much of what `@gobob/bob-sdk` already provides.

The `gateway-bot` (automated test runner) also uses `@gobob/bob-sdk` directly for swap execution, creating a second parallel implementation of the same core logic.

## Goal

1. Move the CLI into the bob monorepo as `bob/gateway-cli/`
2. Rewrite CLI internals to use `@gobob/bob-sdk` as the core
3. Add a `balance` command and transient error retry logic
4. Add npm publishing workflow
5. Enable the gateway-bot to consume the CLI via `--json` mode (Phase 2, separate spec)

Target dependency chain: **Bot → CLI → SDK**

## Package Layout

```
bob/
├── sdk/                      @gobob/bob-sdk (unchanged)
├── gateway-cli/              @gobob/gateway-cli (moved from standalone repo)
│   ├── src/
│   │   ├── cli.ts            Commander.js entry point
│   │   ├── commands/         Command handlers
│   │   │   ├── swap.ts       Execute bridge swap (calls sdk.executeQuote)
│   │   │   ├── quote.ts      Get quote (calls sdk.getQuote)
│   │   │   ├── balance.ts    NEW — token balances across chains
│   │   │   ├── chains.ts     List supported chains
│   │   │   ├── tokens.ts     List tokens on a chain
│   │   │   ├── routes.ts     List bridge routes
│   │   │   ├── status.ts     Check order status
│   │   │   ├── orders.ts     List orders for address
│   │   │   ├── max-spendable.ts  Max spendable BTC
│   │   │   ├── register.ts   Recovery: register signed tx
│   │   │   └── offramp.ts    Hidden alias for swap (backward compat)
│   │   ├── signer/           Thin adapters producing SDK-compatible signers
│   │   │   ├── btc.ts        Layered resolution → BitcoinSigner
│   │   │   └── evm.ts        Layered resolution → viem WalletClient
│   │   ├── output/           JSON shapes + human formatting
│   │   ├── config/           Env vars + TOML config loading
│   │   └── util/             Amount parsing, chain aliases, progress, retry
│   ├── bin/gateway-cli.ts
│   ├── tests/
│   ├── package.json
│   └── tsconfig.json
├── tokenlist/
└── ...
```

## SDK Integration Notes

### `@scure/btc-signer` version alignment

The CLI currently uses `@scure/btc-signer@^2.x` while the SDK uses `@^1.x`. The CLI will use the SDK's `ScureBitcoinSigner` for PSBT signing, which uses v1 internally. The CLI's own v2-specific code (WIF decoding in the signer resolution layer) needs to be adapted to use `@scure/base` hex decoding (v1-compatible) or the SDK's signer constructor (`new ScureBitcoinSigner(privateKeyHex)`). The CLI drops its direct `@scure/btc-signer` dependency entirely.

### Execution path: `executeQuote()` vs manual steps

The swap command delegates to `sdk.executeQuote()` for the standard flow — this handles order creation, PSBT signing (hex format, handled internally by SDK), EVM tx submission, and tx registration in one call. The CLI does not need to handle PSBT encoding formats directly.

For the `--unsigned` flow, the CLI calls `sdk.createOrder()` separately and outputs the raw PSBT (base64) or EVM tx info for external signing, then the user calls `gateway-cli register` to complete. This path does not go through `executeQuote()`.

### SDK `RouteInfo` lacks token metadata

The SDK's generated `RouteInfo` type has flat `srcToken`/`dstToken` as bare address strings — no `symbol` or `decimals`. The CLI needs this metadata for the `tokens` command, `balance` command, and amount parsing.

Solution: The CLI maintains a thin `enrichRoute()` adapter that takes SDK `RouteInfo` and resolves token metadata from the `@gobob/tokenlist` package (already in the monorepo at `bob/tokenlist/`). This is a lookup by chain + address → symbol + decimals. The tokenlist is a local dependency (`"@gobob/tokenlist": "file:../tokenlist"`).

Note: The tokenlist uses `chainId: number` while SDK `RouteInfo` uses chain name strings. The CLI maintains a chain name → chainId mapping (e.g., `"bob" → 60808`, `"ethereum" → 1`) for the tokenlist lookup. Also, the tokenlist package exports raw `.ts` files (`"main": "index.ts"`), so the CLI imports it directly and the build step compiles it together.

### SDK quote type adaptation

The SDK returns union types (`GatewayQuote = GatewayQuoteOneOf | GatewayQuoteOneOf1 | GatewayQuoteOneOf2`) wrapping `onramp`, `offramp`, and `layerZero` variants. The CLI's existing JSON output uses flat shapes (`QuoteJson`, `SwapSuccessJson`).

The CLI command handlers include a `flattenQuote()` adapter that detects the variant and maps it to the existing flat output shape. The public JSON contract does not change — this is an internal mapping layer.

**Variant detection:** Check which key is present (`onramp`, `offramp`, or `layerZero`) on the union type.

**Field mapping by variant:**

| CLI output field | Onramp | Offramp | LayerZero |
|---|---|---|---|
| `inputAmount` | `.onramp.inputAmount.amount` | `.offramp.inputAmount.amount` | `.layerZero.inputAmount.amount` |
| `outputAmount` | `.onramp.outputAmount.amount` | `.offramp.outputAmount.amount` | `.layerZero.outputAmount.amount` |
| `fees` | `.onramp.fees.amount` | `.offramp.fees.amount` | `.layerZero.fees.amount` |
| `slippage` | `.onramp.slippage` (string) | `.offramp.slippage` (number, convert to string) | N/A (omit from output) |
| `estimatedTime` | `.onramp.estimatedTimeInSecs` | `.offramp.estimatedTimeInSecs` | `.layerZero.estimatedTimeInSecs` |

### SDK `getQuote()` parameter naming

The SDK's `GatewayApiClient.getQuote()` accepts `GetQuoteParams` with `fromChain`/`toChain`/`fromToken`/`toToken`. Internally the SDK maps these to `srcChain`/`dstChain` for the REST call. The CLI's `asset-chain-parser.ts` must produce `fromChain`/`toChain` names (not `srcChain`/`dstChain`) when calling the SDK.

## What Changes in the CLI

### Deleted (replaced by SDK)

| Current CLI code | Replaced by |
|---|---|
| `src/api/client.ts` — custom REST client | `GatewayApiClient` from SDK |
| `src/api/types.ts` — hand-written types | SDK's generated OpenAPI types + `enrichRoute()` adapter |
| `src/signer/btc.ts` — PSBT signing logic | SDK's `ScureBitcoinSigner` |
| `src/signer/evm.ts` — tx build/broadcast | SDK's viem `WalletClient` integration |
| `src/util/mempool.ts` — fee estimation portion | SDK's Esplora fee estimation |
| `src/polling/poll-order.ts` — order polling | Polling utility using `sdk.getOrder()` (see below) |
| Direct `@scure/btc-signer` dependency | Used transitively via SDK |
| `src/util/mempool.ts` — fee estimation portion | SDK's Esplora fee estimation |

Note on polling: The existing `poll-order.ts` (64 lines) has structured timeout handling and terminal status detection. This logic is preserved but rewritten to call `sdk.getOrder()` instead of the custom API client. It remains a utility in `src/util/poll-order.ts`, not inlined.

Note on mempool: `mempool.ts` serves two purposes: fee estimation (replaced by SDK) and `findPendingMempoolTx()` for poll-timeout fallback (kept — used when BTC delivery is unconfirmed at poll timeout). The fallback portion is retained in the swap command.

Note on nonce management: `waitForNonceClear()` from the current EVM signer is kept in the swap command flow. The SDK's `executeQuote()` does not handle nonce management, and back-to-back swaps need this.

### Kept (CLI's unique value)

- `src/cli.ts` — Commander.js command definitions (including hidden `offramp` alias)
- `src/commands/*` — command handlers, rewritten to call SDK methods
- `src/signer/btc.ts` (thin) — layered resolution only: `--private-key` → `BITCOIN_PRIVATE_KEY` env → `BITCOIN_SIGNER` external → `--unsigned`. Produces a `BitcoinSigner` for the SDK.
- `src/signer/evm.ts` (thin) — layered resolution only: `--private-key` → `EVM_PRIVATE_KEY` env → `--keystore` → `EVM_SIGNER` external → `--unsigned`. Produces a viem `WalletClient`. Note: `ethers` remains as a dependency for keystore decryption (`Wallet.fromEncryptedJson`); viem does not have a keystore equivalent.
- `src/config/` — env vars + TOML config loading
- `src/output/` — JSON shapes + human-readable formatting
- `src/util/amount-parser.ts` — flexible input parsing (`$50`, `0.1BTC`, `100sat`, `raw:`)
- `src/util/asset-chain-parser.ts` — `ASSET:CHAIN` syntax
- `src/util/price-oracle.ts` — Binance/Coinbase price fetching for `$` amounts
- `src/util/progress.ts` — stderr progress output (includes `--verbose` mode)
- `src/util/retry.ts` — retry with exponential backoff
- Confirmation prompt logic (currently inline in `swap.ts`, not a separate file)

### Added

#### 1. `balance` command

**Usage:**
```bash
gateway-cli balance <address> [--chain <chain>] [--json]
```

**Behavior:**
- No `--chain`: returns balances across all gateway-supported chains
- `--chain bitcoin`: BTC confirmed/unconfirmed + max spendable
- `--chain bob` (or ethereum, base, etc.): native gas token + gateway-supported tokens

Tokens are determined from `sdk.getRoutes()` (enriched via tokenlist for metadata). Only non-zero balances are returned — zero-balance tokens and empty chains are omitted.

**BTC balance fetching:** Uses the SDK's Esplora client (`getAddressUtxos()`) to compute confirmed/unconfirmed balances, and `sdk.getMaxSpendable()` for max spendable. The SDK's UTXO response includes confirmation status, so confirmed = sum of confirmed UTXOs, unconfirmed = sum of unconfirmed UTXOs.

**EVM balance fetching:** For each chain, the CLI creates a viem `publicClient` using the configured RPC URL. It uses `publicClient.getBalance()` for the native gas token and `publicClient.multicall()` with `erc20Abi.balanceOf` for gateway tokens (batched in a single RPC call per chain).

**RPC URL resolution:** The CLI supports per-chain RPC URLs via env vars and config.toml. Resolution order (first match wins):

1. Per-chain env var: `EVM_RPC_URL_BOB`, `EVM_RPC_URL_ETHEREUM`, `EVM_RPC_URL_BASE`, etc. (chain name uppercased)
2. Per-chain config.toml:
   ```toml
   [rpc]
   bob = "https://rpc.gobob.xyz"
   ethereum = "https://eth.llamarpc.com"
   base = "https://mainnet.base.org"
   ```
3. Fallback: `EVM_RPC_URL` env var (single default for all chains)
4. Built-in public RPCs as last resort

**Route caching:**
- `getRoutes()` response cached to `~/.gateway-cli/cache/routes.json` with ISO 8601 timestamp
- Cache TTL configurable in `~/.gateway-cli/config.toml` as a duration string (`"1h"`, `"12h"`, `"24h"`, `"7d"`):
  ```toml
  [cache]
  ttl = "24h"
  ```
- Default TTL: 24 hours
- `routes` command warms the cache as a side effect
- `--no-cache` flag to force re-fetch

**JSON output:**
```json
{
  "bitcoin": {
    "address": "bc1q...",
    "confirmed": "0.01000000",
    "maxSpendable": "0.00950000"
  },
  "bob": {
    "address": "0x...",
    "native": { "symbol": "ETH", "balance": "0.05000000" },
    "tokens": [
      { "symbol": "USDC", "address": "0x...", "balance": "150.000000" }
    ]
  }
}
```

Note: `native` field is included for EVM chains when balance is non-zero. BTC `unconfirmed` field is omitted when zero.

**Human-readable output:**
```
Bitcoin (bc1q...)
  Confirmed:     0.01000000 BTC
  Max spendable: 0.00950000 BTC

BOB (0x...)
  ETH:  0.05000000
  USDC: 150.000000
```

#### 2. Transient error retry

The `swap` command retries automatically on transient errors (TRM Labs screening delays, BTC propagation waits, rate limits).

Note: This changes existing retry behavior. The current CLI uses flat 10s backoff with 3 retries for registration only. The new behavior applies broader transient error retry with exponential backoff.

**Default behavior (retry enabled):**
- 5 attempts, exponential backoff (5s, 10s, 20s, 40s, 80s)
- `--json` mode emits retry events to stderr:
  ```json
  {"event":"retry","reason":"TRM screening delay","attempt":2,"maxAttempts":5,"nextRetryIn":10}
  ```
- Human mode prints: `Retrying (2/5): TRM screening delay, waiting 10s...`
- Final result (success or failure) goes to stdout as normal

**`--no-retry` flag:**
- Exits immediately on transient error
- Exit code 6 (new)
- JSON output includes retry hint:
  ```json
  {"error":"TRM screening delay","retryable":true,"code":6}
  ```

#### 3. Exit codes

| Code | Meaning |
|---|---|
| 0 | Success |
| 1 | General error |
| 2 | Poll timeout |
| 3 | Registration failed (recovery via `register` command) |
| 4 | Price oracle error |
| 5 | Insufficient funds |
| 6 | Retryable error (with `--no-retry`) |

## Publishing

### package.json

```json
{
  "name": "@gobob/gateway-cli",
  "version": "0.2.0",
  "main": "dist/src/cli.js",
  "bin": { "gateway-cli": "./bin/gateway-cli.ts" },
  "files": ["dist", "bin"],
  "dependencies": {
    "@gobob/bob-sdk": "file:../sdk",
    "@gobob/tokenlist": "file:../tokenlist"
  }
}
```

### npm workflow (`cli-npm.yml`)

Mirrors `sdk-npm.yml` with CLI-specific tag filtering:
- Triggered on tag push matching `cli-v*` (e.g., `cli-v0.2.0`, `cli-v0.3.0-rc0`)
- **Prerequisite:** Update `sdk-npm.yml` to filter on `sdk-v*` tags (currently triggers on `*`, which would cause a spurious SDK publish on CLI tags)
- Before publish: replaces `file:../sdk` with `"@gobob/bob-sdk": "^5.0.0"` and `file:../tokenlist` with `"@gobob/tokenlist": "^1.0.0"` in package.json
- Builds TypeScript
- Publishes with `--access public`
- Tags containing "rc" publish to `rc` dist-tag
- Uses OIDC for npm authentication

## External Interface

No breaking changes. All existing commands (including hidden `offramp` alias), `--json` output shapes, `--verbose` flag, and exit codes 0-5 remain identical. New additions:
- `balance` command
- `--no-retry` flag on `swap`
- Exit code 6
- Behavioral change: transient error retry is now exponential backoff (5 attempts) instead of flat 10s (3 attempts) for registration
