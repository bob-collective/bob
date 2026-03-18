import { createSdkClient } from "../adapter/sdk-client.js";
import { loadConfig } from "../config/index.js";
import { fetchFeeRate } from "../util/mempool.js";
import type { MaxSpendableJson } from "../output/json-shapes.js";

export interface MaxSpendableOptions {
  address: string;
  btcFeeRate?: number;
  json: boolean;
}

export async function handleMaxSpendable(opts: MaxSpendableOptions): Promise<string> {
  const config = loadConfig();
  const sdk = createSdkClient(config.apiUrl);
  const [result, feeRate] = await Promise.all([
    sdk.getMaxSpendable(opts.address),
    opts.btcFeeRate ? Promise.resolve(opts.btcFeeRate) : fetchFeeRate(),
  ]);

  const shape: MaxSpendableJson = {
    asset: "BTC",
    chain: "bitcoin",
    address: opts.address,
    maxSpendableSat: result.amount.amount,
    balanceSat: result.amount.amount,
    estimatedFeeSat: "0",
    feeRateSatPerVbyte: feeRate,
  };

  if (opts.json) return JSON.stringify(shape, null, 2);

  const sats = BigInt(result.amount.amount);
  const btc = (Number(sats) / 1e8).toFixed(8).replace(/\.?0+$/, "");
  return [
    `Max spendable: ${btc} BTC  (${sats.toLocaleString()} sat)`,
    `  Fee rate: ${feeRate} sat/vbyte  (mempool.space)`,
  ].join("\n");
}
