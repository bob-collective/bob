import { getEnrichedRoutes, type EnrichedRoute, type EnrichedToken, getNativeToken } from '../util/route-provider.js';
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

function getUniqueChains(routes: EnrichedRoute[]): string[] {
  return [...new Set(routes.flatMap(r => [r.srcChain, r.dstChain]))];
}

function getUniqueTokensForChain(
  chain: string,
  routes: EnrichedRoute[],
): EnrichedToken[] {
  const seen = new Set<string>();
  const tokens: EnrichedToken[] = [];
  for (const r of routes) {
    for (const t of [r.srcToken, r.dstToken]) {
      if (t.chain === chain && t.address !== 'BTC' && !seen.has(t.address.toLowerCase())) {
        seen.add(t.address.toLowerCase());
        tokens.push(t);
      }
    }
  }
  return tokens;
}

async function getBtcChainBalance(
  address: string,
  opts: BalanceOptions,
): Promise<BalanceJson[string]> {
  const bal = await getTokenBalance('bitcoin', address, { address: 'BTC', symbol: 'BTC', decimals: BTC_DECIMALS });
  return {
    address,
    balance: formatBtc(BigInt(bal.total)),
    allSpendable: formatBtc(BigInt(Number(bal.allSpendable))),
  };
}

async function getEvmChainBalance(
  address: string,
  chain: string,
  routes: EnrichedRoute[],
  opts: BalanceOptions,
): Promise<BalanceJson[string]> {
  const chainTokens = getUniqueTokensForChain(chain, routes);
  const nt = getNativeToken(chain);

  const nativeToken = { address: '0x0000000000000000000000000000000000000000', symbol: nt.symbol, decimals: nt.decimals };
  const nativeBal = await getTokenBalance(chain, address, nativeToken, {
    feeToken: opts.feeToken,
    feeReserve: opts.feeReserve,
  });

  const tokenBalances = await Promise.all(
    chainTokens.map(async (t) => {
      const bal = await getTokenBalance(chain, address, t, {
        feeToken: opts.feeToken,
        feeReserve: opts.feeReserve,
      });
      return {
        symbol: t.symbol,
        address: t.address,
        balance: formatUnits(BigInt(bal.total), t.decimals),
        allSpendable: formatUnits(BigInt(bal.allSpendable), t.decimals),
      };
    }),
  );

  return {
    address,
    native: {
      symbol: nt.symbol,
      balance: formatUnits(BigInt(nativeBal.total), nt.decimals),
      allSpendable: formatUnits(BigInt(nativeBal.allSpendable), nt.decimals),
    },
    tokens: tokenBalances,
  };
}

export async function handleBalance(address: string, opts: BalanceOptions): Promise<BalanceJson> {
  const enriched = await getEnrichedRoutes();
  const chains = opts.chain
    ? [opts.chain]
    : getUniqueChains(enriched);

  const entries = await Promise.all(
    chains.map(async (chain): Promise<[string, BalanceJson[string]]> => {
      try {
        const data = chain === 'bitcoin'
          ? await getBtcChainBalance(address, opts)
          : await getEvmChainBalance(address, chain, enriched, opts);
        return [chain, data];
      } catch {
        return [chain, { address, error: true }];
      }
    }),
  );

  return Object.fromEntries(entries);
}
