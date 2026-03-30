# Gateway-CLI Code Review

**Reviewer:** Qwen (Senior JavaScript Engineer)  
**Date:** 2026-03-30  
**Scope:** Full codebase audit for correctness, security, code quality, and performance

---

## Executive Summary

The gateway-cli codebase is generally well-structured with good separation of concerns, comprehensive test coverage, and modern TypeScript patterns. However, several **critical security issues**, **potential bugs**, and **code quality concerns** were identified that require immediate attention before production deployment.

### Severity Legend
- 🔴 **Critical** — Security vulnerability or data loss risk
- 🟠 **High** — Functional bug or significant reliability issue
- 🟡 **Medium** — Code quality, maintainability, or edge case handling
- 🟢 **Low** — Minor improvements, style inconsistencies

---

## Critical Issues (🔴)

### 1. Missing `bin/` Directory Referenced in `package.json`

**File:** `package.json`  
**Severity:** 🔴 Critical

```json
"bin": {
  "gateway-cli": "./bin/gateway-cli.js"
},
```

**Issue:** The `bin/` directory does not exist in the repository. The `package.json` references `./bin/gateway-cli.js` as the CLI entry point, but this file is missing. This will cause the package to fail installation via npm.

**Impact:** 
- `npm install -g @gobob/gateway-cli` will fail or create a broken symlink
- Package cannot be executed after installation

**Fix Required:** Create `bin/gateway-cli.js` with:
```javascript
#!/usr/bin/env node
import "../dist/src/cli.js";
```

---

### 2. Private Key Logging Risk via Error Messages

**Files:** `src/cli.ts`, `src/commands/swap.ts`  
**Severity:** 🔴 Critical

**Issue:** Error messages may inadvertently expose private keys through stack traces or error context. The `errorMessage()` function in `cli.ts` blindly stringifies errors:

```typescript
// src/cli.ts:17
return err instanceof Error ? err.message : String(err);
```

**Risk:** If any error object contains the private key in its properties or message, it will be logged to stdout/stderr.

**Fix Required:**
1. Implement a sanitization function that redacts potential secrets from error messages
2. Never include key material in error messages
3. Add tests to verify private keys are never logged

---

### 3. No Validation of Private Key Format Before Use

**Files:** `src/chains/bitcoin.ts`, `src/chains/evm.ts`  
**Severity:** 🔴 Critical

```typescript
// src/chains/bitcoin.ts:22
export function deriveBtcAddress(key: string): Promise<string> {
  const signer = ScureBitcoinSigner.fromKey(key);
  return signer.getP2WPKHAddress();
}

// src/chains/evm.ts:168
export function deriveEvmAddress(key: string): string {
  const account = privateKeyToAccount((isHex(key) ? key : `0x${key}`) as Hex);
  return account.address;
}
```

**Issue:** 
- No validation that the key is a valid format before attempting derivation
- Malformed keys could cause undefined behavior or security issues
- No length checks, character validation, or checksum verification

**Fix Required:**
1. Add key format validation before use
2. Provide clear error messages for invalid key formats
3. Consider adding key type detection (WIF vs hex) with explicit validation

---

### 4. Insecure Price Oracle — No Rate Limiting or Cache

**File:** `src/util/price-oracle.ts`  
**Severity:** 🔴 Critical

```typescript
export async function fetchPrice(symbol: string): Promise<PriceResult> {
  const [binance, coinbase] = await Promise.allSettled([
    fromBinance(symbol),
    fromCoinbase(symbol),
  ]);
  // ...
}
```

**Issues:**
1. **No rate limiting** — Repeated calls could trigger API rate limits or bans
2. **No caching** — Every USD amount conversion makes fresh API calls
3. **No circuit breaker** — Continuous failures could cause DoS
4. **5-second timeout is too short** for production reliability

**Impact:** 
- Users could be rate-limited during heavy usage
- Unnecessary API calls increase latency and failure risk
- Potential for price manipulation during rapid calls

**Fix Required:**
1. Implement in-memory caching with TTL (e.g., 30 seconds)
2. Add rate limiting per API provider
3. Increase timeout to 10+ seconds
4. Add circuit breaker pattern for repeated failures

---

### 5. RPC URL Probing Vulnerable to Supply Chain Attack

**File:** `src/util/rpc-resolver.ts`  
**Severity:** 🔴 Critical

