# gateway-cli

CLI for [BOB Gateway](https://docs.gobob.xyz/gateway/overview) — swap between BTC and EVM tokens from the terminal.

## Install

```bash
npm install -g @gobob/gateway-cli
gateway-cli --help
```

### From source

```bash
pnpm install && pnpm build
npm link                      # makes gateway-cli available globally
gateway-cli --help
```

For development without linking:

```bash
pnpm cli:dev --help
```

## Quick start

```bash
# Set your keys
export BITCOIN_PRIVATE_KEY="<wif-or-hex>"
export EVM_PRIVATE_KEY="<hex>"

# Get a quote
gateway-cli quote --src BTC --dst USDC:base --amount 0.05BTC --recipient 0xYourAddress

# Execute a swap
gateway-cli swap --src BTC --dst USDC:base --amount 0.05BTC --recipient 0xYourAddress

# Send everything
gateway-cli swap --src BTC --dst USDC:base --amount ALL --recipient 0xYourAddress

# Check your balances (derives addresses from keys)
gateway-cli balance
```

## Commands

### `swap`

Execute a cross-chain swap.

```bash
gateway-cli swap --src BTC --dst USDC:base --amount 0.05BTC --recipient 0x...
```

**Required:** `--src`, `--dst`, `--amount`, `--recipient`

### `quote`

Preview a swap without executing.

```bash
gateway-cli quote --src BTC --dst USDC:base --amount 100USD --recipient 0x...
```

### `balance`

Show token balances across chains.

```bash
gateway-cli balance                           # derive addresses from env keys
gateway-cli balance bc1q... 0x123...          # explicit addresses
gateway-cli balance 0x123... --chain base     # specific chain
```

### `routes`

List supported routes, chains, or tokens.

```bash
gateway-cli routes                            # all routes
gateway-cli routes --chains                   # supported chains
gateway-cli routes --tokens base              # tokens on Base
gateway-cli routes --src-chain bitcoin        # routes from BTC
```

### `status`, `orders`, `register`

```bash
gateway-cli status <order-id>                 # check order status
gateway-cli orders <address>                  # list orders for address
gateway-cli register <order-id> <txid>        # manually register a tx (recovery)
```

## Amount format

The `--amount` flag accepts multiple formats:

| Format | Example | Meaning |
|--------|---------|---------|
| Token suffix | `0.05BTC` | 0.05 BTC in human-readable units |
| USD | `100USD` | $100 worth (via price oracle) |
| Atomic | `5000000` | 5,000,000 satoshis / wei / etc. |
| All | `ALL` | Max spendable balance |

Bare numbers are always atomic units — you can't accidentally send 100 BTC by typing `100`.

## Configuration

All config via environment variables. No config files.

| Variable | Description |
|----------|------------|
| `BITCOIN_PRIVATE_KEY` | BTC private key (WIF or hex) |
| `EVM_PRIVATE_KEY` | EVM private key (hex) |
| `GATEWAY_API_URL` | Gateway API base URL (default: production) |
| `BTC_FEE_RATE` | Bitcoin fee rate in sat/vbyte (default: mempool fastest) |
| `EVM_RPC_URL_<CHAIN>` | Custom RPC URL per chain (e.g. `EVM_RPC_URL_ETHEREUM`) |

## All flags

### Shared (quote + swap)

```
--src <asset[:chain]>    Source asset (e.g. BTC, USDC:ethereum)
--dst <asset[:chain]>    Destination asset
--amount <value>         Amount (see format table above)
--recipient <address>    Recipient address
--sender <address>       Sender address (optional)
--slippage <bps>         Slippage tolerance in basis points (default: 300)
--gas-refill-usd <usd>   ETH gas refill on destination
--btc-fee-rate <sat>     Bitcoin fee rate override
--fee-token <address>    ERC20 token for gas payment (paymaster)
--fee-reserve <amount>   Amount of fee token to reserve for gas
--json                   Output as JSON
```

### Swap only

```
--private-key <key>      Signing key (or use env vars)
--unsigned               Output unsigned PSBT/tx without signing
--no-wait                Exit after submitting without polling
--no-retry               Fail immediately on transient errors
--timeout <seconds>      Polling timeout (default: 1800)
```

## Output modes

By default, output is human-readable. Add `--json` for machine-readable JSON output.

```bash
# Human-readable
gateway-cli quote --src BTC --dst USDC:base --amount 0.05BTC --recipient 0x...

# JSON
gateway-cli quote --src BTC --dst USDC:base --amount 0.05BTC --recipient 0x... --json
```

## Publishing to npm

Tag a release to trigger the GitHub Actions publish workflow:

```bash
git tag cli-v0.2.0
git push origin cli-v0.2.0
```

Release candidates use the `rc` npm tag:

```bash
git tag cli-v0.3.0-rc0
git push origin cli-v0.3.0-rc0
```


## Error handling

- **Transient errors** (rate limits, timeouts): automatically retried on quote and registration steps. Use `--no-retry` to disable.
- **Registration failure**: if a signed tx fails to register, the error includes the order ID and a recovery command: `gateway-cli register <order-id> <txid>`.
- **Balance errors**: per-chain failures show "N/A" instead of failing the entire command.
