export const BTC_ZERO_ADDRESS = "0x0000000000000000000000000000000000000000";

export interface RouteInfo {
  srcChain: string;
  dstChain: string;
  srcToken: TokenInfo;
  dstToken: TokenInfo;
}

export interface TokenInfo {
  address: string;
  symbol: string;
  decimals: number;
  chain: string;
}

export interface GatewayQuoteParams {
  srcChain: string;
  dstChain: string;
  sender?: string;
  recipient: string;
  srcToken: string;
  dstToken: string;
  amount: string;
  gasRefill?: string;
  slippage: number;
  affiliateId?: string;
}

export interface GatewayQuote {
  quoteId: string;
  inputAmount: string;
  outputAmount: string;
  feeBreakdown: FeeBreakdown;
  estimatedTimeInSecs: number;
  layerzeroFee?: FeeComponent;
}

export interface FeeBreakdown {
  solverFee: FeeComponent;
  protocolFee: FeeComponent;
  affiliateFee?: FeeComponent;
  executionFee?: FeeComponent;
  layerzeroFee?: FeeComponent;
  inclusionFee?: FeeComponent;
}

export interface FeeComponent {
  amount: string;
  address: string;
  chain: string;
}

export interface TxInfo {
  to: string;
  data: string;
  value: string;
  chain: string;
}

export interface GatewayCreateOrderResponse {
  orderId: string;
  psbtBase64?: string;
  bitcoinAddress?: string;
  txInfo?: TxInfo;
}

export type OrderStatus =
  | "success"
  | "strategy_skipped"
  | "strategy_failed"
  | "refunded"
  | { inProgress: { bumpFeeTx?: TxInfo; refundTx?: TxInfo } }
  | { failed: { refundTx?: TxInfo } };

export interface GatewayOrderInfo {
  orderId: string;
  status: OrderStatus;
  inputAmount: string;
  outputAmount?: string;
  srcChain: string;
  dstChain: string;
  createdAt: string;
}

export interface MaxSpendable {
  userAddress: string;
  amount: string;
}

export interface RegisterTxRequest {
  orderId: string;
  bitcoinTxid?: string;
  bitcoinTxHex?: string;
  evmTxhash?: string;
}

export interface RegisterTxResponse {
  success: boolean;
}
