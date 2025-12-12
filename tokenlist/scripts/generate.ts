import fs from 'node:fs';
import path from 'node:path';
import url from 'node:url';
import { glob } from 'glob';
import {
  baseUrl,
  datadir,
  outfile,
  outfileBob,
  outfileOverrides,
  schema,
  supportedChainMapping,
} from '../config';
import { Entries, Token, TokenData } from '../types';
import { version } from '../package.json';
import { Address, getAddress } from 'viem';
import { bob } from 'viem/chains';
import { TokenId } from '../generated-types';

const [major, minor, patch] = version.split('.');

function addTokens(tokens: Token[][]) {
  return tokens.reduce(
    (list, tokens) => {
      list.tokens.push(...tokens);

      return list;
    },
    {
      $schema: schema,
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
        acc[supportedChainMapping[chain].id] = bridgeAddress;

        return acc;
      }, {} as Record<number, Address>);

      const out: Token = {
        chainId: supportedChainMapping[chain].id,
        address: getAddress(token.address),
        name: token.name ?? tokenData.name,
        symbol: token.symbol ?? tokenData.symbol,
        decimals: tokenData.decimals,
        logoURI,
        extensions: {
          tokenId,
          bridge,
        },
      };
      return out;
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
        acc[supportedChainMapping[chain].id] = bridgeAddress;

        return acc;
      }, {} as Record<number, Address>);

      const out: Token = {
        chainId: supportedChainMapping[chain].id,
        address: getAddress(token.address),
        name: token.overrides?.name ?? token.name ?? tokenData.name,
        symbol: token.overrides?.symbol ?? token.symbol ?? tokenData.symbol,
        decimals: tokenData.decimals,
        logoURI,
        extensions: {
          tokenId,
          bridge,
        },
      };
      return out;
    });
  });
}

const tokenlistData = fs
  .readdirSync(datadir)
  .sort((a, b) => {
    return a.toLowerCase().localeCompare(b.toLowerCase());
  })
  .map<[TokenId, TokenData, string]>((folder) => {
    const data: TokenData = JSON.parse(
      fs.readFileSync(path.join(datadir, folder, 'data.json'), 'utf8'),
    );
    const logofiles = glob.sync(path.join(datadir, folder, 'logo.{webp,svg}'));
    const logoext = logofiles[0].endsWith('webp') ? 'webp' : 'svg';

    return [
      folder as TokenId,
      data,
      url.resolve(baseUrl, path.join(datadir, folder, `logo.${logoext}`)),
    ];
  });

// ---- create tokenlist ----
const tokenlist = addTokens(mapToTokenlist(tokenlistData));

fs.writeFileSync(outfile, JSON.stringify(tokenlist, null, 2));

// ---- create BOB tokenlist ----
const bobTokenlist = structuredClone(tokenlist);

bobTokenlist.tokens = tokenlist.tokens.filter(
  (token) => token.chainId === bob.id,
);

fs.writeFileSync(outfileBob, JSON.stringify(bobTokenlist, null, 2));

// ---- create tokenlist with overrides ----
const uiTokenlist = addTokens(mapToOverridesTokenlist(tokenlistData));

fs.writeFileSync(outfileOverrides, JSON.stringify(uiTokenlist, null, 2));
