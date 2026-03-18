import { createSdkClient } from "../adapter/sdk-client.js";
import { loadConfig } from "../config/index.js";
import { formatOutput } from "../output/formatter.js";

interface StatusOptions {
  orderId: string;
  json: boolean;
}

export async function handleStatus(opts: StatusOptions): Promise<string> {
  const config = loadConfig();
  const sdk = createSdkClient(config.apiUrl);
  const order = await sdk.api.getOrder({ id: opts.orderId });
  return formatOutput(order, opts.json ? "json" : "human");
}
