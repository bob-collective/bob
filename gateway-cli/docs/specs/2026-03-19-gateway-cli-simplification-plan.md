# Gateway CLI Simplification — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Simplify gateway-cli by removing TOML config, route caching, `--dry-run`, and custom retry, consolidating small files, and rewriting tests.

**Architecture:** Mechanical refactor — no new features, no structural redesign. Each task removes or simplifies one concern. Tests are rewritten last since the old tests target deleted abstractions.

**Tech Stack:** TypeScript, Commander, Viem, Zod, p-retry, Vitest

**Spec:** `gateway-cli/docs/specs/2026-03-19-gateway-cli-simplification-design.md`

---

### Task 1: Remove TOML config, route caching, and simplify config

This is a single atomic task because config, rpc-resolver, route-provider, and their consumers are tightly coupled — changing config.ts breaks call sites in swap.ts, balance.ts, and route-provider.ts. All must change together.

**Files:**
- Rewrite: `gateway-cli/src/config.ts`
- Rewrite: `gateway-cli/src/util/rpc-resolver.ts`
- Modify: `gateway-cli/src/util/route-provider.ts`
- Modify: `gateway-cli/src/commands/swap.ts`
- Modify: `gateway-cli/src/commands/balance.ts`
- Modify: `gateway-cli/src/cli.ts`
- Modify: `gateway-cli/src/util/input-resolver.ts`
- Modify: `gateway-cli/package.json`
- Delete: `gateway-cli/src/constants.ts`

- [ ] **Step 1: Remove toml dependency**

```bash
cd gateway-cli && pnpm remove toml
```

- [ ] **Step 2: Rewrite config.ts**

Replace the entire file contents with:

```typescript
import { GatewaySDK } from "@gobob/bob-sdk";

export const BTC_DECIMALS = 8;

export interface Config {
  apiUrl?: string;
  bitcoinPrivateKey?: string;
  evmPrivateKey?: string;
  timeoutMs: number;
  slippageBps: number;
  btcFeeRate?: number;
}

let _config: Config | null = null;

export function loadConfig(): Config {
  if (_config) return _config;
  const feeRate = process.env.BTC_FEE_RATE ? parseInt(process.env.BTC_FEE_RATE, 10) : undefined;
  _config = {
    apiUrl: process.env.GATEWAY_API_URL,
    bitcoinPrivateKey: process.env.BITCOIN_PRIVATE_KEY,
    evmPrivateKey: process.env.EVM_PRIVATE_KEY,
    timeoutMs: 1_800_000,
    slippageBps: 300,
    btcFeeRate: feeRate && !isNaN(feeRate) ? feeRate : undefined,
  };
  return _config;
}

let _sdk: InstanceType<typeof GatewaySDK> | null = null;

export function getSdk(): InstanceType<typeof GatewaySDK> {
  if (!_sdk) {
    const { apiUrl } = loadConfig();
    _sdk = apiUrl ? new GatewaySDK(apiUrl) : new GatewaySDK();
  }
  return _sdk;
}
```

- [ ] **Step 3: Delete constants.ts**

```bash
rm gateway-cli/src/constants.ts
```

- [ ] **Step 4: Update BTC_DECIMALS imports**

In `gateway-cli/src/util/input-resolver.ts`, change:
```
import { BTC_DECIMALS } from "../constants.js";
```
to:
```
import { BTC_DECIMALS } from "../config.js";
```

In `gateway-cli/src/util/route-provider.ts`, change:
```
import { BTC_DECIMALS } from '../constants.js';
```
to:
```
import { BTC_DECIMALS } from '../config.js';
```

- [ ] **Step 5: Rewrite rpc-resolver.ts**

Replace the entire file contents with:

```typescript
import { supportedChainsMapping } from '@gobob/bob-sdk';
import type { Chain } from 'viem';

/** Resolve RPC URL from env var EVM_RPC_URL_<CHAIN>, or undefined for viem defaults. */
export function resolveRpcUrl(chainName: string): string | undefined {
  return process.env[`EVM_RPC_URL_${chainName.toUpperCase()}`];
}

/** Get the viem Chain object for a gateway chain name, if supported. */
export function getViemChain(chainName: string): Chain | undefined {
  return (supportedChainsMapping as Record<string, Chain>)[chainName];
}
```

- [ ] **Step 6: Strip caching from route-provider.ts**

Remove these imports at the top of the file:
```typescript
import * as fs from 'fs';
import * as path from 'path';
import * as os from 'os';
```

Change the config import from:
```typescript
import { getSdk, loadConfig } from '../config.js';
```
to:
```typescript
import { getSdk } from '../config.js';
```

Delete the entire cache section (lines 104-134):
```typescript
// ─── Route cache ─────────────────────────────────────────────────────────

const CACHE_DIR = path.join(os.homedir(), '.gateway-cli', 'cache');
const CACHE_FILE = path.join(CACHE_DIR, 'routes.json');

interface CacheEntry {
  fetchedAt: string;
  routes: RouteInfo[];
}

function parseTtl(ttl: string): number {
  const match = ttl.match(/^(\d+)(h|d)$/);
  if (!match) return 24 * 60 * 60 * 1000;
  const [, num, unit] = match;
  return unit === 'h' ? Number(num) * 3600_000 : Number(num) * 86400_000;
}

function readCache(ttl: string): RouteInfo[] | null {
  if (!fs.existsSync(CACHE_FILE)) return null;
  try {
    const data: CacheEntry = JSON.parse(fs.readFileSync(CACHE_FILE, 'utf-8'));
    const age = Date.now() - new Date(data.fetchedAt).getTime();
    if (age >= parseTtl(ttl)) return null;
    return data.routes;
  } catch { return null; }
}

function writeCache(routes: RouteInfo[]): void {
  fs.mkdirSync(CACHE_DIR, { recursive: true });
  fs.writeFileSync(CACHE_FILE, JSON.stringify({ fetchedAt: new Date().toISOString(), routes }));
}
```

Replace the `getEnrichedRoutes` function (lines 153-170) with:
```typescript
/** Fetch routes from SDK and enrich with tokenlist metadata. */
export async function getEnrichedRoutes(): Promise<EnrichedRoute[]> {
  const sdk = getSdk();
  const routes = await sdk.getRoutes();
  return enrichRoutes(routes);
}
```

