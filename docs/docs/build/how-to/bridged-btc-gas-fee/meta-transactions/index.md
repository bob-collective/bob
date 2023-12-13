# Gas Fee Payment in wBTC using Meta Transactions (OpenGSN & ERC-2771)

In this example, we will show how wBTC can be used for gas fee payments using the OpenGSN and [ERC-2771 standard](https://eips.ethereum.org/EIPS/eip-2771) on the BOB testnet. This enables users to transact without the necessity to own ETH.

:::tip Live Demo

Check out the live demo at [demo-meta-transactions.gobob.xyz](https://demo-meta-transactions.gobob.xyz).

:::

:::info Example Code

Check out the code of the demo in [this repository](https://github.com/bob-collective/demo-meta-transactions-transfer/).

:::

## OpenGSN

[OpenGSN](https://opengsn.org/), or the Ethereum Gas Station Network, is a decentralized solution abstracting gas fee payments away from users. OpenGSN enables gasless transactions, allowing users to interact with smart contracts without needing ETH. It provides use cases such as privacy-focused transactions, payments in ERC-20 tokens, off-chain payments, and onboarding subsidies. Learn more in [OpenGSN docs](https://docs.opengsn.org/).

### Smart contract changes

To enable OpenGSN functionality in smart contracts, developers need to make specific modifications. The smart contracts must inherit from the `ERC2771Recipient` contract. It's crucial to note that, when working with GSN recipient contracts, developers must refrain from using `msg.sender` directly. Instead, they should utilize `_msgSender()`, a function provided by `ERC2771Recipient`. This ensures accurate retrieval of user addresses during transactions and facilitates seamless integration with the Gas Station Network.
[Read the full smart contract modification instructions here.](https://docs.opengsn.org/contracts/#writing-gsn-capable-contracts)

![preview](preview.png)

## Using the dApp

This application contains a simple form that allows you to transfer wBTC between accounts with the gas fee paid in wBTC. Simply enter the wBTC amount and the recipient's EVM address and the transaction will be sent to the Open Gas Network relay and relayed using the wBTC paymaster.

:::note
Before the first relayed transaction is done, the paymaster smart contract has to be approved to spend your wBTC. That is why there will be a transaction request before the first relayed transfer transaction.
:::

### Try it out

#### Connecting MetaMask and funding your account

1. Go to [demo-meta-transactions.gobob.xyz](https://demo-meta-transactions.gobob.xyz)
2. Fund your account with Sepolia ETH from the [Sepolia testnet faucet](https://faucetlink.to/sepolia).
3. Transfer Sepolia ETH to BOB using [Superbridge](https://puff-bob-jznbxtoq7h.testnets.superbridge.app/).
4. Connect with your MetaMask account and get wBTC by clicking on the 'Get Tokens' button in the application header. (You will receive 30,000 wBTC that you can spend.)
5. Add wBTC to your MetaMask, wBTC address is `0x2868d708e442A6a940670d26100036d426F1e16b`

#### Making a transfer

1. Input the amount you wish to send and the recipient's address into the form fields.
2. Click on 'Approve & Transfer' to approve the paymaster to spend your wBTC and to sign the relayed transaction.
3. Wait for the transfer transaction to be relayed.

## Local development

### Contract addresses and links

- WBTC contract address: `0x2868d708e442A6a940670d26100036d426F1e16b`
- Paymaster contract address: `0x777FA19ea9e771018678161ABf2f1E2879D3cA6C`
- OpenGSN relayer: `https://gsn-relay-sepolia.gobob.xyz/`

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
    throw new Error("Injected provider not found!");
  }

  const { address, abi } = contracts[contractType];

  const config = {
    preferredRelays: ["https://gsn-relay-sepolia.gobob.xyz/"],
    performDryRunViewRelayCall: false,
    gasPriceSlackPercent: 1000,
    maxPaymasterDataLength: 100,
    paymasterAddress: erc20PaymasterAddress,
  };

  const gsnProvider = await RelayProvider.newProvider({
    provider: window.ethereum,
    config,
    overrideDependencies: { asyncPaymasterData: getErc20PaymasterData },
  }).init();

  const ethersProvider = new providers.Web3Provider(gsnProvider);

  const relayedContract = new Contract(
    address,
    abi,
    ethersProvider.getSigner()
  );

  return relayedContract;
};
```

Then the `Contract` instance can be used in a standard way:

```typescript
const transferTx = await relayedContract.transfer(
  form.address,
  atomicAmount.toString()
);

await transferTx.wait();
```
