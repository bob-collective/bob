import { bob, bobSepolia, mainnet, sepolia } from 'viem/chains';
import { mapByName } from './utils';

export const schema =
  'https://raw.githubusercontent.com/Uniswap/token-lists/refs/heads/main/src/tokenlist.schema.json';
export const baseUrl =
  'https://raw.githubusercontent.com/bob-collective/bob/refs/heads/master/tokenlist/';

export const datadir = './data';
export const outfile = 'token-list.json';
export const outfileBob = 'token-list-bob.json';

const supportedMainnetChains = [mainnet, bob];
const supportedTestnetChains = [sepolia, bobSepolia];
export const supportedChains = [
  ...supportedMainnetChains,
  ...supportedTestnetChains,
];

export const supportedMainnetChainsMapping = mapByName(supportedMainnetChains);
export const supportedTestnetChainsMapping = mapByName(supportedTestnetChains);
export const supportedChainMapping = mapByName(supportedChains);
