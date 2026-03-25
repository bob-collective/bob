import { getInnerQuote } from "@gobob/bob-sdk";
import { getRoutes } from "../util/route-provider.js";
import { resolveSwapInputs } from "../util/input-resolver.js";
import { MempoolClient } from "@gobob/bob-sdk";
import { loadConfig, getSdk } from "../config.js";
import { resolveRecipient } from "../chains/index.js";
import type { QuoteJson, ConfirmationData } from "../output.js";
import type { GetQuoteParams } from "@gobob/bob-sdk";

export interface QuoteOptions {
  src: string;
  dst: string;
  amount: string;
  recipient?: string;
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

  const routes = await getRoutes();
  const { srcAsset, dstAsset, atomicUnits, display } = await resolveSwapInputs(
    opts.src, opts.dst, opts.amount, routes,
    { senderAddress: opts.sender },
  );

  // ── Resolve recipient ────────────────────────────────────────────────────
  const recipient = await resolveRecipient(dstAsset.chain, opts.recipient, config);

  let feeRate = opts.btcFeeRate ?? config.btcFeeRate;
  if (srcAsset.chain === "bitcoin" && feeRate == null) {
    const fees = await new MempoolClient().getRecommendedFees();
    feeRate = fees.fastestFee;
  }

  const quote = await sdk.getQuote({
    fromChain: srcAsset.chain,
    toChain: dstAsset.chain,
    fromToken: srcAsset.address,
    toToken: dstAsset.address,
    toUserAddress: recipient,
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
      gasRefillUsd: opts.gasRefillUsd ? String(opts.gasRefillUsd) : undefined,
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
      recipient: recipient,
      gasRefillUsd: opts.gasRefillUsd ? String(opts.gasRefillUsd) : undefined,
    },
  };
}
