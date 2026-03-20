import type { RegisterTx } from '@gobob/bob-sdk';
import { getSdk } from '../config.js';
import { getBtcBalance, deriveBtcAddress, resolveBtcSigner } from './bitcoin.js';
import {
  getEvmNativeBalance,
  getEvmTokenBalance,
  deriveEvmAddress,
  resolveEvmSigner,
  isNativeToken,
} from './evm.js';

// ─── Chain family registry ──────────────────────────────────────────────────

export type ChainFamily = 'bitcoin' | 'evm';

export function getChainFamily(chain: string): ChainFamily {
  if (chain === 'bitcoin') return 'bitcoin';
  return 'evm';
}

// ─── Token balance ──────────────────────────────────────────────────────────

export interface TokenBalance {
  total: string;        // atomic units, no deductions
  allSpendable: string; // atomic units, minus fees/reserves
}

export interface GetTokenBalanceOpts {
  decimals?: number;
  feeToken?: string;
  feeReserve?: string;
}

export async function getTokenBalance(
  chain: string,
  address: string,
  token: { address: string; symbol: string; decimals: number },
  opts?: GetTokenBalanceOpts,
): Promise<TokenBalance> {
  const family = getChainFamily(chain);

  if (family === 'bitcoin') {
    const sdk = getSdk();
    return getBtcBalance(address, sdk);
  }

  // EVM family
  if (isNativeToken(chain, token.symbol)) {
    return getEvmNativeBalance(chain, address);
  }
  return getEvmTokenBalance(
    chain,
    address,
    token.address,
    token.decimals,
    opts?.feeToken,
    opts?.feeReserve,
  );
}

// ─── Address derivation ─────────────────────────────────────────────────────

export async function deriveAddress(chain: string, key: string): Promise<string> {
  const family = getChainFamily(chain);
  if (family === 'bitcoin') return deriveBtcAddress(key);
  return deriveEvmAddress(key);
}

// ─── Signer resolution ──────────────────────────────────────────────────────

export type BtcSigner = Awaited<ReturnType<typeof resolveBtcSigner>>;
export type EvmSigner = ReturnType<typeof resolveEvmSigner>;

export async function resolveSigner(
  chain: string,
  key: string,
): Promise<BtcSigner | EvmSigner> {
  const family = getChainFamily(chain);
  if (family === 'bitcoin') return resolveBtcSigner(key);
  return resolveEvmSigner(key, chain);
}

// ─── Registration payload ───────────────────────────────────────────────────

export function buildRegisterPayload(
  srcChain: string,
  dstChain: string,
  orderId: string,
  txId: string,
): RegisterTx {
  if (getChainFamily(srcChain) === 'bitcoin') {
    return { onramp: { orderId, bitcoinTxHex: txId } };
  }
  if (getChainFamily(dstChain) === 'bitcoin') {
    return { offramp: { orderId, evmTxhash: txId } };
  }
  return { layerZero: { orderId, evmTxhash: txId } };
}

// Re-export chain-specific modules for direct access
export { getBtcBalance, deriveBtcAddress, resolveBtcSigner } from './bitcoin.js';
export {
  getEvmNativeBalance,
  getEvmTokenBalance,
  deriveEvmAddress,
  resolveEvmSigner,
  isNativeToken,
  NATIVE_GAS_BUFFER,
} from './evm.js';
