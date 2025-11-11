import fs from 'node:fs';
import path from 'node:path';
import { glob } from 'glob';
import { baseUrl, datadir, outfile, supportedChainMapping } from '../config';
import { Entries, Token, TokenData } from '../types';
import { version } from '../package.json';

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
    const logofiles = glob.sync(
      `${path.join(datadir, folder)}/logo.{webp,svg}`,
    );
    const logoext = logofiles[0].endsWith('webp') ? 'webp' : 'svg';

    return (Object.entries(data.tokens) as Entries<typeof data.tokens>).map(
      ([chain, token]) => {
        console.log('===', chain);
        const out = {
          chainId: supportedChainMapping[chain].id,
          address: token.address,
          name: token.overrides?.name ?? data.name,
          symbol: token.overrides?.symbol ?? data.symbol,
          decimals: token.overrides?.decimals ?? data.decimals,
          logoURI: path.join(baseUrl, 'data', folder, `logo.${logoext}`),
          extensions: {
            opTokenId: folder,
            bridge: token.overrides?.bridge,
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
