import { supportedChainsMapping } from '@gobob/bob-sdk';
import type { Chain } from 'viem';

/** Resolve RPC URL from env var EVM_RPC_URL_<CHAIN>, or undefined for viem defaults. */
export function resolveRpcUrl(chainName: string): string | undefined {
  return process.env[`EVM_RPC_URL_${chainName.toUpperCase()}`];
}

/** Get the viem Chain object for a gateway chain name, if supported. */
export function getViemChain(chainName: string): Chain | undefined {
  return (supportedChainsMapping as Record<string, Chain>)[chainName];
}
