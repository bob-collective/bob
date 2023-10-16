---
sidebar_position: 2
---

# Relay

We have chosen to use the production ready **tBTC-v2** (summa / keep-network) relay contracts and supporting libraries to support the initial development of the BOB stack. The contracts are already well-optimized for gas consumption and have been used on mainnet Ethereum for quite some time.

A specific advantage of using the Simple Payment Verification (SPV) "Light Relay" developed for tBTC is that we do not need to store all block headers from the genesis / initialization height. It uses stateless SPV proofs and provides some recency guarantee using Bitcoin's difficulty adjustment based on the latest retarget.

## How does it work?

- The light relay is initialized to the beginning of a difficulty period (epoch)
- A "maintainer" submits `proofLength` block headers before and after the retarget
- The relay validates the chain and updates the expected difficulty for blocks in that epoch
- A user can then submit a transaction proof in that or the last period
  - Requires header chain of at least `txProofDifficultyFactor`
  
## Notes

There was only one issue highlighted in the Least Authority audit related to the SPV client which was also identified in the interBTC (Substrate / Polkadot) code [here](https://github.com/interlay/interbtc/issues/1073). We can solve this issue by checking the coinbase proof as was implemented there. Since BOB will be deployed as a rollup we can make some tradeoffs with regard to gas consumption.

## Using the relay

The code for the light relay is in `src/relay/LightRelay.sol` which stores the difficulty for the current and previous epoch. To update this it is possible to use `retarget(headers)` with `proofLength * 2` block headers from Bitcoin (before and after the retarget) serialized sequentially. 

:::tip BOB SDK

Use the `getBitcoinHeaders` function to automatically read these from the configured Electrs REST API.

:::

### Validating merkle proofs (SPV)

To check the inclusion of a specific transaction, the `BitcoinTx.validateProof` function can be used. See `test/LightRelay.t.sol` for an example. This requires the serialized transaction and merkle proof with `txProofDifficultyFactor` block headers to prove sufficient work has been built on top.

:::tip BOB SDK

Refer to the `getBitcoinTxProof` and `getBitcoinTxInfo` functions to encode the expected arguments.

:::

### Validating merkle proofs (SPV + witness)

To check that witness data is also included according to the relay we need to do the following:

1. verify coinbase is included (tx + merkle proof)
2. verify payment is included (tx + merkle proof)
3. validate witness commitment (extract root from coinbase, provide merkle proof for wtxids)

:::info

Why might you want to do this? Under normal SPV assumptions it is not possible to prove witness data (such as Ordinal inscriptions) are included on the main chain.

:::

Use the `WitnessTx.validateWitnessProof` function to verify witness data is included. See `test/WitnessTx.t.sol` for an example. As above, this requires the serialized transaction and merkle proof for the **coinbase** transaction. To verify the witness data is included we need to encode the **payment** arguments differently. Check the expected structs in `src/bridge/WitnessTx.sol`, it requires a `witnessVector` and separate witness merkle root hash built using the block's "wtxids" - transactions serialized with the witness data and then hashed according to Bitcoin's double sha2.

:::warning BOB SDK

This approach is still experimental and not yet fully supported by the SDK. To construct the arguments as before use `getBitcoinTxProof` but set `forWitness` to `true` for `getBitcoinTxInfo` to get the `witnessVector`. To construct the witness merkle proof follow the test in `sdk/test/utils.test.ts` using `getMerkleProof` with the full raw block data.

:::

### Checking output amounts

To extract the output amount `BitcoinTx.getTxOutputValue` can be be used to extract the amount transfered to a specific address. See `test/BitcoinTx.t.sol` for an example. The address is the `keccak256` hash of the expected `scriptPubKey`.

:::tip BOB SDK

Use `getBitcoinTxInfo` and pass the `outputVector`.

:::
