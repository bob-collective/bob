import { getEnrichedRoutes, getNativeToken, getUniqueChains, getTokensForChain } from '../util/route-provider.js';
import { formatUnits } from 'viem';
import { formatBtc } from '@gobob/bob-sdk';
import { getTokenBalance } from '../chains/index.js';
import { BTC_DECIMALS } from '../config.js';
import type { BalanceJson } from '../output.js';

export interface BalanceOptions {
  chain?: string;
  feeToken?: string;
  feeReserve?: string;
}

export async function handleBalance(address: string, opts: BalanceOptions): Promise<BalanceJson> {
  const enriched = await getEnrichedRoutes();
  const chains = opts.chain ? [opts.chain] : getUniqueChains(enriched);

  const entries = await Promise.all(
    chains.map(async (chain): Promise<[string, BalanceJson[string]]> => {
      try {
        if (chain === 'bitcoin') {
          const bal = await getTokenBalance('bitcoin', address, { address: 'BTC', symbol: 'BTC', decimals: BTC_DECIMALS });
          return ['bitcoin', {
            address,
            balance: formatBtc(BigInt(bal.total)),
            allSpendable: formatBtc(BigInt(bal.allSpendable)),
          }];
        }

        const nt = getNativeToken(chain);
        const nativeBal = await getTokenBalance(chain, address,
          { address: '0x0000000000000000000000000000000000000000', symbol: nt.symbol, decimals: nt.decimals },
          { feeToken: opts.feeToken, feeReserve: opts.feeReserve });

        const chainTokens = getTokensForChain(chain, enriched);
        const tokenBals = await Promise.all(chainTokens.map(async (t) => {
          const bal = await getTokenBalance(chain, address, t, { feeToken: opts.feeToken, feeReserve: opts.feeReserve });
          return {
            symbol: t.symbol,
            address: t.address,
            balance: formatUnits(BigInt(bal.total), t.decimals),
            allSpendable: formatUnits(BigInt(bal.allSpendable), t.decimals),
          };
        }));

        return [chain, {
          address,
          native: {
            symbol: nt.symbol,
            balance: formatUnits(BigInt(nativeBal.total), nt.decimals),
            allSpendable: formatUnits(BigInt(nativeBal.allSpendable), nt.decimals),
          },
          tokens: tokenBals,
        }];
      } catch {
        return [chain, { address, error: true }];
      }
    }),
  );

  return Object.fromEntries(entries);
}
