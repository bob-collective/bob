import { getInnerQuote } from "@gobob/bob-sdk";
import { getRoutes } from "../util/route-provider.js";
import { resolveSwapInputs, humanToAtomic } from "../util/input-resolver.js";
import { fetchPrice } from "../util/price-oracle.js";
import { MempoolClient } from "@gobob/bob-sdk";
import { loadConfig, getSdk } from "../config.js";
import { resolveRecipient, deriveAddress, getChainFamily, resolvePrivateKey } from "../chains/index.js";
import type { QuoteJson, ConfirmationData } from "../output.js";
import type { GetQuoteParams } from "@gobob/bob-sdk";

/** Quote command options. */
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

/** Quote command result with quote data and confirmation display. */
export interface QuoteResult {
  quote: QuoteJson;
  confirmation: ConfirmationData;
}

/**
 * Handle the quote command: fetch a swap quote without executing.
 * Resolves asset inputs, recipient, and fee rate, then requests quote from Gateway.
 * 
 * @param opts - Quote options including source, destination, amount
 * @returns Quote data and confirmation display info
 */
export async function handleQuote(opts: QuoteOptions): Promise<QuoteResult> {
  const config = loadConfig();
  const sdk = getSdk();
  const slippageBps = opts.slippage ?? config.slippageBps;

  const routes = await getRoutes();

  // Derive sender from env keys only when needed (--amount ALL requires it).
  // Don't derive eagerly — a malformed key shouldn't break ordinary quotes.
  let senderAddress = opts.sender;
  if (!senderAddress && opts.amount.toUpperCase() === "ALL") {
    const srcRaw = opts.src.includes(":") ? opts.src.split(":")[1] : opts.src;
    const srcFamily = getChainFamily(srcRaw === "BTC" || srcRaw === "btc" ? "bitcoin" : srcRaw);
    const key = resolvePrivateKey(srcFamily === "bitcoin" ? "bitcoin" : "evm", undefined, config);
    if (key) {
      try {
        senderAddress = await deriveAddress(srcFamily === "bitcoin" ? "bitcoin" : "evm", key);
      } catch (err) {
        const envVar = srcFamily === "bitcoin" ? "BITCOIN_PRIVATE_KEY" : "EVM_PRIVATE_KEY";
        throw new Error(`Failed to derive sender address from ${envVar}: ${err instanceof Error ? err.message : String(err)}\n  Use --sender <address> to provide the address directly.`);
      }
    }
  }

  const { srcAsset, dstAsset, atomicUnits, display } = await resolveSwapInputs(
    opts.src, opts.dst, opts.amount, routes,
    { senderAddress },
  );

  // ── Resolve recipient ────────────────────────────────────────────────────
  const recipient = await resolveRecipient(dstAsset.chain, opts.recipient, config);

  // Fee rate is informational only — the Gateway backend determines the actual fee.
  // We show the current mempool rate so users know what to expect.
  let feeRate: number | undefined;
  if (srcAsset.chain === "bitcoin") {
    feeRate = opts.btcFeeRate ?? config.btcFeeRate;
    if (feeRate == null) {
      const fees = await new MempoolClient().getRecommendedFees();
      feeRate = fees.fastestFee;
    }
  }

  const gasRefillWei = opts.gasRefillUsd
    ? humanToAtomic((opts.gasRefillUsd / (await fetchPrice("ETH")).priceUsd).toFixed(18), 18)
    : undefined;

  const quote = await sdk.getQuote({
    fromChain: srcAsset.chain,
    toChain: dstAsset.chain,
    fromToken: srcAsset.address,
    toToken: dstAsset.address,
    toUserAddress: recipient,
    fromUserAddress: senderAddress,
    amount: atomicUnits,
    maxSlippage: slippageBps,
    gasRefill: gasRefillWei ? BigInt(gasRefillWei) : undefined,
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
