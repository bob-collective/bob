import { createSdkClient } from "../adapter/sdk-client.js";
import { enrichRoutes } from "../adapter/route-enricher.js";
import { getOrFetchRoutes } from "../util/route-cache.js";
import { loadConfig } from "../config/index.js";
import { formatTable } from "../output/formatter.js";
import type { ChainJson } from "../output/json-shapes.js";
import type { EnrichedRoute } from "../adapter/route-enricher.js";

function deriveChains(routes: EnrichedRoute[]): ChainJson[] {
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

export async function handleChains(opts: { json: boolean }): Promise<string> {
  const config = loadConfig();
  const sdk = createSdkClient(config.apiUrl);
  const routes = await getOrFetchRoutes(() => sdk.getRoutes(), config.cache.ttl);
  const enriched = enrichRoutes(routes);
  const data = deriveChains(enriched);

  if (opts.json) return JSON.stringify(data, null, 2);

  return formatTable(
    [{ label: "Chain", width: 20 }, { label: "Chain ID", width: 0 }],
    data.map((c) => [c.canonical, String(c.chainId ?? "\u2014")]),
  );
}
