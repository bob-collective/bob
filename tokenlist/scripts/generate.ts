import fs from 'node:fs';
import path from 'node:path';
import url from 'node:url';
import { glob } from 'glob';
import {
  baseUrl,
  datadir,
  outfile,
  schema,
  supportedChainMapping,
} from '../config';
import { Entries, Token, TokenData } from '../types';
import { version } from '../package.json';
import { getAddress } from 'viem';

const [major, minor, patch] = version.split('.');

const content = fs
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
            bridge: token.bridge,
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

fs.writeFileSync(outfile, JSON.stringify(content, null, 2));
