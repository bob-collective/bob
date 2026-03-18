import { createSdkClient } from '../adapter/sdk-client.js';
import { enrichRoutes, type EnrichedRoute, type EnrichedToken } from '../adapter/route-enricher.js';
import { getOrFetchRoutes } from '../util/route-cache.js';
import { resolveRpcUrl } from '../util/rpc-resolver.js';
import { loadConfig, type Config } from '../config/index.js';
import { createPublicClient, http, erc20Abi, formatUnits } from 'viem';
import { EsploraClient } from '@gobob/bob-sdk';
import type { BalanceJson } from '../output/json-shapes.js';

export interface BalanceOptions {
  chain?: string;
  json: boolean;
  noCache?: boolean;
}

function getNativeSymbol(chain: string): string {
  const symbols: Record<string, string> = {
    bob: 'ETH', ethereum: 'ETH', base: 'ETH', arbitrum: 'ETH', optimism: 'ETH',
    bsc: 'BNB', polygon: 'MATIC', avalanche: 'AVAX',
  };
  return symbols[chain] ?? 'ETH';
}

function formatBtc(sats: number): string {
  return (sats / 1e8).toFixed(8);
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

async function getBtcBalance(address: string, sdk: ReturnType<typeof createSdkClient>) {
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
  config: Config,
) {
  const rpcUrl = resolveRpcUrl(chain, config.rpc);
  const client = createPublicClient({ transport: http(rpcUrl) });

  const nativeBalance = await client.getBalance({ address: address as `0x${string}` });

  const chainTokens = getUniqueTokensForChain(chain, routes);
  const tokenBalances = chainTokens.length > 0
    ? await client.multicall({
        contracts: chainTokens.map(t => ({
          address: t.address as `0x${string}`,
          abi: erc20Abi,
          functionName: 'balanceOf' as const,
          args: [address as `0x${string}`],
        })),
      })
    : [];

  const nonZeroTokens = chainTokens
    .map((t, i) => ({ ...t, balance: (tokenBalances[i]?.result as bigint) ?? 0n }))
    .filter(t => t.balance > 0n)
    .map(t => ({ symbol: t.symbol, address: t.address, balance: formatUnits(t.balance, t.decimals) }));

  const hasNative = nativeBalance > 0n;
  const hasTokens = nonZeroTokens.length > 0;

  if (!hasNative && !hasTokens) return null;

  return {
    address,
    ...(hasNative ? { native: { symbol: getNativeSymbol(chain), balance: formatUnits(nativeBalance, 18) } } : {}),
    ...(hasTokens ? { tokens: nonZeroTokens } : {}),
  };
}

function formatHumanBalance(result: BalanceJson): string {
  const lines: string[] = [];
  for (const [chain, data] of Object.entries(result)) {
    lines.push(`${chain}  (${data.address})`);
    if (data.confirmed !== undefined) {
      lines.push(`  Confirmed:     ${data.confirmed} BTC`);
    }
    if (data.unconfirmed !== undefined) {
      lines.push(`  Unconfirmed:   ${data.unconfirmed} BTC`);
    }
    if (data.maxSpendable !== undefined) {
      lines.push(`  Max spendable: ${data.maxSpendable} BTC`);
    }
    if (data.native) {
      lines.push(`  ${data.native.symbol}: ${data.native.balance}`);
    }
    if (data.tokens) {
      for (const t of data.tokens) {
        lines.push(`  ${t.symbol}: ${t.balance}`);
      }
    }
  }
  if (lines.length === 0) return 'No non-zero balances found.';
  return lines.join('\n');
}

export async function handleBalance(address: string, opts: BalanceOptions): Promise<string> {
  const config = loadConfig();
  const sdk = createSdkClient(config.apiUrl);

  const routes = await getOrFetchRoutes(() => sdk.getRoutes(), config.cache.ttl, opts.noCache);
  const enriched = enrichRoutes(routes);

  // Determine which chains to query
  const chains = opts.chain
    ? [opts.chain]
    : [...new Set(enriched.flatMap(r => [r.srcChain, r.dstChain]))];

  const result: BalanceJson = {};

  for (const chain of chains) {
    if (chain === 'bitcoin') {
      const btcBalance = await getBtcBalance(address, sdk);
      if (btcBalance.confirmed > 0 || btcBalance.unconfirmed > 0) {
        result.bitcoin = {
          address,
          confirmed: formatBtc(btcBalance.confirmed),
          ...(btcBalance.unconfirmed > 0 ? { unconfirmed: formatBtc(btcBalance.unconfirmed) } : {}),
          maxSpendable: formatBtc(btcBalance.maxSpendable),
        };
      }
    } else {
      const evmBalance = await getEvmBalances(address, chain, enriched, config);
      if (evmBalance) {
        result[chain] = evmBalance;
      }
    }
  }

  if (opts.json) return JSON.stringify(result, null, 2);
  return formatHumanBalance(result);
}
