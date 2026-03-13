# Project Guidelines

## Overview

BOB (Build on Bitcoin) — a hybrid chain fusing Bitcoin security with Ethereum versatility. This monorepo contains Solidity smart contracts, a Rust Bitcoin relay, a TypeScript SDK, documentation site, and a token list.

## Architecture

| Component | Path | Language | Purpose |
|-----------|------|----------|---------|
| Contracts | `contracts/` | Solidity | Light relay, gateway, and utility contracts (Foundry) |
| Relayer | `relayer/` | Rust | Bitcoin relay service |
| Crates | `crates/` | Rust | Shared libraries — `bindings` (generated) and `bob-utils` |
| SDK | `sdk/` | TypeScript | Client SDK (`@gobob/bob-sdk`) |
| Docs | `docs/` | TypeScript | Docusaurus documentation site |
| Token List | `tokenlist/` | TypeScript | BOB token list and metadata |

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