- [ ] **Step 7: Update swap.ts — remove config.rpc and config.noWait**

Change the `resolveEvmSigner` function signature from:
```typescript
function resolveEvmSigner(key: string | undefined, unsigned: boolean, chainName: string, config: Config): EvmSignerResult {
```
to:
```typescript
function resolveEvmSigner(key: string | undefined, unsigned: boolean, chainName: string): EvmSignerResult {
```

Inside `resolveEvmSigner`, change:
```typescript
  const rpcUrl = resolveRpcUrl(chainName, config.rpc);
```
to:
```typescript
  const rpcUrl = resolveRpcUrl(chainName);
```

Update the call site in `handleSwap` (line 68) from:
```typescript
    : resolveEvmSigner(opts.privateKey ?? config.evmPrivateKey, opts.unsigned, evmChain, config);
```
to:
```typescript
    : resolveEvmSigner(opts.privateKey ?? config.evmPrivateKey, opts.unsigned, evmChain);
```

Remove the `type Config` from the import:
```typescript
import { loadConfig, getSdk, type Config } from "../config.js";
```
to:
```typescript
import { loadConfig, getSdk } from "../config.js";
```

Change line 207 from:
```typescript
  if (!opts.wait || config.noWait) {
```
to:
```typescript
  if (!opts.wait) {
```

- [ ] **Step 8: Update balance.ts — remove config.rpc and noCache**

Change the `getEvmBalances` function signature from:
```typescript
async function getEvmBalances(
  address: string,
  chain: string,
  routes: EnrichedRoute[],
  config: Config,
) {
```
to:
```typescript
async function getEvmBalances(
  address: string,
  chain: string,
  routes: EnrichedRoute[],
) {
```

Inside `getEvmBalances`, change:
```typescript
  const rpcUrl = resolveRpcUrl(chain, config.rpc);
```
to:
```typescript
  const rpcUrl = resolveRpcUrl(chain);
```

Remove `noCache` from `BalanceOptions`:
```typescript
export interface BalanceOptions {
  chain?: string;
}
```

Update `handleBalance` — change:
```typescript
  const enriched = await getEnrichedRoutes({ noCache: opts.noCache });
```
to:
```typescript
  const enriched = await getEnrichedRoutes();
```

Update the `getEvmBalances` call site from:
```typescript
      const evmBalance = await getEvmBalances(address, chain, enriched, config);
```
to:
```typescript
      const evmBalance = await getEvmBalances(address, chain, enriched);
```

Remove `type Config` from balance.ts imports:
```typescript
import { loadConfig, getSdk, type Config } from '../config.js';
```
to:
```typescript
import { loadConfig, getSdk } from '../config.js';
```

- [ ] **Step 9: Update cli.ts — remove getConfig, --no-cache, config fallbacks**

Remove the `getConfig` helper function:
```typescript
async function getConfig() {
  const { loadConfig } = await import("./config.js");
  return loadConfig();
}
```

Remove the `--no-cache` option from balance command:
```typescript
  .option("--no-cache", "Skip route cache")
```

Update the balance action call from:
```typescript
    render(await handleBalance(address, { chain: opts.chain, noCache: opts.cache === false }), modeOf(opts), formatBalance);
```
to:
```typescript
    render(await handleBalance(address, { chain: opts.chain }), modeOf(opts), formatBalance);
```

Replace the `quote` action body with:
```typescript
  .action(withErrorHandling(async (opts) => {
    const mode = modeOf(opts);
    const parsed = quoteSchema.parse(opts);
    if (!parsed.recipient) throw new Error("--recipient is required");
    const { handleQuote } = await import("./commands/quote.js");
    const result = await handleQuote({ ...parsed, recipient: parsed.recipient, sender: parsed.sender });
    render(result.quote, mode, () => formatConfirmation(result.confirmation));
  }));
```

Replace the `runSwap` function with:
```typescript
async function runSwap(opts: any) {
  const mode = modeOf(opts);
  const parsed = swapSchema.parse(opts);
  if (!parsed.recipient) throw new Error("--recipient is required");

  const log = createLogger(mode);
  const { handleSwap } = await import("./commands/swap.js");
  const result = await handleSwap({ ...parsed, recipient: parsed.recipient, sender: parsed.sender }, log);

  switch (result.type) {
    case "dryRun":
      render(result.quote, mode, () => formatConfirmation(result.confirmation));
      break;
    case "unsigned":
    case "cancelled":
    case "submitted":
    case "confirmed":
    case "mempoolPending":
      render("data" in result ? result.data : result, mode);
      break;
  }
}
```

Note: The `dryRun` case stays temporarily — it will be removed in Task 2.

- [ ] **Step 10: Verify build**

```bash
cd gateway-cli && npx tsc --noEmit
```

Expected: Compilation succeeds.

- [ ] **Step 11: Commit**

```bash
git add -A gateway-cli/src/ gateway-cli/package.json gateway-cli/pnpm-lock.yaml
git commit -m "refactor: remove TOML config, route caching, simplify rpc-resolver"
```

---

### Task 2: Drop --dry-run

**Files:**
- Modify: `gateway-cli/src/commands/swap.ts`
- Modify: `gateway-cli/src/schemas.ts`
- Modify: `gateway-cli/src/cli.ts`

- [ ] **Step 1: Remove dryRun from schemas.ts**

In `swapSchema`, remove:
```typescript
  dryRun: z.boolean().default(false),
```

- [ ] **Step 2: Remove --dry-run from cli.ts**

In `addSwapOptions()`, remove:
```typescript
    .option("--dry-run", "Show quote and exit without creating an order", false)
```

In `runSwap()`, remove the `dryRun` case from the switch:
```typescript
    case "dryRun":
      render(result.quote, mode, () => formatConfirmation(result.confirmation));
      break;
```

- [ ] **Step 3: Remove dry-run from swap.ts**

Remove `dryRun` from `SwapOptions`:
```typescript
  dryRun: boolean;
```

