import { GatewayApiClient } from "../api/client.js";
import { parseAssetChain } from "../util/asset-chain-parser.js";
import { parseAmount } from "../util/amount-parser.js";
import { fetchFeeRate } from "../util/mempool.js";
import { usdToWei } from "../util/price-oracle.js";
import { formatOutput, formatConfirmation } from "../output/formatter.js";
import type { QuoteJson } from "../output/json-shapes.js";

export interface QuoteOptions {
  apiUrl: string;
  src: string;
  dst: string;
  amount: string;
  recipient: string;
  sender?: string;
  slippageBps: number;
  gasRefillUsd?: number;
  btcFeeRate?: number;
  json: boolean;
}

export async function handleQuote(opts: QuoteOptions): Promise<string> {
  const client = new GatewayApiClient(opts.apiUrl);
  const routes = await client.getRoutes();

  const srcAsset = parseAssetChain(opts.src, routes);
  const dstAsset = parseAssetChain(opts.dst, routes);
  const isBtcSrc = srcAsset.chain === "bitcoin";

  const parsed = await parseAmount(opts.amount, srcAsset.symbol, srcAsset.decimals);

  let feeRate = opts.btcFeeRate;
  if (isBtcSrc && !feeRate) feeRate = await fetchFeeRate();

  const gasRefillWei = opts.gasRefillUsd ? await usdToWei(opts.gasRefillUsd) : undefined;

  const quote = await client.getQuote({
    srcChain: srcAsset.chain,
    dstChain: dstAsset.chain,
    srcToken: srcAsset.address,
    dstToken: dstAsset.address,
    amount: parsed.atomicUnits,
    recipient: opts.recipient,
    sender: opts.sender,
    slippage: opts.slippageBps,
    gasRefill: gasRefillWei,
  });

  const shape: QuoteJson = {
    srcAmount: parsed.atomicUnits,
    srcAsset: srcAsset.symbol,
    dstAmount: quote.outputAmount,
    dstAsset: dstAsset.symbol,
    dstChain: dstAsset.chain,
    slippageBps: opts.slippageBps,
    feeRateSatPerVbyte: feeRate,
    gasRefillEth: gasRefillWei,
  };

  if (opts.json) return JSON.stringify(shape, null, 2);

  return formatConfirmation({
    srcAmount: parsed.atomicUnits,
    srcAsset: srcAsset.symbol,
    srcDisplay: parsed.display,
    dstAmount: quote.outputAmount,
    dstAsset: dstAsset.symbol,
    dstChain: dstAsset.chain,
    feeRateSatPerVbyte: feeRate,
    slippageBps: opts.slippageBps,
    recipient: opts.recipient,
  });
}
