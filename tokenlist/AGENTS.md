# AGENTS.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
pnpm build              # Full build: generates types, tokenlist JSONs, then verifies
pnpm build:tokenlist    # Generate tokenlist JSON files from /data
pnpm build:types        # Regenerate token-ids.ts from current /data entries
pnpm verify             # Validate schema + on-chain data (runs automatically after build)
```

There is no lint or test runner configured.

## Architecture

This package is the authoritative token registry for the BOB ecosystem. The source of truth is the `/data` directory — one subdirectory per token (named by symbol), each containing a `data.json` and a logo asset.

**Data flow:**

```
data/[TOKEN]/data.json  →  scripts/build-tokenlist.ts  →  tokenlist.json
                                                        →  tokenlist-bob.json
                                                        →  tokenlist-overrides.json
data/*/                 →  scripts/build-types.ts      →  token-ids.ts (auto-generated)
```

**Key files:**

- `config.ts` — Chain definitions, supported chain list, file path constants, and GitHub logo URI base. All chain-to-chainId mappings live here.
- `types.ts` — `TokenData` (raw input shape from data.json), `Token` (output shape for tokenlist JSONs), `SupportedChain` type.
- `index.ts` — Public API surface; re-exports from config, types, token-ids, utils.
- `token-ids.ts` — Auto-generated union type of all token symbols. Never edit by hand; run `pnpm build:types`.
- `token.schema.json` — JSON Schema attached to data.json files for IDE autocompletion.

**data.json structure:**

Each token directory's `data.json` specifies:
- `name`, `symbol`, `decimals`, `logoURI`
- `addresses` — map of chain name (e.g. `"bob"`, `"ethereum"`, `"op-mainnet"`) → contract address
- `bridgeInfo` — maps source chain name → `{ tokenAddress }` for cross-chain bridge relationships
- `overrides` — per-chain UI overrides (e.g. rename symbol to `"USDC.e"` on BOB)
- Optional metadata: `website`, `twitter`, `description`

Chain names used as keys in `addresses`/`bridgeInfo`/`overrides` must match the `SupportedChain` type defined in `config.ts`. Chain IDs are only used in the output JSONs.

**Output JSONs:**

- `tokenlist.json` — All tokens across all chains, flat format
- `tokenlist-bob.json` — BOB-chain entries only
- `tokenlist-overrides.json` — Full list with per-chain overrides applied

These are committed to the repo and consumed downstream via the npm package.

**Adding a new token:**

1. Create `data/[SYMBOL]/data.json` (use `token.schema.json` for structure)
2. Add a logo asset (`logo.svg` or `logo.webp`)
3. Run `pnpm build` — this regenerates `token-ids.ts`, all JSON outputs, and runs on-chain verification