Remove the `dryRun` variant from `SwapResult`:
```typescript
  | { type: "dryRun"; quote: QuoteJson; confirmation: ConfirmationData }
```

Remove the entire dry-run section (lines 86-116 in the original, may have shifted after Task 1 edits):
```typescript
  // ─── Dry-run ───────────────────────────────────────────────────────────────

  const initialQuote = await sdk.getQuote(quoteParams);
  const initialOutputAmount = getInnerQuote(initialQuote).outputAmount.amount;

  if (opts.dryRun) {
    return {
      type: "dryRun",
      quote: {
        srcAmount: parsed.atomicUnits,
        srcAsset: srcAsset.symbol,
        dstAmount: initialOutputAmount,
        dstAsset: dstAsset.symbol,
        dstChain: dstAsset.chain,
        slippageBps: slippageBps,
        feeRateSatPerVbyte: opts.btcFeeRate,
      },
      confirmation: {
        srcAmount: parsed.atomicUnits,
        srcAsset: srcAsset.symbol,
        srcDisplay: parsed.display,
        dstAmount: initialOutputAmount,
        dstAsset: dstAsset.symbol,
        dstChain: dstAsset.chain,
        feeRateSatPerVbyte: opts.btcFeeRate,
        slippageBps: slippageBps,
        recipient: opts.recipient,
        gasRefillUsd: opts.gasRefill ? String(opts.gasRefill) : undefined,
      },
    };
  }
```

Remove `QuoteJson` and `ConfirmationData` from the output.js import:
```typescript
import type { Logger, ConfirmationData, SwapSuccessJson, SwapSubmittedJson, SwapMempoolPendingJson, QuoteJson } from "../output.js";
```
becomes:
```typescript
import type { Logger, SwapSuccessJson, SwapSubmittedJson, SwapMempoolPendingJson } from "../output.js";
```

- [ ] **Step 4: Verify build**

```bash
cd gateway-cli && npx tsc --noEmit
```

- [ ] **Step 5: Commit**

```bash
git add gateway-cli/src/commands/swap.ts gateway-cli/src/schemas.ts gateway-cli/src/cli.ts
git commit -m "refactor: drop --dry-run from swap — use quote command instead"
```

---

### Task 3: Replace retry with p-retry inline in swap.ts

**Files:**
- Modify: `gateway-cli/package.json`
- Delete: `gateway-cli/src/util/retry.ts`
- Modify: `gateway-cli/src/commands/swap.ts`
- Modify: `gateway-cli/src/cli.ts`

- [ ] **Step 1: Install p-retry**

```bash
cd gateway-cli && pnpm add p-retry
```

- [ ] **Step 2: Inline retry logic into swap.ts**

Replace the retry import:
```typescript
import { retryWithBackoff, executeWithTransientRetry } from "../util/retry.js";
```
with:
```typescript
import pRetry, { AbortError } from "p-retry";
```

Add transient error detection and retry helpers after the imports (before the Types section):

```typescript
// ─── Transient error detection + retry ──────────────────────────────────────

const TRANSIENT_PATTERNS = [
  /TRM screening/i,
  /429/,
  /Too Many Requests/i,
  /rate limit/i,
  /not yet propagated/i,
  /BTC propagation/i,
  /timeout/i,
  /ECONNRESET/,
  /ETIMEDOUT/,
];

function isTransientError(err: unknown): boolean {
  const msg = err instanceof Error ? err.message : String(err);
  return TRANSIENT_PATTERNS.some((p) => p.test(msg));
}

async function withTransientRetry<T>(
  fn: () => Promise<T>,
  opts: { noRetry?: boolean; log: Logger },
): Promise<T> {
  if (opts.noRetry) return fn();
  return pRetry(
    async (attemptNumber) => {
      try {
        return await fn();
      } catch (err) {
        if (isTransientError(err)) {
          opts.log.progress(`Retrying (${attemptNumber}/6)...`);
          throw err;
        }
        throw new AbortError(err instanceof Error ? err.message : String(err));
      }
    },
    { retries: 5 },
  );
}

async function withRegistrationRetry<T>(
  fn: () => Promise<T>,
  log: Logger,
): Promise<T> {
  return pRetry(
    async (attemptNumber) => {
      if (attemptNumber > 1) log.progress(`  Retrying registration (attempt ${attemptNumber})...`);
      return fn();
    },
    { retries: 3 },
  );
}
```

- [ ] **Step 3: Update swap.ts call sites**

Replace the `executeWithTransientRetry` call:
```typescript
  const result = await executeWithTransientRetry(executeCore, { noRetry: !opts.retry, log });
```
with:
```typescript
  const result = await withTransientRetry(executeCore, { noRetry: !opts.retry, log });
```

Replace the BTC registration retry:
```typescript
      try {
        await retryWithBackoff(
          () => sdk.api.registerTx({ registerTx: { onramp: { orderId, bitcoinTxHex: txId } } }),
          { onRetry: (attempt) => log.progress(`  Retrying registration (attempt ${attempt})...`) },
        );
      } catch (err) { throw registrationError(err, orderId, txId); }
```
with:
```typescript
      try {
        await withRegistrationRetry(
          () => sdk.api.registerTx({ registerTx: { onramp: { orderId, bitcoinTxHex: txId } } }),
          log,
        );
      } catch (err) { throw registrationError(err, orderId, txId); }
```

Replace the EVM registration retry:
```typescript
      try {
        await retryWithBackoff(
          () => sdk.api.registerTx(registerPayload),
          { onRetry: (attempt) => log.progress(`  Retrying registration (attempt ${attempt})...`) },
        );
      } catch (err) { throw registrationError(err, orderId, txId); }
```
with:
```typescript
      try {
        await withRegistrationRetry(
          () => sdk.api.registerTx(registerPayload),
          log,
        );
      } catch (err) { throw registrationError(err, orderId, txId); }
```

- [ ] **Step 4: Remove `retryable` check from cli.ts error handler**

