# Use Metamask to Inscribe and Transfer Ordinals, and Transfer BTC

This example demonstrates how a Metamask Snap can be used to inscribe and transfer ordinals, and transfer BTC, using only Metamask.

:::info Example Code

The code for this example can be [found in the GitHub repository](https://github.com/bob-collective/demo-brc20-metamask)

:::

## Demo

We have created a testnet demo of the application at [https://ordinals.gobob.xyz](https://ordinals.gobob.xyz), allowing anyone to inscribe, view and transfer ordinals, and transfer BTC.

:::info Metamask Flask

This demo uses [Metamask Snaps](https://metamask.io/snaps/) and can only be used with [Metamask Flask](https://metamask.io/flask/). Metamask Snaps are not currently supported on mobile wallets, so this demo will only run in the desktop version of Chrome or Firefox.

:::

### Creating a test profile

To use this demo, you will need to create a new browser profile to use with the Metamask Flask extension, as having Metamask and Metamask Flask installed in the same browser profile can cause issues.

We recommend creating two test profiles so that you can test transferring Ordinals and BTC between two accounts.

### Installing the Metamask Snap

Once you have installed Metamask Flask, click the 'Connect Wallet' button on https://ordinals.gobob.xyz. Metamask will prompt you to install the Snap, and give it permissions.

### Funding your account

One you have connected your wallet, you will see your derived BTC address in place of the 'Connect Wallet' button. Click on the address to copy it to the clipboard and then fund it using a BTC faucet:

- [https://coinfaucet.eu/en/btc-testnet/](https://coinfaucet.eu/en/btc-testnet/)
- [https://bitcoinfaucet.uo1.net/](https://bitcoinfaucet.uo1.net/)

You can check your BTC balance by clicking on the 'Transfer BTC' button.

:::note

BTC may show in your balance before it is confirmed. If you try to use the application before the funds are confirmed, your balance will be too low and you may see errors.

:::

### Inscribing a text ordinal

- Click on the 'Inscribe Ordinal' button, and then the 'Text' tab.
- Enter the inscription text, and submit the form.
- The ordinal will be shown in the Ordinals Portfolio table (this may take a few seconds)

### Inscribing an image ordinal

:::info Supported file types

Only `png` and `jpg` files are currently supported.

:::

- Click on the 'Inscribe Ordinal' button, and then the 'Image' tab.
- Click on Select File (only png and jpg files are supported), select your image, and submit the form.
- The ordinal will be shown in the Ordinals Portfolio table (this may take a few seconds)

### Transferring an ordinal

- Click on the 'Transfer Ordinal' button next to the ordinal you would like to transfer.
- Enter a testnet BTC address, and submit the form.

### Transferring BTC

- Click on the 'Transfer BTC' button at the top of the page.
- Enter a testnet BTC address, and submit the form.
