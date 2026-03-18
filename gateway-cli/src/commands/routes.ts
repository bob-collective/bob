import { GatewayApiClient } from "../api/client.js";
import { formatOutput } from "../output/formatter.js";
import type { RouteInfo } from "../api/types.js";

interface RoutesOptions {
  apiUrl: string;
  json: boolean;
  from?: string;
  to?: string;
}

export async function handleRoutes(
  opts: RoutesOptions
): Promise<string> {
  const client = new GatewayApiClient(opts.apiUrl);
  let routes: RouteInfo[] = await client.getRoutes();

  if (opts.from) {
    const from = opts.from.toLowerCase();
    routes = routes.filter(
      (r) => r.srcChain.toLowerCase() === from
    );
  }
  if (opts.to) {
    const to = opts.to.toLowerCase();
    routes = routes.filter(
      (r) => r.dstChain.toLowerCase() === to
    );
  }

  return formatOutput(routes, opts.json ? "json" : "human");
}
