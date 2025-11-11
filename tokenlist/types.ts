import { Address } from 'viem';
import { supportedChainMapping } from './config';

export type KebabCase<T extends string> = T extends `${infer S}${infer E}`
  ? E extends `${Uncapitalize<E>}`
    ? `${Uncapitalize<S>}${KebabCase<E>}`
    : `${Uncapitalize<S>}-${KebabCase<E>}`
  : T;

export type ValueOf<T> = T[keyof T];
export type Entries<T> = [keyof T, ValueOf<T>][];

export type SuppertedChain = keyof typeof supportedChainMapping;

type Overrides = Partial<
  Pick<TokenData, Exclude<keyof TokenData, 'tokens'>> & {
    bridge: Record<SuppertedChain, Address>;
  }
>;

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
    opTokenId: string;
    bridge?: Record<SuppertedChain, Address>;
  };
};
