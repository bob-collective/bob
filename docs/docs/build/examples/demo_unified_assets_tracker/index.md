# Demo: Unified Assets Tracker


:::note

Source code for this demo can be found in [this repository](https://github.com/bob-collective/demo-unified-assets-tracker/).

:::

![image](https://github.com/bob-collective/demo-unified-assets-tracker/assets/47864599/c13783e0-5cbe-4a30-89d7-3c12a39cb408)

Simple app that allows tracking and transfers of both evm and bitcoin assets in unified manner using MetaMask bitcoin snap extension.

This project consists of 2 components: ui and api. In order to run it successfully, both have to be running at the same time. For those purposes, there are 2 dependencies: `pnpm` nodejs package manager, and `cargo` Rust package manager.

### Running the API

1. [Obtain Unisat API key to use their service](https://docs.unisat.io/dev/open-api#getting-an-api-key). And put the key in `api/unisat_api_key.txt` file.
2. Move to api directory `$ cd api/`
3. Compile and run the service with `cargo run`
4. Api server should be now running at `localhost:8000`
5. Api documentation can be found in `api/README.md` directory.  


### Installing the UI project

1. Move to ui directory `$ cd ui/`
2. Install [pnpm](https://pnpm.io/installation)
3. Run `pnpm install`

### Connecting to network

1. Go to [Conduit](https://app.conduit.xyz/published/view/fluffy-bob-7mjgi9pmtg)
2. Click the 'Add to wallet button.' to add Bob L2 network to your metamask wallet.


### Start UI

1. Run `$ pnpm dev` in `ui` directory
2. UI is now running locally on port 5173 
3. Go to `localhost:5173` and click on `Connect wallet`, your wallet will get connected and bitcoin snap extension will be installed. After this, you have to approve all the permission in Metamask to access the bitcoin addresses.
4. Now you have bitcoin address derived from your Metamask account, you can send testnet BTC or brc20s to see them in the app and move them around.

### Notes
- App runs on bitcoin testnet, so make sure correct network is used.
- This app supports only MetaMask as it uses bitcoin snap extension to manage your bitcoin accounts via MetaMask.
- Bitcoin transfers are not implemented yet.
