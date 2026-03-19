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

async function getBtcBalance(address: string, sdk: ReturnType<typeof getSdk>) {
  const esplora = new EsploraClient();
  const [balance, maxSpendable] = await Promise.all([
    esplora.getBalance(address),
    sdk.getMaxSpendable(address),
  ]);
  return {
    confirmed: balance.confirmed,
    unconfirmed: balance.unconfirmed,
    maxSpendable: Number(maxSpendable.amount.amount),
  };
}

async function getEvmBalances(
  address: string,
  chain: string,
  routes: EnrichedRoute[],
) {
  const rpcUrl = resolveRpcUrl(chain);
  const viemChain = getViemChain(chain);
  const client = createPublicClient({ chain: viemChain, transport: http(rpcUrl) });

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

  const nonZeroTokens = chainTokens
    .map((t, i) => ({ ...t, balance: (tokenBalances[i]?.result as bigint) ?? 0n }))
    .filter(t => t.balance > 0n)
    .map(t => ({ symbol: t.symbol, address: t.address, balance: formatUnits(t.balance, t.decimals) }));

  const hasNative = nativeBalance > 0n;
  const hasTokens = nonZeroTokens.length > 0;

  if (!hasNative && !hasTokens) return null;

  const nt = getNativeToken(chain);
  return {
    address,
    ...(hasNative ? { native: { symbol: nt.symbol, balance: formatUnits(nativeBalance, nt.decimals) } } : {}),
    ...(hasTokens ? { tokens: nonZeroTokens } : {}),
  };
}

export async function handleBalance(address: string, opts: BalanceOptions): Promise<BalanceJson> {
  const sdk = getSdk();

  const enriched = await getEnrichedRoutes();

  // Determine which chains to query
  const chains = opts.chain
    ? [opts.chain]
    : [...new Set(enriched.flatMap(r => [r.srcChain, r.dstChain]))];

  const result: BalanceJson = {};

  const entries = await Promise.all(chains.map(async (chain): Promise<[string, BalanceJson[string]] | null> => {
    if (chain === 'bitcoin') {
      const btcBalance = await getBtcBalance(address, sdk);
      const total = btcBalance.confirmed + btcBalance.unconfirmed;
      if (total > 0) {
        return ['bitcoin', {
          address,
          balance: formatBtc(BigInt(total)),
          maxSpendable: formatBtc(BigInt(btcBalance.maxSpendable)),
        }];
      }
    } else {
      const evmBalance = await getEvmBalances(address, chain, enriched);
      if (evmBalance) return [chain, evmBalance];
    }
    return null;
  }));

  for (const entry of entries) {
    if (entry) result[entry[0]] = entry[1];
  }

  return result;
}
