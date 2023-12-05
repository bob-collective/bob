# Gas Fee Payment in WBTC using Meta Transactions (OpenGSN & ERC-2771)

In this example, we will show how WBTC can be used for gas fee payments using the OpenGSN and [ERC-2771 standard](https://eips.ethereum.org/EIPS/eip-2771) on the BOB testnet. This enables users to transact without the necessity to own ETH.

:::info Example Code

The source code for this demo can be found in [this repository](https://github.com/bob-collective/demo-meta-transactions-transfer/).

:::

## OpenGSN
OpenGSN, or the Ethereum Gas Station Network, is a decentralized solution abstracting gas fee payments away from users. OpenGSN enables gasless transactions, allowing users to interact with smart contracts without needing ETH. It provides use cases such as privacy-focused transactions, payments in ERC-20 tokens, off-chain payments, and onboarding subsidies. Learn more in [OpenGSN docs](https://docs.opengsn.org/).


### Smart contract changes

To enable OpenGSN functionality in smart contracts, developers need to make specific modifications. The smart contracts must inherit from the `ERC2771Recipient` contract. It's crucial to note that, when working with GSN recipient contracts, developers must refrain from using `msg.sender` directly. Instead, they should utilize `_msgSender()`, a function provided by `ERC2771Recipient`. This ensures accurate retrieval of user addresses during transactions and facilitates seamless integration with the Gas Station Network.
[Read the full smart contract modification instructions here.](https://docs.opengsn.org/contracts/#writing-gsn-capable-contracts)


![preview](preview.png)

## Using the dApp

This application contains a simple form that allows you to transfer WBTC between accounts with the gas fee paid in WBTC. Simply enter the WBTC amount and the recipient's EVM address and the transaction will be sent to the Open Gas Network relay and relayed using the WBTC paymaster. 

:::note
Before the first relayed transaction is done, the paymaster smart contract has to be approved to spend your WBTC. That is why there will be a transaction request before the first relayed transfer transaction.
:::

### Try it out

1. Go to [demo-meta-transactions-transfer.vercel.app](https://demo-meta-transactions-transfer.vercel.app/) 
2. Connect with your MetaMask account and get WBTC by clicking on the 'Get Tokens' button in the application header. (You will receive 30,000 WBTC that you can spend.)
4. Input the amount you wish to send and the recipient's address into the form fields.
5. Click on 'Approve & Transfer' to approve the paymaster to spend your WBTC and to sign the relayed transaction.
6. Wait for the transfer transaction to be relayed. 

## Local development

### Contract addresses and links

WBTC contract address: `0x833d9398A3DBa68994AdE7Db42Ff597831933aeD`

Paymaster contract address: `0x7F1c9BFcBcc36a09a24473af485cf25e6cfe3Fd6`

OpenGSN relayer:
`https://gsn-relay-fluffy-bob.gobob.xyz`


### Installing the project

1. Install [pnpm](https://pnpm.io/installation)
2. Run `pnpm install`

### Starting the project

1. Run `pnpm run dev`
2. Open `localhost:5173` in browser.

## Interacting with OpenGSN relay

To allow simple interaction with the relay `ethers.Contract` instance is created using OpenGSN provider:
```typescript
const getRelayedContract = async (contractType: ContractType) => {
  if (!window.ethereum) {
    throw new Error('Injected provider not found!');
  }

  const { address, abi } = contracts[contractType];

  const config = {
    preferredRelays: ['https://gsn-relay-fluffy-bob.gobob.xyz'],
    performDryRunViewRelayCall: false,
    gasPriceSlackPercent: 1000,
    maxPaymasterDataLength: 100,
    paymasterAddress: erc20PaymasterAddress
  };

  const gsnProvider = await RelayProvider.newProvider({
    provider: window.ethereum,
    config,
    overrideDependencies: { asyncPaymasterData: getErc20PaymasterData }
  }).init();

  const ethersProvider = new providers.Web3Provider(gsnProvider);

  const relayedContract = new Contract(address, abi, ethersProvider.getSigner());

  return relayedContract;
};
```

Then the `Contract` instance can be used in a standard way:
```typescript
const transferTx = await relayedContract.transfer(form.address, atomicAmount.toString());

await transferTx.wait();
```


