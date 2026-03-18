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
│   │   │   └── register.ts   Recovery: register signed tx
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

## What Changes in the CLI

### Deleted (replaced by SDK)

| Current CLI code | Replaced by |
|---|---|
| `src/api/client.ts` — custom REST client | `GatewayApiClient` from SDK |
| `src/api/types.ts` — hand-written types | SDK's generated OpenAPI types |
| `src/signer/btc.ts` — PSBT signing logic | SDK's `ScureBitcoinSigner` |
| `src/signer/evm.ts` — tx build/broadcast | SDK's viem `WalletClient` integration |
| `src/util/mempool.ts` — fee estimation | SDK's Esplora fee estimation |
| `src/polling/poll-order.ts` — order polling | `sdk.getOrder()` in a loop |

### Kept (CLI's unique value)

- `src/cli.ts` — Commander.js command definitions
- `src/commands/*` — command handlers, rewritten to call SDK methods
- `src/signer/btc.ts` (thin) — layered resolution only: `--private-key` → `BITCOIN_PRIVATE_KEY` env → `BITCOIN_SIGNER` external → `--unsigned`. Produces a `BitcoinSigner` for the SDK.
- `src/signer/evm.ts` (thin) — layered resolution only: `--private-key` → `EVM_PRIVATE_KEY` env → `--keystore` → `EVM_SIGNER` external → `--unsigned`. Produces a viem `WalletClient`.
- `src/config/` — env vars + TOML config loading
- `src/output/` — JSON shapes + human-readable formatting
- `src/util/amount-parser.ts` — flexible input parsing (`$50`, `0.1BTC`, `100sat`, `raw:`)
- `src/util/asset-chain-parser.ts` — `ASSET:CHAIN` syntax
- `src/util/progress.ts` — stderr progress output
- `src/util/retry.ts` — retry with exponential backoff
- `src/util/confirm.ts` — interactive prompts

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

Tokens are determined from `sdk.getRoutes()`. Only non-zero balances are returned — zero-balance tokens and empty chains are omitted.

**Route caching:**
- `getRoutes()` response cached to `~/.gateway-cli/cache/routes.json` with timestamp
- Cache TTL configurable in `~/.gateway-cli/config.toml`:
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
    "tokens": [
      { "symbol": "USDC", "address": "0x...", "balance": "150.000000" }
    ]
  }
}
```

**Human-readable output:**
```
Bitcoin (bc1q...)
  Confirmed:     0.01000000 BTC
  Unconfirmed:   0.00000000 BTC
  Max spendable: 0.00950000 BTC

BOB (0x...)
  USDC: 150.000000
```

#### 2. Transient error retry

The `swap` command retries automatically on transient errors (TRM Labs screening delays, BTC propagation waits, rate limits).

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
    "@gobob/bob-sdk": "file:../sdk"
  }
}
```

### npm workflow (`cli-npm.yml`)

Mirrors `sdk-npm.yml`:
- Triggered on tag push
- Before publish: replaces `file:../sdk` with `"@gobob/bob-sdk": "^5.x"` in package.json
- Builds TypeScript
- Publishes with `--access public`
- Tags containing "rc" publish to `rc` dist-tag
- Uses OIDC for npm authentication

## External Interface

No breaking changes. All existing commands, `--json` output shapes, and exit codes 0-5 remain identical. New additions:
- `balance` command
- `--no-retry` flag on `swap`
- Exit code 6
