import { getEnrichedRoutes, getUniqueChains } from '../util/route-provider.js';
import { getChainBalances } from '../chains/index.js';
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
        return [chain, await getChainBalances(chain, address, { feeToken: opts.feeToken, feeReserve: opts.feeReserve })];
      } catch {
        return [chain, { address, error: true }];
      }
    }),
  );

  return Object.fromEntries(entries);
}
