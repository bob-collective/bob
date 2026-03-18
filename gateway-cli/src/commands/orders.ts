import { createSdkClient } from "../adapter/sdk-client.js";
import { loadConfig } from "../config/index.js";
import { formatOutput } from "../output/formatter.js";

interface OrdersOptions {
  address: string;
  json: boolean;
}

export async function handleOrders(opts: OrdersOptions): Promise<string> {
  const config = loadConfig();
  const sdk = createSdkClient(config.apiUrl);
  const orders = await sdk.getOrders(opts.address as `0x${string}`);
  return formatOutput(orders, opts.json ? "json" : "human");
}
