import { Chain, configureChains, createConfig } from 'wagmi';

import { WalletConnectConnector } from 'wagmi/connectors/walletConnect';
import { publicProvider } from 'wagmi/providers/public';
import { L2_BLOCK_EXPLORER, L2_CHAIN_ID, L2_MULTICALL3_ADDRESS, L2_RPC_URL, L2_WSS_URL } from '../config';

import { InjectedConnector } from 'wagmi/connectors/injected';
import Web3AuthConnectorInstance from './Web3AuthConnectorInstance';
const L2_PROJECT_ID = import.meta.env.VITE_WALLET_CONNECT_PROJECT_ID as string;

const L2_METADATA = {
  name: 'BOB: Peer to Peer Swap',
  description: 'BOB Peer to Peer Swap Demo',
  url: 'https://demo.gobob.xyz',
  icons: ['https://uploads-ssl.webflow.com/64e85c2f3609488b3ed725f4/64ecae53ef4b561482f1c49f_bob1.jpg']
};

const L2_CHAIN_CONFIG = {
  id: L2_CHAIN_ID,
  name: 'BOB L2 Demo',
  network: 'BOB-L2-Demo',
  nativeCurrency: {
    decimals: 18,
    name: 'Bob',
    symbol: 'BOB'
  },
  rpcUrls: {
    public: { http: [L2_RPC_URL], webSocket: [L2_WSS_URL] },
    default: { http: [L2_RPC_URL], webSocket: [L2_WSS_URL] }
  },
  blockExplorers: {
    default: { name: 'BobScan', url: L2_BLOCK_EXPLORER }
  },
  contracts: {
    multicall3: {
      address: L2_MULTICALL3_ADDRESS
    }
  }
} as const satisfies Chain;

const chains = [L2_CHAIN_CONFIG];

const { publicClient, webSocketPublicClient } = configureChains(chains, [publicProvider()]);

const config = createConfig({
  autoConnect: true,
  connectors: [
    new WalletConnectConnector({
      chains,
      options: { projectId: L2_PROJECT_ID, showQrModal: false, metadata: L2_METADATA }
    }),
    new InjectedConnector({ chains, options: { shimDisconnect: true } }), // new InjectedConnector({ chains, options: { shimDisconnect: true } })
    Web3AuthConnectorInstance(chains)
  ],
  webSocketPublicClient,
  publicClient
});

export { L2_CHAIN_CONFIG, L2_METADATA, L2_PROJECT_ID, config, publicClient };
