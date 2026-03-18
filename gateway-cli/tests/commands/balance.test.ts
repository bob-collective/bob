import { describe, it, expect, vi, beforeEach } from 'vitest';
import { handleBalance } from '../../src/commands/balance.js';

// ─── SDK mock ────────────────────────────────────────────────────────────────

const mockGetMaxSpendable = vi.fn();
const mockGetRoutes = vi.fn();

vi.mock('../../src/adapter/sdk-client.js', () => ({
  createSdkClient: vi.fn(() => ({
    getMaxSpendable: mockGetMaxSpendable,
    getRoutes: mockGetRoutes,
  })),
}));

// ─── Config mock ─────────────────────────────────────────────────────────────

vi.mock('../../src/config/index.js', () => ({
  loadConfig: vi.fn(() => ({
    apiUrl: 'https://example.com',
    cache: { ttl: '24h' },
    rpc: {},
  })),
}));

// ─── Route cache mock (pass-through) ────────────────────────────────────────

vi.mock('../../src/util/route-cache.js', () => ({
  getOrFetchRoutes: vi.fn(async (fetcher: () => Promise<any>) => fetcher()),
}));

// ─── Route enricher mock ────────────────────────────────────────────────────

vi.mock('../../src/adapter/route-enricher.js', () => ({
  enrichRoutes: vi.fn((routes: any) => routes),
}));

// ─── EsploraClient mock ─────────────────────────────────────────────────────

const mockGetBalance = vi.fn();

vi.mock('@gobob/bob-sdk', () => ({
  EsploraClient: vi.fn().mockImplementation(() => ({
    getBalance: mockGetBalance,
  })),
}));

// ─── viem mock ──────────────────────────────────────────────────────────────

const mockViemGetBalance = vi.fn();
const mockMulticall = vi.fn();

vi.mock('viem', () => ({
  createPublicClient: vi.fn(() => ({
    getBalance: mockViemGetBalance,
    multicall: mockMulticall,
  })),
  http: vi.fn(),
  erc20Abi: [],
  formatUnits: vi.fn((value: bigint, decimals: number) => {
    return (Number(value) / 10 ** decimals).toString();
  }),
}));

// ─── RPC resolver mock ──────────────────────────────────────────────────────

vi.mock('../../src/util/rpc-resolver.js', () => ({
  resolveRpcUrl: vi.fn(() => 'https://rpc.example.com'),
}));

// ─── Enriched route fixtures ────────────────────────────────────────────────

const btcToBobRoutes = [
  {
    srcChain: 'bitcoin',
    dstChain: 'bob',
    srcToken: { address: 'BTC', symbol: 'BTC', decimals: 8, chain: 'bitcoin' },
    dstToken: { address: '0xWBTC', symbol: 'WBTC', decimals: 8, chain: 'bob' },
  },
  {
    srcChain: 'bitcoin',
    dstChain: 'bob',
    srcToken: { address: 'BTC', symbol: 'BTC', decimals: 8, chain: 'bitcoin' },
    dstToken: { address: '0xUSDC', symbol: 'USDC', decimals: 6, chain: 'bob' },
  },
];

