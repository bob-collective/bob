import { GatewayApiClient } from "../api/client.js";
import { formatOutput } from "../output/formatter.js";

interface OrdersOptions {
  apiUrl: string;
  address: string;
  json: boolean;
}

export async function handleOrders(opts: OrdersOptions): Promise<string> {
  const client = new GatewayApiClient(opts.apiUrl);
  const orders = await client.getOrders(opts.address);
  return formatOutput(orders, opts.json ? "json" : "human");
}
