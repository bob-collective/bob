import { CHAIN_NAMESPACES, WALLET_ADAPTERS } from '@web3auth/base';
import { Web3AuthOptions } from '@web3auth/modal';
import { OpenloginAdapter } from '@web3auth/openlogin-adapter';
import { ethers } from 'ethers';
import { createContext, useCallback, useContext, useEffect, useState } from 'react';

import AccountAbstraction from '@safe-global/account-abstraction-kit-poc';
import { Web3AuthModalPack } from '@safe-global/auth-kit';

import Safe, { EthersAdapter } from '@safe-global/protocol-kit';
import { GelatoRelayPack } from '@safe-global/relay-kit';
import { MetaTransactionData } from '@safe-global/safe-core-sdk-types';
import { useQuery } from '@tanstack/react-query';

const goerliChain: Chain = {
  id: '0x5',
  token: 'gETH',
  label: 'Görli',
  shortName: 'gor',
  rpcUrl: 'https://goerli.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161',
  blockExplorerUrl: 'https://goerli.etherscan.io',
  color: '#fbc02d',
  transactionServiceUrl: 'https://safe-transaction-goerli.safe.global'
};

type Chain = {
  id: string;
  token: string;
  rpcUrl: string;
  shortName: string;
  label: string;
  color?: string;
  icon?: string;
  blockExplorerUrl: string;
  transactionServiceUrl?: string;
  faucetUrl?: string;
};

type accountAbstractionContextValue = {
  ownerAddress?: string;
  safes: string[];
  chain?: Chain;
  isAuthenticated: boolean;
  web3Provider?: ethers.providers.Web3Provider;
  loginWeb3Auth: () => void;
  logoutWeb3Auth: () => void;
  safeSelected?: string;
  safeBalance?: string;
  gelatoTaskId?: string;
  relayTransaction: (transaction: MetaTransactionData) => Promise<void>;
  isRelayerLoading: boolean;
  setSafeSelected: React.Dispatch<React.SetStateAction<string>>;
};

