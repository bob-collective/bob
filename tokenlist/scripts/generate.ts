import fs from 'node:fs';
import path from 'node:path';
import url from 'node:url';
import { glob } from 'glob';
import {
  baseUrl,
  datadir,
  outfile,
  outfileBob,
  schema,
  supportedChainMapping,
} from '../config';
import { Entries, Token, TokenData } from '../types';
import { version } from '../package.json';
import { Address, getAddress } from 'viem';

const [major, minor, patch] = version.split('.');

const tokenlist = fs
  .readdirSync(datadir)
  .sort((a, b) => {
    return a.toLowerCase().localeCompare(b.toLowerCase());
  })
  .map((folder) => {
    const data: TokenData = JSON.parse(
      fs.readFileSync(path.join(datadir, folder, 'data.json'), 'utf8'),
    );
    const logofiles = glob.sync(path.join(datadir, folder, 'logo.{webp,svg}'));
    const logoext = logofiles[0].endsWith('webp') ? 'webp' : 'svg';

    return (Object.entries(data.tokens) as Entries<typeof data.tokens>).map(
      ([chain, token]) => {
        const bridge = (
          Object.entries(token.bridge || {}) as Entries<
            NonNullable<typeof token.bridge>
          >
        ).reduce((acc, [chain, bridgeAddress]) => {
          acc[supportedChainMapping[chain].id] = bridgeAddress;

          return acc;
        }, {} as Record<number, Address>);

        const out = {
          chainId: supportedChainMapping[chain].id,
          address: getAddress(token.address),
          name: token.overrides?.name ?? data.name,
          symbol: token.overrides?.symbol ?? data.symbol,
          decimals: data.decimals,
          logoURI: url.resolve(
            baseUrl,
            path.join(datadir, folder, `logo.${logoext}`),
          ),
          extensions: {
            tokenId: folder,
            bridge,
          },
        };
        return out;
      },
    );
  })
  .reduce(
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

fs.writeFileSync(outfile, JSON.stringify(tokenlist, null, 2));

const bobTokenlist = structuredClone(tokenlist);

bobTokenlist.tokens = tokenlist.tokens.filter(
  (token) => token.chainId === 60808,
);

fs.writeFileSync(outfileBob, JSON.stringify(bobTokenlist, null, 2));
