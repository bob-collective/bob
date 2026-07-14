import { getInnerQuoteV3, resolveOwnerAddress } from "../util/quote.js";
import { getRoutes } from "../util/route-provider.js";
import { resolveSwapInputs, parseAssetChain, buildTokenIndex } from "../util/input-resolver.js";
import { MempoolClient } from "@gobob/bob-sdk";
import { loadConfig, getSdk } from "../config.js";
import { resolveRecipient, deriveAddress, getChainFamily, resolvePrivateKey } from "../chains/index.js";
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

  // Resolve the source through the authoritative resolver (as `swap` does) rather than
  // re-parsing `opts.src` here: an ad-hoc parser silently disagrees with parseAssetChain on
  // case, aliases and bare symbols, and would touch a key before the asset is even validated.
  const tokenIndex = buildTokenIndex(routes);
  const srcFamily = getChainFamily(parseAssetChain(opts.src, routes, tokenIndex).chain);

  // Derive sender from env keys only when needed:
  //   - `--amount ALL` needs it to read the source balance;
  //   - an EVM source needs it as the ownerAddress — but only when `--owner` did not
  //     already supply one, since resolveOwnerAddress prefers the explicit value.
  // Don't derive eagerly — a malformed EVM key must not break a quote that needs no key.
  const needsSenderAsOwner = srcFamily === "evm" && !opts.ownerAddress;
  const needsSenderForBalance = opts.amount.toUpperCase() === "ALL";
  let senderAddress = opts.sender;
  if (!senderAddress && (needsSenderAsOwner || needsSenderForBalance)) {
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

  const ownerAddress = resolveOwnerAddress({
    explicit: opts.ownerAddress,
    srcChain: srcAsset.chain,
    senderAddress,
    recipient,
  });

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
    },
  };
}