describe('handleBalance', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockGetRoutes.mockResolvedValue(btcToBobRoutes);
  });

  // ── BTC balance ─────────────────────────────────────────────────────────

  it('returns BTC balance with confirmed + unconfirmed + maxSpendable', async () => {
    mockGetBalance.mockResolvedValueOnce({ confirmed: 50000000, unconfirmed: 1000000, total: 51000000 });
    mockGetMaxSpendable.mockResolvedValueOnce({
      amount: { address: 'BTC', amount: '49500000', chain: 'bitcoin' },
      userAddress: 'bc1qtest',
    });

    const result = await handleBalance('bc1qtest', { chain: 'bitcoin', json: true });
    const parsed = JSON.parse(result);

    expect(parsed.bitcoin).toBeDefined();
    expect(parsed.bitcoin.address).toBe('bc1qtest');
    expect(parsed.bitcoin.confirmed).toBe('0.50000000');
    expect(parsed.bitcoin.unconfirmed).toBe('0.01000000');
    expect(parsed.bitcoin.maxSpendable).toBe('0.49500000');
  });

  it('omits unconfirmed when zero', async () => {
    mockGetBalance.mockResolvedValueOnce({ confirmed: 50000000, unconfirmed: 0, total: 50000000 });
    mockGetMaxSpendable.mockResolvedValueOnce({
      amount: { address: 'BTC', amount: '49500000', chain: 'bitcoin' },
      userAddress: 'bc1qtest',
    });

    const result = await handleBalance('bc1qtest', { chain: 'bitcoin', json: true });
    const parsed = JSON.parse(result);

    expect(parsed.bitcoin.confirmed).toBe('0.50000000');
    expect(parsed.bitcoin.unconfirmed).toBeUndefined();
    expect(parsed.bitcoin.maxSpendable).toBe('0.49500000');
  });

  it('omits bitcoin entirely when balance is zero', async () => {
    mockGetBalance.mockResolvedValueOnce({ confirmed: 0, unconfirmed: 0, total: 0 });
    mockGetMaxSpendable.mockResolvedValueOnce({
      amount: { address: 'BTC', amount: '0', chain: 'bitcoin' },
      userAddress: 'bc1qtest',
    });

    const result = await handleBalance('bc1qtest', { chain: 'bitcoin', json: true });
    const parsed = JSON.parse(result);

    expect(parsed.bitcoin).toBeUndefined();
  });

  // ── EVM balance ─────────────────────────────────────────────────────────

  it('returns EVM native + token balances', async () => {
    mockViemGetBalance.mockResolvedValueOnce(1000000000000000000n); // 1 ETH
    mockMulticall.mockResolvedValueOnce([
      { result: 100000000n, status: 'success' },   // 1 WBTC (8 decimals)
      { result: 5000000n, status: 'success' },      // 5 USDC (6 decimals)
    ]);

    const result = await handleBalance('0xABC', { chain: 'bob', json: true });
    const parsed = JSON.parse(result);

    expect(parsed.bob).toBeDefined();
    expect(parsed.bob.address).toBe('0xABC');
    expect(parsed.bob.native.symbol).toBe('ETH');
    expect(parsed.bob.native.balance).toBe('1');
    expect(parsed.bob.tokens).toHaveLength(2);
    expect(parsed.bob.tokens[0].symbol).toBe('WBTC');
    expect(parsed.bob.tokens[1].symbol).toBe('USDC');
  });

  it('omits EVM chain when all balances are zero', async () => {
    mockViemGetBalance.mockResolvedValueOnce(0n);
    mockMulticall.mockResolvedValueOnce([
      { result: 0n, status: 'success' },
      { result: 0n, status: 'success' },
    ]);

    const result = await handleBalance('0xABC', { chain: 'bob', json: true });
    const parsed = JSON.parse(result);

    expect(parsed.bob).toBeUndefined();
  });

  it('omits tokens array when only native balance exists', async () => {
    mockViemGetBalance.mockResolvedValueOnce(500000000000000000n); // 0.5 ETH
    mockMulticall.mockResolvedValueOnce([
      { result: 0n, status: 'success' },
      { result: 0n, status: 'success' },
    ]);

    const result = await handleBalance('0xABC', { chain: 'bob', json: true });
    const parsed = JSON.parse(result);

    expect(parsed.bob.native).toBeDefined();
    expect(parsed.bob.tokens).toBeUndefined();
  });

  it('omits native when only token balances exist', async () => {
    mockViemGetBalance.mockResolvedValueOnce(0n);
    mockMulticall.mockResolvedValueOnce([
      { result: 100000000n, status: 'success' },
      { result: 0n, status: 'success' },
    ]);

    const result = await handleBalance('0xABC', { chain: 'bob', json: true });
    const parsed = JSON.parse(result);

    expect(parsed.bob.native).toBeUndefined();
    expect(parsed.bob.tokens).toHaveLength(1);
    expect(parsed.bob.tokens[0].symbol).toBe('WBTC');
  });

  // ── Chain filter ────────────────────────────────────────────────────────

  it('queries only the specified chain', async () => {
    mockViemGetBalance.mockResolvedValueOnce(1000000000000000000n);
    mockMulticall.mockResolvedValueOnce([
      { result: 0n, status: 'success' },
      { result: 0n, status: 'success' },
    ]);

    const result = await handleBalance('0xABC', { chain: 'bob', json: true });
    const parsed = JSON.parse(result);

    // Should only have bob, not bitcoin
    expect(parsed.bitcoin).toBeUndefined();
    expect(parsed.bob).toBeDefined();
  });

  // ── Multi-chain (no --chain) ──────────────────────────────────────────

  it('queries all chains from routes when no --chain specified', async () => {
    // BTC balance
    mockGetBalance.mockResolvedValueOnce({ confirmed: 10000000, unconfirmed: 0, total: 10000000 });
    mockGetMaxSpendable.mockResolvedValueOnce({
      amount: { address: 'BTC', amount: '9500000', chain: 'bitcoin' },
      userAddress: 'bc1qtest',
    });
    // Bob balance
    mockViemGetBalance.mockResolvedValueOnce(2000000000000000000n);
    mockMulticall.mockResolvedValueOnce([
      { result: 50000000n, status: 'success' },
      { result: 0n, status: 'success' },
    ]);

    const result = await handleBalance('bc1qtest', { json: true });
    const parsed = JSON.parse(result);

    expect(parsed.bitcoin).toBeDefined();
    expect(parsed.bob).toBeDefined();
    expect(parsed.bitcoin.confirmed).toBe('0.10000000');
    expect(parsed.bob.native.balance).toBe('2');
  });

  // ── Human-readable output ─────────────────────────────────────────────

  it('returns human-readable format when json is false', async () => {
    mockGetBalance.mockResolvedValueOnce({ confirmed: 50000000, unconfirmed: 1000000, total: 51000000 });
    mockGetMaxSpendable.mockResolvedValueOnce({
      amount: { address: 'BTC', amount: '49500000', chain: 'bitcoin' },
      userAddress: 'bc1qtest',
    });

    const result = await handleBalance('bc1qtest', { chain: 'bitcoin', json: false });

    expect(result).toContain('bitcoin');
    expect(result).toContain('bc1qtest');
    expect(result).toContain('Confirmed:');
    expect(result).toContain('Unconfirmed:');
    expect(result).toContain('Max spendable:');
    expect(result).toContain('BTC');
  });

  it('returns empty message when no balances found', async () => {
    mockGetBalance.mockResolvedValueOnce({ confirmed: 0, unconfirmed: 0, total: 0 });
    mockGetMaxSpendable.mockResolvedValueOnce({
      amount: { address: 'BTC', amount: '0', chain: 'bitcoin' },
      userAddress: 'bc1qtest',
    });
    mockViemGetBalance.mockResolvedValueOnce(0n);
    mockMulticall.mockResolvedValueOnce([
      { result: 0n, status: 'success' },
      { result: 0n, status: 'success' },
    ]);

    const result = await handleBalance('bc1qtest', { json: false });

    expect(result).toBe('No non-zero balances found.');
  });
});
