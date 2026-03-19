# Gateway CLI Simplification

## Context

The gateway-cli is a developer/ops tool wrapping the BOB Gateway SDK. A prior implementation over-abstracted the codebase with adapter layers, separate signer modules, and unnecessary indirection. A refactoring pass has already removed most of that (~18 files deleted). This spec covers the remaining simplifications to bring the CLI to its minimal, maintainable form.

**Target audience:** Power users / developers scripting against BOB Gateway, and the BOB team for internal ops/testing.

**Current state:** ~1,600 lines across 17 source files.
**Target state:** ~1,380 lines across 14 source files.

## Changes

### 1. Drop `--dry-run` from swap

The `--dry-run` flag on `swap` is redundant with the `quote` command — both fetch a quote and display amounts/fees without creating an order. Remove the flag, the `SwapResult.dryRun` variant, and the early-return path in `swap.ts`.

Users who want to preview a swap use `gateway-cli quote` instead.

**Cleanup:** After removing the dry-run early return, the `initialQuote` / `initialOutputAmount` variables used only by that path become dead code. Remove them and defer the quote fetch to the order-creation step.

### 2. Remove TOML config

The `~/.gateway-cli/config.toml` file adds discovery burden for marginal benefit. Replace with env vars + CLI flags + hardcoded defaults.

**Config precedence:** CLI flags > env vars > hardcoded defaults.

**Env vars:**

| Variable | Purpose | Default |
|----------|---------|---------|
| `GATEWAY_API_URL` | SDK base URL | production |
| `BITCOIN_PRIVATE_KEY` | BTC signer key | none |
| `EVM_PRIVATE_KEY` | EVM signer key | none |
| `BTC_FEE_RATE` | sat/vbyte override | none (uses mempool) |
| `EVM_RPC_URL_<CHAIN>` | Per-chain RPC override | viem chain default |

Note: `GATEWAY_API_URL` and `BTC_FEE_RATE` are new env vars that must be wired into `loadConfig()`. Currently these values only came from TOML.

**Hardcoded defaults:**

| Setting | Value |
|---------|-------|
| `slippageBps` | 300 (3%) — matches current TOML default |
| `timeout` | 1,800,000ms (30 min) — matches current TOML default; Bitcoin confirmations require long waits |

**Behavioral notes:**
- `recipient` and `sender` become purely CLI-flag-driven after TOML removal (no env var fallback). `--recipient` is effectively mandatory on every `quote` and `swap` invocation.
- `config.noWait` (previously TOML-sourced) is always `false` after removal. Simplify `swap.ts` to check only `opts.wait`, not `config.noWait`.
- The `offramp` hidden alias (delegating to `runSwap`) is retained unchanged.

Remove the `toml` dependency.

### 3. Remove route caching

The `~/.gateway-cli/cache/routes.json` file with TTL-based caching adds ~30 lines of filesystem and TTL logic to `route-provider.ts`. Route fetches from the SDK are fast enough to call fresh each time.

Remove: `readCache()`, `writeCache()`, TTL parsing, cache path constants, and the `fs`/`path`/`os` imports that only served the cache. The `~/.gateway-cli/` directory concept is eliminated entirely (no config file, no cache).

**Touchpoints:**
- Remove `--no-cache` flag from the `balance` command in `cli.ts`
- Remove `noCache` option plumbing in `balance.ts`
- Remove `cacheOptions` parameter from `getEnrichedRoutes()` in `route-provider.ts`

### 4. Replace custom retry with `p-retry`

The current `retry.ts` (75 lines) implements custom exponential backoff and transient error detection. Replace with `p-retry` which provides exponential backoff with jitter out of the box.

Since `swap.ts` is the only consumer, inline the retry logic directly there (~20 lines). Delete `retry.ts`.

**Two distinct retry contexts exist and must be preserved:**

1. **Transient retry (whole swap flow):** Wraps the entire `executeCore` function. Uses the 9 transient error regex patterns as `shouldRetry` predicate. `--no-retry` flag disables this.

2. **Registration retry (just `registerTx` calls):** Retries unconditionally on any error since a failed registration after a successful on-chain tx means the user needs manual recovery. This should remain unconditional (not gated by transient patterns).

Both become `pRetry()` calls with different options.

Add `p-retry` as a dependency.

