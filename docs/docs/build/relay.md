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

- To check the inclusion of a specific transaction, the `BitcoinTx.validateProof` function can be used. See `test/LightRelay.t.sol` for an example.   
- `BitcoinTx.getTxOutputValue` can be be used to check the amount transfered to a specific address in a given `txOut`. See `test/BitcoinTx.t.sol` for an example.
