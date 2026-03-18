import { createSdkClient } from "../adapter/sdk-client.js";
import { enrichRoutes } from "../adapter/route-enricher.js";
import { getOrFetchRoutes } from "../util/route-cache.js";
import { loadConfig } from "../config/index.js";
import { formatOutput } from "../output/formatter.js";
import type { EnrichedRoute } from "../adapter/route-enricher.js";

interface RoutesOptions {
  json: boolean;
  from?: string;
  to?: string;
}

export async function handleRoutes(
  opts: RoutesOptions
): Promise<string> {
  const config = loadConfig();
  const sdk = createSdkClient(config.apiUrl);
  const routes = await getOrFetchRoutes(() => sdk.getRoutes(), config.cache.ttl);
  let enriched: EnrichedRoute[] = enrichRoutes(routes);

  if (opts.from) {
    const from = opts.from.toLowerCase();
    enriched = enriched.filter(
      (r) => r.srcChain.toLowerCase() === from
    );
  }
  if (opts.to) {
    const to = opts.to.toLowerCase();
    enriched = enriched.filter(
      (r) => r.dstChain.toLowerCase() === to
    );
  }

  return formatOutput(enriched, opts.json ? "json" : "human");
}