```typescript
const CHAINLIST_URL = "https://chainlist.org/rpcs.json";

async function fetchChainlist(): Promise<ChainlistEntry[] | null> {
  try {
    const res = await fetch(CHAINLIST_URL, { signal: AbortSignal.timeout(10_000) });
    // ...
  }
}
```

**Issues:**
1. **No integrity verification** — chainlist.org response is not validated
2. **HTTP-only URLs accepted** — Could return insecure endpoints
3. **No allowlist** — Any RPC URL from chainlist is trusted
4. **In-memory cache could be poisoned** — Once a bad URL is cached, it persists

**Impact:** 
- If chainlist.org is compromised, malicious RPC URLs could be used
- User transactions could be intercepted or manipulated
- Private keys could be exfiltrated via malicious RPC responses

**Fix Required:**
1. Implement RPC URL allowlist for supported chains
2. Add HTTPS-only enforcement
3. Consider pinning known-good RPC URLs
4. Add integrity checks or signatures for chainlist data

---

## High Severity Issues (🟠)

### 6. Missing Error Handling for `Promise.any` in RPC Probing

**File:** `src/util/rpc-resolver.ts`  
**Severity:** 🟠 High

```typescript
async function raceCandidates(urls: string[]): Promise<string | undefined> {
  if (urls.length === 0) return undefined;

  const result = await Promise.any(
    urls.map(url => probeRpc(url).then(r => {
      if (!r) throw new Error("probe failed");
      return r.url;
    }))
  ).catch(() => undefined);

  return result;
}
```

**Issue:** `Promise.any` throws `AggregateError` when all promises reject. The `.catch()` returns `undefined`, but this silent failure could lead to confusing behavior downstream.

**Impact:** If all RPC probes fail, the function returns `undefined`, which may cause viem to use default RPCs that could be unreliable.

**Fix Required:**
1. Log warning when all probes fail
2. Fall back to known-default RPC URLs
3. Consider throwing a descriptive error instead of silent failure

---

### 7. Tokenlist Import Uses Hardcoded Commit Hash

**File:** `package.json`  
**Severity:** 🟠 High

```json
"@gobob/tokenlist": "github:bob-collective/bob#295be799d481d24c84eeb2e30983309a40c650ec&path:/tokenlist"
```

**Issue:** The tokenlist dependency is pinned to a specific commit hash. This will never receive security updates or bug fixes unless manually updated.

**Impact:** 
- Security vulnerabilities in tokenlist won't be patched automatically
- New tokens won't be available without manual updates
- Risk of using outdated/compromised token metadata

**Fix Required:**
1. Use a semantic version tag instead of commit hash
2. Implement automated dependency updates (e.g., Dependabot)
3. Add CI check for tokenlist updates

---

### 8. No Validation of Recipient Address Format

**File:** `src/chains/index.ts`  
**Severity:** 🟠 High

```typescript
export async function resolveRecipient(
  chain: string,
  explicit: string | undefined,
  config: { bitcoinPrivateKey?: string; evmPrivateKey?: string },
): Promise<string> {
  if (explicit) return explicit;  // ← No validation!
  // ...
}
```

**Issue:** When an explicit recipient is provided, no validation is performed. This could lead to:
- Funds sent to invalid addresses (lost forever)
- Wrong chain address format (e.g., EVM address for BTC recipient)

**Fix Required:**
1. Validate BTC addresses with `isValidBtcAddress()` before returning
2. Validate EVM addresses with `isAddress()` before returning
3. Provide clear error messages for invalid formats

---

### 9. Race Condition in Nonce Handling for EVM Transactions

**File:** `src/commands/swap.ts`  
**Severity:** 🟠 High

```typescript
// Wait for pending nonce to settle
if (opts.sender) {
  const addr = opts.sender as `0x${string}`;
  const deadline = Date.now() + 120_000;
  while (Date.now() < deadline) {
    const [latest, pending] = await Promise.all([
      publicClient.getTransactionCount({ address: addr, blockTag: "latest" }),
      publicClient.getTransactionCount({ address: addr, blockTag: "pending" }),
    ]);
    if (pending <= latest) break;
    await new Promise(r => setTimeout(r, 5000));
  }
}
```

**Issue:** 
- This is a band-aid fix for nonce management, not a proper solution
- 2-minute timeout may not be sufficient during network congestion
- No handling if timeout is reached with pending transactions still outstanding

