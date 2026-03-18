import { createSdkClient } from "../adapter/sdk-client.js";
import { enrichRoutes } from "../adapter/route-enricher.js";
import { getOrFetchRoutes } from "../util/route-cache.js";
import { loadConfig } from "../config/index.js";
import { resolveChain } from "../util/chain-ids.js";
import { formatTable } from "../output/formatter.js";
import type { TokenJson } from "../output/json-shapes.js";

export async function handleTokens(opts: {
  chain: string;
  json: boolean;
}): Promise<string> {
  const canonical = resolveChain(opts.chain);
  const config = loadConfig();
  const sdk = createSdkClient(config.apiUrl);
  const routes = await getOrFetchRoutes(() => sdk.getRoutes(), config.cache.ttl);
  const enriched = enrichRoutes(routes);

  const seen = new Set<string>();
  const tokens: TokenJson[] = [];
  for (const route of enriched) {
    for (const token of [route.srcToken, route.dstToken]) {
      if (token.chain === canonical && !seen.has(token.address)) {
        seen.add(token.address);
        tokens.push({ symbol: token.symbol, address: token.address, decimals: token.decimals });
      }
    }
  }

  if (tokens.length === 0) {
    throw new Error(`no tokens found on chain "${canonical}". Run 'gateway-cli chains' to see supported chains.`);
  }

  if (opts.json) return JSON.stringify(tokens, null, 2);

  return formatTable(
    [{ label: "Symbol", width: 10 }, { label: "Address", width: 44 }, { label: "Decimals", width: 0 }],
    tokens.map((t) => [t.symbol, t.address, String(t.decimals)]),
  );
}
