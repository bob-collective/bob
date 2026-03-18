# Gateway CLI SDK Migration — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Move `@gobob/gateway-cli` into the bob monorepo and rewrite its internals to use `@gobob/bob-sdk`, then add `balance` command, transient retry, and publishing workflow.

**Architecture:** Copy existing CLI source into `bob/gateway-cli/`, swap dependencies to SDK + tokenlist, then iteratively replace the custom API client and signer logic with SDK calls. Adapter layers (`enrichRoute`, `flattenQuote`) bridge SDK types to the CLI's existing output contract. New features (balance, retry) are added after the core migration.

**Tech Stack:** TypeScript, Commander.js, `@gobob/bob-sdk` (GatewaySDK, ScureBitcoinSigner, EsploraClient), `@gobob/tokenlist`, viem, ethers (keystore only), vitest.

**Spec:** `gateway-cli/docs/specs/2026-03-18-gateway-cli-sdk-migration.md`

**Source CLI repo:** `/Users/nud3l/code/bob-gateway-cli` (reference for copying)

---

## Prerequisites

Before starting this plan, the SDK needs two export additions:

**In `sdk/src/gateway/index.ts`**, add:
```typescript
export { ScureBitcoinSigner } from './utils';
export { BitcoinSigner } from './types';
```

These are already defined in the SDK but not re-exported through the barrel. Without this, the CLI cannot import `ScureBitcoinSigner` or `BitcoinSigner` from `@gobob/bob-sdk`.

---

## File Structure

### New files to create

| File | Responsibility |
|---|---|
| `src/adapter/route-enricher.ts` | Enrich SDK `RouteInfo` with token metadata from tokenlist |
| `src/adapter/quote-flattener.ts` | Flatten SDK quote union types to CLI's flat output shapes |
| `src/adapter/sdk-client.ts` | Singleton SDK client factory (initializes `GatewaySDK`) |
| `src/commands/balance.ts` | New balance command |
| `src/util/route-cache.ts` | Route caching to `~/.gateway-cli/cache/routes.json` |
| `src/util/rpc-resolver.ts` | Per-chain RPC URL resolution |
| `tests/adapter/route-enricher.test.ts` | Tests for route enricher |
| `tests/adapter/quote-flattener.test.ts` | Tests for quote flattener |
| `tests/adapter/sdk-client.test.ts` | Tests for SDK client factory |
| `tests/commands/balance.test.ts` | Tests for balance command |
| `tests/util/route-cache.test.ts` | Tests for route caching |
| `tests/util/rpc-resolver.test.ts` | Tests for RPC resolver |
| `.github/workflows/cli-npm.yml` | npm publish workflow |

### Files to modify (from copied CLI source)

| File | What changes |
|---|---|
| `package.json` | Replace deps: `@gobob/bob-sdk`, `@gobob/tokenlist`; drop `@scure/btc-signer` |
| `src/commands/chains.ts` | Use SDK + enrichRoute instead of API client |
| `src/commands/tokens.ts` | Use SDK + enrichRoute instead of API client |
| `src/commands/routes.ts` | Use SDK + route cache; warm cache on run |
| `src/commands/quote.ts` | Use SDK `getQuote()` + flattenQuote; map params to `fromChain`/`toChain` |
| `src/commands/swap.ts` | Use SDK `executeQuote()` for standard flow; keep `--unsigned` path; add `--no-retry` |
| `src/commands/status.ts` | Use SDK `getOrder()` |
| `src/commands/orders.ts` | Use SDK `getOrders()` |
| `src/commands/register.ts` | Use SDK `registerTx()` |
| `src/commands/max-spendable.ts` | Use SDK `getMaxSpendable()` |
| `src/signer/btc.ts` | Keep resolution logic; produce `BitcoinSigner` interface; use `ScureBitcoinSigner` |
| `src/signer/evm.ts` | Keep resolution logic; produce viem `WalletClient`; keep `waitForNonceClear` |
| `src/polling/poll-order.ts` | Call `sdk.getOrder()` instead of API client |
| `src/util/asset-chain-parser.ts` | Output `fromChain`/`toChain` names for SDK; use enriched routes |
| `src/util/retry.ts` | Add transient error detection + structured stderr events |
| `src/util/mempool.ts` | Keep `findPendingMempoolTx()` only; remove `fetchFeeRate()` |
| `src/config/index.ts` | Add per-chain RPC, cache TTL config |
| `src/cli.ts` | Add `balance` command; add `--no-retry` flag; exit code 6 |
| `src/output/json-shapes.ts` | Add `BalanceJson`, `RetryEventJson`; add `retryable` to `ErrorJson` |

### Files to delete (after migration)

| File | Reason |
|---|---|
| `src/api/client.ts` | Replaced by SDK |
| `src/api/types.ts` | Replaced by SDK types + adapters |
| `tests/api/client.test.ts` | No longer needed |

---

## Task 1: Scaffold Package in Monorepo

**Files:**
- Create: `bob/gateway-cli/package.json`
- Create: `bob/gateway-cli/tsconfig.json`
- Create: `bob/gateway-cli/vitest.config.ts`
- Copy: all `src/`, `tests/`, `bin/` from `bob-gateway-cli`

- [ ] **Step 1: Copy CLI source into monorepo**

```bash
cd /Users/nud3l/code/bob/gateway-cli
# Copy source, tests, bin, configs from standalone CLI
cp -r /Users/nud3l/code/bob-gateway-cli/src .
cp -r /Users/nud3l/code/bob-gateway-cli/tests .
cp -r /Users/nud3l/code/bob-gateway-cli/bin .
cp /Users/nud3l/code/bob-gateway-cli/tsconfig.json .
cp /Users/nud3l/code/bob-gateway-cli/vitest.config.ts .
```

- [ ] **Step 2: Create package.json with SDK + tokenlist deps**