**Fix Required:**
1. Implement proper nonce queue/management
2. Use viem's built-in nonce management if available
3. Add explicit error when nonce cannot be resolved

---

### 10. Slippage Calculation Can Be Negative

**File:** `src/commands/swap.ts`  
**Severity:** 🟠 High

```typescript
const slipBps = outputAmount && finalOrder.dstInfo?.amount
  ? Math.round((1 - Number(finalOrder.dstInfo.amount) / Number(outputAmount)) * 10000)
  : 0;
```

**Issue:** If the actual output is **better** than quoted (user gets more), slippage becomes negative, which is semantically incorrect.

**Fix Required:**
```typescript
const slipBps = outputAmount && finalOrder.dstInfo?.amount
  ? Math.max(0, Math.round((1 - Number(finalOrder.dstInfo.amount) / Number(outputAmount)) * 10000))
  : 0;
```

---

## Medium Severity Issues (🟡)

### 11. Inconsistent Error Handling in `handleBalance`

**File:** `src/commands/balance.ts`  
**Severity:** 🟡 Medium

```typescript
for (const addr of addresses) {
  const family = classifyAddress(addr);

  if (opts.chain?.length) {
    for (const chain of opts.chain) {
      const chainFamily = getChainFamily(chain);
      if (family === "bitcoin" && chainFamily === "evm") {
        console.warn(`Warning: BTC address '${addr}' is not valid for EVM chain '${chain}'`);
        process.exitCode = 1;  // ← Sets exit code but continues!
      }
      // ...
    }
  }
  // ...continues to query balance anyway
}
```

**Issue:** Warning is logged and `exitCode` is set, but execution continues. This is inconsistent — either fail fast or skip invalid chains.

**Fix Required:** Choose one pattern:
1. Fail fast: `throw new Error(...)` instead of warning
2. Skip invalid: `continue` after warning, don't query balance
3. Filter chains: Only query chains compatible with address type

---

### 12. `getAllBalances` Silently Catches All Errors

**File:** `src/chains/index.ts`  
**Severity:** 🟡 Medium

```typescript
const entries = await Promise.all(
  chains.map(async (chain): Promise<[string, ChainBalance]> => {
    try {
      // ...
    } catch {
      return [chain, { address, error: true }];  // ← Silent catch!
    }
  }),
);
```

**Issue:** All errors are caught and converted to `{ error: true }` without logging. This makes debugging impossible.

**Fix Required:**
1. Log errors to stderr (not stdout)
2. Include error message in the result for debugging
3. Consider a `--verbose` flag for detailed error output

---

### 13. `formatBalance` Has Incorrect BTC Assumption

**File:** `src/output.ts`  
**Severity:** 🟡 Medium

```typescript
export function formatBalance(result: BalanceJson): string {
  for (const [chain, data] of Object.entries(result)) {
    // ...
    if (data.balance !== undefined) lines.push(`  Balance:       ${data.balance} BTC`);
    // Always says "BTC" even for EVM chains!
  }
}
```

**Issue:** The formatter hardcodes "BTC" suffix, but this function is used for EVM chains too.

**Fix Required:**
```typescript
const symbol = chain === "bitcoin" ? "BTC" : getNativeToken(chain).symbol;
if (data.balance !== undefined) lines.push(`  Balance:       ${data.balance} ${symbol}`);
```

---

### 14. `humanToAtomic` Has No Overflow Protection

**File:** `src/util/input-resolver.ts`  
**Severity:** 🟡 Medium

```typescript
export function humanToAtomic(human: string, decimals: number): string {
  const [intPart, fracPart = ""] = human.split(".");
  const padded = fracPart.padEnd(decimals, "0").slice(0, decimals);
  const raw = intPart + padded;
  return BigInt(raw).toString();
}
```

**Issue:** No validation for:
- Negative numbers
- Non-numeric input
- Overflow beyond BigInt limits (unlikely but possible)

**Fix Required:**
1. Validate input is a valid positive decimal string
2. Add max value check
3. Return error or throw for invalid input

---

### 15. `buildTokenIndex` Is Inefficient

**File:** `src/util/input-resolver.ts`  
**Severity:** 🟡 Medium

