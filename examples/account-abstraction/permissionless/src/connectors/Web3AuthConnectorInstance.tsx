// Web3Auth Libraries
import { Web3AuthConnector } from './web3-auth-connector';
import { Web3Auth } from '@web3auth/modal';
import { EthereumPrivateKeyProvider } from '@web3auth/ethereum-provider';
import { OpenloginAdapter } from '@web3auth/openlogin-adapter';
import { Chain } from 'wagmi';
import { optimismGoerli } from 'viem/chains';

export default function Web3AuthConnectorInstance(chains: Chain[]) {
  // Create Web3Auth Instance
  const name = 'My App Name';
  // const chainConfig = {
  //   chainNamespace: 'eip155',
  //   chainId: '0x385',
  //   rpcTarget: L2_RPC_URL,
  //   displayName: 'BOB',
  //   blockExplorer: L2_BLOCK_EXPLORER,
  //   ticker: 'ETH',
  //   tickerName: 'Ethereum'
  // };

  const chainConfig = {
    chainNamespace: 'eip155',
    chainId: '0x1a4',
    rpcTarget: optimismGoerli.rpcUrls.default.http[0],
    displayName: optimismGoerli.name,
    blockExplorer: optimismGoerli.blockExplorers.default.url,
    ticker: optimismGoerli.nativeCurrency.symbol,
    tickerName: optimismGoerli.nativeCurrency.name
  };

  const web3AuthInstance = new Web3Auth({
    clientId: 'BNb7d5td0513e3XyZ3PLzk-jE6YtISkFLLXRKLqMrPzPZXiV9SxqWvn7w0MCDBFsth6w9Fgx9JWW2jcfKR56mUc', // Get your Client ID from the Web3Auth Dashboard
    web3AuthNetwork: 'sapphire_devnet',
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    chainConfig: chainConfig as any,
    uiConfig: {
      appName: name,
      loginMethodsOrder: ['github', 'google'],
      defaultLanguage: 'en',
      modalZIndex: '2147483647'
    },
    enableLogging: true
  });

  // Add openlogin adapter for customisations
  const privateKeyProvider = new EthereumPrivateKeyProvider({
    config: { chainConfig }
  });

  const openloginAdapterInstance = new OpenloginAdapter({
    privateKeyProvider,
    adapterSettings: {
      network: 'cyan',
      uxMode: 'popup'
    }
  });
  web3AuthInstance.configureAdapter(openloginAdapterInstance);

  return new Web3AuthConnector({
    chains,
    options: {
      web3AuthInstance
    }
  });
}
