# BRC21

A POC implementation for the BRC-21 Token Standard.

:::note

This is not production-ready code and is only meant to be used for experimentation.

:::

## Basic Protocol

The basic protocol is as follows:

### Minting

1. Mint the locked tokens on Bitcoin via an inscription
2. Lock the underlying token in the smart contract and provide proof that the inscription locks the same amount of tokens

Indexers now accept the Bitcoin-minted BRC21 as minted

### Transfer

1. Transfer BRC21 just like a BRC20 on Bitcoin

### Redeeming

1. Redeem BRC21 on Bitcoin
2. Proof BRC21 redeem to this contract and unlock tokens

### Reference

See the full protocol at https://interlay-labs.gitbook.io/brc-21/

## Follow the Example

Go to the `brc21-poc` directory to follow the example.

- [brc21-poc](https://github.com/interlay/bob/blob/master/examples/brc21-poc/README.md#getting-started)