```typescript
export function buildTokenIndex(routes: RouteInfo[]): TokenIndex {
  const byChainAndSymbol = new Map<string, TokenMeta>();
  const byChainAndAddress = new Map<string, TokenMeta>();

  const seen = new Set<string>();
  for (const r of routes) {
    for (const [chain, addr] of [[r.srcChain, r.srcToken], [r.dstChain, r.dstToken]] as const) {
      const dedup = `${chain}:${addr}`;
      if (seen.has(dedup)) continue;
      seen.add(dedup);
      // ...
    }
  }
  return { byChainAndSymbol, byChainAndAddress };
}
```

**Issue:** This function is called in `resolveSwapInputs` for every swap, but the index could be cached.

**Fix Required:**
1. Cache the index in memory
2. Rebuild only when routes change
3. Consider lazy initialization

---

### 16. `parseAmount` Error Messages Leak Implementation Details

**File:** `src/util/input-resolver.ts`  
**Severity:** 🟡 Medium

```typescript
const AMOUNT_HELP = `Expected one of:
  0.05BTC       human-readable (converted to atomic)
  100USD        USD value (converted via price oracle)
  5000000       atomic units (satoshis, wei, etc.)
  ALL           max spendable balance`;
```

**Issue:** While helpful, mentioning "atomic units" and "price oracle" may confuse end users.

**Fix Required:** Use user-friendly language:
```
Expected one of:
  0.05BTC     Amount in BTC
  100USD      USD dollar amount
  5000000     Raw satoshi amount
  ALL         Your entire balance
```

---

### 17. Test Mocks Don't Match Production Code Structure

**Files:** Multiple test files  
**Severity:** 🟡 Medium

**Issue:** Tests mock internal implementation details that may change:

```typescript
// tests/commands/swap.test.ts
vi.mock("../../src/config.js", () => ({
  loadConfig: vi.fn(() => ({ /* ... */ })),
  getSdk: vi.fn(() => ({ /* ... */ })),
}));
```

**Risk:** Tests may pass while production code fails if mock structure diverges.

**Fix Required:**
1. Mock at module boundaries, not internal functions
2. Use integration tests for critical paths
3. Add snapshot tests for error messages

---

### 18. `getTokensForChain` Uses Dynamic `require()`

**File:** `src/util/route-provider.ts`  
**Severity:** 🟡 Medium

```typescript
export function getTokensForChain(chain: string, routes: RouteInfo[]): Array<{ address: string; symbol: string; decimals: number }> {
  // Lazy import to avoid circular dependency at module load time
  const { getTokenMetadata } = require('../chains/evm.js') as typeof import('../chains/evm.js');
  // ...
}
```

**Issue:** 
- `require()` in ESM module is a code smell
- Indicates circular dependency that should be refactored
- May cause issues with tree-shaking and bundlers

**Fix Required:**
1. Refactor to eliminate circular dependency
2. Move `getTokenMetadata` to a shared utility module
3. Use proper ESM imports

---

## Low Severity Issues (🟢)

### 19. Version Mismatch Between `package.json` and `cli.ts`

**Files:** `package.json`, `src/cli.ts`  
**Severity:** 🟢 Low

```json
// package.json
"version": "0.1.0",
```

```typescript
// src/cli.ts:23
.version("0.2.0");
```

**Issue:** Version strings don't match.

**Fix:** Keep versions in sync, or better, read from `package.json`:
```typescript
import pkg from '../package.json' assert { type: 'json' };
.version(pkg.version);
```

---

### 20. `NATIVE_GAS_BUFFER` Is a Magic Number

**File:** `src/chains/evm.ts`  
**Severity:** 🟢 Low

```typescript
export const NATIVE_GAS_BUFFER = 900_000n;
```

**Issue:** No documentation for why 900,000 gas was chosen.

**Fix Required:** Add comment explaining the rationale:
```typescript
// Conservative buffer for gas estimation. Covers 99th percentile of simple transfers.
// Based on analysis of historical gas usage (21k base + buffer for price fluctuations).
export const NATIVE_GAS_BUFFER = 900_000n;
```

---

### 21. `CHAIN_ALIASES` Is Not Extensible by Users

**File:** `src/util/input-resolver.ts`  
**Severity:** 🟢 Low

```typescript
export const CHAIN_ALIASES: Record<string, string> = {
  btc: "bitcoin",
  eth: "ethereum",
  // ...
};
```

**Issue:** Users cannot add custom aliases without code changes.

**Enhancement:** Support environment variable for custom aliases:
```typescript
const envAliases = JSON.parse(process.env.CHAIN_ALIASES || '{}');
export const CHAIN_ALIASES = { ...builtInAliases, ...envAliases };
```