```json
{
  "name": "@gobob/gateway-cli",
  "version": "0.2.0",
  "description": "CLI for bridging Bitcoin to/from EVM chains via BOB Gateway",
  "main": "dist/src/cli.js",
  "types": "dist/src/cli.d.ts",
  "bin": {
    "gateway-cli": "./bin/gateway-cli.ts"
  },
  "files": ["dist", "bin"],
  "scripts": {
    "build": "tsc",
    "test": "vitest run",
    "test:watch": "vitest",
    "cli": "tsx bin/gateway-cli.ts"
  },
  "dependencies": {
    "@gobob/bob-sdk": "file:../sdk",
    "@gobob/tokenlist": "file:../tokenlist",
    "commander": "^13.1.0",
    "ethers": "^6.16.0",
    "toml": "^3.0.0",
    "viem": "^2.31.3"
  },
  "devDependencies": {
    "tsx": "^4.19.4",
    "typescript": "^5.9.3",
    "vitest": "^3.2.4"
  }
}
```

Note: `@scure/btc-signer` is NOT listed — it comes transitively via `@gobob/bob-sdk`.

Also update `tsconfig.json` to add `"resolveJsonModule": true` (needed for importing `tokenlist.json`).

- [ ] **Step 3: Install dependencies**

```bash
cd /Users/nud3l/code/bob/gateway-cli
pnpm install
```

- [ ] **Step 4: Verify build passes**

```bash
pnpm build
```

Expected: Build will FAIL due to `import * as btc from "@scure/btc-signer"` in `src/signer/btc.ts`. This is expected — the signer rewrite in Task 8 will fix it. Skip to step 5 (tests mock the signer, so they should still pass).

- [ ] **Step 5: Verify existing tests pass**

```bash
pnpm test
```

Expected: All existing tests pass (they mock the API client, so SDK is not involved yet).

- [ ] **Step 6: Commit**

```bash
git add .
git commit -m "feat: scaffold gateway-cli package in monorepo

Copy source from standalone repo, update dependencies to use
@gobob/bob-sdk and @gobob/tokenlist as local workspace deps."
```

---

## Task 2: SDK Client Factory

**Files:**
- Create: `src/adapter/sdk-client.ts`
- Create: `tests/adapter/sdk-client.test.ts`

- [ ] **Step 1: Write failing test**

```typescript
// tests/adapter/sdk-client.test.ts
import { describe, it, expect } from 'vitest';
import { createSdkClient } from '../../src/adapter/sdk-client';

describe('createSdkClient', () => {
  it('creates a GatewaySDK instance with default URL', () => {
    const sdk = createSdkClient();
    expect(sdk).toBeDefined();
    expect(sdk.getRoutes).toBeTypeOf('function');
    expect(sdk.getQuote).toBeTypeOf('function');
  });

  it('creates a GatewaySDK instance with custom URL', () => {
    const sdk = createSdkClient('https://gateway-api-staging.gobob.xyz');
    expect(sdk).toBeDefined();
  });
});
```

- [ ] **Step 2: Run test to verify it fails**

```bash
pnpm test -- tests/adapter/sdk-client.test.ts
```

Expected: FAIL — module not found.

- [ ] **Step 3: Write implementation**

```typescript
// src/adapter/sdk-client.ts
import { GatewaySDK } from '@gobob/bob-sdk';

const DEFAULT_API_URL = 'https://gateway-api-mainnet.gobob.xyz';

let _instance: InstanceType<typeof GatewaySDK> | null = null;
let _currentUrl: string | undefined;

export function createSdkClient(apiUrl?: string): InstanceType<typeof GatewaySDK> {
  const url = apiUrl || DEFAULT_API_URL;
  if (!_instance || _currentUrl !== url) {
    _instance = new GatewaySDK(url);
    _currentUrl = url;
  }
  return _instance;
}

export function resetSdkClient(): void {
  _instance = null;
  _currentUrl = undefined;
}
```

- [ ] **Step 4: Run test to verify it passes**

```bash
pnpm test -- tests/adapter/sdk-client.test.ts
```

- [ ] **Step 5: Commit**

```bash
git add src/adapter/sdk-client.ts tests/adapter/sdk-client.test.ts
git commit -m "feat: add SDK client factory"
```

---

## Task 3: Route Enricher Adapter

**Files:**
- Create: `src/adapter/route-enricher.ts`
- Create: `tests/adapter/route-enricher.test.ts`

- [ ] **Step 1: Write failing test**

```typescript
// tests/adapter/route-enricher.test.ts
import { describe, it, expect } from 'vitest';
import { enrichRoute, type EnrichedRoute } from '../../src/adapter/route-enricher';

const mockSdkRoute = {
  srcChain: 'bitcoin',
  srcToken: 'BTC',
  dstChain: 'bob',
  dstToken: '0x03c7054bcb39f7b2e5b2c7acb37583e32d70cfa',
};

describe('enrichRoute', () => {
  it('enriches a route with token metadata from tokenlist', () => {
    const enriched = enrichRoute(mockSdkRoute);
    expect(enriched.srcToken).toEqual({
      address: 'BTC',
      symbol: 'BTC',
      decimals: 8,
      chain: 'bitcoin',
    });
    expect(enriched.dstToken.symbol).toBeDefined();
    expect(enriched.dstToken.decimals).toBeTypeOf('number');
    expect(enriched.dstToken.chain).toBe('bob');
  });

  it('returns address as symbol for unknown tokens', () => {
    const route = { ...mockSdkRoute, dstToken: '0x0000000000000000000000000000000000000099' };
    const enriched = enrichRoute(route);
    expect(enriched.dstToken.address).toBe('0x0000000000000000000000000000000000000099');
  });
});
```

- [ ] **Step 2: Run test to verify it fails**

```bash
pnpm test -- tests/adapter/route-enricher.test.ts
```

- [ ] **Step 3: Write implementation**

