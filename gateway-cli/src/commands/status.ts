import { GatewayApiClient } from "../api/client.js";
import { formatOutput } from "../output/formatter.js";

interface StatusOptions {
  apiUrl: string;
  orderId: string;
  json: boolean;
}

export async function handleStatus(opts: StatusOptions): Promise<string> {
  const client = new GatewayApiClient(opts.apiUrl);
  const order = await client.getOrder(opts.orderId);
  return formatOutput(order, opts.json ? "json" : "human");
}
