import { createSdkClient } from "../adapter/sdk-client.js";
import { loadConfig } from "../config/index.js";
import { formatOutput } from "../output/formatter.js";
import type { RegisterTx } from "@gobob/bob-sdk";

interface RegisterOptions {
  orderId: string;
  txid: string;
  json: boolean;
}

export async function handleRegister(
  opts: RegisterOptions,
): Promise<string> {
  const config = loadConfig();
  const sdk = createSdkClient(config.apiUrl);
  const isEvm = opts.txid.startsWith("0x");

  const registerTx: RegisterTx = isEvm
    ? { offramp: { orderId: opts.orderId, evmTxhash: opts.txid } }
    : { onramp: { orderId: opts.orderId, bitcoinTxid: opts.txid } };

  const result = await sdk.api.registerTx({ registerTx });
  return formatOutput(result, opts.json ? "json" : "human");
}
