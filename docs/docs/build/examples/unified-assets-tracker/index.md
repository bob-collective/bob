# Unified EVM and Bitcoin Assets with the MetaMask Bitcoin Snap Extension

:::info MetaMask Flask

This demo uses [MetaMask Snaps](https://metamask.io/snaps/) and can only be used with [MetaMask Flask](https://metamask.io/flask/). MetaMask Snaps are not currently supported on mobile wallets, so this demo will only run in the desktop version of Chrome or Firefox.

:::

This app allows tracking and transferring both EVM and Bitcoin assets in a unified manner using the [BOB MetaMask snap extension](https://github.com/bob-collective/btcsnap).

![image](https://github.com/bob-collective/demo-unified-assets-tracker/assets/47864599/c13783e0-5cbe-4a30-89d7-3c12a39cb408)

This project consists of two components: UI and API. To run it successfully, both have to be running at the same time. For those purposes, there are two dependencies:

1. `pnpm` nodejs package manager.
2. `cargo` Rust package manager.

:::info Example Code

The source code for this demo can be found in [this repository](https://github.com/bob-collective/demo-unified-assets-tracker/).

:::

## Running the API

1. [Obtain Unisat API key to use their service](https://docs.unisat.io/dev/open-api#getting-an-api-key). Put the key in `api/unisat_api_key.txt` file.
2. Move to API directory `$ cd api/`
3. Compile and run the service with `cargo run`
4. The API server should be now running at `localhost:8000`
5. The API documentation can be found in `api/README.md` directory.

## Installing the UI project

1. Move to UI directory `$ cd ui/`
2. Install [pnpm](https://pnpm.io/installation)
3. Run `pnpm install`

## Connecting to network

1. Go to [Conduit](https://app.conduit.xyz/published/view/fluffy-bob-7mjgi9pmtg)
2. Click the 'Add to wallet button.' to add Bob L2 network to your MetaMask wallet.

## Start UI

1. Run `pnpm dev` in `ui` directory.
2. The UI is now running locally on port 5173.
3. Go to `localhost:5173` and click on `Connect wallet`, your wallet will get connected and the Bitcoin snap extension will be installed. After this, you have to approve all the permissions in MetaMask to access the Bitcoin addresses.
4. Now you have Bitcoin address derived from your MetaMask account, you can send testnet BTC or brc20s to see them in the app and move them around.

## Notes

- The app runs on Bitcoin testnet, so make sure the correct network is used.
- This app supports only MetaMask as it uses Bitcoin snap extension to manage your Bitcoin accounts via MetaMask.
- Bitcoin transfers are not implemented yet.
