---
sidebar_position: 1
---

# Getting Started

:::note

This is alpha-stage software. There will be dragons. If you are planning to play with this, feel free to reach out in the #builders-lounge on our Discord server for help, feedback, and to report bugs.

:::

## Installation

### Install Rust and Cargo

Follow the steps from the [cargo book](https://doc.rust-lang.org/cargo/getting-started/installation.html) to install Rust and Cargo.

### Install Cargo Contracts

Follow the steps from the [cargo contracts project](https://github.com/paritytech/cargo-contract#installation).

At this point, you should be able to run `cargo contract --help` and see the help output.

## Create a Project

To create a new project, run the following command:

```bash
cargo contract new <project-name>
```

This will create a project with the following structure:

```bash
<project-name>
├── lib.rs
├── Cargo.toml
├── .gitignore
```

By default, the contract is initialized with the `flipper` example from the [ink! documentation](https://github.com/paritytech/ink-examples/tree/main/flipper).

### Test

Writing and executing unit tests is the easiest way to test your contracts. These do not require running a local node but rather use `cargo` to perform the tests.

Test the contract:

```bash
cargo test
```

## Useful resources

- ink! documentation: https://use.ink/getting-started/setup
- Hackathon template: https://github.com/scio-labs/inkathon
