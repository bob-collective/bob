import {
  arbitrum,
  avalanche,
  base,
  bob,
  bobSepolia,
  bsc,
  mainnet,
  optimism,
  polygon,
  sepolia,
} from 'viem/chains';
import { mapByName } from './utils';

export const schema =
  'https://raw.githubusercontent.com/Uniswap/token-lists/refs/heads/main/src/tokenlist.schema.json';
export const baseUrl =
  'https://raw.githubusercontent.com/bob-collective/bob/refs/heads/master/tokenlist/';

export const datadir = './data';
export const outfile = 'tokenlist.json';
export const outfileBob = 'tokenlist-bob.json';
export const outfileUI = 'tokenlist-overrides.json';

const supportedMainnetChains = [
  mainnet,
  bob,
  bsc,
  base,
  optimism,
  arbitrum,
  polygon,
  avalanche,
];
const supportedTestnetChains = [sepolia, bobSepolia];
export const supportedChains = [
  ...supportedMainnetChains,
  ...supportedTestnetChains,
];

export const supportedMainnetChainsMapping = mapByName(supportedMainnetChains);
export const supportedTestnetChainsMapping = mapByName(supportedTestnetChains);
export const supportedChainMapping = mapByName(supportedChains);