```typescript
// src/adapter/route-enricher.ts
import type { RouteInfo } from '@gobob/bob-sdk';
import { SUPPORTED_CHAIN_MAP } from '@gobob/tokenlist';
import tokenlistJson from '@gobob/tokenlist/tokenlist.json';

export interface EnrichedToken {
  address: string;
  symbol: string;
  decimals: number;
  chain: string;
}

export interface EnrichedRoute {
  srcChain: string;
  dstChain: string;
  srcToken: EnrichedToken;
  dstToken: EnrichedToken;
}

// SDK uses short chain slugs ("bsc", "bob") but tokenlist uses kebab-cased
// viem chain names ("bnb-smart-chain", "bob"). Map SDK names → tokenlist keys.
const SDK_TO_TOKENLIST_CHAIN: Record<string, string> = {
  bob: 'bob',
  ethereum: 'ethereum',
  base: 'base',
  arbitrum: 'arbitrum',
  optimism: 'optimism',
  bsc: 'bnb-smart-chain',
  polygon: 'polygon',
  avalanche: 'avalanche',
};

// Build chainName → chainId lookup from tokenlist's SUPPORTED_CHAIN_MAP
const chainNameToId: Record<string, number> = {};
for (const [name, chain] of Object.entries(SUPPORTED_CHAIN_MAP)) {
  chainNameToId[name] = (chain as { id: number }).id;
}

// tokenlist.json is Uniswap-format: { tokens: [...] }
const tokens = tokenlistJson.tokens as Array<{
  address: string; symbol: string; decimals: number; chainId: number;
}>;

const BTC_TOKEN: EnrichedToken = {
  address: 'BTC', symbol: 'BTC', decimals: 8, chain: 'bitcoin',
};

function resolveToken(address: string, sdkChainName: string): EnrichedToken {
  if (sdkChainName === 'bitcoin' || address === 'BTC') return BTC_TOKEN;

  const tokenlistChainName = SDK_TO_TOKENLIST_CHAIN[sdkChainName] ?? sdkChainName;
  const chainId = chainNameToId[tokenlistChainName];
  if (chainId) {
    const token = tokens.find(
      t => t.chainId === chainId && t.address.toLowerCase() === address.toLowerCase()
    );
    if (token) {
      return { address: token.address, symbol: token.symbol, decimals: token.decimals, chain: sdkChainName };
    }
  }
  // Fallback: use truncated address as symbol, assume 18 decimals
  return { address, symbol: address.slice(0, 10) + '...', decimals: 18, chain: sdkChainName };
}

export function enrichRoute(route: RouteInfo): EnrichedRoute {
  return {
    srcChain: route.srcChain,
    dstChain: route.dstChain,
    srcToken: resolveToken(route.srcToken, route.srcChain),
    dstToken: resolveToken(route.dstToken, route.dstChain),
  };
}

export function enrichRoutes(routes: RouteInfo[]): EnrichedRoute[] {
  return routes.map(enrichRoute);
}
```

- [ ] **Step 4: Run test to verify it passes**

```bash
pnpm test -- tests/adapter/route-enricher.test.ts
```

- [ ] **Step 5: Commit**

```bash
git add src/adapter/route-enricher.ts tests/adapter/route-enricher.test.ts
git commit -m "feat: add route enricher adapter for tokenlist metadata"
```

---

## Task 4: Quote Flattener Adapter

**Files:**
- Create: `src/adapter/quote-flattener.ts`
- Create: `tests/adapter/quote-flattener.test.ts`

- [ ] **Step 1: Write failing test**

```typescript
// tests/adapter/quote-flattener.test.ts
import { describe, it, expect } from 'vitest';
import { flattenQuote, type FlatQuote } from '../../src/adapter/quote-flattener';

describe('flattenQuote', () => {
  it('flattens an onramp quote', () => {
    const sdkQuote = {
      onramp: {
        inputAmount: { amount: '100000000', address: 'BTC', chain: 'bitcoin' },
        outputAmount: { amount: '99000000', address: '0xabc', chain: 'bob' },
        fees: { amount: '1000000', address: 'BTC', chain: 'bitcoin' },
        slippage: '300',
        estimatedTimeInSecs: 600,
      },
    };
    const flat = flattenQuote(sdkQuote as any);
    expect(flat.inputAmount).toBe('100000000');
    expect(flat.outputAmount).toBe('99000000');
    expect(flat.fees).toBe('1000000');
    expect(flat.slippage).toBe('300');
    expect(flat.estimatedTime).toBe(600);
    expect(flat.variant).toBe('onramp');
  });

  it('flattens an offramp quote (converts slippage number to string)', () => {
    const sdkQuote = {
      offramp: {
        inputAmount: { amount: '1000000', address: '0xabc', chain: 'bob' },
        outputAmount: { amount: '99000', address: 'BTC', chain: 'bitcoin' },
        fees: { amount: '1000', address: '0xabc', chain: 'bob' },
        slippage: 300,
        estimatedTimeInSecs: 900,
      },
    };
    const flat = flattenQuote(sdkQuote as any);
    expect(flat.slippage).toBe('300');
    expect(flat.variant).toBe('offramp');
  });

  it('flattens a layerZero quote (no slippage)', () => {
    const sdkQuote = {
      layerZero: {
        inputAmount: { amount: '500000', address: '0xabc', chain: 'ethereum' },
        outputAmount: { amount: '490000', address: '0xdef', chain: 'bob' },
        fees: { amount: '10000', address: '0xabc', chain: 'ethereum' },
        estimatedTimeInSecs: 300,
      },
    };
    const flat = flattenQuote(sdkQuote as any);
    expect(flat.slippage).toBeUndefined();
    expect(flat.variant).toBe('layerZero');
  });
});
```

- [ ] **Step 2: Run test to verify it fails**

```bash
pnpm test -- tests/adapter/quote-flattener.test.ts
```

- [ ] **Step 3: Write implementation**

