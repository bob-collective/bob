import { bob, bobSepolia, mainnet, sepolia } from 'viem/chains';
import { mapByName } from './utils';

export const outfile = 'token-list.json';
export const datadir = './data';
export const baseUrl =
  'https://raw.githubusercontent.com/bob-collective/ethereum-optimism.github.io/refs/heads/master';

const supportedMainnetChains = [mainnet, bob];
const supportedTestnetChains = [sepolia, bobSepolia];
const supportedChain = [...supportedMainnetChains, ...supportedTestnetChains];

export const supportedMainnetChainsMapping = mapByName(supportedMainnetChains);
export const supportedTestnetChainsMapping = mapByName(supportedTestnetChains);
export const supportedChainMapping = mapByName(supportedChain);
