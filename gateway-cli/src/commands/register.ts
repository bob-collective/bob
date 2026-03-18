import { GatewayApiClient } from "../api/client.js";
import { formatOutput } from "../output/formatter.js";

interface RegisterOptions {
  apiUrl: string;
  orderId: string;
  txid: string;
  json: boolean;
}

export async function handleRegister(
  opts: RegisterOptions,
): Promise<string> {
  const client = new GatewayApiClient(opts.apiUrl);
  const isEvm = opts.txid.startsWith("0x");

  const result = await client.registerTx({
    orderId: opts.orderId,
    ...(isEvm
      ? { evmTxhash: opts.txid }
      : { bitcoinTxid: opts.txid }),
  });

  return formatOutput(result, opts.json ? "json" : "human");
}
