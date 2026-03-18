import { describe, it, expect } from 'vitest';
import { flattenQuote, detectVariant, type FlatQuote } from '../../src/adapter/quote-flattener';

describe('detectVariant', () => {
  it('detects onramp', () => {
    expect(detectVariant({ onramp: {} } as any)).toBe('onramp');
  });
  it('detects offramp', () => {
    expect(detectVariant({ offramp: {} } as any)).toBe('offramp');
  });
  it('detects layerZero', () => {
    expect(detectVariant({ layerZero: {} } as any)).toBe('layerZero');
  });
  it('throws on unknown', () => {
    expect(() => detectVariant({} as any)).toThrow('Unknown quote variant');
  });
});

describe('flattenQuote', () => {
  it('flattens an onramp quote', () => {
    const sdkQuote = {
      onramp: {
        inputAmount: { amount: '100000000', address: 'BTC', chain: 'bitcoin' },
        outputAmount: { amount: '99000000', address: '0xabc', chain: 'bob' },
        fees: { amount: '1000000', address: 'BTC', chain: 'bitcoin' },
        slippage: '300',
        estimatedTimeInSecs: 600,
      },
    };
    const flat = flattenQuote(sdkQuote as any);
    expect(flat.inputAmount).toBe('100000000');
    expect(flat.outputAmount).toBe('99000000');
    expect(flat.fees).toBe('1000000');
    expect(flat.slippage).toBe('300');
    expect(flat.estimatedTime).toBe(600);
    expect(flat.variant).toBe('onramp');
  });

  it('flattens an offramp quote (converts slippage number to string)', () => {
    const sdkQuote = {
      offramp: {
        inputAmount: { amount: '1000000', address: '0xabc', chain: 'bob' },
        outputAmount: { amount: '99000', address: 'BTC', chain: 'bitcoin' },
        fees: { amount: '1000', address: '0xabc', chain: 'bob' },
        slippage: 300,
        estimatedTimeInSecs: 900,
      },
    };
    const flat = flattenQuote(sdkQuote as any);
    expect(flat.slippage).toBe('300');
    expect(flat.variant).toBe('offramp');
  });

  it('flattens a layerZero quote (no slippage)', () => {
    const sdkQuote = {
      layerZero: {
        inputAmount: { amount: '500000', address: '0xabc', chain: 'ethereum' },
        outputAmount: { amount: '490000', address: '0xdef', chain: 'bob' },
        fees: { amount: '10000', address: '0xabc', chain: 'ethereum' },
        estimatedTimeInSecs: 300,
      },
    };
    const flat = flattenQuote(sdkQuote as any);
    expect(flat.slippage).toBeUndefined();
    expect(flat.variant).toBe('layerZero');
  });

  it('preserves raw quote for downstream access', () => {
    const sdkQuote = {
      onramp: {
        inputAmount: { amount: '100', address: 'BTC', chain: 'bitcoin' },
        outputAmount: { amount: '99', address: '0x1', chain: 'bob' },
        fees: { amount: '1', address: 'BTC', chain: 'bitcoin' },
      },
    };
    const flat = flattenQuote(sdkQuote as any);
    expect(flat.raw).toBe(sdkQuote);
  });
});
