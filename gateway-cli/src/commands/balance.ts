import { getEnrichedRoutes, type EnrichedRoute, type EnrichedToken } from '../util/route-provider.js';
import { resolveRpcUrl, getViemChain } from '../util/rpc-resolver.js';
import { getSdk } from '../config.js';
import { createPublicClient, http, erc20Abi, formatUnits } from 'viem';
import { EsploraClient, formatBtc } from '@gobob/bob-sdk';
import { getNativeToken } from '../util/route-provider.js';
import type { BalanceJson } from '../output.js';

export interface BalanceOptions {
  chain?: string;
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

async function getBtcBalance(
  address: string,
  sdk: ReturnType<typeof getSdk>,
): Promise<BalanceJson[string]> {
  const esplora = new EsploraClient();
  const [bal, maxSpendable] = await Promise.all([
    esplora.getBalance(address),
    sdk.getMaxSpendable(address),
  ]);
  const total = bal.confirmed + bal.unconfirmed;
  return { address, balance: formatBtc(BigInt(total)), maxSpendable: formatBtc(BigInt(Number(maxSpendable.amount.amount))) };
}

async function getEvmChainBalance(
  address: string,
  chain: string,
  routes: EnrichedRoute[],
): Promise<BalanceJson[string]> {
  const client = createPublicClient({ chain: getViemChain(chain), transport: http(resolveRpcUrl(chain)) });
  const chainTokens = getUniqueTokensForChain(chain, routes);

  const [nativeBalance, tokenBalances] = await Promise.all([
    client.getBalance({ address: address as `0x${string}` }),
    chainTokens.length > 0
      ? client.multicall({
          contracts: chainTokens.map(t => ({
            address: t.address as `0x${string}`,
            abi: erc20Abi,
            functionName: 'balanceOf' as const,
            args: [address as `0x${string}`],
          })),
        })
      : [],
  ]);

  const nt = getNativeToken(chain);
  const tokens = chainTokens
    .map((t, i) => ({ symbol: t.symbol, address: t.address, balance: formatUnits((tokenBalances[i]?.result as bigint) ?? 0n, t.decimals) }));

  return {
    address,
    native: { symbol: nt.symbol, balance: formatUnits(nativeBalance, nt.decimals) },
    tokens,
  };
}

export async function handleBalance(address: string, opts: BalanceOptions): Promise<BalanceJson> {
  const sdk = getSdk();
  const enriched = await getEnrichedRoutes();
  const chains = opts.chain
    ? [opts.chain]
    : getUniqueChains(enriched);

  const entries = await Promise.all(
    chains.map(async (chain): Promise<[string, BalanceJson[string]]> => {
      try {
        const data = chain === 'bitcoin'
          ? await getBtcBalance(address, sdk)
          : await getEvmChainBalance(address, chain, enriched);
        return [chain, data];
      } catch {
        return [chain, { address, error: true }];
      }
    }),
  );

  return Object.fromEntries(entries);
}
