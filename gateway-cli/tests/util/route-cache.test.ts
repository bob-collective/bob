import { describe, it, expect, vi, beforeEach } from 'vitest';
import { parseTtl, isCacheValid, readCache, writeCache, getOrFetchRoutes } from '../../src/util/route-cache';
import * as fs from 'fs';

vi.mock('fs');

describe('parseTtl', () => {
  it('parses hours', () => expect(parseTtl('24h')).toBe(24 * 60 * 60 * 1000));
  it('parses days', () => expect(parseTtl('7d')).toBe(7 * 24 * 60 * 60 * 1000));
  it('defaults to 24h on invalid', () => expect(parseTtl('invalid')).toBe(24 * 60 * 60 * 1000));
});

describe('isCacheValid', () => {
  it('returns false when cache does not exist', () => {
    vi.mocked(fs.existsSync).mockReturnValue(false);
    expect(isCacheValid('24h')).toBe(false);
  });

  it('returns true when cache is fresh', () => {
    vi.mocked(fs.existsSync).mockReturnValue(true);
    vi.mocked(fs.readFileSync).mockReturnValue(
      JSON.stringify({ fetchedAt: new Date().toISOString(), routes: [] })
    );
    expect(isCacheValid('24h')).toBe(true);
  });

  it('returns false when cache is stale', () => {
    vi.mocked(fs.existsSync).mockReturnValue(true);
    const staleDate = new Date(Date.now() - 25 * 60 * 60 * 1000).toISOString();
    vi.mocked(fs.readFileSync).mockReturnValue(
      JSON.stringify({ fetchedAt: staleDate, routes: [] })
    );
    expect(isCacheValid('24h')).toBe(false);
  });
});

describe('getOrFetchRoutes', () => {
  beforeEach(() => { vi.resetAllMocks(); });

  it('fetches and caches when no cache exists', async () => {
    vi.mocked(fs.existsSync).mockReturnValue(false);
    vi.mocked(fs.mkdirSync).mockReturnValue(undefined as any);
    vi.mocked(fs.writeFileSync).mockReturnValue(undefined);

    const mockRoutes = [{ srcChain: 'bitcoin', srcToken: 'BTC', dstChain: 'bob', dstToken: '0x1' }];
    const fetcher = vi.fn().mockResolvedValue(mockRoutes);

    const result = await getOrFetchRoutes(fetcher, '24h', false);
    expect(result).toEqual(mockRoutes);
    expect(fetcher).toHaveBeenCalled();
    expect(fs.writeFileSync).toHaveBeenCalled();
  });

  it('returns cached routes when cache is fresh', async () => {
    vi.mocked(fs.existsSync).mockReturnValue(true);
    const cachedRoutes = [{ srcChain: 'bitcoin', srcToken: 'BTC', dstChain: 'bob', dstToken: '0x2' }];
    vi.mocked(fs.readFileSync).mockReturnValue(
      JSON.stringify({ fetchedAt: new Date().toISOString(), routes: cachedRoutes })
    );

    const fetcher = vi.fn();
    const result = await getOrFetchRoutes(fetcher, '24h', false);
    expect(result).toEqual(cachedRoutes);
    expect(fetcher).not.toHaveBeenCalled();
  });

  it('bypasses cache when noCache is true', async () => {
    vi.mocked(fs.existsSync).mockReturnValue(true);
    vi.mocked(fs.mkdirSync).mockReturnValue(undefined as any);
    vi.mocked(fs.writeFileSync).mockReturnValue(undefined);
    vi.mocked(fs.readFileSync).mockReturnValue(
      JSON.stringify({ fetchedAt: new Date().toISOString(), routes: [] })
    );

    const freshRoutes = [{ srcChain: 'bitcoin', srcToken: 'BTC', dstChain: 'ethereum', dstToken: '0x3' }];
    const fetcher = vi.fn().mockResolvedValue(freshRoutes);

    const result = await getOrFetchRoutes(fetcher, '24h', true);
    expect(result).toEqual(freshRoutes);
    expect(fetcher).toHaveBeenCalled();
  });
});
