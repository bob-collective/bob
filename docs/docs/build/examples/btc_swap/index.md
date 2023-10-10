# BTC Swap

This example demonstrates how BOB can be used to build a peer to peer swap application which allows BTC to be exchanged for an ERC20 token without the use of a custodian.

:::note

This example app is a work in progress, and the example application repository is currently set to private.

:::

## Set up local environment

1. Clone [https://github.com/bob-collective/bob](https://github.com/bob-collective/bob)
2. Install [Docker](https://www.docker.com)
3. Run `docker-compose up`
4. Verify that Regtest and Electrum are running by checking for the latest blocks at [http://localhost:3002/blocks](http://localhost:3002/blocks)
5. Add BOB L2 testnet to your EVM wallet and fund it with ETH using [Conduit](https://app.conduit.xyz/published/view/fluffy-bob-7mjgi9pmtg)

An example application can be found at [https://github.com/bob-collective/bob-ui-poc](https://github.com/bob-collective/bob-ui-poc). This has been built using [Viem](https://viem.sh/), [Wagmi](https://wagmi.sh/), React and Typescript, but you can interact with the smart contract using your preferred tools and frameworks.

## Funding your wallet with ERC20 tokens

TODO: Add faucet UI.

Add the contact addresses for supported ERC20 tokens to your Ethereum wallet:

- ZBTC: `0xd6cd079ee8bc26b5000a5e1ea8d434c840e3434b`
- USDT: `0x3c252953224948E441aAfdE7b391685201ccd3bC`

## Basic Protocol

### Getting the smart contract ABIs

This is done using a React hook which extends Viem's `getContract` method. This allows contract member types to be inferred, rather than respecified in the application.

```
import { getContract } from 'viem';
import { usePublicClient, useWalletClient } from 'wagmi';
import { contracts, ContractType } from '../constants';
import { useMemo } from 'react';

// Wrapper around ethers Contract to automatically get contract types
// with read and write objects automatically constructed from provider and signer.
const useContract = (contractType: ContractType) => {
  const publicClient = usePublicClient();
  const { data: walletClient } = useWalletClient();

  return useMemo(() => {
    const { address, abi } = contracts[contractType];

    return getContract({
      address,
      abi,
      publicClient,
      walletClient: walletClient ?? undefined
    });
  }, [walletClient, publicClient, contractType]);
};

export { useContract };
```

This can then be called in the application:

```
// contracts/config.ts
const contracts = {
  [ContractType.BTC_MARKETPLACE]: {
    address: "0xd6cd079ee8bc26b5000a5e1ea8d434c840e3434b",
    abi: BtcMarketplaceAbi,
  },
} as const;

// App.tsx
const {
  read: readBtcMarketplace,
  write: writeBtcMarketplace
} = useContract(ContractType.BTC_MARKETPLACE);
```
