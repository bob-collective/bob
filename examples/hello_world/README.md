# Hello World

Run the default `flipper` example from the [ink! documentation](https://use.ink/getting-started/creating-an-ink-project).

## Build

```bash
cargo contract build --release
```

## Test

```bash
cargo test
```

## Run a Substrate node

Follow the [instructions](https://use.ink/getting-started/setup/#installing-the-substrate-smart-contracts-node) to setup a barebones node.

```bash
substrate-contracts-node
```

## Deploy

### UI

Use the [contracts UI with local node](https://contracts-ui.substrate.io/?rpc=ws://127.0.0.1:9944)

### CLI

```bash
cargo contract upload --suri //Alice
cargo contract instantiate --suri //Alice --args true
```

## Call the contract

```bash
cargo contract build
cargo contract upload --suri //Alice

cargo contract instantiate --suri //Alice --args true
# The output of this command will contain the contract address,
# insert it in the command below.

# Call the get() function
cargo contract call --contract ... --message get --dry-run --suri //Alice

# Call the flip() function
cargo contract call --contract ... --message flip --suri //Alice
```
