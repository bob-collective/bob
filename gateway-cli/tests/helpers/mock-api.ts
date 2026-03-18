import { vi } from "vitest";
import type { RouteInfo } from "../../src/api/types.js";

export const MOCK_ROUTES: RouteInfo[] = [
  {
    srcChain: "bitcoin",
    dstChain: "ethereum",
    srcToken: { address: "0x0000000000000000000000000000000000000000", symbol: "BTC", decimals: 8, chain: "bitcoin" },
    dstToken: { address: "0xA0b86991c6218b36c1D19D4a2e9Eb0cE3606eB48", symbol: "USDC", decimals: 6, chain: "ethereum" },
  },
  {
    srcChain: "ethereum",
    dstChain: "bitcoin",
    srcToken: { address: "0xA0b86991c6218b36c1D19D4a2e9Eb0cE3606eB48", symbol: "USDC", decimals: 6, chain: "ethereum" },
    dstToken: { address: "0x0000000000000000000000000000000000000000", symbol: "BTC", decimals: 8, chain: "bitcoin" },
  },
];

export function createMockClient(overrides: Record<string, any> = {}) {
  return {
    getRoutes: vi.fn().mockResolvedValue(MOCK_ROUTES),
    getQuote: vi.fn().mockResolvedValue({ outputAmount: "5000000000", ...overrides.quote }),
    createOrder: vi.fn().mockResolvedValue({
      orderId: "test-order-id",
      psbtBase64: "dGVzdA==",
      ...overrides.order,
    }),
    registerTx: vi.fn().mockResolvedValue(undefined),
    getOrder: vi.fn().mockResolvedValue({ status: "success", outputAmount: "5000000000", ...overrides.order }),
    getOrders: vi.fn().mockResolvedValue([]),
    getMaxSpendable: vi.fn().mockResolvedValue({ amount: "100000" }),
    ...overrides,
  };
}