### 5. File consolidation

**Merge into `config.ts`:**
- `constants.ts` (1 line: `BTC_DECIMALS`) — becomes an export from config
- Remaining config logic (env var loading, defaults, `getSdk()` singleton) — ~35 lines total

Note: `sdk-client.ts` already merged into `config.ts` on this branch. `constants.ts` is the only remaining file to merge.

**Simplify `rpc-resolver.ts`:**
- Remove TOML `Record<string, string>` parameter from `resolveRpcUrl()`
- Function becomes a one-liner: `process.env[`EVM_RPC_URL_${chain.toUpperCase()}`]`
- `getViemChain()` stays as-is
- Update call sites in `balance.ts` and `swap.ts` to drop the `config.rpc` argument
- ~10 lines total

**Delete files:**
- `src/constants.ts`
- `src/util/retry.ts`

### 6. Test rewrite

The existing tests are written against old abstractions (many already deleted on this branch). Rewrite rather than patch.

**Test files (one per module with real logic):**

| Test file | Covers |
|-----------|--------|
| `commands/swap.test.ts` | Swap orchestration, unsigned mode, transient + registration retry, polling |
| `commands/quote.test.ts` | Quote fetching, amount resolution |
| `commands/balance.test.ts` | Multi-chain balance aggregation |
| `commands/routes.test.ts` | Route/chain/token listing and filtering |
| `commands/status.test.ts` | Order status lookup |
| `commands/orders.test.ts` | Order listing |
| `commands/register.test.ts` | Transaction registration recovery |
| `util/input-resolver.test.ts` | Asset parsing, amount conversion, chain aliases |
| `util/route-provider.test.ts` | Route enrichment from tokenlist (no cache tests) |
| `util/price-oracle.test.ts` | Dual-source price fetching with fallback |
| `e2e/cli.test.ts` | End-to-end command invocation |

**Delete all other test files**, including: `config/index.test.ts`, `adapter/route-enricher.test.ts`, `adapter/sdk-client.test.ts`, `util/asset-chain-parser.test.ts`, `util/route-cache.test.ts`, `commands/chains.test.ts`, `commands/tokens.test.ts`, `commands/max-spendable.test.ts`, `commands/offramp.test.ts`.

## Final file structure

```
src/
  cli.ts                  ~170 lines  Commander setup
  config.ts                ~35 lines  Env vars, defaults, getSdk(), BTC_DECIMALS
  output.ts               ~196 lines  Formatters + logger
  schemas.ts               ~60 lines  Zod validation schemas
  commands/
    swap.ts               ~290 lines  Orchestration + inline p-retry
    quote.ts               ~80 lines  Quote fetching
    balance.ts            ~115 lines  Multi-chain balance (no cache option)
    routes.ts              ~65 lines  Route/chain/token listing
    register.ts            ~14 lines  Tx registration recovery
    status.ts               ~6 lines  Order status
    orders.ts               ~6 lines  Order listing
  util/
    input-resolver.ts     ~146 lines  Asset/amount parsing
    route-provider.ts     ~110 lines  Route enrichment (no caching)
    price-oracle.ts        ~48 lines  BTC/ETH price from Binance/Coinbase
    rpc-resolver.ts        ~10 lines  Per-chain RPC URL resolution

tests/
  commands/               7 test files (one per command)
  util/                   3 test files (input-resolver, route-provider, price-oracle)
  e2e/                    1 test file (cli integration)
```

## Dependencies

**Removed:** `toml`
**Added:** `p-retry`
**Unchanged:** `@gobob/bob-sdk`, `@gobob/tokenlist`, `commander`, `viem`, `zod`

## Risks

1. **p-retry is ESM-only.** The project uses ESM (`.js` import extensions), so this should be fine. Verify `tsconfig.json` and build output are compatible.

2. **Registration retry semantics.** The registration retry must remain unconditional — a failed registration after a successful on-chain tx means potential fund loss without manual recovery via `gateway-cli register`.

## Out of scope

- Restructuring `swap.ts` orchestration (complexity is inherent to the domain)
- Pushing enrichment logic into the SDK
- Adding progress callbacks to SDK's `executeQuote()`
- Balance command per-chain error isolation
- Flattening the directory structure (commands/util split is reasonable)