```typescript
// src/adapter/quote-flattener.ts
import type { GatewayQuote } from '@gobob/bob-sdk';

export type QuoteVariant = 'onramp' | 'offramp' | 'layerZero';

export interface FlatQuote {
  variant: QuoteVariant;
  inputAmount: string;
  outputAmount: string;
  fees: string;
  slippage?: string;
  estimatedTime?: number;
  raw: any; // original SDK quote for fields not in flat shape
}

export function detectVariant(quote: GatewayQuote): QuoteVariant {
  if ('onramp' in quote && quote.onramp) return 'onramp';
  if ('offramp' in quote && quote.offramp) return 'offramp';
  if ('layerZero' in quote && quote.layerZero) return 'layerZero';
  throw new Error('Unknown quote variant');
}

export function flattenQuote(quote: GatewayQuote): FlatQuote {
  const variant = detectVariant(quote);
  const inner = (quote as any)[variant];

  const flat: FlatQuote = {
    variant,
    inputAmount: inner.inputAmount.amount,
    outputAmount: inner.outputAmount.amount,
    fees: inner.fees.amount,
    raw: quote,
  };

  if (variant !== 'layerZero' && inner.slippage !== undefined) {
    flat.slippage = String(inner.slippage);
  }

  if (inner.estimatedTimeInSecs !== undefined) {
    flat.estimatedTime = inner.estimatedTimeInSecs;
  }

  return flat;
}
```

- [ ] **Step 4: Run test to verify it passes**

```bash
pnpm test -- tests/adapter/quote-flattener.test.ts
```

- [ ] **Step 5: Commit**

```bash
git add src/adapter/quote-flattener.ts tests/adapter/quote-flattener.test.ts
git commit -m "feat: add quote flattener adapter for SDK union types"
```

---

## Task 5: Per-Chain RPC Resolver

**Files:**
- Create: `src/util/rpc-resolver.ts`
- Create: `tests/util/rpc-resolver.test.ts`
- Modify: `src/config/index.ts`

- [ ] **Step 1: Write failing test**

```typescript
// tests/util/rpc-resolver.test.ts
import { describe, it, expect, beforeEach, afterEach } from 'vitest';
import { resolveRpcUrl } from '../../src/util/rpc-resolver';

describe('resolveRpcUrl', () => {
  const originalEnv = process.env;

  beforeEach(() => { process.env = { ...originalEnv }; });
  afterEach(() => { process.env = originalEnv; });

  it('resolves per-chain env var first', () => {
    process.env.EVM_RPC_URL_BOB = 'https://custom-bob-rpc.com';
    process.env.EVM_RPC_URL = 'https://default-rpc.com';
    expect(resolveRpcUrl('bob', {})).toBe('https://custom-bob-rpc.com');
  });

  it('falls back to config.toml rpc section', () => {
    const tomlRpc = { bob: 'https://toml-bob-rpc.com' };
    expect(resolveRpcUrl('bob', tomlRpc)).toBe('https://toml-bob-rpc.com');
  });

  it('falls back to EVM_RPC_URL', () => {
    process.env.EVM_RPC_URL = 'https://default-rpc.com';
    expect(resolveRpcUrl('bob', {})).toBe('https://default-rpc.com');
  });

  it('falls back to built-in public RPCs', () => {
    const url = resolveRpcUrl('bob', {});
    expect(url).toBeDefined();
    expect(url).toContain('http');
  });
});
```

- [ ] **Step 2: Run test to verify it fails**

```bash
pnpm test -- tests/util/rpc-resolver.test.ts
```

- [ ] **Step 3: Write implementation**

```typescript
// src/util/rpc-resolver.ts

const PUBLIC_RPCS: Record<string, string> = {
  bob: 'https://rpc.gobob.xyz',
  ethereum: 'https://eth.llamarpc.com',
  base: 'https://mainnet.base.org',
  arbitrum: 'https://arb1.arbitrum.io/rpc',
  optimism: 'https://mainnet.optimism.io',
  bsc: 'https://bsc-dataseed.binance.org',
};

export function resolveRpcUrl(
  chainName: string,
  tomlRpc: Record<string, string>,
): string {
  // 1. Per-chain env var
  const envKey = `EVM_RPC_URL_${chainName.toUpperCase()}`;
  const perChainEnv = process.env[envKey];
  if (perChainEnv) return perChainEnv;

  // 2. TOML config
  const tomlUrl = tomlRpc[chainName];
  if (tomlUrl) return tomlUrl;

  // 3. Fallback env var
  const defaultEnv = process.env.EVM_RPC_URL;
  if (defaultEnv) return defaultEnv;

  // 4. Built-in public RPCs
  const publicRpc = PUBLIC_RPCS[chainName];
  if (publicRpc) return publicRpc;

  throw new Error(`No RPC URL configured for chain "${chainName}". Set EVM_RPC_URL_${chainName.toUpperCase()} or add [rpc] ${chainName} = "..." to config.toml`);
}
```

- [ ] **Step 4: Run test to verify it passes**

```bash
pnpm test -- tests/util/rpc-resolver.test.ts
```

- [ ] **Step 5: Update config/index.ts to parse [rpc] and [cache] TOML sections**

Add `rpc` and `cache` fields to the `Config` interface in `src/config/index.ts`. Parse `[rpc]` as `Record<string, string>` and `[cache]` with a `ttl` string field (default `"24h"`).

- [ ] **Step 6: Run all config tests**

```bash
pnpm test -- tests/config/
```

- [ ] **Step 7: Commit**

```bash
git add src/util/rpc-resolver.ts tests/util/rpc-resolver.test.ts src/config/index.ts
git commit -m "feat: add per-chain RPC URL resolver and config support"
```

---

## Task 6: Route Cache

**Files:**
- Create: `src/util/route-cache.ts`
- Create: `tests/util/route-cache.test.ts`

- [ ] **Step 1: Write failing test**

```typescript
// tests/util/route-cache.test.ts
import { describe, it, expect, vi, beforeEach } from 'vitest';
import { readCache, writeCache, isCacheValid, parseTtl } from '../../src/util/route-cache';
import * as fs from 'fs';

vi.mock('fs');

describe('route-cache', () => {
  describe('parseTtl', () => {
    it('parses hours', () => expect(parseTtl('24h')).toBe(24 * 60 * 60 * 1000));
    it('parses days', () => expect(parseTtl('7d')).toBe(7 * 24 * 60 * 60 * 1000));
    it('defaults to 24h on invalid', () => expect(parseTtl('invalid')).toBe(24 * 60 * 60 * 1000));
  });

  describe('isCacheValid', () => {
    it('returns false when cache does not exist', () => {
      vi.mocked(fs.existsSync).mockReturnValue(false);
      expect(isCacheValid('24h')).toBe(false);
    });

    it('returns true when cache is fresh', () => {
      vi.mocked(fs.existsSync).mockReturnValue(true);
      vi.mocked(fs.readFileSync).mockReturnValue(
        JSON.stringify({ fetchedAt: new Date().toISOString(), routes: [] })
      );
      expect(isCacheValid('24h')).toBe(true);
    });
  });
});
```

