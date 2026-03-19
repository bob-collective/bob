# Unified `--amount` Flag

## Context

The gateway-cli currently has three amount flags: `--amount` (human-readable), `--amount-atomic` (smallest units), and `--amount-usd` (USD via price oracle). This is not ergonomic — too many flags, custom mutual-exclusion validation, and no way to send a max balance. There's also a risk of catastrophic unit confusion (sending 100 BTC when you meant 100 satoshis).

## Design

### Single `--amount` flag

Replace all three flags with one: `--amount <value>`.

**Parsing rules (case-insensitive):**

| Pattern | Example | Interpretation |
|---------|---------|---------------|
| `ALL` | `--amount ALL` | Max spendable balance |
| Ends with `USD` | `--amount 100USD` | USD → price oracle → atomic units |
| Ends with source token symbol | `--amount 0.05BTC` | Human-readable → converted to atomic |
| Bare integer (no decimal, no suffix) | `--amount 5000000` | Atomic units (satoshis, wei, etc.) |

**Safe by default:** A bare number is always atomic units — the smallest denomination. You cannot accidentally send 100 BTC by typing `100`; that sends 100 satoshis.

**Case-insensitive:** `0.05BTC`, `0.05btc`, `0.05Btc` all work.

**Token suffix must match source asset:** `--amount 100USDC` with `--src BTC` is an error.

### `ALL` implementation

Resolves to max spendable balance based on source asset type:

| Source | How `ALL` resolves |
|--------|-------------------|
| BTC | `sdk.getMaxSpendable(senderAddress)` — accounts for mining fees |
| EVM token (USDC, WBTC, etc.) | `balanceOf(senderAddress)` — full token balance; gas paid from native token |
| Native EVM (ETH) | `getBalance(senderAddress) - (estimateFeesPerGas() × 900_000)` — reserves gas with 900k gas buffer based on observed gateway tx costs |

The 900k gas buffer is derived from real transaction data: BOB gateway txs use ~200k gas, Ethereum LayerZero ~370k, and complex Ethereum gateway swaps up to ~877k.

**Sender address required:** `ALL` needs a sender address to look up balances. The sender is derived from:
1. `--private-key` flag → derive address
2. `BITCOIN_PRIVATE_KEY` / `EVM_PRIVATE_KEY` env vars → derive address
3. `--sender` flag (for `--unsigned` mode)

If no sender can be determined, error:
```
--amount ALL requires a sender address. Use --private-key, --sender, or set BITCOIN_PRIVATE_KEY / EVM_PRIVATE_KEY.
```

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

### Error messages

Invalid input shows a help message:
```
Invalid amount "abc". Expected one of:
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
- `gateway-cli/src/util/input-resolver.ts` — new `parseAmount()`, remove `resolveAmount`/`parseAmountUsd`/`AmountInput`
- `gateway-cli/src/commands/swap.ts` — resolve `ALL` to balance, update amount handling
- `gateway-cli/src/commands/quote.ts` — resolve `ALL` to balance, update amount handling
- `gateway-cli/src/cli.ts` — replace three `--amount*` options with one
- `gateway-cli/src/commands/balance.ts` — possibly extract shared balance-lookup logic
- Tests updated accordingly
