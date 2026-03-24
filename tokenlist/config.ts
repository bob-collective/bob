import {
  arbitrum,
  avalanche,
  base,
  berachain,
  bob,
  bobSepolia,
  bsc,
  mainnet,
  optimism,
  polygon,
  sei,
  sepolia,
  soneium,
  sonic,
  swellchain,
  telos,
  unichain,
} from 'viem/chains';
import { defineChain } from 'viem'
import { mapByName } from './utils';

export const TOKENLIST_SCHEMA_URL =
  'https://raw.githubusercontent.com/Uniswap/token-lists/refs/heads/main/src/tokenlist.schema.json';
export const TOKENLIST_BASE_URL =
  'https://raw.githubusercontent.com/bob-collective/bob/refs/heads/master/tokenlist/';

export const DATA_DIR = './data';
export const OUTFILE_TOKENLIST = 'tokenlist.json';
export const OUTFILE_BOB = 'tokenlist-bob.json';
export const OUTFILE_OVERRIDES = 'tokenlist-overrides.json';
export const OUTFILE_TYPES = 'token-ids.ts';

const supportedMainnetChains = [
  mainnet,
  bob,
  bsc,
  base,
  defineChain({
    ...optimism,
    rpcUrls: {
      default: {
        http: ['https://optimism-rpc.publicnode.com'],
      },
    }
  }),
  arbitrum,
  polygon,
  avalanche,
  unichain,
  swellchain,
  sei,
  soneium,
  berachain,
  sonic,
  telos,
];
const supportedTestnetChains = [sepolia, bobSepolia];
export const SUPPORTED_CHAINS = [
  ...supportedMainnetChains,
  ...supportedTestnetChains,
];

export const SUPPORTED_MAINNET_CHAINS = mapByName(supportedMainnetChains);
export const SUPPORTED_TESTNET_CHAINS = mapByName(supportedTestnetChains);
export const SUPPORTED_CHAIN_MAP = mapByName(SUPPORTED_CHAINS);
