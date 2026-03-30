# gateway-cli Code Review

> Reviewed by: Claude (senior JS/TS engineer lens)
> Scope: All source files under `src/`
> Methodology: Static analysis — every line treated as a potential flaw

---

## CRITICAL

### 1. `swap.ts:143` — Nonce check uses wrong address variable

**Code:**
```typescript
if (opts.sender) {
  const addr = opts.sender as `0x${string}`;
```

`opts.sender` is the raw CLI flag (`--sender`). When the user omits `--sender`, the actual signing address is `senderAddress` (derived on line 49 from the private key). The nonce check is **entirely skipped** in that case.

**Impact:** If the user relies on `EVM_PRIVATE_KEY` without `--sender`, a previously pending nonce is never waited on, risking a nonce collision and a failed or dropped transaction.

**Fix:** Replace `opts.sender` with `senderAddress` throughout the nonce-wait block.

---

### 2. `rpc-resolver.ts:55–59` — RPC probe never validates the returned chain ID

**Code:**
```typescript
const json = await res.json();
if (!json.result) return null;
return { url, latency: Date.now() - start };
```

`probeRpc` accepts any RPC that returns a truthy `eth_chainId` result. The actual chain ID in the response is never checked against the expected `chainId`.

**Impact:** A compromised or misconfigured entry in chainlist.org can pass the probe and be selected as the active RPC. Subsequent balance reads and transaction broadcasts go to the wrong chain.

**Fix:** Validate `parseInt(json.result, 16) === chainId` before returning success.

---

### 3. `chains/evm.ts:172–174` — Multicall failures silently report 0 balance

**Code:**
```typescript
const multicallResults = results[idx] as Array<{ result?: bigint }>;
tokenBals = erc20s.map((t, i) => {
  const bal = (multicallResults[i]?.result as bigint) ?? 0n;
```

viem's `multicall` returns `{ status: 'success', result } | { status: 'failure', error }` per call. The cast to `Array<{ result?: bigint }>` discards `status`. A failed `balanceOf` call (reverted, wrong ABI, RPC error) is indistinguishable from a zero balance.

**Impact:** User sees 0 for a token they actually hold, or proceeds with a swap using an incorrect spendable balance.

**Fix:** Check `multicallResults[i].status === 'success'` before reading `.result`; propagate or warn on failures.

---

## HIGH

### 4. `swap.ts:158–164` — `chain: null` disables chain ID validation

**Code:**
```typescript
txId = await walletClient.sendTransaction({
  account: walletClient.account!,
  chain: null,   // ← bypasses viem's chain check
  to: txInfo.to as `0x${string}`,
  ...
});
```

viem checks that the connected wallet's chain matches the `chain` parameter. Passing `null` removes this safeguard entirely.

**Impact:** If the Gateway returns transaction data for the wrong EVM chain (bug or attack), the transaction is still broadcast. Funds lost with no recourse.

**Fix:** Parse `txInfo.chainId` (if the SDK returns it) and compare to `getChainConfig(evmChain).id` before sending. At minimum, remove `chain: null` so viem's default protection applies.

---

### 5. `schemas.ts:15–22` — `positiveInt` silently truncates floats

**Code:**
```typescript
const positiveInt = z.string().transform((v, ctx) => {
  const n = parseInt(v, 10);
  if (isNaN(n) || n < 0) { ... }
  return n;
});
```

`parseInt("3.7", 10)` returns `3`. So `--slippage 3.7` silently becomes 3 bps, `--timeout 59.9` becomes 59 seconds, `--btc-fee-rate 12.5` becomes 12 sat/vbyte. No error is thrown.

**Impact:** User intent is silently altered. Swap may execute with lower slippage than expected and be rejected by the API.

**Fix:** Use `Number(v)` and add `!Number.isInteger(n)` to the rejection condition.

---

### 6. `cli.ts:147` / `evm.ts:100` — Scientific notation bypasses validation and crashes `BigInt`

**Validation path:**
```typescript
// cli.ts:147
const n = Number(opts.feeReserve);
if (!Number.isInteger(n) || isNaN(n)) throw new Error(...);

// schemas.ts feeReserve refine
const n = Number(v); return Number.isInteger(n) && n >= 0;
```

`Number("1e18") = 1e18`, and `Number.isInteger(1e18) === true` in JavaScript.

**Crash site:**
```typescript
// evm.ts:100
const reserved = BigInt(feeReserve);  // feeReserve is still the string "1e18"
// → SyntaxError: Cannot convert 1e18 to a BigInt
```

**Impact:** Any `--fee-reserve 1e18` input (common for wei-scale amounts) causes an unhandled crash with a confusing SyntaxError, not a validation error.

**Fix:** In the refine/validator, also reject strings that contain `e` or `E` (scientific notation), or parse via `BigInt` in the validator directly.

