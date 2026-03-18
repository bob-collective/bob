import { createSdkClient } from "../adapter/sdk-client.js";
import { flattenQuote } from "../adapter/quote-flattener.js";
import { enrichRoutes } from "../adapter/route-enricher.js";
import { getOrFetchRoutes } from "../util/route-cache.js";
import { parseAssetChain } from "../util/asset-chain-parser.js";
import { parseAmount } from "../util/amount-parser.js";
import { fetchFeeRate } from "../util/mempool.js";
import { formatConfirmation } from "../output/formatter.js";
import { loadConfig } from "../config/index.js";
import type { QuoteJson } from "../output/json-shapes.js";
import type { GetQuoteParams } from "@gobob/bob-sdk";

export interface QuoteOptions {
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
  const config = loadConfig();
  const sdk = createSdkClient(config.apiUrl);

  // Get enriched routes for token resolution
  const routes = await getOrFetchRoutes(() => sdk.getRoutes(), config.cache.ttl);
  const enriched = enrichRoutes(routes);

  const srcAsset = parseAssetChain(opts.src, enriched);
  const dstAsset = parseAssetChain(opts.dst, enriched);
  const isBtcSrc = srcAsset.chain === "bitcoin";

  const parsed = await parseAmount(opts.amount, srcAsset.symbol, srcAsset.decimals);

  let feeRate = opts.btcFeeRate;
  if (isBtcSrc && !feeRate) feeRate = await fetchFeeRate();

  // Build SDK params
  const quoteParams: GetQuoteParams = {
    fromChain: srcAsset.chain,
    toChain: dstAsset.chain,
    fromToken: srcAsset.address,
    toToken: dstAsset.address,
    toUserAddress: opts.recipient,
    fromUserAddress: opts.sender,
    amount: parsed.atomicUnits,
    maxSlippage: opts.slippageBps,
  };

  const quote = await sdk.getQuote(quoteParams);
  const flat = flattenQuote(quote);

  const shape: QuoteJson = {
    srcAmount: parsed.atomicUnits,
    srcAsset: srcAsset.symbol,
    dstAmount: flat.outputAmount,
    dstAsset: dstAsset.symbol,
    dstChain: dstAsset.chain,
    slippageBps: opts.slippageBps,
    feeRateSatPerVbyte: feeRate,
  };

  if (opts.json) return JSON.stringify(shape, null, 2);

  return formatConfirmation({
    srcAmount: parsed.atomicUnits,
    srcAsset: srcAsset.symbol,
    srcDisplay: parsed.display,
    dstAmount: flat.outputAmount,
    dstAsset: dstAsset.symbol,
    dstChain: dstAsset.chain,
    feeRateSatPerVbyte: feeRate,
    slippageBps: opts.slippageBps,
    recipient: opts.recipient,
  });
}
