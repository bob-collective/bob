# @gobob/tokenlist

This package exports the list of tokens supported and used by the bob dApp.

## Installation (recommended)

Install via pnpm by adding this dependency to your package.json (replace `<commit_hash>` with the commit you want to pin):

```json
"@gobob/tokenlist": "github:bob-collective/bob#<commit_hash>&path:/tokenlist"
```

Then run:

```bash
pnpm install
```

## Usage

Import the token list and use it in your app. The package exposes a JSON token list (array of token objects). Typical token fields include: `address`, `symbol`, `name`, `decimals`, `chainId`, and `logoURI`.

Example (ESM):

```js
import tokenList from '@gobob/tokenlist/tokenlist.json';
// or
// const tokenList = require('@gobob/tokenlist')

console.log(tokenList);
console.log(tokenList.tokens[0]);
```

## Updating tokens / Contributing

- Make changes in the repository under /tokenlist.
- Open a PR against bob-collective/bob and include tests or validation if applicable.
- Follow the repository's contribution guidelines.

## License

See the repository root for license and copyright information.
