# Unified `--amount` Flag — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace three amount flags (`--amount`, `--amount-atomic`, `--amount-usd`) with a single `--amount` flag supporting suffixes and an `ALL` keyword for max balance.

**Architecture:** New `parseAmount()` function in input-resolver.ts handles all parsing. Returns a sentinel `{ type: "all" }` for ALL; callers (quote/swap) resolve it with balance lookups. String-based decimal shifting replaces floating-point conversion.

**Tech Stack:** TypeScript, Zod, viem (for EVM balance/gas estimation)

**Spec:** `gateway-cli/docs/specs/2026-03-19-unified-amount-flag-design.md`

---

### Task 1: New amount parser + schema (atomic change)

All files must change together since the schema, parser, CLI flags, and command handlers are tightly coupled.

**Files:**
- Modify: `gateway-cli/src/util/input-resolver.ts`
- Modify: `gateway-cli/src/schemas.ts`
- Modify: `gateway-cli/src/cli.ts`
- Modify: `gateway-cli/src/commands/quote.ts`
- Modify: `gateway-cli/src/commands/swap.ts`

- [ ] **Step 1: Rewrite amount types and parser in input-resolver.ts**

Replace the entire "Amount resolution" section (lines 98-146) with:

```typescript
// ─── Amount parsing ─────────────────────────────────────────────────────────

export type ParsedAmount =
  | { type: "atomic"; atomicUnits: string; display: string }
  | { type: "all" };

/** Convert human-readable decimal string to atomic units using string math (no floating-point). */
function humanToAtomic(human: string, decimals: number): string {
  const [intPart, fracPart = ""] = human.split(".");
  const padded = fracPart.padEnd(decimals, "0").slice(0, decimals);
  const raw = intPart + padded;
  return BigInt(raw).toString(); // strips leading zeros
}

const AMOUNT_HELP = `Expected one of:
  0.05BTC       human-readable (converted to atomic)
  100USD        USD value (converted via price oracle)
  5000000       atomic units (satoshis, wei, etc.)
  ALL           max spendable balance`;

export async function parseAmount(
  raw: string,
  srcSymbol: string,
  srcDecimals: number,
): Promise<ParsedAmount> {
  const trimmed = raw.trim();
  if (!trimmed || trimmed.includes(" ")) {
    throw new Error(`Invalid amount "${raw}". ${AMOUNT_HELP}`);
  }

  const upper = trimmed.toUpperCase();

  // ALL → sentinel
  if (upper === "ALL") return { type: "all" };

  // Ends with USD → price oracle
  if (upper.endsWith("USD")) {
    const numStr = trimmed.slice(0, -3);
    const usdValue = parseFloat(numStr);
    if (isNaN(usdValue) || usdValue <= 0) throw new Error(`Invalid amount "${raw}". ${AMOUNT_HELP}`);
    const { priceUsd, source } = await fetchPrice(srcSymbol);
    const humanAmount = usdValue / priceUsd;
    const atomicUnits = humanToAtomic(humanAmount.toFixed(srcDecimals), srcDecimals);
    return {
      type: "atomic",
      atomicUnits,
      display: `${humanAmount.toFixed(8).replace(/\.?0+$/, "")} ${srcSymbol} ($${usdValue} via ${source})`,
    };
  }

  // Ends with source token symbol → human-readable
  if (upper.endsWith(srcSymbol.toUpperCase())) {
    const numStr = trimmed.slice(0, -srcSymbol.length);
    const num = parseFloat(numStr);
    if (isNaN(num) || num <= 0) throw new Error(`Invalid amount "${raw}". ${AMOUNT_HELP}`);
    const atomicUnits = humanToAtomic(numStr, srcDecimals);
    return { type: "atomic", atomicUnits, display: `${numStr} ${srcSymbol}` };
  }

  // Bare integer → atomic units
  if (/^\d+$/.test(trimmed)) {
    const val = BigInt(trimmed);
    if (val <= 0n) throw new Error(`Invalid amount "${raw}". Amount must be positive.`);
    return { type: "atomic", atomicUnits: val.toString(), display: `${trimmed} (atomic)` };
  }

  throw new Error(`Invalid amount "${raw}". ${AMOUNT_HELP}`);
}
```

Remove the old functions: `toAtomicUnits`, `parseAmountUsd`, `resolveAmount`, `AmountInput` interface.

Update `resolveSwapInputs` to accept a single `amount: string` instead of `AmountInput`:

```typescript
export async function resolveSwapInputs(
  src: string,
  dst: string,
  amount: string,
  enriched: EnrichedRoute[],
) {
  const tokenIndex = buildTokenIndex(enriched);
  const srcAsset = parseAssetChain(src, enriched, tokenIndex);
  const dstAsset = parseAssetChain(dst, enriched, tokenIndex);
  const parsed = await parseAmount(amount, srcAsset.symbol, srcAsset.decimals);
  return { srcAsset, dstAsset, parsed };
}
```

