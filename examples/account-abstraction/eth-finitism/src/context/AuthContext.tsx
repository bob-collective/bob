import { BaseAccountAPI, ClientConfig, ERC4337EthersProvider, HttpRpcClient, SimpleAccountAPI } from '@account-abstraction/sdk';
import { Signer, providers } from 'ethers';
import { createContext, useContext, useState } from 'react';
import { useAccount } from 'wagmi';
import { ENTRY_POINT_ADDRESS } from '../constants/erc4337';

import { EntryPoint__factory } from '@account-abstraction/contracts';
import { SimpleAccountFactory__factoryClass } from './customFactory';
import { DeterministicDeployer } from './customDetDeployer';
import { JsonRpcProvider } from '@ethersproject/providers';

type accountAbstractionContextValue = {
  accountAPI?: BaseAccountAPI;
  bundlerClient?: HttpRpcClient;
  address?: `0x${string}`;
  ownerAddress?: `0x${string}`;
};

const initialState = {
  accountAPI: undefined
};

const accountAbstractionContext = createContext<accountAbstractionContextValue>(initialState);

const factoryInstance = new SimpleAccountFactory__factoryClass();

async function wrapProvider (
  originalProvider: JsonRpcProvider,
  config: ClientConfig,
  originalSigner: Signer = originalProvider.getSigner()
): Promise<ERC4337EthersProvider> {
  const entryPoint = EntryPoint__factory.connect(config.entryPointAddress, originalProvider)
  // Initial SimpleAccount instance is not deployed and exists just for the interface
  const detDeployer = new DeterministicDeployer(originalProvider)

  const SimpleAccountFactory = await detDeployer.deterministicDeploy(factoryInstance, 0, [entryPoint.address])
  const smartAccountAPI = new SimpleAccountAPI({
    provider: originalProvider,
    entryPointAddress: entryPoint.address,
    owner: originalSigner,
    factoryAddress: SimpleAccountFactory,
    paymasterAPI: config.paymasterAPI
  })
  console.log('config=', config)
  const chainId = await originalProvider.getNetwork().then(net => net.chainId)
  const httpRpcClient = new HttpRpcClient(config.bundlerUrl, config.entryPointAddress, chainId)
  return await new ERC4337EthersProvider(
    chainId,
    config,
    originalSigner,
    originalProvider,
    httpRpcClient,
    entryPoint,
    smartAccountAPI
  ).init()
}


const useAccountAbstraction = () => {
  const context = useContext(accountAbstractionContext);

  if (!context) {
    throw new Error('useAccountAbstraction should be used within a AccountAbstraction Provider');
  }

  return context;
};

const AccountAbstractionProvider = ({ children }: { children: JSX.Element }) => {
  const [accountAPI, setAccountAPI] = useState<BaseAccountAPI>();
  const [bundlerClient, setBundlerClient] = useState<HttpRpcClient>()
  const [address, setAddress] = useState<`0x${string}`>();

  const { address: ownerAddress } = useAccount({
    onConnect: async ({ address, connector }) => {
      if (!address || !connector || !window.ethereum) return;

      const nProvider = new providers.Web3Provider(window.ethereum)

      const config = {
        chainId: await nProvider.getNetwork().then((net) => net.chainId),
        entryPointAddress: ENTRY_POINT_ADDRESS,
        bundlerUrl: 'http://localhost:3000/rpc'
      };

      const owner = nProvider.getSigner();

      const aaProvider = await wrapProvider(nProvider, config, owner);

      // const aa = new SimpleAccountAPI({
      //   provider: nProvider,
      //   entryPointAddress: ENTRY_POINT_ADDRESS,
      //   owner,
      //   factoryAddress: SIMPLE_ACCOUNT_FACTORY_ADDRESS
      // });

      const aaAddress = await aaProvider.smartAccountAPI.getAccountAddress();

      // const aaAddress = await aa.getAccountAddress();

      // const op = await aa.createSignedUserOp({
      //   target: contracts[ContractType.CTF].address,
      //   value: 0n,
      //   data,
      //   maxFeePerGas: 0x6507a5d0,
      //   maxPriorityFeePerGas: 0x6507a5c0
      // });

      // Fund the account.
     await nProvider.getSigner().sendTransaction({
        to: ENTRY_POINT_ADDRESS,
        value: 1000000000000000,
        data: `0xb760faf9000000000000000000000000${aaAddress.slice(2)}`,
        gasLimit: 100000,
      }) .then(async (tx) => await tx.wait());

      setAddress(aaAddress as `0x${string}`);
      setAccountAPI(aaProvider.smartAccountAPI);
      setBundlerClient(aaProvider.httpRpcClient)
    }
  });

  console.log(address);

  console.log()

  const state = {
    accountAPI,
    ownerAddress,
    address,
    bundlerClient
  };

  return <accountAbstractionContext.Provider value={state}>{children}</accountAbstractionContext.Provider>;
};

export { AccountAbstractionProvider, useAccountAbstraction };
