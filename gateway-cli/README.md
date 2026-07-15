# gateway-cli

CLI for [BOB Gateway](https://docs.gobob.xyz/gateway/overview) — swap between BTC and EVM tokens, and send funds on a single chain, from the terminal.

## Quick start

```bash
# Install
npm install -g @gobob/gateway-cli

# Set your keys
export BITCOIN_PRIVATE_KEY="<wif-or-hex>"
export EVM_PRIVATE_KEY="<hex>"

# Get a quote
gateway-cli quote --src BTC --dst USDC:base --amount 0.05BTC --recipient 0xYourAddress

# Execute a swap
gateway-cli swap --src BTC --dst USDC:base --amount 0.05BTC --recipient 0xYourAddress

# Send everything
gateway-cli swap --src BTC --dst USDC:base --amount ALL --recipient 0xYourAddress

# Swap without --recipient (uses derived address from EVM_PRIVATE_KEY)
gateway-cli swap --src BTC --dst USDC:base --amount 0.05BTC

# Send funds directly on a single chain (no Gateway)
gateway-cli send --asset USDC:base --amount 100USDC --to 0xRecipient
gateway-cli send --asset BTC --amount 0.01BTC --to bc1qRecipient

# Check your balances (derives addresses from keys)
gateway-cli balance
```

## Install from source

```bash
pnpm install && pnpm build
npm link                      # makes gateway-cli available globally
gateway-cli --help
```

For development without linking:

```bash
pnpm cli:dev --help
```

## Commands

### `swap`

Execute a cross-chain swap.

```bash
gateway-cli swap --src BTC --dst USDC:base --amount 0.05BTC --recipient 0x...
```

**Required:** `--src`, `--dst`, `--amount`

### `send`

Send BTC or an EVM token (native or ERC20) directly to an address — a plain
single-chain transfer, **not** a Gateway swap.

```bash
gateway-cli send --asset USDC:base --amount 100USDC --to 0x...   # ERC20
gateway-cli send --asset ETH:base --amount 0.1ETH --to 0x...     # EVM native
gateway-cli send --asset BTC --amount 0.01BTC --to bc1q...       # BTC
gateway-cli send --asset BTC --amount ALL --to bc1q...           # sweep entire balance
gateway-cli send --asset USDC:base --amount 100USDC --to 0x... --unsigned   # don't broadcast
```

**Required:** `--asset`, `--amount`, `--to`

- `--asset` accepts a known symbol (`BTC`, `ETH:base`, `USDC:arbitrum`) or a raw
  token address (`0xToken:base`); decimals come from the token list or an on-chain
  lookup. A chain is required for everything except `BTC`.
- `--to` must match the asset's chain family — a BTC address for `BTC`, an EVM
  address otherwise.
- `--amount ALL` sends the full spendable balance: ERC20 = full balance, EVM
  native = balance minus gas, BTC = all UTXOs minus the network fee.
- The signing key comes from `--private-key` or the env keys. The sender is the
  key's own address (there is no `--sender`); BTC sends from a P2WPKH (`bc1q`) address.

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
gateway-cli balance --chain base,bob          # multiple chains (comma-separated)
gateway-cli balance --chain base --chain bob  # multiple chains (repeated)
gateway-cli balance --non-zero                # only non-zero balances and tokens
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
--recipient <address>    Recipient address (optional if destination wallet key is set)
--sender <address>       Sender address (optional)
--slippage <bps>         Slippage tolerance in basis points (default: 300)
--btc-fee-rate <sat>     Bitcoin fee rate override
--fee-token <address>    ERC20 token for gas payment (paymaster)
--fee-reserve <amount>   Amount of fee token to reserve for gas
--json                   Output as JSON
```

> **Note:** When `--recipient` is omitted, the CLI derives the recipient from the
> destination chain's private key (`BITCOIN_PRIVATE_KEY` for BTC destinations,
> `EVM_PRIVATE_KEY` for EVM destinations). BTC recipients use P2WPKH addresses
> (`bc1q...`). Other BTC address types are not yet supported. An explicit
> `--recipient` always overrides the derived address.

### Swap only

```
--private-key <key>      Signing key (or use env vars)
--unsigned               Output unsigned PSBT/tx without signing
--no-wait                Exit after submitting without polling
--no-retry               Fail immediately on transient errors (retry is on by default)
--timeout <seconds>      Polling timeout (default: 1800, max: 86400)
```

### Send only

```
--asset <asset[:chain]>  Asset to send (e.g. BTC, ETH:base, USDC:arbitrum, 0xToken:base)
--amount <value>         Amount (see format table above)
--to <address>           Recipient address (BTC or EVM, must match the asset chain)
--private-key <key>      Signing key (or use env vars)
--btc-fee-rate <sat>     Bitcoin fee rate override
--unsigned               Output unsigned tx (EVM) or PSBT (BTC) without broadcasting
--no-wait                Broadcast without waiting for confirmation
--timeout <seconds>      Confirmation wait timeout (default: 1800)
--json                   Output as JSON
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

- **Transient errors** (rate limits, timeouts): automatically retried, but only *before* the
  wallet is asked to sign. Retrying is enabled by default; use `--no-retry` to disable.
- **Point of no return**: once the wallet has been asked to sign, the swap is never retried,
  however transient the error looks — re-running `executeQuote` would broadcast a second
  transaction and send the funds twice. Such a failure is reported as terminal, tells you not
  to re-run, and points at `gateway-cli orders <owner-address>` to check whether an order exists.
- **Registration failure**: if a signed tx fails to register, the error includes the order ID and a recovery command: `gateway-cli register <order-id> <txid>`.
- **Status read failures while polling**: a swap's funds are committed once it is submitted, so
  the order status is the only authority on whether it failed. Errors while *reading* that status
  (gateway 5xx, network failures) are retried until `--timeout`, never reported as swap failures.
- **`--timeout` reached without a terminal status**: reported as `in_flight` (exit 0), carrying
  the order id and source txid — the swap is still settling, not failed. Follow it up with
  `gateway-cli status <order-id>`.
- **Balance errors**: per-chain failures show "N/A" instead of failing the entire command.
