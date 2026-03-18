import type {
  RouteInfo,
  GatewayQuote,
  GatewayOrderInfo,
  GatewayCreateOrderResponse,
  MaxSpendable,
} from "../../src/api/types.js";

export const mockRoutes: RouteInfo[] = [
  {
    srcChain: "bitcoin",
    dstChain: "base",
    srcToken: {
      address: "0x0000000000000000000000000000000000000000",
      symbol: "BTC",
      decimals: 8,
      chain: "bitcoin",
    },
    dstToken: {
      address: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913",
      symbol: "USDC",
      decimals: 6,
      chain: "base",
    },
  },
];

export const mockQuote: GatewayQuote = {
  quoteId: "quote-123",
  inputAmount: "5000000",
  outputAmount: "4812300000",
  feeBreakdown: {
    solverFee: {
      amount: "12000",
      address: "0x0000000000000000000000000000000000000000",
      chain: "bitcoin",
    },
    protocolFee: {
      amount: "3000",
      address: "0x0000000000000000000000000000000000000000",
      chain: "bitcoin",
    },
  },
  estimatedTimeInSecs: 600,
};

export const mockOrder: GatewayOrderInfo = {
  orderId: "order-456",
  status: "success",
  inputAmount: "5000000",
  outputAmount: "4812300000",
  srcChain: "bitcoin",
  dstChain: "base",
  createdAt: "2026-03-12T10:00:00Z",
};

export const mockCreateOrderResponse: GatewayCreateOrderResponse = {
  orderId: "order-456",
  psbtBase64: "cHNidP8BAH0CAAAA...",
};

export const mockMaxSpendable: MaxSpendable = {
  userAddress: "bc1qexample",
  amount: "10000000",
};
