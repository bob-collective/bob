---
sidebar_position: 3
sidebar_label: Interact with Bitcoin from BOB Smart Contracts
---

# Bitcoin Light Client

We have chosen to use the production-ready [**tBTC-v2**](https://github.com/keep-network/tbtc-v2/blob/main/solidity/contracts/relay/LightRelay.sol) (summa / keep-network) relay contracts and supporting libraries to support the initial development of the BOB stack. The contracts are already well-optimized for gas consumption and have been used on mainnet Ethereum for quite some time.

A specific advantage of using the Simple Payment Verification (SPV) "Light Relay" developed for tBTC is that we do not need to store all block headers from the genesis / initialization height. It uses stateless SPV proofs and provides some recency guarantee using Bitcoin's difficulty adjustment based on the latest retarget.

## Features

- Proof that a Bitcoin transaction happened on Bitcoin to a smart contract on BOB
- Verify Bitcoin block headers from smart contracts on BOB

## How Does it Work?

- The light relay is initialized to the beginning of a difficulty period (epoch)
- A "maintainer" submits `proofLength` block headers before and after the retarget
- The relay validates the chain and updates the expected difficulty for blocks in that epoch
- A user can then submit a transaction proof in that or the last period
  - Requires header chain of at least `txProofDifficultyFactor`

## Using The Relay

The code for the light relay is in [`src/relay/LightRelay.sol`](https://github.com/bob-collective/bob/blob/master/src/relay/LightRelay.sol) which stores the difficulty for the current and previous epoch. To update this, use `retarget(headers)` with `proofLength * 2` Bitcoin block headers (before and after the retarget) serialized sequentially.

### Adding BOB contracts as dependency

To add the BOB contracts to your own projects, if your project is using Foundry, you can simply run `forge install bob-collective/bob` to add BOB contracts as a dependency to your project.

### Build the code

To build all the contracts, run `forge build`.

### Run the tests

To run the built-in tests, run `forge test`.

### Using the Contracts from TypeScript

:::tip BOB SDK

To get the required input data for the contract, use the `getBitcoinHeaders` function to automatically read `numBlocks` from the configured Electrs REST API.

:::

### Validating Merkle Proofs (SPV)

To check the inclusion of a specific transaction, the `BitcoinTx.validateProof` function can be used. See [`test/LightRelay.t.sol`](https://github.com/bob-collective/bob/blob/master/test/LightRelay.t.sol) for an example. This requires the serialized transaction and merkle proof with `txProofDifficultyFactor` block headers to prove sufficient work has been built on top.

:::tip BOB SDK

Refer to the `getBitcoinTxProof` and `getBitcoinTxInfo` functions to encode the expected arguments.

:::

### **Validating Merkle Proofs (SPV + Witness)**  

:::info  
Why might you want to do this?  
Under normal SPV assumptions, it is not possible to prove witness data (such as Ordinal inscriptions) are included on the main chain.  
:::

To verify witness data, follow these steps:  

1. **Verify coinbase is included** (transaction + Merkle proof).  
2. **Verify payment is included** (transaction + Merkle proof).  
3. **Validate witness commitment**:  
   - Extract the root from the coinbase.  
   - Provide a Merkle proof for wtxids.  

Use the `WitnessTx.validateWitnessProof` function to verify witness data inclusion.  
See [`test/WitnessTx.t.sol`](https://github.com/bob-collective/bob/blob/master/test/WitnessTx.t.sol) for an example.  

This requires:  
- **Serialized transaction**  
- **Merkle proof for the coinbase transaction**  
- **Witness vector and witness Merkle root hash** (constructed from "wtxids")  

Check the expected structs in [`src/bridge/WitnessTx.sol`](https://github.com/bob-collective/bob/blob/master/src/bridge/WitnessTx.sol).  

:::warning BOB SDK

This approach is still experimental and not yet fully supported by the SDK. To construct the arguments as before use `getBitcoinTxProof` but set `forWitness` to `true` for `getBitcoinTxInfo` to get the `witnessVector`. To construct the witness merkle proof follow the test in [`sdk/test/utils.test.ts`](https://github.com/bob-collective/bob/blob/master/sdk/test/utils.test.ts) using `getMerkleProof` with the full raw block data.

:::

### Checking Output Amounts

To extract the output amount `BitcoinTx.processTxOutputs` can be used to extract the amount transferred to a specific address. See [`test/BitcoinTx.t.sol`](https://github.com/bob-collective/bob/blob/master/test/BitcoinTx.t.sol) for an example. The address is derived from the `keccak256` hash of the expected `scriptPubKey`.

:::tip BOB SDK

Use `getBitcoinTxInfo` and pass the `outputVector`.

:::
