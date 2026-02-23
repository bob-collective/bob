import fs from 'node:fs';
import path from 'node:path';
import url from 'node:url';
import { glob } from 'glob';
import {
  DATA_DIR,
  OUTFILE_BOB,
  OUTFILE_OVERRIDES,
  OUTFILE_TOKENLIST,
  SUPPORTED_CHAIN_MAP,
  TOKENLIST_BASE_URL,
  TOKENLIST_SCHEMA_URL,
} from '../config';
import { Entries, Token, TokenData } from '../types';
import { version } from '../package.json';
import { Address, getAddress } from 'viem';
import { bob } from 'viem/chains';
import type { TokenId } from '../token-ids';

const [major, minor, patch] = version.split('.');

function buildTokenlist(tokens: Token[][]) {
  return tokens.reduce(
    (list, tokens) => {
      list.tokens.push(...tokens);

      return list;
    },
    {
      $schema: TOKENLIST_SCHEMA_URL,
      name: 'BOB Tokens',
      timestamp: new Date().toISOString(),
      version: {
        major: parseInt(major, 10),
        minor: parseInt(minor, 10),
        patch: parseInt(patch, 10),
      },
      tokens: [] as Token[],
    },
  );
}

function mapToTokenlist(data: [TokenId, TokenData, string][]) {
  return data.map(([tokenId, tokenData, logoURI]) => {
    return (
      Object.entries(tokenData.tokens) as Entries<typeof tokenData.tokens>
    ).map(([chain, token]) => {
      const bridge = (
        Object.entries(token.bridge || {}) as Entries<
          NonNullable<typeof token.bridge>
        >
      ).reduce((acc, [chain, bridgeAddress]) => {
        acc[SUPPORTED_CHAIN_MAP[chain].id] = bridgeAddress;

        return acc;
      }, {} as Record<number, Address>);

      return {
        chainId: SUPPORTED_CHAIN_MAP[chain].id,
        address: getAddress(token.address),
        name: token.name ?? tokenData.name,
        symbol: token.symbol ?? tokenData.symbol,
        decimals: token.decimals ?? tokenData.decimals,
        logoURI,
        extensions: {
          tokenId,
          bridge,
        },
      } as Token;
    });
  });
}

function mapToOverridesTokenlist(data: [TokenId, TokenData, string][]) {
  return data.map(([tokenId, tokenData, logoURI]) => {
    return (
      Object.entries(tokenData.tokens) as Entries<typeof tokenData.tokens>
    ).map(([chain, token]) => {
      const bridge = (
        Object.entries(token.bridge || {}) as Entries<
          NonNullable<typeof token.bridge>
        >
      ).reduce((acc, [chain, bridgeAddress]) => {
        acc[SUPPORTED_CHAIN_MAP[chain].id] = bridgeAddress;

        return acc;
      }, {} as Record<number, Address>);

      return {
        chainId: SUPPORTED_CHAIN_MAP[chain].id,
        address: getAddress(token.address),
        name: token.overrides?.name ?? token.name ?? tokenData.name,
        symbol: token.overrides?.symbol ?? token.symbol ?? tokenData.symbol,
        decimals:
          token.overrides?.decimals ?? token.decimals ?? tokenData.decimals,
        logoURI,
        extensions: {
          tokenId,
          bridge,
        },
      } as Token;
    });
  });
}

const tokenlistData = fs
  .readdirSync(DATA_DIR)
  .sort((a, b) => {
    return a.toLowerCase().localeCompare(b.toLowerCase());
  })
  .map<[TokenId, TokenData, string]>((folder) => {
    const data: TokenData = JSON.parse(
      fs.readFileSync(path.join(DATA_DIR, folder, 'data.json'), 'utf8'),
    );
    const logofiles = glob.sync(path.join(DATA_DIR, folder, 'logo.{webp,svg}'));
    const logoext = logofiles[0].endsWith('webp') ? 'webp' : 'svg';

    return [
      folder as TokenId,
      data,
      url.resolve(TOKENLIST_BASE_URL, path.join(DATA_DIR, folder, `logo.${logoext}`)),
    ];
  });

// Build tokenlist
const tokenlist = buildTokenlist(mapToTokenlist(tokenlistData));

fs.writeFileSync(OUTFILE_TOKENLIST, JSON.stringify(tokenlist, null, 2));

// Build BOB tokenlist
const bobTokenlist = structuredClone(tokenlist);

bobTokenlist.tokens = tokenlist.tokens.filter(
  (token) => token.chainId === bob.id,
);

fs.writeFileSync(OUTFILE_BOB, JSON.stringify(bobTokenlist, null, 2));

// Build tokenlist with overrides
const uiTokenlist = buildTokenlist(mapToOverridesTokenlist(tokenlistData));

fs.writeFileSync(OUTFILE_OVERRIDES, JSON.stringify(uiTokenlist, null, 2));
