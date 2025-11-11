import { Address } from 'viem';
import { supportedChainMapping, supportedChains } from './config';

export type KebabCase<T extends string> = T extends `${infer S} ${infer E}`
  ? `${Lowercase<S>}-${KebabCase<E>}`
  : Lowercase<T>;

export type ValueOf<T> = T[keyof T];
export type Entries<T> = [keyof T, ValueOf<T>][];

export type SuppertedChain = keyof typeof supportedChainMapping;
export type SupportedChainId = (typeof supportedChains)[number]['id'];

type Overrides = Partial<Pick<TokenData, 'name' | 'symbol'>>;

export type TokenData = {
  name: string;
  symbol: string;
  decimals: number;
  description: string;
  website: string;
  twitter: string;
  tokens: Record<
    SuppertedChain,
    {
      address: Address;
      bridge?: Record<SuppertedChain, Address>;
      overrides?: Overrides;
    }
  >;
};

export type Token = {
  name: string;
  address: Address;
  symbol: string;
  decimals: number;
  chainId: number;
  logoURI: string;
  extensions: {
    tokenId: string;
    bridge?: Record<SupportedChainId, Address>;
  };
};
