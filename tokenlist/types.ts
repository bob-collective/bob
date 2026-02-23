import { Address } from 'viem';
import { SUPPORTED_CHAIN_MAP, SUPPORTED_CHAINS } from './config';
import type { TokenId } from './token-ids';

export type KebabCase<T extends string> = T extends `${infer S} ${infer E}`
  ? `${Lowercase<S>}-${KebabCase<E>}`
  : Lowercase<T>;

export type ValueOf<T> = T[keyof T];
export type Entries<T> = [keyof T, ValueOf<T>][];

export type SupportedChain = keyof typeof SUPPORTED_CHAIN_MAP;
export type SupportedChainId = (typeof SUPPORTED_CHAINS)[number]['id'];

type Overrides = Partial<Pick<TokenData, 'name' | 'symbol' | 'decimals'>>;

export type TokenData = {
  name: string;
  symbol: string;
  decimals: number;
  description?: string;
  website?: string;
  twitter?: string;
  tokens: Record<
    SupportedChain,
    {
      address: Address;
      name?: string;
      symbol?: string;
      decimals?: number;
      bridge?: Record<SupportedChain, Address>;
      overrides?: Overrides;
    }
  >
};

export type Token = {
  name: string;
  address: Address;
  symbol: string;
  decimals: number;
  chainId: number;
  logoURI: string;
  extensions: {
    tokenId: TokenId;
    bridge?: Record<SupportedChainId, Address>;
  };
};