---

### 7. `swap.ts:44` — Duplicate chain family detection before input resolution

**Code:**
```typescript
const srcFamily = getChainFamily(
  opts.src.includes(":")
    ? opts.src.split(":")[1]
    : opts.src === "BTC" || opts.src === "btc"
      ? "bitcoin"
      : opts.src
);
```

This runs **before** `resolveSwapInputs`, which handles the full alias table and chain resolution. The two paths can disagree: this inline logic only special-cases `BTC`/`btc` and `:chain` notation, missing all `CHAIN_ALIASES` entries (e.g., `btc` is in the alias map but also caught here, while `eth`, `arb`, etc. are not).

**Impact:** Wrong chain family → wrong private key loaded → wrong `senderAddress` derived → PSBT built for wrong address, or EVM key used for Bitcoin signing.

**Fix:** Call `resolveSwapInputs` first (or `parseAssetChain` for the src asset only), then derive `srcFamily` from the resolved `srcAsset.chain`.

---

## MEDIUM

### 8. `swap.ts:220` — Mempool fallback runs for layerZero swaps

**Code:**
```typescript
if (getChainFamily(srcAsset.chain) !== "bitcoin" && err instanceof Error && err.message === "pending") {
  const pendingTxs = await new MempoolClient().getAddressMempoolTxs(recipient).catch(() => []);
```

The condition `srcFamily !== "bitcoin"` is true for **both** offramp (EVM→BTC) and layerZero (EVM→EVM). For layerZero, `recipient` is an EVM address. `MempoolClient` always fails with an EVM address; `.catch(() => [])` hides it.

**Impact:** Silent wasted network call on every layerZero poll timeout. The correct fallback is only meaningful for offramp.

**Fix:** Use `variant === "offramp"` instead of the chain family check.

---

### 9. `price-oracle.ts:44` — Binance result used without validating it is > 0

**Code:**
```typescript
if (binance.status === "fulfilled") return { priceUsd: binance.value, source: "binance" };
```

`parseFloat("0")` returns `0`. If Binance returns a zero or negative price (API glitch, maintenance), it wins over a correct Coinbase price.

**Impact:** `usdValue / 0 = Infinity`; `Infinity.toFixed(18)` throws `RangeError`. Or a negative price produces a negative amount that passes into `humanToAtomic` and causes a `BigInt` SyntaxError. Either way: uncaught crash.

**Fix:** Add `binance.value > 0` guard before returning. Same for Coinbase.

---

### 10. `rpc-resolver.ts:85` — Env var key constructed from user-controlled chain name

**Code:**
```typescript
const envUrl = process.env[`EVM_RPC_URL_${chainName.toUpperCase()}`];
```

`chainName` comes from the user-supplied `--src`/`--dst` chain specifier (e.g., `USDC:mychain` → `chainName = "mychain"`). A crafted chain name like `BASE_SECRET_TOKEN` would read `process.env.EVM_RPC_URL_BASE_SECRET_TOKEN`.

**Impact:** Low probability, but violates the principle of never constructing env-var keys from untrusted input. If any sensitive variable happens to match the pattern, its value is used as an RPC URL (and logged or sent in HTTP requests).

**Fix:** Validate `chainName` against a known-chain allowlist before constructing the env key.

---

### 11. `input-resolver.ts:155–156` — Float division before BigInt conversion loses precision

**Code:**
```typescript
const humanAmount = usdValue / priceUsd;
const atomicUnits = humanToAtomic(humanAmount.toFixed(srcDecimals), srcDecimals);
```

`usdValue / priceUsd` is IEEE-754 float64. For 18-decimal tokens, float64 gives ~15 significant digits; the last 3 decimal places of the `toFixed(18)` string are garbage.

**Impact:** Amounts for 18-decimal tokens can be off by hundreds of wei. Acceptable for display, but the actual submitted amount differs slightly from what was shown.

**Fix:** Document the precision limit, or use a decimal math library (e.g., `decimal.js`) for USD conversions.

---

### 12. `swap.ts:113` — Unsafe dynamic property access on SDK response

**Code:**
```typescript
orderId: (order as any)[variant].orderId as string,
outputAmount: getInnerQuote(quote).outputAmount.amount as string,
```

Full `as any` cast. If the SDK changes its response shape (e.g., renames `onramp` to `btcOnramp`), `order[variant]` is `undefined` and `.orderId` throws `Cannot read properties of undefined` with no useful context.

**Impact:** Silent breakage on SDK upgrade. Error message is confusing ("Cannot read properties of undefined (reading 'orderId')") rather than "Gateway returned unexpected order format".

**Fix:** Add a runtime check: `if (!order[variant]) throw new Error(\`Unexpected order structure for variant "${variant}"\`)`.