---

### 22. No TypeScript `as const` for String Literals

**Files:** Multiple  
**Severity:** 🟢 Low

**Issue:** String literals like `"bitcoin"`, `"evm"` are not using `as const`, missing type safety.

**Fix:** Use const assertions:
```typescript
export type ChainFamily = "bitcoin" | "evm";

export function getChainFamily(chain: string): ChainFamily {
  if (chain === "bitcoin") return "bitcoin" as ChainFamily;
  return "evm" as ChainFamily;
}
```

---

### 23. `process.exitCode` Is Set But Program Continues

**Files:** `src/commands/routes.ts`, `src/commands/balance.ts`  
**Severity:** 🟢 Low

```typescript
process.exitCode = 1;
// ...continues execution
```

**Issue:** Inconsistent behavior — sometimes warnings set exit code, sometimes errors don't.

**Fix Required:** Standardize error handling:
1. Warnings should NOT set exit code
2. Errors should throw, not set exit code
3. Let the global error handler manage exit codes

---

### 24. Missing `--help` Examples in README

**File:** `README.md`  
**Severity:** 🟢 Low

**Issue:** README shows command syntax but no example output.

**Enhancement:** Add example output for each command to set user expectations.

---

### 25. No CI/CD Configuration in Repository

**Severity:** 🟢 Low

**Issue:** No GitHub Actions or CI configuration visible in the reviewed files.

**Recommendation:** Add CI workflow for:
1. TypeScript compilation
2. Linting
3. Test execution
4. Build artifact generation

---

## Code Quality Observations (Positive)

### ✅ Good Practices Found

1. **Comprehensive test coverage** — Most critical paths have tests
2. **Type safety** — Extensive use of TypeScript types and Zod validation
3. **Separation of concerns** — Clean modular architecture
4. **Retry logic** — Transient errors are properly retried
5. **User-friendly error messages** — Most errors include recovery instructions
6. **Environment-based configuration** — No hardcoded secrets
7. **Atomic balance values** — Avoids floating-point precision issues

---

## Recommended Priority Order

### Immediate (Before Next Release)
1. 🔴 Create `bin/gateway-cli.js` entry point
2. 🔴 Add private key sanitization to error handling
3. 🔴 Validate private key formats before use
4. 🔴 Add price oracle caching and rate limiting
5. 🔴 Implement RPC URL allowlist

### Short Term (Next Sprint)
6. 🟠 Fix RPC probing error handling
7. 🟠 Update tokenlist to use semver
8. 🟠 Validate recipient address formats
9. 🟠 Fix nonce race condition properly
10. 🟠 Fix negative slippage calculation

### Medium Term (Backlog)
11. 🟡 Standardize error handling in `handleBalance`
12. 🟡 Add error logging to `getAllBalances`
13. 🟡 Fix `formatBalance` BTC assumption
14. 🟡 Add overflow protection to `humanToAtomic`
15. 🟡 Cache token index

---

## Additional Recommendations

### Security
1. **Add audit logging** — Log all swap attempts (without sensitive data)
2. **Implement request signing** — Sign API requests to Gateway
3. **Add rate limiting** — Prevent abuse of CLI commands
4. **Consider hardware wallet support** — Ledger/Trezor integration

### Reliability
1. **Add health checks** — Verify Gateway API availability before swaps
2. **Implement circuit breakers** — For all external API calls
3. **Add metrics/telemetry** — Track success rates, latencies
4. **Create runbook** — Document common failure modes and recovery

### Developer Experience
1. **Add `--dry-run` flag** — Preview swap without execution
2. **Add `--verbose` flag** — Detailed logging for debugging
3. **Add interactive mode** — Prompt for missing parameters
4. **Add config file support** — Optional `.gateway-cli.json` for common settings

---

## Conclusion

The gateway-cli codebase demonstrates solid engineering fundamentals but requires addressing **5 critical issues** and **5 high-severity issues** before production deployment. The most urgent concerns are:

1. **Missing CLI entry point** — Package is fundamentally broken
2. **Private key exposure risk** — Potential security vulnerability
3. **No RPC URL validation** — Supply chain attack vector
4. **Price oracle without caching** — Reliability and DoS risk

After addressing critical and high-severity issues, the codebase should undergo a **professional security audit** before mainnet deployment.
