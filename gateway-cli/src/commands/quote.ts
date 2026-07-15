import { getInnerQuoteV3 } from "../util/quote.js";
import { getRoutes } from "../util/route-provider.js";
import { resolveSwapContext, type SwapContextOptions } from "../util/swap-context.js";
import { MempoolClient } from "@gobob/bob-sdk";
import { loadConfig, getSdk } from "../config.js";
import type { QuoteJson, ConfirmationData } from "../output.js";

/** Quote command options. */
export interface QuoteOptions extends SwapContextOptions {
  btcFeeRate?: number;
}

/** Quote command result with quote data and confirmation display. */
export interface QuoteResult {
  quote: QuoteJson;
  confirmation: ConfirmationData;
}

/**
 * Handle the quote command: fetch a swap quote without executing.
 *
 * The swap is resolved through the SAME {@link resolveSwapContext} that `swap` uses,
 * so the two cannot disagree about the source chain, the sender or the owner — every
 * such disagreement in the past was a bug, most recently an offramp quote sending the
 * Bitcoin recipient as `ownerAddress` and being rejected by the API every time.
 *
 * A quote signs nothing, so it declares no need for a key: one is touched only if the
 * owner or an `--amount ALL` balance genuinely depends on it.
 *
 * @param opts - Quote options including source, destination, amount
 * @returns Quote data and confirmation display info
 */
export async function handleQuote(opts: QuoteOptions): Promise<QuoteResult> {
  const config = loadConfig();
  const sdk = getSdk();

  const routes = await getRoutes();
  const ctx = await resolveSwapContext(opts, routes, config, { signing: false, senderAddress: false });

  // Fee rate is informational only — the Gateway backend determines the actual fee.
  // We show the current mempool rate so users know what to expect.
  let feeRate: number | undefined;
  if (ctx.srcFamily === "bitcoin") {
    feeRate = opts.btcFeeRate ?? config.btcFeeRate;
    if (feeRate == null) {
      const fees = await new MempoolClient().getRecommendedFees();
      feeRate = fees.fastestFee;
    }
  }

  const quote = await sdk.getQuote(ctx.quoteParams);
  const outputAmount = getInnerQuoteV3(quote).outputAmount.amount;

  return {
    quote: {
      srcAmount: ctx.atomicUnits,
      srcAsset: ctx.srcAsset.symbol,
      dstAmount: outputAmount,
      dstAsset: ctx.dstAsset.symbol,
      dstChain: ctx.dstAsset.chain,
      slippageBps: ctx.slippageBps,
      feeRateSatPerVbyte: feeRate,
    },
    confirmation: {
      srcAmount: ctx.atomicUnits,
      srcAsset: ctx.srcAsset.symbol,
      srcDisplay: ctx.display,
      dstAmount: outputAmount,
      dstAsset: ctx.dstAsset.symbol,
      dstChain: ctx.dstAsset.chain,
      feeRateSatPerVbyte: feeRate,
      slippageBps: ctx.slippageBps,
      recipient: ctx.recipient,
    },
  };
}