- [ ] **Step 2: Run test to verify it fails**

```bash
pnpm test -- tests/util/route-cache.test.ts
```

- [ ] **Step 3: Write implementation**

```typescript
// src/util/route-cache.ts
import * as fs from 'fs';
import * as path from 'path';
import * as os from 'os';
import type { RouteInfo } from '@gobob/bob-sdk';

const CACHE_DIR = path.join(os.homedir(), '.gateway-cli', 'cache');
const CACHE_FILE = path.join(CACHE_DIR, 'routes.json');
const DEFAULT_TTL = '24h';

interface CacheEntry {
  fetchedAt: string; // ISO 8601
  routes: RouteInfo[];
}

export function parseTtl(ttl: string): number {
  const match = ttl.match(/^(\d+)(h|d)$/);
  if (!match) return 24 * 60 * 60 * 1000; // default 24h
  const [, num, unit] = match;
  const ms = unit === 'h' ? Number(num) * 3600_000 : Number(num) * 86400_000;
  return ms;
}

export function isCacheValid(ttl: string = DEFAULT_TTL): boolean {
  if (!fs.existsSync(CACHE_FILE)) return false;
  try {
    const data: CacheEntry = JSON.parse(fs.readFileSync(CACHE_FILE, 'utf-8'));
    const age = Date.now() - new Date(data.fetchedAt).getTime();
    return age < parseTtl(ttl);
  } catch { return false; }
}

export function readCache(): RouteInfo[] | null {
  if (!fs.existsSync(CACHE_FILE)) return null;
  try {
    const data: CacheEntry = JSON.parse(fs.readFileSync(CACHE_FILE, 'utf-8'));
    return data.routes;
  } catch { return null; }
}

export function writeCache(routes: RouteInfo[]): void {
  fs.mkdirSync(CACHE_DIR, { recursive: true });
  const entry: CacheEntry = { fetchedAt: new Date().toISOString(), routes };
  fs.writeFileSync(CACHE_FILE, JSON.stringify(entry));
}

export async function getOrFetchRoutes(
  fetchRoutes: () => Promise<RouteInfo[]>,
  ttl: string = DEFAULT_TTL,
  noCache: boolean = false,
): Promise<RouteInfo[]> {
  if (!noCache && isCacheValid(ttl)) {
    const cached = readCache();
    if (cached) return cached;
  }
  const routes = await fetchRoutes();
  writeCache(routes);
  return routes;
}
```

- [ ] **Step 4: Run test to verify it passes**

```bash
pnpm test -- tests/util/route-cache.test.ts
```

- [ ] **Step 5: Commit**

Also add `--no-cache` flag to any command that uses routes (balance, routes, chains, tokens). In `src/cli.ts`, add `.option('--no-cache', 'skip route cache')` to these commands. Thread the `noCache` option through to `getOrFetchRoutes()`.

```bash
git add src/util/route-cache.ts tests/util/route-cache.test.ts
git commit -m "feat: add route caching with configurable TTL and --no-cache"
```

---

## Task 7: Replace API Client in Discovery Commands

**Files:**
- Modify: `src/commands/chains.ts`, `tokens.ts`, `routes.ts`, `status.ts`, `orders.ts`, `max-spendable.ts`, `register.ts`
- Modify: corresponding test files in `tests/commands/`

- [ ] **Step 1: Update chains.ts to use SDK + enrichRoute**

Replace `GatewayApiClient` import with `createSdkClient` + `enrichRoutes`:

```typescript
// src/commands/chains.ts — key changes
import { createSdkClient } from '../adapter/sdk-client';
import { enrichRoutes } from '../adapter/route-enricher';
import { getOrFetchRoutes } from '../util/route-cache';

export async function handleChains(opts: { json: boolean; noCache?: boolean }) {
  const sdk = createSdkClient(config.apiUrl);
  const routes = await getOrFetchRoutes(() => sdk.getRoutes(), config.cache?.ttl, opts.noCache);
  const enriched = enrichRoutes(routes);
  // ... extract unique chains from enriched routes (same logic as before)
}
```

- [ ] **Step 2: Update chains.test.ts**

Mock `@gobob/bob-sdk` module's `GatewaySDK` instead of mocking fetch. Provide SDK-shaped route objects.

- [ ] **Step 3: Run chains tests**

```bash
pnpm test -- tests/commands/chains.test.ts
```

- [ ] **Step 4: Repeat for tokens.ts, routes.ts**

Same pattern: replace API client with SDK + enrichRoutes. Routes command should also call `writeCache()` to warm the route cache.

- [ ] **Step 5: Run tokens and routes tests**

```bash
pnpm test -- tests/commands/tokens.test.ts tests/commands/routes.test.ts
```

- [ ] **Step 6: Update status.ts and orders.ts**

```typescript
// src/commands/status.ts
import { createSdkClient } from '../adapter/sdk-client';

export async function handleStatus(orderId: string, opts: { json: boolean }) {
  const sdk = createSdkClient(config.apiUrl);
  const order = await sdk.getOrder(orderId);
  // ... format output (same as before, types are compatible)
}
```

Same pattern for `orders.ts` using `sdk.getOrders(address)`.

- [ ] **Step 7: Update register.ts**

The SDK's high-level `GatewaySDK` class does not expose `registerTx()`. Access it through the generated API client:

```typescript
// src/commands/register.ts
import { V1Api, Configuration } from '@gobob/bob-sdk';

export async function handleRegister(orderId: string, txId: string, opts: { json: boolean }) {
  const apiConfig = new Configuration({ basePath: config.apiUrl });
  const api = new V1Api(apiConfig);

  const isEvmTx = txId.startsWith('0x');
  const registerTx = isEvmTx
    ? { offramp: { orderId, evmTxhash: txId } }
    : { onramp: { orderId, bitcoinTxHex: txId } };

  const result = await api.registerTx({ registerTx });
  // ... format output
}
```