In `gateway-cli/src/cli.ts`, change:
```typescript
        if (err instanceof Error) {
          if ("retryable" in err) errJson.error.retryable = true;
          if ("orderId" in err) errJson.error.orderId = (err as any).orderId;
          if ("txId" in err) errJson.error.txId = (err as any).txId;
        }
```
to:
```typescript
        if (err instanceof Error) {
          if ("orderId" in err) errJson.error.orderId = (err as any).orderId;
          if ("txId" in err) errJson.error.txId = (err as any).txId;
        }
```

- [ ] **Step 5: Delete retry.ts**

```bash
rm gateway-cli/src/util/retry.ts
```

- [ ] **Step 6: Verify build**

```bash
cd gateway-cli && npx tsc --noEmit
```

- [ ] **Step 7: Commit**

```bash
git add gateway-cli/package.json gateway-cli/pnpm-lock.yaml gateway-cli/src/commands/swap.ts gateway-cli/src/cli.ts
git rm gateway-cli/src/util/retry.ts
git commit -m "refactor: replace custom retry with p-retry, inline into swap"
```

---

### Task 4: Delete obsolete test files

**Files:**
- Delete: All test files in `gateway-cli/tests/`

- [ ] **Step 1: Delete all old test files and recreate directory structure**

```bash
cd gateway-cli && rm -rf tests/
mkdir -p tests/commands tests/util tests/e2e
```

- [ ] **Step 2: Commit**

```bash
cd gateway-cli && git add -A tests/
git commit -m "chore: remove obsolete tests (rewrite follows)"
```

---

### Task 5: Write util tests

**Files:**
- Create: `gateway-cli/tests/util/input-resolver.test.ts`
- Create: `gateway-cli/tests/util/route-provider.test.ts`
- Create: `gateway-cli/tests/util/price-oracle.test.ts`

- [ ] **Step 1: Write input-resolver tests**

```typescript
// gateway-cli/tests/util/input-resolver.test.ts
import { describe, it, expect, vi } from "vitest";
import { resolveChain, parseAssetChain, resolveAmount } from "../../src/util/input-resolver.js";
import type { EnrichedRoute } from "../../src/util/route-provider.js";

const routes: EnrichedRoute[] = [
  {
    srcChain: "bitcoin", dstChain: "base",
    srcToken: { address: "BTC", symbol: "BTC", decimals: 8, chain: "bitcoin" },
    dstToken: { address: "0xUSDC", symbol: "USDC", decimals: 6, chain: "base" },
  },
  {
    srcChain: "ethereum", dstChain: "bob",
    srcToken: { address: "0xWBTC", symbol: "WBTC", decimals: 8, chain: "ethereum" },
    dstToken: { address: "0xWBTC2", symbol: "WBTC", decimals: 8, chain: "bob" },
  },
];

describe("resolveChain", () => {
  it("maps btc alias to bitcoin", () => expect(resolveChain("btc")).toBe("bitcoin"));
  it("maps eth alias to ethereum", () => expect(resolveChain("eth")).toBe("ethereum"));
  it("passes through unknown chains", () => expect(resolveChain("bob")).toBe("bob"));
  it("is case-insensitive", () => expect(resolveChain("ETH")).toBe("ethereum"));
});

describe("parseAssetChain", () => {
  it("resolves BTC without chain", () => {
    const result = parseAssetChain("BTC", routes);
    expect(result.chain).toBe("bitcoin");
    expect(result.symbol).toBe("BTC");
  });

  it("resolves token:chain format", () => {
    const result = parseAssetChain("USDC:base", routes);
    expect(result.chain).toBe("base");
    expect(result.symbol).toBe("USDC");
  });

  it("resolves chain aliases", () => {
    const result = parseAssetChain("WBTC:eth", routes);
    expect(result.chain).toBe("ethereum");
  });

  it("throws for token without chain", () => {
    expect(() => parseAssetChain("USDC", routes)).toThrow("chain required");
  });

  it("throws for unknown token on chain", () => {
    expect(() => parseAssetChain("DOGE:base", routes)).toThrow("unknown token");
  });
});

describe("resolveAmount", () => {
  it("handles atomic units", async () => {
    const result = await resolveAmount({ amountAtomic: "5000000" }, "BTC", 8);
    expect(result.atomicUnits).toBe("5000000");
  });

  it("converts human-readable amounts", async () => {
    const result = await resolveAmount({ amount: "0.05" }, "BTC", 8);
    expect(result.atomicUnits).toBe("5000000");
  });
});
```

- [ ] **Step 2: Write route-provider tests**

```typescript
// gateway-cli/tests/util/route-provider.test.ts
import { describe, it, expect, vi, beforeEach } from "vitest";

const mockGetRoutes = vi.fn();
vi.mock("../../src/config.js", () => ({
  getSdk: vi.fn(() => ({ getRoutes: mockGetRoutes })),
  BTC_DECIMALS: 8,
}));

import { getEnrichedRoutes, getNativeToken } from "../../src/util/route-provider.js";

const rawRoutes = [
  { srcChain: "bitcoin", dstChain: "bob", srcToken: "BTC", dstToken: "0x03C7054BCB39f7b2e5B2c7AcB37583e32D70Cfa3" },
];

describe("getEnrichedRoutes", () => {
  beforeEach(() => vi.clearAllMocks());

  it("fetches routes from SDK and enriches them", async () => {
    mockGetRoutes.mockResolvedValueOnce(rawRoutes);
    const enriched = await getEnrichedRoutes();
    expect(mockGetRoutes).toHaveBeenCalledOnce();
    expect(Array.isArray(enriched)).toBe(true);
  });

  it("always fetches fresh (no caching)", async () => {
    mockGetRoutes.mockResolvedValue(rawRoutes);
    await getEnrichedRoutes();
    await getEnrichedRoutes();
    expect(mockGetRoutes).toHaveBeenCalledTimes(2);
  });
});

describe("getNativeToken", () => {
  it("returns ETH for ethereum", () => {
    const token = getNativeToken("ethereum");
    expect(token.symbol).toBe("ETH");
    expect(token.decimals).toBe(18);
  });

  it("throws for unknown chain", () => {
    expect(() => getNativeToken("solana")).toThrow("unknown chain");
  });
});
```

- [ ] **Step 3: Write price-oracle tests**

