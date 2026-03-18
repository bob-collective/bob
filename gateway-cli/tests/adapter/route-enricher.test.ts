import { describe, it, expect } from 'vitest';
import { enrichRoute, enrichRoutes, type EnrichedRoute } from '../../src/adapter/route-enricher';

const mockSdkRoute = {
  srcChain: 'bitcoin',
  srcToken: 'BTC',
  dstChain: 'bob',
  dstToken: '0x03c7054bcb39f7b2e5b2c7acb37583e32d70cfa',
};

describe('enrichRoute', () => {
  it('resolves BTC as bitcoin source', () => {
    const enriched = enrichRoute(mockSdkRoute);
    expect(enriched.srcToken).toEqual({
      address: 'BTC',
      symbol: 'BTC',
      decimals: 8,
      chain: 'bitcoin',
    });
  });

  it('resolves EVM token from tokenlist', () => {
    const enriched = enrichRoute(mockSdkRoute);
    expect(enriched.dstToken.chain).toBe('bob');
    expect(enriched.dstToken.decimals).toBeTypeOf('number');
    // If token is in tokenlist, symbol should be a real symbol (not truncated address)
    // If not, it falls back to truncated address
  });

  it('returns truncated address for unknown tokens', () => {
    const route = { ...mockSdkRoute, dstToken: '0x0000000000000000000000000000000000000099' };
    const enriched = enrichRoute(route);
    expect(enriched.dstToken.address).toBe('0x0000000000000000000000000000000000000099');
    expect(enriched.dstToken.decimals).toBe(18); // fallback
  });

  it('preserves chain names from SDK', () => {
    const enriched = enrichRoute(mockSdkRoute);
    expect(enriched.srcChain).toBe('bitcoin');
    expect(enriched.dstChain).toBe('bob');
  });
});

describe('enrichRoutes', () => {
  it('enriches an array of routes', () => {
    const routes = [mockSdkRoute, { ...mockSdkRoute, dstChain: 'ethereum' }];
    const enriched = enrichRoutes(routes);
    expect(enriched).toHaveLength(2);
    expect(enriched[0].dstChain).toBe('bob');
    expect(enriched[1].dstChain).toBe('ethereum');
  });
});
