# BRC20 Octopus

This example demonstrates how the BOB SDK can be used to inscribe Ordinals, both text and BRC20s are supported. The demo can be found [here](https://github.com/bob-collective/demo-brc20-octopus), follow the instructions in the README to setup the app and server. It uses the [UniSat Wallet](https://docs.unisat.io/dev/unisat-wallet-api) to interact with the Bitcoin network and sign transactions.

:::tip BOB SDK

To add this functionality to your app use the `inscribeText` function and provide a custom `RemoteSigner` implementation for your wallet. This needs to be able to send Bitcoin to the commit address and sign the PSBT of the reveal transaction. Check the test in [`sdk/test/ordinals.test.ts`](https://github.com/bob-collective/bob/blob/master/sdk/test/ordinals.test.ts) for an example implementation.

:::

With the app (and server) running locally you should see the following page:

![web-app](web-app.png)