- [ ] **Step 8: Update max-spendable.ts**

Replace `client.getMaxSpendable()` with `sdk.getMaxSpendable()`.

- [ ] **Step 9: Run all discovery command tests**

```bash
pnpm test -- tests/commands/
```

- [ ] **Step 10: Commit**

```bash
git add src/commands/ tests/commands/
git commit -m "refactor: replace API client with SDK in discovery commands"
```

---

## Task 8: Simplify BTC Signer

**Files:**
- Modify: `src/signer/btc.ts`
- Modify: `tests/signer/btc-private-key.test.ts`

- [ ] **Step 1: Rewrite btc.ts to produce `BitcoinSigner` interface**

Keep the layered resolution logic (`resolveBtcSigner`). Replace `signPsbtWithPrivateKey()` with `new ScureBitcoinSigner(privateKeyHex)` from SDK. The `ExternalSigner` class adapts to `BitcoinSigner.signAllInputs`. The `--unsigned` path returns a spec indicating no signer.

Key change: `resolveBtcSigner()` now returns `{ signer: BitcoinSigner } | { unsigned: true }` instead of `BtcSignerSpec`.

Import `ScureBitcoinSigner` from `@gobob/bob-sdk` and `BitcoinSigner` type from `@gobob/bob-sdk`.

- [ ] **Step 2: Update btc signer tests**

Update `btc-private-key.test.ts` to test that `resolveBtcSigner` produces a `BitcoinSigner` with `signAllInputs` method. Mock `ScureBitcoinSigner` from SDK.

- [ ] **Step 3: Run signer tests**

```bash
pnpm test -- tests/signer/
```

- [ ] **Step 4: Commit**

```bash
git add src/signer/btc.ts tests/signer/
git commit -m "refactor: simplify BTC signer to produce SDK BitcoinSigner"
```

---

## Task 9: Simplify EVM Signer

**Files:**
- Modify: `src/signer/evm.ts`
- Modify: `tests/signer/evm-private-key.test.ts`, `evm-keystore.test.ts`

- [ ] **Step 1: Rewrite evm.ts to produce viem `WalletClient` + `PublicClient`**

Keep layered resolution (`resolveEvmSigner`). Keep `decryptKeystore()` (ethers dependency). Keep `waitForNonceClear()`. Remove `signAndBroadcastEvm()` — the SDK's `executeQuote()` handles this. The `ExternalSigner` path produces a custom viem account.

`resolveEvmSigner()` now returns `{ walletClient, publicClient }` or `{ unsigned: true }`.

- [ ] **Step 2: Update EVM signer tests**

- [ ] **Step 3: Run signer tests**

```bash
pnpm test -- tests/signer/
```

- [ ] **Step 4: Commit**

```bash
git add src/signer/evm.ts tests/signer/
git commit -m "refactor: simplify EVM signer to produce viem WalletClient"
```

---

## Task 10: Replace Quote Command

**Files:**
- Modify: `src/commands/quote.ts`
- Modify: `src/util/asset-chain-parser.ts`
- Modify: `tests/commands/quote.test.ts`

- [ ] **Step 1: Update asset-chain-parser.ts**

Change output field names from `srcChain`/`dstChain` to `fromChain`/`toChain` to match SDK's `GetQuoteParams`. Use enriched routes from SDK (via route cache) instead of the old API types.

- [ ] **Step 2: Update asset-chain-parser tests**

```bash
pnpm test -- tests/util/asset-chain-parser.test.ts
```

- [ ] **Step 3: Rewrite quote.ts**

Replace `client.getQuote()` with `sdk.getQuote()`. Key parameter mapping:

```typescript
const quoteParams: GetQuoteParams = {
  fromChain: src.chain,       // was srcChain
  toChain: dst.chain,         // was dstChain
  fromToken: src.address,     // was srcToken
  toToken: dst.address,       // was dstToken
  toUserAddress: opts.recipient,     // was recipient
  fromUserAddress: opts.sender,      // was sender (optional)
  amount: parsedAmount.atomic,       // atomic units as string
  maxSlippage: opts.slippage,
};
const quote = await sdk.getQuote(quoteParams);
const flat = flattenQuote(quote);
```

Use `flattenQuote()` to map SDK union type to flat output shape for JSON/human display.

- [ ] **Step 4: Update quote tests**

Mock SDK instead of API client. Use SDK-shaped quote response.

- [ ] **Step 5: Run quote tests**

```bash
pnpm test -- tests/commands/quote.test.ts
```

- [ ] **Step 6: Commit**

```bash
git add src/commands/quote.ts src/util/asset-chain-parser.ts tests/
git commit -m "refactor: replace API client with SDK in quote command"
```

---

## Task 11: Rewrite Swap Command

This is the largest task — the swap command is 327 lines and orchestrates quoting, signing, order creation, registration, and polling.

**Files:**
- Modify: `src/commands/swap.ts`
- Modify: `src/polling/poll-order.ts`
- Modify: `src/util/mempool.ts`
- Modify: `tests/commands/swap.test.ts`

- [ ] **Step 1: Rewrite poll-order.ts to use SDK**

Replace `client.getOrder()` call with `sdk.getOrder()`. Keep timeout logic, terminal state detection, callbacks. Import `GatewaySDK` type.

- [ ] **Step 2: Run poll-order tests**

```bash
pnpm test -- tests/polling/poll-order.test.ts
```

- [ ] **Step 3: Strip `fetchFeeRate()` from mempool.ts**

Remove the fee rate function (SDK handles this). Keep `findPendingMempoolTx()`.

- [ ] **Step 4: Rewrite swap.ts — standard flow**

