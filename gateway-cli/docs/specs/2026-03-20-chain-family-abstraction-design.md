# Chain Family Abstraction

## Context

The gateway-cli hardcodes chain-specific logic as `chain === "bitcoin" ? ... : ...` throughout swap.ts, balance.ts, and register.ts. This assumes everything non-BTC is EVM. Adding Solana or Tron as future gateway chains would require touching every file.

This refactor introduces a chain family abstraction so adding a new chain family means adding one file and updating a registry — not modifying every consumer.

**Scope:** Structure only — no Solana/Tron support. Move existing BTC and EVM logic behind the abstraction.

## Design

### Chain family registry

```
src/chains/
  index.ts       — registry + dispatch functions
  bitcoin.ts     — BTC-specific implementations
  evm.ts         — EVM-specific implementations
```

```typescript
// src/chains/index.ts
type ChainFamily = "bitcoin" | "evm"; // later: "solana" | "tron"

function getChainFamily(chain: string): ChainFamily {
  if (chain === "bitcoin") return "bitcoin";
  return "evm";
}
```

Adding Solana later: create `src/chains/solana.ts`, add `"solana"` to `ChainFamily`, add case to `getChainFamily`, done.

### Balance functions

```typescript
// src/chains/index.ts
interface TokenBalance {
  total: string;         // raw balance, no deductions
  allSpendable: string;  // total minus fees/reserves (used by --amount ALL)
}

getTokenBalance(chain, address, token, opts?) → TokenBalance
```

`opts` includes optional `feeToken` and `feeReserve` for paymaster scenarios.

**Per chain family:**

| Family | Token type | `total` | `allSpendable` |
|--------|-----------|---------|----------------|
| BTC | native | `esplora.confirmed + unconfirmed` | `sdk.getMaxSpendable()` |
| EVM | native (ETH) | `getBalance()` | `getBalance() - (feesPerGas × NATIVE_GAS_BUFFER)` |
| EVM | ERC20 | `balanceOf()` | `balanceOf()` (or `balanceOf() - feeReserve` if token is the fee token) |

`NATIVE_GAS_BUFFER = 900_000n` lives in `chains/evm.ts`.

**Consumers:**
- `balance.ts` command → calls `getTokenBalance` per chain, displays both `total` and `allSpendable`
- `--amount ALL` in swap.ts → calls `getTokenBalance`, uses `allSpendable`
- `resolveAllAmount` in swap.ts is deleted — replaced by `getTokenBalance().allSpendable`

### Signer functions

```typescript
// src/chains/index.ts
deriveAddress(chain, key) → string
resolveSigner(chain, key) → chain-specific signer object
```

**Per chain family:**

| Family | `deriveAddress` | `resolveSigner` returns |
|--------|----------------|----------------------|
| BTC | `ScureBitcoinSigner.fromKey(key).getP2WPKHAddress()` | `{ address, signer: BitcoinSigner }` |
| EVM | `privateKeyToAccount(key).address` | `{ address, walletClient, publicClient }` |

Returns chain-specific types — swap.ts still dispatches the signing step per chain family. The signing logic is too coupled to the Gateway SDK's response shapes to fully abstract behind a unified interface.

### Registration payload

```typescript
// src/chains/index.ts
buildRegisterPayload(srcChain, dstChain, orderId, txId) → RegisterTx
```

Dispatches on both source and destination chain families:

```
srcFamily === "bitcoin"       → { onramp: { orderId, bitcoinTxHex: txId } }
dstFamily === "bitcoin":
  srcFamily === "evm"         → { offramp: { orderId, evmTxhash: txId } }
  else                        → throw "unsupported offramp source chain"
else:
  srcFamily === "evm"         → { layerZero: { orderId, evmTxhash: txId } }
  else                        → throw "unsupported cross-chain source"
```

Adding Solana offramp later: add `srcFamily === "solana"` branch with the appropriate SDK field name.

Used by both `register.ts` and `swap.ts` — eliminates current duplication.

### CLI flags

New optional flags on `balance`, `quote`, and `swap` commands:

```
--fee-token <address>     ERC20 token used to pay gas (paymaster)
--fee-reserve <amount>    Amount of fee token to reserve for gas (default: 0)
```

When `--fee-token` matches the source/displayed token, `allSpendable = total - feeReserve`. Otherwise no effect.

### Balance output

Updated to show both `total` and `allSpendable` for every token:

```
bitcoin  (bc1q...)
  Balance:       0.05000000 BTC
  All spendable: 0.04950000 BTC

bob  (0x123...)
  ETH: 0.5 (all: 0.4991)
  USDC: 1000.00 (all: 995.00)
  WBTC: 0.01 (all: 0.01)
```

`BalanceJson` type updated — tokens become `{ symbol, address, balance, allSpendable }`.

### What moves where

| Current location | Moves to |
|-----------------|----------|
| `balance.ts: getBtcBalance()` | `chains/bitcoin.ts` |
| `balance.ts: getEvmChainBalance()` single-token logic | `chains/evm.ts` |
| `swap.ts: resolveBtcSigner()` | `chains/bitcoin.ts` |
| `swap.ts: resolveEvmSigner()` | `chains/evm.ts` |
| `swap.ts: resolveAllAmount()` | deleted — uses `getTokenBalance().allSpendable` |
| `swap.ts: isNativeToken()` | `chains/evm.ts` |
| `swap.ts: NATIVE_GAS_BUFFER` | `chains/evm.ts` |
| `register.ts: if/else chain routing` | `chains/index.ts: buildRegisterPayload()` |
| `swap.ts: registration if/else` | calls `buildRegisterPayload()` |

### What stays

| File | Keeps |
|------|-------|
| `balance.ts` | Multi-chain display orchestration, calls `getTokenBalance` per chain |
| `swap.ts` | Swap orchestration (quote → sign → register → poll), retry logic |
| `register.ts` | Thin wrapper: fetch order + `buildRegisterPayload` + `registerTx` |
| `quote.ts` | Quote fetching, BTC fee rate lookup |

### Files affected

- Create: `gateway-cli/src/chains/index.ts`
- Create: `gateway-cli/src/chains/bitcoin.ts`
- Create: `gateway-cli/src/chains/evm.ts`
- Modify: `gateway-cli/src/commands/balance.ts` — use `getTokenBalance`, show `allSpendable`
- Modify: `gateway-cli/src/commands/swap.ts` — use chain functions, remove inline signer/balance logic
- Modify: `gateway-cli/src/commands/register.ts` — use `buildRegisterPayload`
- Modify: `gateway-cli/src/cli.ts` — add `--fee-token`, `--fee-reserve` flags
- Modify: `gateway-cli/src/schemas.ts` — add `feeToken`, `feeReserve` fields
- Modify: `gateway-cli/src/output.ts` — update `BalanceJson` type and formatter
- Create: `gateway-cli/tests/chains/bitcoin.test.ts`
- Create: `gateway-cli/tests/chains/evm.test.ts`
- Update: existing tests for changed APIs

### Out of scope

- Actual Solana/Tron implementation
- Paymaster integration (just the `--fee-token`/`--fee-reserve` plumbing)
- Unified sign+submit abstraction (chain families return different signer types)
