import type { GatewayQuote } from '@gobob/bob-sdk';

export type QuoteVariant = 'onramp' | 'offramp' | 'layerZero';

export interface FlatQuote {
  variant: QuoteVariant;
  inputAmount: string;
  outputAmount: string;
  fees: string;
  slippage?: string;
  estimatedTime?: number;
  raw: any; // original SDK quote for fields not in flat shape
}

export function detectVariant(quote: GatewayQuote): QuoteVariant {
  if ('onramp' in quote && quote.onramp) return 'onramp';
  if ('offramp' in quote && quote.offramp) return 'offramp';
  if ('layerZero' in quote && quote.layerZero) return 'layerZero';
  throw new Error('Unknown quote variant');
}

export function flattenQuote(quote: GatewayQuote): FlatQuote {
  const variant = detectVariant(quote);
  const inner = (quote as any)[variant];

  const flat: FlatQuote = {
    variant,
    inputAmount: inner.inputAmount.amount,
    outputAmount: inner.outputAmount.amount,
    fees: inner.fees.amount,
    raw: quote,
  };

  if (variant !== 'layerZero' && inner.slippage !== undefined) {
    flat.slippage = String(inner.slippage);
  }

  if (inner.estimatedTimeInSecs !== undefined) {
    flat.estimatedTime = inner.estimatedTimeInSecs;
  }

  return flat;
}