- [ ] **Step 2: Simplify schemas.ts**

Replace the `amountGroup` and its three fields with a single required field. Remove `atomicAmount` schema.

New `quoteSchema`:
```typescript
export const quoteSchema = z.object({
  src: z.string(),
  dst: z.string(),
  amount: z.string(),
  recipient: z.string().optional(),
  sender: z.string().optional(),
  slippage: positiveInt.optional(),
  gasRefillUsd: positiveNumber.optional(),
  btcFeeRate: positiveInt.optional(),
  json: z.boolean().default(false),
});
```

`swapSchema` stays as `quoteSchema.and(z.object({ ... }))` — no changes to its extension fields.

Remove: `atomicAmount` const, `amountGroup` const, the `.and(amountGroup)` from quoteSchema.

- [ ] **Step 3: Update CLI flags in cli.ts**

In the `quote` command definition, replace:
```typescript
  .option("--amount <value>", "Amount in human-readable units (e.g. 0.1 for 0.1 BTC)")
  .option("--amount-atomic <value>", "Amount in smallest units (e.g. satoshis, wei)")
  .option("--amount-usd <value>", "Amount in USD (converted via price oracle)")
```
with:
```typescript
  .requiredOption("--amount <value>", "Amount: 0.05BTC, 100USDC, 100USD, 5000000 (atomic), ALL")
```

Same change in `addSwapOptions()`.

- [ ] **Step 4: Update quote.ts**

Replace `QuoteOptions` interface — remove `amount?`, `amountAtomic?`, `amountUsd?`, add `amount: string`.

Update `handleQuote` — change the `resolveSwapInputs` call:
```typescript
  const { srcAsset, dstAsset, parsed } = await resolveSwapInputs(opts.src, opts.dst, opts.amount, enriched);
```

After `resolveSwapInputs`, handle ALL for quote:
```typescript
  if (parsed.type === "all") {
    throw new Error("--amount ALL is not supported for quote. Use a specific amount.");
  }
```

Update `quote` usage of `parsed.atomicUnits` — now accessed as `parsed.atomicUnits` (same, since ALL throws).

Remove the `parseAmountUsd` import from swap.ts (it no longer exists).

- [ ] **Step 5: Update swap.ts**

Replace `SwapOptions` interface — remove `amount?`, `amountAtomic?`, `amountUsd?`, add `amount: string`.

Update `handleSwap` — change the `resolveSwapInputs` call:
```typescript
  const { srcAsset, dstAsset, parsed } = await resolveSwapInputs(opts.src, opts.dst, opts.amount, enriched);
```

After `resolveSwapInputs`, resolve ALL for swap:
```typescript
  let atomicUnits: string;
  let display: string;
  if (parsed.type === "all") {
    const resolved = await resolveAllAmount(srcAsset, opts, config, enriched);
    atomicUnits = resolved.atomicUnits;
    display = resolved.display;
  } else {
    atomicUnits = parsed.atomicUnits;
    display = parsed.display;
  }
```

Add the `resolveAllAmount` helper function at the bottom of swap.ts:

```typescript
const NATIVE_GAS_BUFFER = 900_000n;

async function resolveAllAmount(
  srcAsset: { chain: string; address: string; symbol: string; decimals: number },
  opts: { privateKey?: string; sender?: string; unsigned: boolean },
  config: { bitcoinPrivateKey?: string; evmPrivateKey?: string },
  enriched: EnrichedRoute[],
): Promise<{ atomicUnits: string; display: string }> {
  const isBtc = srcAsset.chain === "bitcoin";

  // Derive sender address
  let senderAddress: string | undefined = opts.sender;
  if (!senderAddress) {
    const key = isBtc
      ? (opts.privateKey ?? config.bitcoinPrivateKey)
      : (opts.privateKey ?? config.evmPrivateKey);
    if (!key) {
      throw new Error("--amount ALL requires a sender address. Use --private-key, --sender, or set BITCOIN_PRIVATE_KEY / EVM_PRIVATE_KEY.");
    }
    if (isBtc) {
      const signer = ScureBitcoinSigner.fromKey(key);
      senderAddress = await signer.getP2WPKHAddress();
    } else {
      const account = privateKeyToAccount((key.startsWith("0x") ? key : `0x${key}`) as Hex);
      senderAddress = account.address;
    }
  }

  let atomicUnits: string;
  if (isBtc) {
    const sdk = getSdk();
    const maxSpendable = await sdk.getMaxSpendable(senderAddress);
    atomicUnits = maxSpendable.amount.amount;
  } else if (srcAsset.address === "0x0000000000000000000000000000000000000000" || isNativeToken(srcAsset, enriched)) {
    // Native EVM token (ETH, etc.) — reserve gas
    const rpcUrl = resolveRpcUrl(srcAsset.chain);
    const client = createPublicClient({ chain: getViemChain(srcAsset.chain), transport: http(rpcUrl) });
    const [balance, feeData] = await Promise.all([
      client.getBalance({ address: senderAddress as `0x${string}` }),
      client.estimateFeesPerGas(),
    ]);
    const gasCost = (feeData.maxFeePerGas ?? feeData.gasPrice ?? 0n) * NATIVE_GAS_BUFFER;
    const available = balance > gasCost ? balance - gasCost : 0n;
    atomicUnits = available.toString();
  } else {
    // ERC20 token
    const rpcUrl = resolveRpcUrl(srcAsset.chain);
    const client = createPublicClient({ chain: getViemChain(srcAsset.chain), transport: http(rpcUrl) });
    const balance = await client.readContract({
      address: srcAsset.address as `0x${string}`,
      abi: erc20Abi,
      functionName: "balanceOf",
      args: [senderAddress as `0x${string}`],
    });
    atomicUnits = balance.toString();
  }

  if (BigInt(atomicUnits) === 0n) {
    throw new Error(`No ${srcAsset.symbol} balance found for ${senderAddress}`);
  }

  const { formatUnits } = await import("viem");
  const humanDisplay = formatUnits(BigInt(atomicUnits), srcAsset.decimals);
  return { atomicUnits, display: `${humanDisplay} ${srcAsset.symbol} (max spendable)` };
}

function isNativeToken(asset: { chain: string; symbol: string }, enriched: EnrichedRoute[]): boolean {
  const nt = getNativeToken(asset.chain);
  return asset.symbol.toUpperCase() === nt.symbol.toUpperCase();
}
```

Add the `erc20Abi` import from viem if not already present. The `formatUnits` can be dynamically imported or added to the existing viem import.

Replace all uses of `parsed.atomicUnits` and `parsed.display` with `atomicUnits` and `display` throughout the handler (in `quoteParams`, `gasRefillWei`, the submitted/confirmed return data, etc.).

For `gasRefillUsd` — it currently uses the old `parseAmountUsd`. Replace with a direct price oracle call:
```typescript
  const gasRefillWei = opts.gasRefillUsd
    ? await (async () => {
        const { priceUsd } = await fetchPrice("ETH");
        const ethAmount = opts.gasRefillUsd! / priceUsd;
        return humanToAtomic(ethAmount.toFixed(18), 18);
      })()
    : undefined;
```

Import `fetchPrice` and `humanToAtomic` from input-resolver. Export `humanToAtomic` from input-resolver.

- [ ] **Step 6: Update cli.ts command handlers**

In the `quote` action, change:
```typescript
    const result = await handleQuote({ ...parsed, recipient: parsed.recipient, sender: parsed.sender });
```
This still works since `parsed` now has `amount: string` (single field) instead of three.

In `runSwap`, same — `parsed` now has `amount: string`.

- [ ] **Step 7: Verify build**

```bash
cd gateway-cli && pnpm tsc --noEmit
```

- [ ] **Step 8: Commit**

```bash
git add -A gateway-cli/src/
git commit -m "feat: unified --amount flag with suffix parsing and ALL support

Replaces --amount, --amount-atomic, --amount-usd with single --amount.
Supports: 0.05BTC (human), 100USD (price oracle), 5000000 (atomic), ALL (max balance).
Fixes toAtomicUnits floating-point precision with string-based decimal shifting.

Co-Authored-By: Claude Opus 4.6 (1M context) <noreply@anthropic.com>"
```

---

### Task 2: Update tests

**Files:**
- Modify: `gateway-cli/tests/util/input-resolver.test.ts`
- Modify: `gateway-cli/tests/commands/quote.test.ts`
- Modify: `gateway-cli/tests/commands/swap.test.ts`
- Modify: `gateway-cli/tests/e2e/cli.test.ts`

- [ ] **Step 1: Rewrite input-resolver amount tests**

Replace the `resolveAmount` describe block with `parseAmount` tests:

```typescript
describe("parseAmount", () => {
  beforeEach(() => vi.clearAllMocks());

  it("bare integer → atomic units", async () => {
    const result = await parseAmount("5000000", "BTC", 8);
    expect(result).toEqual({ type: "atomic", atomicUnits: "5000000", display: "5000000 (atomic)" });
  });

  it("token suffix → human to atomic", async () => {
    const result = await parseAmount("0.05BTC", "BTC", 8);
    expect(result).toEqual({ type: "atomic", atomicUnits: "5000000", display: "0.05 BTC" });
  });

  it("token suffix is case-insensitive", async () => {
    const result = await parseAmount("0.05btc", "BTC", 8);
    expect(result.type).toBe("atomic");
    if (result.type === "atomic") expect(result.atomicUnits).toBe("5000000");
  });

  it("USD suffix → price oracle", async () => {
    vi.mocked(fetchPrice).mockResolvedValueOnce({ priceUsd: 50000, source: "binance" });
    const result = await parseAmount("100USD", "BTC", 8);
    expect(result.type).toBe("atomic");
    expect(fetchPrice).toHaveBeenCalledWith("BTC");
  });

  it("ALL → sentinel", async () => {
    const result = await parseAmount("ALL", "BTC", 8);
    expect(result).toEqual({ type: "all" });
  });

  it("all (lowercase) → sentinel", async () => {
    const result = await parseAmount("all", "BTC", 8);
    expect(result).toEqual({ type: "all" });
  });

  it("bare decimal without suffix → error", async () => {
    await expect(parseAmount("0.5", "BTC", 8)).rejects.toThrow("Invalid amount");
  });

  it("negative number → error", async () => {
    await expect(parseAmount("-100", "BTC", 8)).rejects.toThrow("Invalid amount");
  });

  it("zero → error", async () => {
    await expect(parseAmount("0", "BTC", 8)).rejects.toThrow("must be positive");
  });

  it("empty string → error", async () => {
    await expect(parseAmount("", "BTC", 8)).rejects.toThrow("Invalid amount");
  });

  it("internal space → error", async () => {
    await expect(parseAmount("100 USD", "BTC", 8)).rejects.toThrow("Invalid amount");
  });

  it("wrong token suffix → error", async () => {
    await expect(parseAmount("100USDC", "BTC", 8)).rejects.toThrow("Invalid amount");
  });

  it("USDC with 6 decimals", async () => {
    const result = await parseAmount("100USDC", "USDC", 6);
    expect(result).toEqual({ type: "atomic", atomicUnits: "100000000", display: "100 USDC" });
  });

  it("humanToAtomic precision: 0.3 ETH with 18 decimals", async () => {
    const result = await parseAmount("0.3ETH", "ETH", 18);
    if (result.type === "atomic") {
      expect(result.atomicUnits).toBe("300000000000000000");
    }
  });
});
```

Update imports: replace `resolveAmount` with `parseAmount`.

- [ ] **Step 2: Update quote.test.ts**

Change the mock of `resolveSwapInputs` to match new signature (single `amount` string instead of object). Update test fixtures: `amount: "0.05"` → `amount: "0.05BTC"` or `amount: "5000000"`.

The mock returns `parsed: { type: "atomic", atomicUnits: "5000000", display: "0.05 BTC" }` now.

- [ ] **Step 3: Update swap.test.ts**

Change `baseOpts` to use single `amount` field: `amount: "5000000"` (atomic, bare integer).

Update mock of `resolveSwapInputs` if used, or adjust the `parseAmount` behavior in the import mock.

- [ ] **Step 4: Update e2e/cli.test.ts**

Update the swap --help test to verify `--amount` is present and `--amount-atomic`/`--amount-usd` are absent:
```typescript
  it("swap --help shows unified amount flag", () => {
    const output = run("swap", "--help");
    expect(output).toContain("--amount");
    expect(output).not.toContain("--amount-atomic");
    expect(output).not.toContain("--amount-usd");
  });
```

- [ ] **Step 5: Run all tests**

```bash
cd gateway-cli && pnpm vitest run
```

- [ ] **Step 6: Commit**

```bash
git add -A gateway-cli/tests/
git commit -m "test: update tests for unified --amount flag

Co-Authored-By: Claude Opus 4.6 (1M context) <noreply@anthropic.com>"
```

---

### Task 3: Final verification

- [ ] **Step 1: Run full test suite**

```bash
cd gateway-cli && pnpm vitest run
```

- [ ] **Step 2: Verify TypeScript compiles**

```bash
cd gateway-cli && pnpm tsc --noEmit
```

- [ ] **Step 3: Check for stale references**

```bash
cd gateway-cli && grep -rE "amountAtomic|amountUsd|amount-atomic|amount-usd|AmountInput|resolveAmount|parseAmountUsd|toAtomicUnits" src/ --include="*.ts"
```

Expected: No matches (except possibly in the AMOUNT_HELP string which mentions "atomic").

- [ ] **Step 4: Verify e2e**

```bash
cd gateway-cli && pnpm vitest run tests/e2e/
```

- [ ] **Step 5: Final commit if fixes needed**

```bash
git add -A gateway-cli/ && git commit -m "chore: final cleanup for unified amount flag"
```