```typescript
// gateway-cli/tests/util/price-oracle.test.ts
import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { fetchPrice, PriceOracleError } from "../../src/util/price-oracle.js";

describe("fetchPrice", () => {
  const originalFetch = globalThis.fetch;

  beforeEach(() => { globalThis.fetch = vi.fn(); });
  afterEach(() => { globalThis.fetch = originalFetch; });

  it("returns Binance price when available", async () => {
    (globalThis.fetch as any).mockResolvedValueOnce({
      ok: true, json: () => Promise.resolve({ price: "50000.00" }),
    }).mockResolvedValueOnce({
      ok: true, json: () => Promise.resolve({ data: { amount: "50100.00" } }),
    });

    const result = await fetchPrice("BTC");
    expect(result.priceUsd).toBe(50000);
    expect(result.source).toBe("binance");
  });

  it("falls back to Coinbase on Binance failure", async () => {
    (globalThis.fetch as any)
      .mockRejectedValueOnce(new Error("Binance down"))
      .mockResolvedValueOnce({
        ok: true, json: () => Promise.resolve({ data: { amount: "50100.00" } }),
      });

    const result = await fetchPrice("BTC");
    expect(result.priceUsd).toBe(50100);
    expect(result.source).toBe("coinbase");
  });

  it("throws PriceOracleError when both fail", async () => {
    (globalThis.fetch as any)
      .mockRejectedValueOnce(new Error("Binance down"))
      .mockRejectedValueOnce(new Error("Coinbase down"));

    await expect(fetchPrice("BTC")).rejects.toThrow(PriceOracleError);
  });
});
```

- [ ] **Step 4: Run util tests**

```bash
cd gateway-cli && npx vitest run tests/util/
```

Expected: All pass.

- [ ] **Step 5: Commit**

```bash
git add gateway-cli/tests/util/
git commit -m "test: rewrite util tests for simplified modules"
```

---

### Task 6: Write command tests (quote, balance, routes, status, orders, register)

**Files:**
- Create: `gateway-cli/tests/commands/quote.test.ts`
- Create: `gateway-cli/tests/commands/balance.test.ts`
- Create: `gateway-cli/tests/commands/routes.test.ts`
- Create: `gateway-cli/tests/commands/status.test.ts`
- Create: `gateway-cli/tests/commands/orders.test.ts`
- Create: `gateway-cli/tests/commands/register.test.ts`

- [ ] **Step 1: Write quote tests**

```typescript
// gateway-cli/tests/commands/quote.test.ts
import { describe, it, expect, vi, beforeEach } from "vitest";

const mockGetQuote = vi.fn();
const mockGetRecommendedFees = vi.fn();

vi.mock("../../src/config.js", () => ({
  loadConfig: vi.fn(() => ({ slippageBps: 300, timeoutMs: 1800000 })),
  getSdk: vi.fn(() => ({ getQuote: mockGetQuote })),
  BTC_DECIMALS: 8,
}));

vi.mock("@gobob/bob-sdk", async (importOriginal) => {
  const actual = await importOriginal<any>();
  return {
    ...actual,
    MempoolClient: vi.fn().mockImplementation(() => ({
      getRecommendedFees: mockGetRecommendedFees,
    })),
  };
});

import type { EnrichedRoute } from "../../src/util/route-provider.js";

const enrichedRoutes: EnrichedRoute[] = [
  {
    srcChain: "bitcoin", dstChain: "base",
    srcToken: { address: "0x0000000000000000000000000000000000000000", symbol: "BTC", decimals: 8, chain: "bitcoin" },
    dstToken: { address: "0xUSDC", symbol: "USDC", decimals: 6, chain: "base" },
  },
];

vi.mock("../../src/util/route-provider.js", () => ({
  getEnrichedRoutes: vi.fn().mockResolvedValue(enrichedRoutes),
  getNativeToken: vi.fn().mockReturnValue({ symbol: "ETH", decimals: 18 }),
}));

import { handleQuote } from "../../src/commands/quote.js";

describe("handleQuote", () => {
  beforeEach(() => vi.clearAllMocks());

  it("returns quote and confirmation data", async () => {
    const sdkQuote = {
      onramp: {
        inputAmount: { amount: "5000000" },
        outputAmount: { amount: "4812300000" },
      },
    };
    mockGetQuote.mockResolvedValueOnce(sdkQuote);
    mockGetRecommendedFees.mockResolvedValueOnce({ fastestFee: 10 });

    const result = await handleQuote({
      src: "BTC", dst: "USDC:base", amount: "0.05",
      recipient: "0xABC",
    });

    expect(result.quote.srcAsset).toBe("BTC");
    expect(result.quote.dstAsset).toBe("USDC");
    expect(result.quote.dstAmount).toBe("4812300000");
    expect(result.confirmation.recipient).toBe("0xABC");
  });

  it("passes slippage to SDK", async () => {
    const sdkQuote = { onramp: { inputAmount: { amount: "5000000" }, outputAmount: { amount: "4812300000" } } };
    mockGetQuote.mockResolvedValueOnce(sdkQuote);
    mockGetRecommendedFees.mockResolvedValueOnce({ fastestFee: 10 });

    await handleQuote({
      src: "BTC", dst: "USDC:base", amount: "0.05",
      recipient: "0xABC", slippage: 100,
    });

    expect(mockGetQuote).toHaveBeenCalledWith(expect.objectContaining({ maxSlippage: 100 }));
  });
});
```

- [ ] **Step 2: Write balance, routes, status, orders, register tests**

