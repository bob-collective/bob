# Unified `--amount` Flag

## Context

The gateway-cli currently has three amount flags: `--amount` (human-readable), `--amount-atomic` (smallest units), and `--amount-usd` (USD via price oracle). This is not ergonomic — too many flags, custom mutual-exclusion validation, and no way to send a max balance. There's also a risk of catastrophic unit confusion (sending 100 BTC when you meant 100 satoshis).

## Design

### Single `--amount` flag

Replace all three flags with one: `--amount <value>`.

**Parsing rules (case-insensitive, whitespace-trimmed):**

| Pattern | Example | Interpretation |
|---------|---------|---------------|
| `ALL` | `--amount ALL` | Max spendable balance |
| Ends with `USD` | `--amount 100USD` | USD → price oracle → atomic units |
| Ends with source token symbol | `--amount 0.05BTC` | Human-readable → converted to atomic |
| Bare integer (no decimal, no suffix) | `--amount 5000000` | Atomic units (satoshis, wei, etc.) |
| Anything else | `--amount 0.5`, `--amount abc` | Error with help message |

**Safe by default:** A bare number is always atomic units — the smallest denomination. You cannot accidentally send 100 BTC by typing `100`; that sends 100 satoshis.

**Case-insensitive:** `0.05BTC`, `0.05btc`, `0.05Btc` all work.

**Token suffix matching:** The parser checks if `raw.toUpperCase().endsWith(srcSymbol.toUpperCase())`. If so, the prefix is the numeric part. This avoids ambiguity — the parser already knows the source token from `--src`.

**Token suffix must match source asset:** `--amount 100USDC` with `--src BTC` is an error.

**Bare decimal without suffix is an error:** `--amount 0.5` is rejected. This prevents the old `--amount 0.1` behavior (which meant 0.1 human-readable) from silently doing the wrong thing. Users must write `--amount 0.5BTC`.

**Validation:**
- Amounts must be positive (reject zero, negative)
- Internal spaces not allowed (`100 USD` is invalid; must be `100USD`)
- Whitespace is trimmed from start/end

### Human-to-atomic conversion

The current `toAtomicUnits` function uses floating-point math which loses precision for tokens with 18 decimals. Replace with string-based decimal shifting:

1. Split the human amount string on `.`
2. Pad or truncate the fractional part to exactly `decimals` digits
3. Concatenate integer + padded fraction
4. Parse as BigInt

This avoids `0.3 * 1e18` floating-point errors entirely.

### `ALL` implementation

Resolves to max spendable balance based on source asset type:

| Source | How `ALL` resolves |
|--------|-------------------|
| BTC | `sdk.getMaxSpendable(senderAddress)` — accounts for mining fees |
| EVM token (USDC, WBTC, etc.) | `balanceOf(senderAddress)` — full token balance; gas paid from native token |
| Native EVM (ETH) | `getBalance(senderAddress) - (estimateFeesPerGas() × NATIVE_GAS_BUFFER)` — reserves gas |

**Gas buffer constant:** `NATIVE_GAS_BUFFER = 900_000` — derived from real gateway transaction data: BOB ~200k gas, Ethereum LayerZero ~370k, complex Ethereum gateway swaps up to ~877k. This is a named constant, not a magic number. Known limitation: may need adjustment if gateway contracts change or new chains are added.

**Sender address for `ALL`:** Both `quote` and `swap` need a sender to resolve `ALL`. The sender is derived from:
1. `--private-key` flag → derive address
2. `BITCOIN_PRIVATE_KEY` / `EVM_PRIVATE_KEY` env vars → derive address
3. `--sender` flag

For `quote` (which has no `--private-key` flag), the sender comes from env vars or `--sender`. If no sender can be determined:
```
--amount ALL requires a sender address. Use --private-key, --sender, or set BITCOIN_PRIVATE_KEY / EVM_PRIVATE_KEY.
```

**Display string for resolved `ALL`:** After resolution, the display is formatted as `"<amount> <symbol> (max spendable)"`, e.g. `"0.0342 BTC (max spendable)"`.

### CLI changes

**Old flags (removed):**
```
--amount <value>         Amount in human-readable units
--amount-atomic <value>  Amount in smallest units
--amount-usd <value>     Amount in USD
```

**New flag:**
```
--amount <value>         Amount: 0.05BTC, 100USDC, 100USD, 5000000 (atomic), ALL
```

Applied to both `quote` and `swap` commands.

**Backward compatibility:** CLI is v0.2.0, pre-1.0. Breaking changes are acceptable. No deprecated aliases — old flags are removed cleanly.

### Error messages

Invalid input shows a help message:
```
Invalid amount "0.5". Expected one of:
  0.05BTC       human-readable (converted to atomic)
  100USD        USD value (converted via price oracle)
  5000000       atomic units (satoshis, wei, etc.)
  ALL           max spendable balance
```

### Schema changes

Replace the `amountGroup` (three optional fields with mutual-exclusion `.check()`) with a single required `amount: z.string()` field in both `quoteSchema` and `swapSchema`.

### Parser implementation

A single `parseAmount(raw, srcSymbol, srcDecimals)` function in `input-resolver.ts` replaces the current `resolveAmount` + `parseAmountUsd` three-way dispatch.

Returns:
```typescript
type ParsedAmount =
  | { type: "atomic"; atomicUnits: string; display: string }
  | { type: "all" }
```

The `ALL` case is a sentinel — the caller (`handleSwap`, `handleQuote`) resolves it to an actual amount using balance lookups.

### Files affected

- `gateway-cli/src/schemas.ts` — replace amountGroup with single field
- `gateway-cli/src/util/input-resolver.ts` — new `parseAmount()`, fix `toAtomicUnits` precision, remove `resolveAmount`/`parseAmountUsd`/`AmountInput`
- `gateway-cli/src/commands/swap.ts` — resolve `ALL` to balance, update amount handling
- `gateway-cli/src/commands/quote.ts` — resolve `ALL` to balance, update amount handling
- `gateway-cli/src/cli.ts` — replace three `--amount*` options with one
- `gateway-cli/src/commands/balance.ts` — extract single-token balance lookup for reuse by `ALL` resolver
- Tests updated accordingly

### Out of scope

- Making the 900k gas buffer configurable (can add later if needed)
- `ALL` for tokens not in the route list
- Confirmation prompt before sending max balance (the CLI is for scripting/automation)
