import { getInnerQuote } from "@gobob/bob-sdk";
import { getEnrichedRoutes } from "../util/route-provider.js";
import { resolveSwapInputs } from "../util/input-resolver.js";
import { MempoolClient } from "@gobob/bob-sdk";
import { loadConfig, getSdk } from "../config.js";
import type { QuoteJson, ConfirmationData } from "../output.js";
import type { GetQuoteParams } from "@gobob/bob-sdk";

export interface QuoteOptions {
  src: string;
  dst: string;
  amount: string;
  recipient: string;
  sender?: string;
  slippage?: number;
  gasRefillUsd?: number;
  btcFeeRate?: number;
}

export interface QuoteResult {
  quote: QuoteJson;
  confirmation: ConfirmationData;
}

export async function handleQuote(opts: QuoteOptions): Promise<QuoteResult> {
  const config = loadConfig();
  const sdk = getSdk();
  const slippageBps = opts.slippage ?? config.slippageBps;

  const enriched = await getEnrichedRoutes();
  const { srcAsset, dstAsset, atomicUnits, display } = await resolveSwapInputs(
    opts.src, opts.dst, opts.amount, enriched,
    { senderAddress: opts.sender },
  );

  let feeRate = opts.btcFeeRate ?? config.btcFeeRate;
  if (srcAsset.chain === "bitcoin" && !feeRate) {
    const fees = await new MempoolClient().getRecommendedFees();
    feeRate = fees.fastestFee;
  }

  const quote = await sdk.getQuote({
    fromChain: srcAsset.chain,
    toChain: dstAsset.chain,
    fromToken: srcAsset.address,
    toToken: dstAsset.address,
    toUserAddress: opts.recipient,
    fromUserAddress: opts.sender,
    amount: atomicUnits,
    maxSlippage: slippageBps,
  });
  const outputAmount = getInnerQuote(quote).outputAmount.amount;

  return {
    quote: {
      srcAmount: atomicUnits,
      srcAsset: srcAsset.symbol,
      dstAmount: outputAmount,
      dstAsset: dstAsset.symbol,
      dstChain: dstAsset.chain,
      slippageBps,
      feeRateSatPerVbyte: feeRate,
    },
    confirmation: {
      srcAmount: atomicUnits,
      srcAsset: srcAsset.symbol,
      srcDisplay: display,
      dstAmount: outputAmount,
      dstAsset: dstAsset.symbol,
      dstChain: dstAsset.chain,
      feeRateSatPerVbyte: feeRate,
      slippageBps,
      recipient: opts.recipient,
      gasRefillUsd: opts.gasRefillUsd ? String(opts.gasRefillUsd) : undefined,
    },
  };
}
