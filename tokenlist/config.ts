import { bob, bobSepolia, mainnet, sepolia } from 'viem/chains';
import { mapByName } from './utils';

export const schema =
  'https://raw.githubusercontent.com/Uniswap/token-lists/refs/heads/main/src/tokenlist.schema.json';
export const outfile = 'token-list.json';
export const datadir = './data';
export const baseUrl =
  'https://raw.githubusercontent.com/bob-collective/bob/refs/heads/master/tokenlist/';

const supportedMainnetChains = [mainnet, bob];
const supportedTestnetChains = [sepolia, bobSepolia];
const supportedChain = [...supportedMainnetChains, ...supportedTestnetChains];

export const supportedMainnetChainsMapping = mapByName(supportedMainnetChains);
export const supportedTestnetChainsMapping = mapByName(supportedTestnetChains);
export const supportedChainMapping = mapByName(supportedChain);