```typescript
// src/commands/swap.ts — core flow (simplified)
import { createSdkClient } from '../adapter/sdk-client';
import { flattenQuote } from '../adapter/quote-flattener';
import type { BitcoinSigner } from '@gobob/bob-sdk';

export async function handleSwap(opts: SwapOptions) {
  const sdk = createSdkClient(config.apiUrl);

  // 1. Resolve signers
  const btcSigner = isBtcSource ? await resolveBtcSigner(opts) : undefined;
  const { walletClient, publicClient } = isEvmSource ? await resolveEvmSigner(opts) : {};

  // 2. Nonce clearing (EVM offramps)
  if (publicClient && walletClient) {
    await waitForNonceClear(publicClient, walletClient.account.address);
  }

  // 3. Get quote (same params as Task 10)
  const quote = await sdk.getQuote(quoteParams);
  const flat = flattenQuote(quote);

  // 4. Show confirmation
  if (!opts.json) await showConfirmation(flat);

  // 5. Execute — SDK handles order creation + signing + registration
  const result = await sdk.executeQuote({
    quote,
    walletClient,
    publicClient,
    btcSigner: btcSigner?.signer as BitcoinSigner,
  });

  // 6. Poll for completion
  const orderId = result.order.orderId ?? extractOrderId(result.order);
  const finalOrder = await pollOrder(sdk, orderId, opts.timeout);

  // 7. Format output
  return formatSwapResult(finalOrder, flat, result.tx);
}
```

- [ ] **Step 5: Rewrite swap.ts — unsigned flow**

```typescript
// --unsigned path: create order without signing
if (opts.unsigned) {
  const apiConfig = new Configuration({ basePath: config.apiUrl });
  const api = new V1Api(apiConfig);
  const order = await api.createOrder({ gatewayQuote: quote });

  // Onramp: output PSBT
  if ('onramp' in order && order.onramp?.psbtHex) {
    // Convert hex PSBT to base64 for user consumption
    const psbtBase64 = Buffer.from(order.onramp.psbtHex, 'hex').toString('base64');
    return formatUnsignedOutput({ psbt: psbtBase64, orderId: order.onramp.orderId });
  }
  // Offramp/LayerZero: output tx info
  if ('offramp' in order && order.offramp?.tx) {
    return formatUnsignedOutput({ tx: order.offramp.tx, orderId: order.offramp.orderId });
  }
}
```

- [ ] **Step 6: Keep nonce clearing and mempool fallback**

`waitForNonceClear()` runs before `executeQuote()` for EVM offramps. Mempool fallback runs after poll timeout for BTC onramps.

- [ ] **Step 7: Update swap tests**

Mock `GatewaySDK` methods. Test both standard and unsigned flows. Test error paths (registration failure, poll timeout).

- [ ] **Step 8: Verify offramp.ts alias still works**

`src/commands/offramp.ts` re-exports from `swap.ts`. After the rewrite, confirm the re-export still resolves (same export names: `handleSwap`, `RegistrationError`).

- [ ] **Step 9: Run all swap tests (including offramp)**

```bash
pnpm test -- tests/commands/swap.test.ts tests/commands/offramp.test.ts
```

- [ ] **Step 10: Commit**

```bash
git add src/commands/swap.ts src/polling/poll-order.ts src/util/mempool.ts tests/
git commit -m "refactor: rewrite swap command to use SDK executeQuote"
```

---

## Task 12: Delete Old API Client

**Files:**
- Delete: `src/api/client.ts`
- Delete: `src/api/types.ts`
- Delete: `tests/api/client.test.ts`

- [ ] **Step 1: Search for any remaining imports of `src/api/`**

```bash
grep -r "from.*['\"].*api/client" src/ tests/
grep -r "from.*['\"].*api/types" src/ tests/
```

Expected: No results (all imports replaced in previous tasks).

- [ ] **Step 2: Delete files**

```bash
rm src/api/client.ts src/api/types.ts tests/api/client.test.ts
rmdir src/api tests/api 2>/dev/null || true
```

- [ ] **Step 3: Run full test suite**

```bash
pnpm test
```

Expected: All tests pass.

- [ ] **Step 4: Commit**

```bash
git add -A
git commit -m "refactor: remove old API client (replaced by SDK)"
```

---

## Task 13: Transient Error Retry + `--no-retry`

**Files:**
- Modify: `src/util/retry.ts`
- Modify: `src/cli.ts`
- Modify: `src/commands/swap.ts`
- Modify: `src/output/json-shapes.ts`
- Create: `tests/util/transient-retry.test.ts`

- [ ] **Step 1: Write failing test for transient error detection**

```typescript
// tests/util/transient-retry.test.ts
import { describe, it, expect } from 'vitest';
import { isTransientError, retryWithBackoff } from '../../src/util/retry';

describe('isTransientError', () => {
  it('detects TRM screening errors', () => {
    expect(isTransientError(new Error('TRM screening delay'))).toBe(true);
  });
  it('detects rate limit errors', () => {
    expect(isTransientError(new Error('429 Too Many Requests'))).toBe(true);
  });
  it('returns false for non-transient errors', () => {
    expect(isTransientError(new Error('Insufficient funds'))).toBe(false);
  });
});
```

- [ ] **Step 2: Run test to verify it fails**

```bash
pnpm test -- tests/util/transient-retry.test.ts
```

- [ ] **Step 3: Implement transient error detection and retry**

Add to `src/util/retry.ts`:
- `isTransientError(error: Error): boolean` — checks message patterns
- Update `retryWithBackoff()` to emit structured retry events to stderr in JSON mode
- Backoff: 5 attempts, [5s, 10s, 20s, 40s, 80s]

- [ ] **Step 4: Add `--no-retry` flag to CLI**

In `src/cli.ts`, add `--no-retry` option to `addSwapOptions()`. In `withTxErrorHandling()`, add exit code 6 for `TransientError` class.

- [ ] **Step 5: Add `retryable` field to ErrorJson**

In `src/output/json-shapes.ts`, add `retryable?: boolean` to `ErrorJson`.

- [ ] **Step 6: Wire retry into swap command**

In `src/commands/swap.ts`, wrap the quote+execute flow in `retryWithBackoff()` when `--no-retry` is not set. When `--no-retry` IS set and a transient error occurs, throw `TransientError` (caught by `withTxErrorHandling` → exit code 6).

- [ ] **Step 7: Run tests**

```bash
pnpm test -- tests/util/transient-retry.test.ts tests/commands/swap.test.ts
```