```typescript
// gateway-cli/tests/commands/balance.test.ts
import { describe, it, expect, vi, beforeEach } from "vitest";

const mockGetMaxSpendable = vi.fn();
const mockGetBalance = vi.fn();

vi.mock("../../src/config.js", () => ({
  loadConfig: vi.fn(() => ({ slippageBps: 300, timeoutMs: 1800000 })),
  getSdk: vi.fn(() => ({ getMaxSpendable: mockGetMaxSpendable })),
  BTC_DECIMALS: 8,
}));

vi.mock("@gobob/bob-sdk", async (importOriginal) => {
  const actual = await importOriginal<any>();
  return {
    ...actual,
    EsploraClient: vi.fn().mockImplementation(() => ({ getBalance: mockGetBalance })),
    formatBtc: (v: bigint) => (Number(v) / 1e8).toFixed(8),
  };
});

import type { EnrichedRoute } from "../../src/util/route-provider.js";

const enrichedRoutes: EnrichedRoute[] = [
  {
    srcChain: "bitcoin", dstChain: "base",
    srcToken: { address: "BTC", symbol: "BTC", decimals: 8, chain: "bitcoin" },
    dstToken: { address: "0xUSDC", symbol: "USDC", decimals: 6, chain: "base" },
  },
];

vi.mock("../../src/util/route-provider.js", () => ({
  getEnrichedRoutes: vi.fn().mockResolvedValue(enrichedRoutes),
  getNativeToken: vi.fn().mockReturnValue({ symbol: "ETH", decimals: 18 }),
}));

vi.mock("viem", async (importOriginal) => {
  const actual = await importOriginal<any>();
  return {
    ...actual,
    createPublicClient: vi.fn(() => ({
      getBalance: vi.fn().mockResolvedValue(0n),
      multicall: vi.fn().mockResolvedValue([]),
    })),
  };
});

vi.mock("../../src/util/rpc-resolver.js", () => ({
  resolveRpcUrl: vi.fn(),
  getViemChain: vi.fn(),
}));

import { handleBalance } from "../../src/commands/balance.js";

describe("handleBalance", () => {
  beforeEach(() => vi.clearAllMocks());

  it("returns BTC balance for bitcoin chain", async () => {
    mockGetBalance.mockResolvedValueOnce({ confirmed: 50000, unconfirmed: 0 });
    mockGetMaxSpendable.mockResolvedValueOnce({ amount: { amount: "45000" } });

    const result = await handleBalance("bc1qtest", { chain: "bitcoin" });
    expect(result.bitcoin).toBeDefined();
    expect(result.bitcoin.address).toBe("bc1qtest");
  });
});
```

```typescript
// gateway-cli/tests/commands/routes.test.ts
import { describe, it, expect, vi, beforeEach } from "vitest";
import type { EnrichedRoute } from "../../src/util/route-provider.js";

const enrichedRoutes: EnrichedRoute[] = [
  {
    srcChain: "bitcoin", dstChain: "base",
    srcToken: { address: "BTC", symbol: "BTC", decimals: 8, chain: "bitcoin" },
    dstToken: { address: "0xUSDC", symbol: "USDC", decimals: 6, chain: "base" },
  },
  {
    srcChain: "ethereum", dstChain: "bob",
    srcToken: { address: "0xWBTC", symbol: "WBTC", decimals: 8, chain: "ethereum" },
    dstToken: { address: "0xWBTC2", symbol: "WBTC", decimals: 8, chain: "bob" },
  },
];

vi.mock("../../src/util/route-provider.js", () => ({
  getEnrichedRoutes: vi.fn().mockResolvedValue(enrichedRoutes),
}));

import { handleRoutes } from "../../src/commands/routes.js";

describe("handleRoutes", () => {
  beforeEach(() => vi.clearAllMocks());

  it("returns all routes by default", async () => {
    const result = await handleRoutes({});
    expect(result.type).toBe("routes");
    expect(result.data).toHaveLength(2);
  });

  it("lists chains with --chains", async () => {
    const result = await handleRoutes({ chains: true });
    expect(result.type).toBe("chains");
    expect(result.data.map((c: any) => c.canonical)).toContain("bitcoin");
  });

  it("lists tokens for a chain", async () => {
    const result = await handleRoutes({ tokens: "base" });
    expect(result.type).toBe("tokens");
    expect(result.data[0].symbol).toBe("USDC");
  });

  it("filters by source chain", async () => {
    const result = await handleRoutes({ from: "bitcoin" });
    expect(result.type).toBe("routes");
    expect(result.data).toHaveLength(1);
  });
});
```

```typescript
// gateway-cli/tests/commands/status.test.ts
import { describe, it, expect, vi } from "vitest";

const mockGetOrder = vi.fn();
vi.mock("../../src/config.js", () => ({
  getSdk: vi.fn(() => ({ api: { getOrder: mockGetOrder } })),
}));

import { handleStatus } from "../../src/commands/status.js";

describe("handleStatus", () => {
  it("calls sdk.api.getOrder with orderId", async () => {
    mockGetOrder.mockResolvedValueOnce({ id: "abc", status: "success" });
    const result = await handleStatus({ orderId: "abc" });
    expect(mockGetOrder).toHaveBeenCalledWith({ id: "abc" });
    expect(result.status).toBe("success");
  });
});
```

```typescript
// gateway-cli/tests/commands/orders.test.ts
import { describe, it, expect, vi } from "vitest";

const mockGetOrders = vi.fn();
vi.mock("../../src/config.js", () => ({
  getSdk: vi.fn(() => ({ getOrders: mockGetOrders })),
}));

import { handleOrders } from "../../src/commands/orders.js";

describe("handleOrders", () => {
  it("calls sdk.getOrders with address", async () => {
    mockGetOrders.mockResolvedValueOnce([{ id: "1" }]);
    const result = await handleOrders({ address: "0xABC" });
    expect(mockGetOrders).toHaveBeenCalledWith("0xABC");
    expect(result).toHaveLength(1);
  });
});
```

```typescript
// gateway-cli/tests/commands/register.test.ts
import { describe, it, expect, vi } from "vitest";

const mockRegisterTx = vi.fn();
vi.mock("../../src/config.js", () => ({
  getSdk: vi.fn(() => ({ api: { registerTx: mockRegisterTx } })),
}));

import { handleRegister } from "../../src/commands/register.js";

describe("handleRegister", () => {
  it("registers EVM tx with offramp payload", async () => {
    mockRegisterTx.mockResolvedValueOnce({ ok: true });
    await handleRegister({ orderId: "abc", txid: "0xdef" });
    expect(mockRegisterTx).toHaveBeenCalledWith({
      registerTx: { offramp: { orderId: "abc", evmTxhash: "0xdef" } },
    });
  });

  it("registers BTC tx with onramp payload", async () => {
    mockRegisterTx.mockResolvedValueOnce({ ok: true });
    await handleRegister({ orderId: "abc", txid: "txid123" });
    expect(mockRegisterTx).toHaveBeenCalledWith({
      registerTx: { onramp: { orderId: "abc", bitcoinTxid: "txid123" } },
    });
  });
});
```