const initialState = {
  isAuthenticated: false,
  loginWeb3Auth: () => {},
  logoutWeb3Auth: () => {},
  relayTransaction: async () => {},
  setChainId: () => {},
  setSafeSelected: () => {},
  onRampWithStripe: async () => {},
  safes: [],
  isRelayerLoading: true
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
  // owner address from the email  (provided by web3Auth)
  const [ownerAddress, setOwnerAddress] = useState<string>('');

  // safes owned by the user
  const [safes, setSafes] = useState<string[]>([]);

  // chain selected

  // web3 provider to perform signatures
  const [web3Provider, setWeb3Provider] = useState<ethers.providers.Web3Provider>();

  const isAuthenticated = !!ownerAddress;
  const chain = goerliChain;

  // reset React state when you switch the chain
  useEffect(() => {
    setOwnerAddress('');
    setSafes([]);
    setWeb3Provider(undefined);
    setSafeSelected('');
  }, [chain]);

  // authClient
  const [web3AuthModalPack, setWeb3AuthModalPack] = useState<Web3AuthModalPack>();

  useEffect(() => {
    (async () => {
      const options: Web3AuthOptions = {
        clientId: import.meta.env.VITE_WEB3AUTH_CLIENT_ID || '',
        web3AuthNetwork: 'testnet',
        chainConfig: {
          chainNamespace: CHAIN_NAMESPACES.EIP155,
          chainId: chain.id,
          rpcTarget: chain.rpcUrl
        },
        uiConfig: {
          theme: 'dark',
          loginMethodsOrder: ['google', 'facebook']
        }
      };

      const modalConfig = {
        [WALLET_ADAPTERS.TORUS_EVM]: {
          label: 'torus',
          showOnModal: false
        },
        [WALLET_ADAPTERS.METAMASK]: {
          label: 'Metamask',
          showOnDesktop: true,
          showOnMobile: false
        }
      };

      const openloginAdapter = new OpenloginAdapter({
        loginSettings: {
          mfaLevel: 'mandatory'
        },
        adapterSettings: {
          uxMode: 'popup',
          whiteLabel: {
            name: 'Safe'
          }
        }
      });

      const web3AuthModalPack = new Web3AuthModalPack({
        txServiceUrl: chain.transactionServiceUrl
      });

      await web3AuthModalPack.init({
        options,
        adapters: [openloginAdapter],
        modalConfig
      });

      setWeb3AuthModalPack(web3AuthModalPack);
    })();
  }, [chain]);

  // auth-kit implementation
  const loginWeb3Auth = useCallback(async () => {
    if (!web3AuthModalPack) return;

    try {
      const { safes, eoa } = await web3AuthModalPack.signIn();
      const provider = web3AuthModalPack.getProvider() as ethers.providers.ExternalProvider;

      // we set react state with the provided values: owner (eoa address), chain, safes owned & web3 provider
      setOwnerAddress(eoa);
      setSafes(safes || []);
      setWeb3Provider(new ethers.providers.Web3Provider(provider));
    } catch (error) {
      console.log('error: ', error);
    }
  }, [web3AuthModalPack]);

  useEffect(() => {
    if (web3AuthModalPack && web3AuthModalPack.getProvider()) {
      (async () => {
        await loginWeb3Auth();
      })();
    }
  }, [web3AuthModalPack, loginWeb3Auth]);

  const logoutWeb3Auth = () => {
    web3AuthModalPack?.signOut();
    setOwnerAddress('');
    setSafes([]);
    setWeb3Provider(undefined);
    setSafeSelected('');
  };

  // current safe selected by the user
  const [safeSelected, setSafeSelected] = useState<string>('');

  // conterfactual safe Address if its not deployed yet
  useEffect(() => {
    const getSafeAddress = async () => {
      if (web3Provider) {
        const signer = web3Provider.getSigner();
        const relayPack = new GelatoRelayPack('_H2parOk7AeLqmhXkgRhPjVOUUYz31FNFGJA7CwNEzE_');
        const safeAccountAbstraction = new AccountAbstraction(signer);

        await safeAccountAbstraction.init({ relayPack });

        const hasSafes = safes.length > 0;

        const safeSelected = hasSafes ? safes[0] : await safeAccountAbstraction.getSafeAddress();

        setSafeSelected(safeSelected);
      }
    };

    getSafeAddress();
  }, [safes, web3Provider]);

  const safeBalance = useQuery(['balances', safeSelected], {
    enabled: !!safeSelected,
    queryFn: async () => {
      const balance = await web3Provider?.getBalance(safeSelected);

      return balance?.toString();
    },
    refetchInterval: 30000
  });

  const [isRelayerLoading, setIsRelayerLoading] = useState<boolean>(false);
  const [gelatoTaskId, setGelatoTaskId] = useState<string>();

  // relay-kit implementation using Gelato
  const relayTransaction = async (transaction: MetaTransactionData) => {
    if (!web3Provider) return;

    setIsRelayerLoading(true);

    const signer = web3Provider.getSigner();

    const ethAdapter = new EthersAdapter({
      ethers,
      signerOrProvider: signer
    });

    const safeSDK = await Safe.create({
      ethAdapter,
      safeAddress: safeSelected
    });

    const relayPack = new GelatoRelayPack('_H2parOk7AeLqmhXkgRhPjVOUUYz31FNFGJA7CwNEzE_');

    const transactions: MetaTransactionData[] = [transaction];

    const safeTransaction = await relayPack.createRelayedTransaction({
      safe: safeSDK,
      transactions
    });

    const signedSafeTransaction = await safeSDK.signTransaction(safeTransaction);

    const response = await relayPack.executeRelayTransaction(signedSafeTransaction, safeSDK);

    console.log(`Relay Transaction Task ID: https://relay.gelato.digital/tasks/status/${response.taskId}`);

    setIsRelayerLoading(false);
    setGelatoTaskId(gelatoTaskId);
  };

  const state = {
    ownerAddress,
    chain,
    safes,
    isRelayerLoading,
    gelatoTaskId,
    isAuthenticated,

    web3Provider,

    loginWeb3Auth,
    logoutWeb3Auth,

    relayTransaction,
    safeSelected,
    safeBalance: safeBalance.data,
    setSafeSelected
  };

  return <accountAbstractionContext.Provider value={state}>{children}</accountAbstractionContext.Provider>;
};

export { AccountAbstractionProvider, useAccountAbstraction };
