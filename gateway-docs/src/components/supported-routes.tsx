'use client';

import { useEffect, useState } from 'react';

const ROUTES_API = 'https://gateway-api-mainnet.gobob.xyz/v2/get-routes';
const TOKENLIST_URL =
  'https://raw.githubusercontent.com/bob-collective/tokenlist/main/tokenlist.json';
const ZERO_ADDRESS = '0x0000000000000000000000000000000000000000';

const NON_EVM: Record<string, string> = { bitcoin: 'BTC' };
const CHAIN_LABELS: Record<string, string> = { bsc: 'BSC', bob: 'BOB' };

const CHAIN_IDS: Record<string, number> = {
  arbitrum: 42161,
  avalanche: 43114,
  base: 8453,
  bera: 80094,
  bob: 60808,
  bsc: 56,
  ethereum: 1,
  hyperliquid: 999,
  plasma: 9745,
  polygon: 137,
  sei: 1329,
  soneium: 1868,
  sonic: 146,
  unichain: 130,
};

const EXPLORERS: Record<string, string> = {
  arbitrum: 'https://arbiscan.io',
  avalanche: 'https://snowscan.xyz',
  base: 'https://basescan.org',
  bera: 'https://berascan.com',
  bob: 'https://explorer.gobob.xyz',
  bsc: 'https://bscscan.com',
  ethereum: 'https://etherscan.io',
  hyperliquid: 'https://hyperevmscan.io',
  plasma: 'https://plasmascan.to',
  polygon: 'https://polygonscan.com',
  sei: 'https://seitrace.com',
  soneium: 'https://soneium.blockscout.com',
  sonic: 'https://sonicscan.org',
  unichain: 'https://uniscan.xyz',
};

interface Route {
  srcChain: string;
  srcToken: string;
  dstChain: string;
  dstToken: string;
}

interface TokenListEntry {
  address?: string;
  chainId: number;
  symbol: string;
}

const cap = (s: string) => CHAIN_LABELS[s] ?? s[0].toUpperCase() + s.slice(1);
const truncate = (address: string) => `${address.slice(0, 6)}…${address.slice(-4)}`;

/**
 * Renders the live route table from the Gateway API, so the docs never go stale
 * when a new chain or asset is enabled. Fetched client-side on purpose.
 */
export function SupportedRoutes() {
  const [routes, setRoutes] = useState<Route[] | null>(null);
  const [symbols, setSymbols] = useState<Record<string, string>>({});
  const [error, setError] = useState(false);

  useEffect(() => {
    Promise.all([
      fetch(ROUTES_API)
        .then((r) => r.json() as Promise<Route[]>)
        .catch(() => null),
      fetch(TOKENLIST_URL)
        .then((r) => r.json())
        .then((d) => (d.tokens ?? []) as TokenListEntry[])
        .catch(() => [] as TokenListEntry[]),
    ]).then(([routeData, tokens]) => {
      if (!routeData) {
        setError(true);
        return;
      }

      const map: Record<string, string> = {};
      for (const token of tokens) {
        if (token.address) map[`${token.chainId}:${token.address.toLowerCase()}`] = token.symbol;
      }

      setSymbols(map);
      setRoutes(routeData);
    });
  }, []);

  if (error) return <p>Routes could not be loaded.</p>;
  if (!routes) return <p>Loading routes…</p>;

  const symbolFor = (chain: string, address: string) => {
    if (NON_EVM[chain]) return NON_EVM[chain];

    const chainId = CHAIN_IDS[chain];
    if (chainId && address) {
      const key = `${chainId}:${address.toLowerCase()}`;
      if (symbols[key]) return symbols[key];
    }

    return address ? truncate(address) : '';
  };

  const addressCell = (chain: string, address: string) => {
    if (!address) return '';
    if (address.toLowerCase() === ZERO_ADDRESS) return truncate(address);

    const explorer = EXPLORERS[chain];
    if (!explorer) return truncate(address);

    return (
      <a href={`${explorer}/address/${address}`} target="_blank" rel="noreferrer">
        {truncate(address)}
      </a>
    );
  };

  const groups: Record<string, Route[]> = {};
  for (const route of routes) {
    (groups[route.srcChain] ??= []).push(route);
  }

  return (
    <div>
      {Object.keys(groups)
        .sort()
        .map((chain) => {
          const rows = groups[chain].sort(
            (a, b) =>
              a.dstChain.localeCompare(b.dstChain) || a.srcToken.localeCompare(b.srcToken),
          );

          return (
            <div key={chain}>
              <h2>From {cap(chain)}</h2>
              <table>
                <thead>
                  <tr>
                    <th>Source Token</th>
                    <th>Source Address</th>
                    <th>Destination Chain</th>
                    <th>Destination Token</th>
                    <th>Destination Address</th>
                  </tr>
                </thead>
                <tbody>
                  {rows.map((route, i) => (
                    <tr key={i}>
                      <td>{symbolFor(route.srcChain, route.srcToken)}</td>
                      <td>{addressCell(route.srcChain, route.srcToken)}</td>
                      <td>{cap(route.dstChain)}</td>
                      <td>{symbolFor(route.dstChain, route.dstToken)}</td>
                      <td>{addressCell(route.dstChain, route.dstToken)}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          );
        })}
    </div>
  );
}