- [ ] **Step 3: Run command tests**

```bash
cd gateway-cli && npx vitest run tests/commands/
```

Expected: All pass.

- [ ] **Step 4: Commit**

```bash
git add gateway-cli/tests/commands/
git commit -m "test: rewrite command tests for simplified CLI"
```

---

### Task 7: Write swap tests

**Files:**
- Create: `gateway-cli/tests/commands/swap.test.ts`

- [ ] **Step 1: Write swap tests**

```typescript
// gateway-cli/tests/commands/swap.test.ts
import { describe, it, expect, vi, beforeEach } from "vitest";

// ─── SDK + config mocks ─────────────────────────────────────────────────────

const mockGetQuote = vi.fn();
const mockGetRoutes = vi.fn();
const mockGetOrder = vi.fn();
const mockCreateOrder = vi.fn();
const mockRegisterTx = vi.fn();

vi.mock("../../src/config.js", () => ({
  loadConfig: vi.fn(() => ({
    slippageBps: 300,
    timeoutMs: 5000,
    bitcoinPrivateKey: undefined,
    evmPrivateKey: undefined,
  })),
  getSdk: vi.fn(() => ({
    getQuote: mockGetQuote,
    getRoutes: mockGetRoutes,
    getOrder: mockGetOrder,
    api: { createOrder: mockCreateOrder, registerTx: mockRegisterTx },
  })),
  BTC_DECIMALS: 8,
}));

// ─── Route provider mock ────────────────────────────────────────────────────

import type { EnrichedRoute } from "../../src/util/route-provider.js";

const enrichedRoutes: EnrichedRoute[] = [
  {
    srcChain: "bitcoin", dstChain: "base",
    srcToken: { address: "0x0000000000000000000000000000000000000000", symbol: "BTC", decimals: 8, chain: "bitcoin" },
    dstToken: { address: "0xUSDC", symbol: "USDC", decimals: 6, chain: "base" },
  },
];

vi.mock("../../src/util/route-provider.js", () => ({
  getEnrichedRoutes: vi.fn().mockResolvedValue(enrichedRoutes),
  getNativeToken: vi.fn().mockReturnValue({ symbol: "ETH", decimals: 18 }),
}));

// ─── RPC resolver mock ──────────────────────────────────────────────────────

vi.mock("../../src/util/rpc-resolver.js", () => ({
  resolveRpcUrl: vi.fn(),
  getViemChain: vi.fn(),
}));

// ─── BTC signer mock ───────────────────────────────────────────────────────

vi.mock("@gobob/bob-sdk", async (importOriginal) => {
  const actual = await importOriginal<any>();
  return {
    ...actual,
    ScureBitcoinSigner: {
      fromKey: vi.fn(() => ({
        signAllInputs: vi.fn().mockResolvedValue("signed-tx-hex"),
        getP2WPKHAddress: vi.fn().mockResolvedValue("bc1qtest"),
      })),
    },
    MempoolClient: vi.fn().mockImplementation(() => ({
      getRecommendedFees: vi.fn().mockResolvedValue({ fastestFee: 10 }),
      getAddressMempoolTxs: vi.fn().mockResolvedValue([]),
    })),
  };
});

// ─── p-retry mock (no real delays) ──────────────────────────────────────────

vi.mock("p-retry", () => ({
  default: vi.fn(async (fn: any, opts: any) => {
    let lastError: any;
    const retries = opts?.retries ?? 3;
    for (let i = 1; i <= retries + 1; i++) {
      try { return await fn(i); } catch (e: any) {
        if (e?.name === "AbortError") throw e.originalError ?? new Error(e.message);
        lastError = e;
      }
    }
    throw lastError;
  }),
  AbortError: class AbortError extends Error {
    readonly name = "AbortError";
    readonly originalError?: Error;
    constructor(msg: string) { super(msg); }
  },
}));

import { handleSwap } from "../../src/commands/swap.js";
import type { Logger } from "../../src/output.js";

// ─── Fixtures ───────────────────────────────────────────────────────────────

const onrampQuote = {
  onramp: {
    inputAmount: { amount: "5000000", address: "BTC", chain: "bitcoin" },
    outputAmount: { amount: "4812300000", address: "0xUSDC", chain: "base" },
    sender: "bc1qtest",
  },
};

const onrampOrder = {
  onramp: { orderId: "order-456", address: "bc1qgateway", psbtHex: "70736274ff" },
};

const confirmedOrder = {
  id: "order-456", status: "success",
  dstInfo: { amount: "4812300000" },
};

const silentLogger: Logger = { progress: () => {}, warn: () => {} };

const baseOpts = {
  src: "BTC", dst: "USDC:base", amount: "0.05",
  recipient: "0xABC",
  unsigned: false, wait: true, retry: true,
  privateKey: "cTestPrivateKey",
};

describe("handleSwap", () => {
  beforeEach(() => vi.clearAllMocks());

  it("executes full onramp BTC flow", async () => {
    mockGetQuote.mockResolvedValue(onrampQuote);
    mockCreateOrder.mockResolvedValueOnce(onrampOrder);
    mockRegisterTx.mockResolvedValueOnce({ onramp: { txid: "txid-123" } });
    mockGetOrder.mockResolvedValueOnce(confirmedOrder);

    const result = await handleSwap(baseOpts, silentLogger);
    expect(result.type).toBe("confirmed");
    if (result.type === "confirmed") {
      expect(result.data.orderId).toBe("order-456");
    }
  });

  it("returns submitted with --no-wait", async () => {
    mockGetQuote.mockResolvedValue(onrampQuote);
    mockCreateOrder.mockResolvedValueOnce(onrampOrder);
    mockRegisterTx.mockResolvedValueOnce({ onramp: { txid: "txid-123" } });

    const result = await handleSwap({ ...baseOpts, wait: false }, silentLogger);
    expect(result.type).toBe("submitted");
    expect(mockGetOrder).not.toHaveBeenCalled();
  });

  it("returns unsigned PSBT with --unsigned", async () => {
    mockGetQuote.mockResolvedValue(onrampQuote);
    mockCreateOrder.mockResolvedValueOnce(onrampOrder);

    const result = await handleSwap({ ...baseOpts, unsigned: true, privateKey: undefined }, silentLogger);
    expect(result.type).toBe("unsigned");
    if (result.type === "unsigned") {
      expect(result.orderId).toBe("order-456");
      expect(result.psbtBase64).toBe(Buffer.from("70736274ff", "hex").toString("base64"));
    }
  });

  it("throws when no BTC signer configured for signed flow", async () => {
    await expect(
      handleSwap({ ...baseOpts, privateKey: undefined }, silentLogger),
    ).rejects.toThrow("no signer configured for Bitcoin");
  });

  it("throws RegistrationError on registration failure", async () => {
    mockGetQuote.mockResolvedValue(onrampQuote);
    mockCreateOrder.mockResolvedValueOnce(onrampOrder);
    mockRegisterTx.mockRejectedValue(new Error("registration failed"));

    await expect(
      handleSwap(baseOpts, silentLogger),
    ).rejects.toThrow("CRITICAL");
  });

  it("retries transient errors and aborts on non-transient", async () => {
    mockGetQuote.mockResolvedValue(onrampQuote);
    // First createOrder fails with transient, second succeeds
    mockCreateOrder
      .mockRejectedValueOnce(new Error("TRM screening delay"))
      .mockResolvedValueOnce(onrampOrder);
    mockRegisterTx.mockResolvedValueOnce({ onramp: { txid: "txid-123" } });
    mockGetOrder.mockResolvedValueOnce(confirmedOrder);

    const result = await handleSwap(baseOpts, silentLogger);
    expect(result.type).toBe("confirmed");
    expect(mockCreateOrder).toHaveBeenCalledTimes(2);
  });

  it("does not retry non-transient errors", async () => {
    mockGetQuote.mockResolvedValue(onrampQuote);
    mockCreateOrder.mockRejectedValueOnce(new Error("Insufficient funds"));

    await expect(
      handleSwap(baseOpts, silentLogger),
    ).rejects.toThrow("Insufficient funds");
    expect(mockCreateOrder).toHaveBeenCalledTimes(1);
  });

  it("skips retry with --no-retry", async () => {
    mockGetQuote.mockResolvedValue(onrampQuote);
    mockCreateOrder.mockRejectedValueOnce(new Error("TRM screening delay"));

    await expect(
      handleSwap({ ...baseOpts, retry: false }, silentLogger),
    ).rejects.toThrow("TRM screening");
    expect(mockCreateOrder).toHaveBeenCalledTimes(1);
  });
});
```

