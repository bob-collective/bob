// src/output/json-shapes.ts

export interface SwapSuccessJson {
  orderId: string;
  status: "confirmed" | "strategy_skipped" | "strategy_failed";
  srcAmount: string;
  srcAsset: string;
  dstAmount: string;
  dstAsset: string;
  dstChain: string;
  quotedDstAmount: string;
  actualSlippageBps: number;
  txId: string;
  elapsedMs: number;
}

export interface SwapSubmittedJson {
  orderId: string;
  status: "submitted";
  srcAmount: string;
  srcAsset: string;
  dstAsset: string;
  dstChain: string;
  txId: string;
}

export interface SwapMempoolPendingJson {
  orderId: string;
  status: "mempool_pending";
  srcAmount: string;
  srcAsset: string;
  dstAsset: string;
  dstChain: string;
  mempoolTxId: string;
}

export interface QuoteJson {
  srcAmount: string;
  srcAsset: string;
  dstAmount: string;
  dstAsset: string;
  dstChain: string;
  feeNetworkBtc?: string;
  feeProtocolBps?: number;
  slippageBps: number;
  feeRateSatPerVbyte?: number;
  gasEstimateEth?: string;
  gasRefillEth?: string;
}

export interface ErrorJson {
  error: {
    code: number;
    message: string;
    orderId?: string;
    txId?: string;
    retryable?: boolean;
  };
}

export interface ChainJson {
  canonical: string;
  aliases: string[];
  chainId: number | null;
}

export interface TokenJson {
  symbol: string;
  address: string;
  decimals: number;
}

export interface MaxSpendableJson {
  asset: string;
  chain: string;
  address: string;
  maxSpendableSat: string;
  balanceSat: string;
  estimatedFeeSat: string;
  feeRateSatPerVbyte: number;
}

export interface BalanceJson {
  [chain: string]: {
    address: string;
    confirmed?: string;      // BTC only
    unconfirmed?: string;    // BTC only, omitted when zero
    maxSpendable?: string;   // BTC only
    native?: { symbol: string; balance: string };  // EVM only
    tokens?: Array<{ symbol: string; address: string; balance: string }>;  // EVM only
  };
}
