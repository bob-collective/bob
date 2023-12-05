# Gas Fee Payment in WBTC using Account Abstraction (ERC-4337)

In this example, we will show how WBTC can be used for gas fee payment using the [ERC-4337 account abstraction standard](https://eips.ethereum.org/EIPS/eip-4337) on the BOB testnet. This enables users to use smart contract wallets and transact without the need to own ETH.

:::info Example Code

The source code for this demo can be found in [this repository](https://github.com/bob-collective/demo-account-abstraction-transfer/).

:::

## Smart Contract wallets

This demo showcases WBTC transfer from the smart contract wallet. Smart contract wallets utilize smart contracts rather than single private keys found in Externally Owned Address (EOA) wallets. The programmable nature of smart contract wallets allows for diverse use cases. Unlike EOAs, smart contract wallets can contain logic but rely on EOAs to sign the user operations. Therefore, integration of the smart contract wallet into UI adds an additional complexity - users need to distinguish between the signer (EAO) and the account that holds assets (smart contract wallet).

![preview](preview.png)

## Using the dApp

This application uses the ERC-4337 standard and showcases how a smart contract wallet can be integrated. It contains an implementation of a custom account abstraction client that simplifies the integration of this standard into the UI. 


The application consists of a simple form that allows you to send WBTC from the smart contract account with the gas fee paid in WBTC. To use it enter the WBTC amount and the recipient's EVM address. Then the injected wallet will ask for a user operation signature. After that, a signed user operation will be sent to the bundler which will broadcast it to the network.

:::note
Before the first user operation can be made, the paymaster smart contract has to be approved to spend your WBTC. That is why the first wallet prompt will be the WBTC approval transaction request.
:::

### Try it out

1. Go to [demo-acccount-abstraction-transfer.vercel.app](https://demo-account-abstraction-transfer.vercel.app/) 
2. Connect with your MetaMask account and get WBTC by clicking on the 'Get Tokens' button in the application header. (You will receive 30,000 WBTC that you can spend.)
3. Add WBTC to your MetaMask, WBTC address is `0x28A13b11551f91651e8Da8Cd997886aA0B46CD16`
4. Transfer WBTC to the smart contract account via MetaMask (displayed above the amount field in the form).
5. Input the amount you wish to send and the recipient's address into the form fields.
6. Click on 'Transfer'. This will require three interactions with MetaMask: first one will deposit small amount of ETH to the entry point contract so that you can sign the user operation that allows the paymaster contract to spend your WBTC. Then you will sign the approval user operation. Finally, you will sign the transfer user operation. 
7. Wait for the transfer user operation to be executed.


## Limitations

Given that ERC-4337 is still relatively new, there is not a lot of support for this standard available yet. 

## Local development

### Contract addresses and links
WBTC contract address: `0x28A13b11551f91651e8Da8Cd997886aA0B46CD16`

Entry point contract address: `0x7A660708DB3D56BB0dC3694344777c805716Fca7`

WBTC paymaster address:
`0xD8Ae58534d5488571E248DdC0A3aD42aD5dBaD26`

Bundler (eth-infinitism):
`https://bundler-fluffy-bob.gobob.xyz/rpc`

### Installing the project

1. Install [pnpm](https://pnpm.io/installation)
2. Run `pnpm install`


### Starting the project

1. Run `pnpm run dev`
2. Open `localhost:5173` in browser.

## Using account abstraction client

To allow easy integration of ERC-4337 into dApps, a simple account abstraction client is included in this repository. This client handles smart account creation and bundler connection, manages user operations and allows paymaster usage. This repository also includes a React hook and context provider which enable straightforward usage of the client in the React application.

To use `AaClient` in your app wrap it in the `AccountAbstractionProvider`:
```typescript
<AccountAbstractionProvider>
  <App />
</AccountAbstractionProvider>
```

Now you can use the `useAccountAbstraction` hook anywhere within the app to get the client and utilize its functionality:
```typescript
const { client } = useAccountAbstraction();  
...
const userOp = await client.createUserOp({
address: contract.address,
callData,
value: 0,
nonce: approvalUserOpNonce
});

const transferResult = await client.signAndSendUserOp(userOp);
  

```

To view the example of a full account abstraction flow please navigate to the `src/App.tsx` component.