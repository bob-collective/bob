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

async function getChainBalance(
  address: string,
  chain: string,
  routes: EnrichedRoute[],
  sdk: ReturnType<typeof getSdk>,
): Promise<BalanceJson[string] | null> {
  if (chain === 'bitcoin') {
    const esplora = new EsploraClient();
    const [bal, maxSpendable] = await Promise.all([
      esplora.getBalance(address),
      sdk.getMaxSpendable(address),
    ]);
    const total = bal.confirmed + bal.unconfirmed;
    if (total === 0) return null;
    return { address, balance: formatBtc(BigInt(total)), maxSpendable: formatBtc(BigInt(Number(maxSpendable.amount.amount))) };
  }

  const rpcUrl = resolveRpcUrl(chain);
  const client = createPublicClient({ chain: getViemChain(chain), transport: http(rpcUrl) });
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

  const tokens = chainTokens
    .map((t, i) => ({ ...t, balance: (tokenBalances[i]?.result as bigint) ?? 0n }))
    .filter(t => t.balance > 0n)
    .map(t => ({ symbol: t.symbol, address: t.address, balance: formatUnits(t.balance, t.decimals) }));

  if (nativeBalance === 0n && tokens.length === 0) return null;

  const nt = getNativeToken(chain);
  return {
    address,
    ...(nativeBalance > 0n ? { native: { symbol: nt.symbol, balance: formatUnits(nativeBalance, nt.decimals) } } : {}),
    ...(tokens.length > 0 ? { tokens } : {}),
  };
}

export async function handleBalance(address: string, opts: BalanceOptions): Promise<BalanceJson> {
  const sdk = getSdk();
  const enriched = await getEnrichedRoutes();
  const chains = opts.chain
    ? [opts.chain]
    : [...new Set(enriched.flatMap(r => [r.srcChain, r.dstChain]))];

  const entries = await Promise.all(
    chains.map(async (chain) => {
      const data = await getChainBalance(address, chain, enriched, sdk);
      return data ? [chain, data] as const : null;
    }),
  );

  return Object.fromEntries(entries.filter(Boolean) as [string, BalanceJson[string]][]);
}
