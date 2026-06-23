import { getInnerQuoteV3 } from "../util/quote.js";
import { getRoutes } from "../util/route-provider.js";
import { resolveSwapInputs, humanToAtomic, parseAssetChain, buildTokenIndex } from "../util/input-resolver.js";
import { fetchPrice } from "../util/price-oracle.js";
import { MempoolClient } from "@gobob/bob-sdk";
import { loadConfig, getSdk } from "../config.js";
import { resolveRecipient, deriveAddress, resolvePrivateKey, privateKeyEnvVar } from "../chains/index.js";
import type { QuoteJson, ConfirmationData } from "../output.js";

/** Quote command options. */
export interface QuoteOptions {
  src: string;
  dst: string;
  amount: string;
  recipient?: string;
  sender?: string;
  ownerAddress?: string;
  slippage?: number;
  gasRefillUsd?: number;
  btcFeeRate?: number;
  feeToken?: string;
  feeReserve?: string;
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
  const tokenIndex = buildTokenIndex(routes);

  // Derive sender from env keys only when needed (--amount ALL requires it).
  let senderAddress = opts.sender;
  if (!senderAddress && opts.amount.toUpperCase() === "ALL") {
    const srcChain = parseAssetChain(opts.src, routes, tokenIndex).chain;
    const key = resolvePrivateKey(srcChain, undefined, config);
    if (key) {
      try {
        senderAddress = await deriveAddress(srcChain, key);
      } catch (err) {
        throw new Error(`Failed to derive sender address from ${privateKeyEnvVar(srcChain)}: ${err instanceof Error ? err.message : String(err)}\n  Use --sender <address> to provide the address directly.`);
      }
    }
  }

  const { srcAsset, dstAsset, atomicUnits, display } = await resolveSwapInputs(
    opts.src, opts.dst, opts.amount, routes,
    { senderAddress, feeToken: opts.feeToken, feeReserve: opts.feeReserve },
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

  // ownerAddress (required by V3) is the EVM-side address controlling the order.
  // Use the explicit --owner override when given; otherwise derive the EVM-side
  // address: recipient for onramp (BTC→EVM), sender for offramp/tokenSwap.
  const ownerAddress = opts.ownerAddress ?? (srcAsset.chain === "bitcoin" ? recipient : (senderAddress ?? recipient));

  const quote = await sdk.getQuote({
    fromChain: srcAsset.chain,
    toChain: dstAsset.chain,
    fromToken: srcAsset.address,
    toToken: dstAsset.address,
    toUserAddress: recipient,
    fromUserAddress: senderAddress,
    ownerAddress,
    amount: atomicUnits,
    maxSlippage: slippageBps,
    gasRefill: gasRefillWei ? BigInt(gasRefillWei) : undefined,
  });
  const outputAmount = getInnerQuoteV3(quote).outputAmount.amount;

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
