import { GatewayApiClient } from "../api/client.js";
import { formatTable } from "../output/formatter.js";
import type { ChainJson } from "../output/json-shapes.js";
import type { RouteInfo } from "../api/types.js";

function deriveChains(routes: RouteInfo[]): ChainJson[] {
  const seen = new Set<string>();
  const result: ChainJson[] = [];
  for (const route of routes) {
    for (const chain of [route.srcChain, route.dstChain]) {
      if (!seen.has(chain)) {
        seen.add(chain);
        result.push({ canonical: chain, aliases: [], chainId: null });
      }
    }
  }
  return result.sort((a, b) => a.canonical.localeCompare(b.canonical));
}

export async function handleChains(opts: { apiUrl: string; json: boolean }): Promise<string> {
  const client = new GatewayApiClient(opts.apiUrl);
  const routes = await client.getRoutes();
  const data = deriveChains(routes);

  if (opts.json) return JSON.stringify(data, null, 2);

  return formatTable(
    [{ label: "Chain", width: 20 }, { label: "Chain ID", width: 0 }],
    data.map((c) => [c.canonical, String(c.chainId ?? "—")]),
  );
}
