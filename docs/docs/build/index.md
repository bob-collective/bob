---
sidebar_position: 1
---

# Getting Started

:::note

This is alpha-stage software. There will be dragons. If you are planning to play with this, feel free to reach out in the #builders-lounge on our Discord server for help, feedback, and to report bugs.

:::

## Contracts

### Installation

#### Installing prerequisites

Follow the steps from the [foundry book](https://book.getfoundry.sh/getting-started/installation) to install the Foundry toolsuite, which contains the `forge`, `cast`, `anvil` and `chisel` tools.

#### Adding BOB contracts as dependency

To add the contracts to your own projects, if your project is using `forge`, you can simply run `forge install bob-collective/bob` to add BOB contracts as a dependency to your project.  

### Build the code

To build all the contracts, run `forge build`.

### Run the tests

To run the built-in tests, run `forge test`.

### Examples

The repository contains various code samples in the `src/swap` folder. To deploy a contract like the bitcoin marketplace swap example, run `forge create src/swap/Btc_Marketplace.sol:BtcMarketPlace --rpc-url 'https://l2-fluffy-bob-7mjgi9pmtg.t.conduit.xyz' --private-key "your private key here" --chain 901 --verify --verifier blockscout --verifier-url 'https://explorerl2-fluffy-bob-7mjgi9pmtg.t.conduit.xyz/api?'`.

You can then interact with the contract using through the normal means, e.g. via cast: `cast send 0x0b7bb3e86b620b06e8cdc0e72e142b0bff8c3804 "placeBtcSellOrder(uint,address,uint)" --rpc-url "https://l2-fluffy-bob-7mjgi9pmtg.t.conduit.xyz" --private-key "your private key here" 1 0x0000000000000000000000000000000000000001 1`

You will be able to see your contract deployed using the [explorer](https://explorerl2-ideal-lavender-parrotfish-s3gs5qpznm.t.conduit.xyz/). Interacting with the contract via the explorer is currently not possible due to a deployment bug in conduit.

