# Project Guidelines

## Overview

BOB (Build on Bitcoin) — a hybrid chain fusing Bitcoin security with Ethereum versatility. This monorepo contains Solidity smart contracts, a Rust Bitcoin relay, a TypeScript SDK, and a documentation site.

## Architecture

| Component | Path | Language | Purpose |
|-----------|------|----------|---------|
| Contracts | `contracts/` | Solidity | Light relay, gateway, and utility contracts (Foundry) |
| Relayer | `relayer/` | Rust | Bitcoin relay service |
| Crates | `crates/` | Rust | Shared libraries — `bindings` (generated) and `bob-utils` |
| SDK | `sdk/` | TypeScript | Client SDK (`@gobob/bob-sdk`) |
| Docs | `docs/` | TypeScript | Docusaurus documentation site |

## Code Style

### Rust
- Toolchain: nightly (see `rust-toolchain.toml`)
- Formatter: `cargo fmt` (see `rustfmt.toml` — max heuristics, crate-level imports)
- `crates/bindings/` is auto-generated — do not edit manually

### Solidity
- Framework: Foundry
- Formatter: `forge fmt`
- Optimizer enabled (5000 runs)
- Dependencies managed via `forge install` (not npm)

### TypeScript (SDK)
- Package manager: pnpm
- Linter: eslint — run `pnpm run lint`
- Formatter: prettier — run `pnpm run format`
- Tests: vitest — run `pnpm run test`

## Build and Test

```shell
# Full build (generates bindings, then builds Rust)
make build

# Contracts only
forge build --root contracts
FOUNDRY_PROFILE=optimized forge build --root contracts # production

# Rust only (after bindings exist)
cargo build

# SDK
cd sdk && pnpm install && pnpm run build

# Run all tests
make test

# Contract tests only
forge test --root contracts

# Rust tests only
cargo test

# SDK tests only
cd sdk && pnpm run test

# Formatting check
make fmt

# Regenerate Solidity → Rust bindings
make bindings

# Regenerate gateway API client from OpenAPI spec
make openapi
```

## Agent operating style

- Default to action when asked for a change: inspect the code, make the edit, run the smallest useful verification, and report the result. If the request is a question, exploratory, or a review, answer it and do not modify files unless asked.
- Read a file before making claims about it. Grounded answers beat plausible guesses.
- Weigh reversibility before acting. Local, reversible actions (editing files, running tests, local builds) need no confirmation. Confirm before anything hard to reverse or shared: force-pushing, deleting branches, publishing packages, deploying contracts, or broadcasting transactions. Never bypass a safety check (`--no-verify`, skipped hooks) to make something pass.
- Keep responses concise. Before finishing, self-check against the request and call out any verification that was not run.

## Keeping changes minimal and correct

- Make only the changes requested or clearly necessary. A bug fix does not need the surrounding code refactored; a small feature does not need extra configurability or speculative abstractions.
- Before writing new code, check whether an existing dependency or a package in this repo already does the job.
- Put validation and fallbacks at system boundaries (user input, RPC and API responses, on-chain data), not on states that internal invariants already rule out.
- Treat comments as hints, not evidence; they may be stale. Verify against the code, a test, or an on-chain constraint. After changing behavior, update every comment, doc, and test name that no longer describes it.
- Write general solutions, not ones that only satisfy the tests. Never hard-code expected values or special-case test inputs.
- A test must be able to fail on the bug it covers. Mocking the code under test, or asserting only that a mock was called, is not coverage.
- Remove temporary files and scratch scripts before finishing.

## Ducklings review

Ducklings reviews every PR automatically when it opens.

- Finish the change, self-review the full diff against `master`, and run the relevant checks before opening the PR. Do not request the first review manually.
- Severity is advisory. P0 and P1 findings need a fix or a rejection backed by evidence. Fix a P2 only when it is a real defect or a worthwhile simplification. P3 findings may be ignored.
- Fix the cause, not the comment. Before patching a finding, work out why the code allowed it. When findings cluster in one area, simplify or redesign that area instead of patching them one by one; deleting a code path or edge case is often the right fix.
- Batch all review-driven changes into one new head, then request one follow-up review by commenting exactly `@ducklings-app review` with no other text.
- If P0–P2 findings remain after that follow-up, stop and hand the PR to a human with the open findings and a proposed fix or redesign. Do not keep requesting reviews.