- [ ] **Step 8: Commit**

```bash
git add src/util/retry.ts src/cli.ts src/commands/swap.ts src/output/json-shapes.ts tests/
git commit -m "feat: add transient error retry with --no-retry flag (exit code 6)"
```

---

## Task 14: Balance Command

**Files:**
- Create: `src/commands/balance.ts`
- Create: `tests/commands/balance.test.ts`
- Modify: `src/cli.ts`
- Modify: `src/output/json-shapes.ts`

- [ ] **Step 1: Write failing test**

```typescript
// tests/commands/balance.test.ts
import { describe, it, expect, vi } from 'vitest';
import { handleBalance } from '../../src/commands/balance';

// Mock SDK, viem, route cache
vi.mock('../../src/adapter/sdk-client');
vi.mock('../../src/util/route-cache');

describe('handleBalance', () => {
  it('returns BTC balance for bitcoin chain', async () => {
    // Mock EsploraClient.getBalance() and sdk.getMaxSpendable()
    const result = await handleBalance({
      address: 'bc1qtest...',
      chain: 'bitcoin',
      json: true,
    });
    const parsed = JSON.parse(result);
    expect(parsed.bitcoin).toBeDefined();
    expect(parsed.bitcoin.confirmed).toBeDefined();
  });

  it('returns EVM balances for a specific chain', async () => {
    // Mock viem multicall for token balances
    const result = await handleBalance({
      address: '0xtest...',
      chain: 'bob',
      json: true,
    });
    const parsed = JSON.parse(result);
    expect(parsed.bob).toBeDefined();
  });

  it('omits zero balances', async () => {
    // Mock all balances as 0
    const result = await handleBalance({
      address: '0xtest...',
      chain: 'bob',
      json: true,
    });
    const parsed = JSON.parse(result);
    expect(parsed.bob).toBeUndefined(); // omitted because all zero
  });
});
```

- [ ] **Step 2: Run test to verify it fails**

```bash
pnpm test -- tests/commands/balance.test.ts
```

- [ ] **Step 3: Implement balance command**

`src/commands/balance.ts`:
- Accept `address`, optional `--chain`, `--json`
- Use route cache (enriched routes) to determine which tokens to query per chain
- BTC: `EsploraClient.getBalance(address)` for confirmed/unconfirmed + `sdk.getMaxSpendable(address)`
- EVM: `resolveRpcUrl()` → create viem `publicClient` → `getBalance()` for native + `multicall()` for ERC20 balances
- Filter out zero balances
- Format as JSON or human-readable

- [ ] **Step 4: Add BalanceJson to json-shapes.ts**

```typescript
export interface BalanceJson {
  [chain: string]: {
    address: string;
    confirmed?: string;
    unconfirmed?: string;
    maxSpendable?: string;
    native?: { symbol: string; balance: string };
    tokens?: Array<{ symbol: string; address: string; balance: string }>;
  };
}
```

- [ ] **Step 5: Register balance command in cli.ts**

Add `program.command('balance')` with `<address>` argument, `--chain` option, `--json` flag.

- [ ] **Step 6: Run balance tests**

```bash
pnpm test -- tests/commands/balance.test.ts
```

- [ ] **Step 7: Commit**

```bash
git add src/commands/balance.ts tests/commands/balance.test.ts src/cli.ts src/output/json-shapes.ts
git commit -m "feat: add balance command with multi-chain token balances"
```

---

## Task 15: Publishing Workflow

**Files:**
- Create: `.github/workflows/cli-npm.yml`
- Modify: `.github/workflows/sdk-npm.yml` (tag filter)

- [ ] **Step 1: Create `cli-npm.yml`**

```yaml
name: Publish CLI to npm

on:
  push:
    tags:
      - 'cli-v*'

jobs:
  publish:
    runs-on: ubuntu-latest
    permissions:
      id-token: write
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: '24'
          registry-url: 'https://registry.npmjs.org'

      - name: Install tokenlist
        run: cd tokenlist && npm ci

      - name: Install SDK
        run: cd sdk && npm ci && npm run build

      - name: Install CLI
        run: cd gateway-cli && npm ci

      - name: Build CLI
        run: cd gateway-cli && npm run build

      - name: Prepare for publish
        run: |
          cd gateway-cli
          VERSION=$(node -p "require('./package.json').version")
          # Replace file: deps with npm versions
          node -e "
            const pkg = require('./package.json');
            pkg.dependencies['@gobob/bob-sdk'] = '^5.0.0';
            pkg.dependencies['@gobob/tokenlist'] = '^1.0.0';
            require('fs').writeFileSync('package.json', JSON.stringify(pkg, null, 2));
          "
          echo "VERSION=$VERSION" >> $GITHUB_ENV

      - name: Publish
        run: |
          cd gateway-cli
          if echo "$VERSION" | grep -q "rc"; then
            npm publish --access public --tag rc
          else
            npm publish --access public
          fi
```

- [ ] **Step 2: Update `sdk-npm.yml` tag filter**

Change `tags: ['*']` to `tags: ['sdk-v*']` in `.github/workflows/sdk-npm.yml`.

- [ ] **Step 3: Commit**

```bash
git add .github/workflows/cli-npm.yml .github/workflows/sdk-npm.yml
git commit -m "ci: add CLI npm publish workflow, filter SDK tags"
```

---

## Task 16: Final Cleanup + Full Test Run

- [ ] **Step 1: Remove `@scure/btc-signer` from package.json if still listed**

```bash
grep "scure/btc-signer" gateway-cli/package.json
```

If found, remove it.

- [ ] **Step 2: Run full build**

```bash
cd /Users/nud3l/code/bob/gateway-cli && pnpm build
```

- [ ] **Step 3: Run full test suite**

```bash
pnpm test
```

Expected: All tests pass.

- [ ] **Step 4: Verify CLI works end-to-end**

```bash
pnpm cli chains --json
pnpm cli routes --json
pnpm cli balance bc1q... --chain bitcoin --json
```

- [ ] **Step 5: Final commit**

```bash
git add -A
git commit -m "chore: final cleanup after SDK migration"
```
