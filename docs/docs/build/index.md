---
sidebar_position: 1
---

# Getting started

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

Write the contract in the generated `lib.rs` file. 

Build and run tests like above.

## Deploy

Deploy the contract to the local testnet:

```bash
cargo contract upload
```

## Interact

Interact with the contract on the local testnet:

```bash
cargo contract call
```

## Useful resources

- ink! documentation: https://use.ink/getting-started/setup
- hackathon template: https://github.com/scio-labs/inkathon
