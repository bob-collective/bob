# @gobob/tokenlist

Token list package for the BOB dApp. Provides a standardized token list format with multi-chain support, bridge information, and UI overrides.

## Contents

- [Installation](#installation)
- [Usage](#usage)
- [Adding New Tokens](#adding-new-tokens)
- [Scripts](#scripts)

---

## Installation

```json
{
  "@gobob/tokenlist": "github:bob-collective/bob#<commit_hash>&path:/tokenlist"
}
```

```bash
pnpm install
```

---

## Usage

```typescript
// ESM
import tokenList from '@gobob/tokenlist/tokenlist.json';

// CommonJS
const tokenList = require('@gobob/tokenlist/tokenlist.json');
```

### Exports

| File | Description |
|------|-------------|
| `tokenlist.json` | Complete token list across all chains |
| `tokenlist-bob.json` | Tokens on BOB chain only |
| `tokenlist-overrides.json` | Token list with UI overrides applied |

### TypeScript Types

The package exports TypeScript types for type-safe development:

```typescript
import { TokenId } from '@gobob/tokenlist/token-ids';
import type { Token, TokenData, SupportedChain } from '@gobob/tokenlist/types';
```

| Type | Description |
|------|-------------|
| `TokenId` | Union of all token identifiers (e.g., `'WBTC' | 'USDT' | ...`) |
| `Token` | Single token object from the tokenlist |
| `TokenData` | Token metadata structure used in `data.json` files |
| `SupportedChain` | Union of supported chain names |

---

## Adding New Tokens

### 1. Create Token Directory

```
data/
└── MYTOKEN/
    ├── data.json
    └── logo.{svg|webp}
```

### 2. Add Token Metadata

Create `data.json`:

```json
{
  "name": "My Token",
  "symbol": "MYTOKEN",
  "decimals": 18,
  "tokens": {
    "ethereum": {
      "address": "0x...",
      "bridge": {
        "bob": "0x..."
      }
    },
    "bob": {
      "address": "0x..."
    }
  },
  "overrides": {
    "bob": {
      "symbol": "MYTOKEN",
      "name": "My Token (BOB)"
    }
  }
}
```

### 3. Add Logo

Add `logo.svg` or `logo.webp` (min 200x200px for raster).

### 4. Build

**Required:** After adding a token, regenerate the JSON files:

```bash
pnpm build
```

This updates 3 files:
- `tokenlist.json` — all tokens (excludes `overrides`)
- `tokenlist-bob.json` — BOB chain tokens
- `tokenlist-overrides.json` — tokens with overrides applied

### 5. Verify

```bash
pnpm verify
```

---

## Scripts

| Command | Description |
|---------|-------------|
| `pnpm build` | Generate types + build tokenlist |
| `pnpm build:tokenlist` | Generate the 3 tokenlist JSON files |
| `pnpm build:types` | Generate TypeScript types |
| `pnpm verify` | Validate against schema |

---

## Notes

**Adding new chains:** When adding chains to `config.ts`, update [`token.schema.json`](./token.schema.json) to enable JSON schema autocompletion in `data.json` files.

---

## Contributing

Open a PR against `bob-collective/bob` with your changes.

## License

MIT
