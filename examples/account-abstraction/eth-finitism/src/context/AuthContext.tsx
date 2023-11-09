import { BaseAccountAPI, wrapProvider } from '@account-abstraction/sdk';
import { providers } from 'ethers';
import { createContext, useContext, useState } from 'react';
import { useAccount } from 'wagmi';
import { L2_RPC_URL } from '../config';
import { ENTRY_POINT_ADDRESS } from '../constants/erc4337';

type accountAbstractionContextValue = {
  accountAPI?: BaseAccountAPI;
  address?: `0x${string}`;
  ownerAddress?: `0x${string}`;
};

const initialState = {
  accountAPI: undefined
};

const accountAbstractionContext = createContext<accountAbstractionContextValue>(initialState);

const useAccountAbstraction = () => {
  const context = useContext(accountAbstractionContext);

  if (!context) {
    throw new Error('useAccountAbstraction should be used within a AccountAbstraction Provider');
  }

  return context;
};

const AccountAbstractionProvider = ({ children }: { children: JSX.Element }) => {
  const [accountAPI, setAccountAPI] = useState<BaseAccountAPI>();
  const [address, setAddress] = useState<`0x${string}`>();

  const { address: ownerAddress } = useAccount({
    onConnect: async ({ address, connector }) => {
      if (!address || !connector) return;

      const nProvider = new providers.JsonRpcProvider(L2_RPC_URL);

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

      setAddress(aaAddress as `0x${string}`);
      setAccountAPI(aaProvider.smartAccountAPI);
    }
  });

  console.log(address);

  const state = {
    accountAPI,
    ownerAddress,
    address
  };

  return <accountAbstractionContext.Provider value={state}>{children}</accountAbstractionContext.Provider>;
};

export { AccountAbstractionProvider, useAccountAbstraction };