- [ ] **Step 2: Run swap tests**

```bash
cd gateway-cli && npx vitest run tests/commands/swap.test.ts
```

Expected: All pass.

- [ ] **Step 3: Commit**

```bash
git add gateway-cli/tests/commands/swap.test.ts
git commit -m "test: rewrite swap tests with p-retry mocking and transient retry coverage"
```

---

### Task 8: Write e2e tests

**Files:**
- Create: `gateway-cli/tests/e2e/cli.test.ts`

- [ ] **Step 1: Write e2e smoke tests**

```typescript
// gateway-cli/tests/e2e/cli.test.ts
import { describe, it, expect } from "vitest";
import { execFileSync } from "node:child_process";
import { fileURLToPath } from "node:url";
import { dirname, resolve } from "node:path";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const CLI = resolve(__dirname, "../../bin/gateway-cli.ts");

function run(...args: string[]): string {
  return execFileSync("npx", ["tsx", CLI, ...args], { encoding: "utf-8" });
}

describe("CLI smoke tests", () => {
  it("--help shows description", () => {
    expect(run("--help")).toContain("Swap between BTC");
  });

  it("--version shows semver", () => {
    expect(run("--version")).toMatch(/\d+\.\d+\.\d+/);
  });

  it("swap --help shows flags (no --dry-run)", () => {
    const output = run("swap", "--help");
    expect(output).toContain("--unsigned");
    expect(output).toContain("--src");
    expect(output).not.toContain("--dry-run");
  });

  it("quote --help shows flags", () => {
    const output = run("quote", "--help");
    expect(output).toContain("--src");
    expect(output).toContain("--amount");
  });

  it("routes --help shows flags", () => {
    const output = run("routes", "--help");
    expect(output).toContain("--src-chain");
  });

  it("balance --help does not show --no-cache", () => {
    const output = run("balance", "--help");
    expect(output).not.toContain("--no-cache");
  });

  it("offramp --help works (hidden alias)", () => {
    const output = run("offramp", "--help");
    expect(output).toContain("--src");
  });
});
```

- [ ] **Step 2: Run e2e tests**

```bash
cd gateway-cli && npx vitest run tests/e2e/
```

Expected: All pass.

- [ ] **Step 3: Commit**

```bash
git add gateway-cli/tests/e2e/
git commit -m "test: rewrite e2e smoke tests for simplified CLI"
```

---

### Task 9: Final verification

- [ ] **Step 1: Run full test suite**

```bash
cd gateway-cli && npx vitest run
```

Expected: All tests pass.

- [ ] **Step 2: Verify TypeScript compiles cleanly**

```bash
cd gateway-cli && npx tsc --noEmit
```

Expected: No errors.

- [ ] **Step 3: Verify no stale imports or dead code**

```bash
cd gateway-cli && grep -rE "constants\.js|retry\.js|route-cache|config/index|sdk-client\.js|from.*toml|dryRun|dry-run|noCache|no-cache|config\.rpc|config\.noWait|config\.recipient|config\.sender|config\.cache" src/ --include="*.ts"
```

Expected: No matches.

- [ ] **Step 4: Count lines**

```bash
cd gateway-cli && find src -name '*.ts' | xargs wc -l | tail -1
```

Expected: ~1,350-1,400 lines (down from ~1,600).

- [ ] **Step 5: Final commit if any fixes were needed**

```bash
git add -A gateway-cli/ && git commit -m "chore: final cleanup after simplification"
```
