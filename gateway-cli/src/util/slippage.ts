import type { GatewayQuoteV4, GetQuoteParams } from "@gobob/bob-sdk";
import type { SlippageJson } from "../output.js";
import { getInnerQuoteV4 } from "./quote.js";

/**
 * The slippage tolerance a quote was actually priced with, in basis points, or
 * `undefined` if the quote carries no usable value.
 *
 * The gateway resolves it per route when the request omits `slippage`, and echoes an
 * explicit value back otherwise — so this, not the flag, is the figure to show. The
 * onramp variant carries it as a string, offramp and tokenSwap as numbers.
 *
 * Parsed strictly: `Number("")` is `0`, so a blank onramp string would otherwise be
 * reported as zero tolerance rather than as missing.
 */
export function quotedSlippageBps(quote: GatewayQuoteV4): number | undefined {
  const raw = getInnerQuoteV4(quote).slippage;
  const bps = typeof raw === "string" ? (/^\d+$/.test(raw.trim()) ? Number(raw) : NaN) : raw;
  return Number.isSafeInteger(bps) && bps >= 0 ? bps : undefined;
}

/** An explicit `--slippage` that undercuts what the gateway recommends for the route. */
export interface SlippageWarning {
  requestedBps: number;
  recommendedBps: number;
  message: string;
}

/**
 * Compare an explicit `--slippage` against the gateway's recommendation for the route.
 *
 * With an explicit value the gateway echoes it back, so the recommendation is only
 * visible by asking for a second quote without one. A tolerance below it is accepted
 * by the quote but is likely to fail at settlement — the order refunds instead of
 * filling — so the user is warned, not refused: they may have a reason to be tight.
 *
 * The probe is best-effort. It decides nothing but a warning, so a failed probe must
 * never fail the quote or swap that asked for it.
 */
export async function checkSlippage(
  getQuote: (params: GetQuoteParams) => Promise<GatewayQuoteV4>,
  params: GetQuoteParams,
): Promise<SlippageWarning | undefined> {
  const requestedBps = params.maxSlippage;
  if (requestedBps === undefined) return undefined;

  let recommendedBps: number | undefined;
  try {
    recommendedBps = quotedSlippageBps(await getQuote({ ...params, maxSlippage: undefined }));
  } catch {
    return undefined;
  }
  if (recommendedBps === undefined || requestedBps >= recommendedBps) return undefined;

  return {
    requestedBps,
    recommendedBps,
    message:
      `--slippage ${requestedBps} bps is below the gateway's recommended ${recommendedBps} bps for this route.\n` +
      `  The swap will likely fail and be refunded. Omit --slippage to use the recommendation.`,
  };
}

/**
 * The JSON slippage fields for a quote: what it was priced with, whether the user asked
 * for it, and the warning from {@link checkSlippage}, if any.
 */
export function slippageJson(
  quote: GatewayQuoteV4,
  requestedBps: number | undefined,
  warning: SlippageWarning | undefined,
): SlippageJson {
  return {
    slippageBps: quotedSlippageBps(quote),
    slippageRequested: requestedBps !== undefined,
    ...(warning && { recommendedSlippageBps: warning.recommendedBps, slippageWarning: warning.message }),
  };
}