---

## LOW / CODE QUALITY

### 13. `route-provider.ts:29` — CommonJS `require()` used to break circular dependency

**Code:**
```typescript
const { getTokenMetadata } = require('../chains/evm.js') as typeof import('../chains/evm.js');
```

Dynamic CJS `require()` in an ESM TypeScript project is a structural workaround. The circular dependency (`route-provider → evm → route-provider`) indicates that `getTokenMetadata` belongs in a shared module, not in `evm.ts`.

**Impact:** Works at runtime but breaks tree-shaking, confuses bundlers, and hides a design flaw. Type safety relies on a `typeof import()` cast that won't catch drift.

---

### 14. `cli.ts:23` — Error mode detection is fragile

**Code:**
```typescript
const mode = modeOf(args.find(a => a?.json !== undefined) ?? {});
```

The `withErrorHandling` wrapper infers output mode by scanning the runtime `args` array for an object with a `json` property. If Commander's argument ordering changes (new positional args, new Commander version), this silently defaults to human mode on errors even when `--json` was passed.

---

### 15. `chains/index.ts:51` — `getAllBalances` swallows all errors silently

**Code:**
```typescript
} catch {
  return [chain, { address, error: true }];
}
```

Any error — RPC down, invalid address, rate limit, malformed response — produces the same `{ error: true }` with no message. The CLI renders this as "N/A".

**Impact:** Users cannot diagnose why a chain shows "N/A". A network misconfiguration looks identical to an unsupported address type.

**Fix:** Capture `error.message` and surface it in the output (at minimum in `--json` mode).

---

### 16. `config.ts:24` — Default slippage of 300 bps (3%) is high and undocumented

```typescript
slippageBps: 300,
```

3% slippage is unusually generous for an automated swap CLI. A user running scripted swaps may unknowingly accept far worse execution than expected. This default is not shown in `--help` output.

**Fix:** Lower default to 50–100 bps, or prominently document the default in `--help` and README.

---

### 17. `schemas.ts:6–22` — Validators named "positive" accept zero

Both `positiveNumber` and `positiveInt` use `n < 0` as the rejection condition, so `0` passes validation. `--slippage 0` is silently accepted; the swap will fail at the API level with a cryptic error rather than a clear validation message.

**Fix:** Use `n <= 0` to enforce truly positive values.

---

### 18. `swap.ts:199` — Poll loop gives no elapsed time or attempt count

```typescript
log.progress(`  Waiting for confirmation...`);
```

This identical message is printed every 15 seconds for up to 30 minutes. The user has no feedback on elapsed time, attempt number, or estimated remaining time.

**Fix:** Include elapsed time: `Waiting for confirmation... (${Math.round((Date.now() - startMs) / 1000)}s elapsed)`.

---

## Summary Table

| # | File | Line | Severity | Issue |
|---|------|------|----------|-------|
| 1 | `swap.ts` | 143 | Critical | Nonce check uses `opts.sender` instead of `senderAddress` |
| 2 | `rpc-resolver.ts` | 55–59 | Critical | Probe doesn't verify returned chain ID |
| 3 | `chains/evm.ts` | 172–174 | Critical | Multicall failures silently become 0 balance |
| 4 | `swap.ts` | 158–164 | High | `chain: null` disables chain ID validation |
| 5 | `schemas.ts` | 15–22 | High | `positiveInt` truncates floats silently |
| 6 | `cli.ts` / `evm.ts` | 147 / 100 | High | Scientific notation bypasses validation, crashes `BigInt` |
| 7 | `swap.ts` | 44 | High | Duplicate chain family detection can diverge from resolver |
| 8 | `swap.ts` | 220 | Medium | Mempool fallback runs for layerZero swaps too |
| 9 | `price-oracle.ts` | 44 | Medium | Zero/negative price from Binance causes downstream crash |
| 10 | `rpc-resolver.ts` | 85 | Medium | Env var key constructed from user-controlled input |
| 11 | `input-resolver.ts` | 155–156 | Medium | Float division loses precision for 18-decimal tokens |
| 12 | `swap.ts` | 113 | Medium | Unsafe `as any` dynamic property access on SDK response |
| 13 | `route-provider.ts` | 29 | Low | CJS `require()` used to paper over circular dependency |
| 14 | `cli.ts` | 23 | Low | Error mode detection relies on args array shape |
| 15 | `chains/index.ts` | 51 | Low | All chain errors silently collapsed to `{ error: true }` |
| 16 | `config.ts` | 24 | Low | 3% default slippage undocumented and unexpectedly high |
| 17 | `schemas.ts` | 6–22 | Low | "positive" validators accept 0 |
| 18 | `swap.ts` | 199 | Low | Poll loop has no elapsed time or attempt counter |
