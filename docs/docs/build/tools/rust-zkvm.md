# Rust zkVM

## RISC Zero

The [RISC Zero zero-knowledge virtual machine](https://dev.risczero.com/api/zkvm/) (zkVM) lets you prove correct execution of arbitrary Rust code in your Ethereum contract. [For example](https://dev.risczero.com/api/blockchain-integration/bonsai-on-eth), any smart contract can call the RISC Zero Verifier contract to trustlessly verify that the zk proof of the off-chain computation is valid. While you can generate proofs on your own hardware, you can also use their [Bonsai API and SDK](https://dev.risczero.com/api/generating-proofs/remote-proving) to generate them.

**Supported Networks**

<!-- TODO: Verify RISC Zero on test/mainnet -->

- BOB Mainnet
- BOB Sepolia (Testnet)
