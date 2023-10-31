## Local development

### Installing the project

1. Install [pnpm](https://pnpm.io/installation)
2. Run `pnpm install`

### Connecting Metamask

1. Go to [Conduit](https://app.conduit.xyz/published/view/fluffy-bob-7mjgi9pmtg)
2. Click the 'Add to wallet button.'

### Funding your account

#### Native token

1. Create a new account in your wallet
2. From the L2 Faucet section in [Conduit](https://app.conduit.xyz/published/view/fluffy-bob-7mjgi9pmtg), enter your address and click the 'Claim' button.

Note: we have seen instances of this failing. If this happens, the api can be called from a terminal:
`curl -XPOST -i https://faucetl2-fluffy-bob-7mjgi9pmtg.t.conduit.xyz/drip/0x4062e44077b1e58C3D630a0e4e632fF81868e448`

#### Other supported tokens

1. This can be done either by using the faucet button in the UI or by interacting with the smart contract in [Conduit](https://explorerl2-fluffy-bob-7mjgi9pmtg.t.conduit.xyz/address/[address])

### Starting the project

1. Run `pnpm dev`

### Browser support

This application is tested using:

- Chrome
- Brave
- Firefox

In the following environments:

- Linux
- MacOS
- Windows

It does not currently support any mobile wallets.
