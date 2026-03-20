import type { RegisterTx } from '@gobob/bob-sdk';
import { getSdk } from '../config.js';
import { getRoutes, getUniqueChains, getTokensForChain } from '../util/route-provider.js';
import { getBtcBalance, deriveBtcAddress, resolveBtcSigner } from './bitcoin.js';
import {
  getEvmNativeBalance,
  getEvmTokenBalance,
  deriveEvmAddress,
  resolveEvmSigner,
} from './evm.js';
import type { ChainBalanceRaw } from './evm.js';

// ─── Chain family registry ──────────────────────────────────────────────────

export type ChainFamily = 'bitcoin' | 'evm';

export function getChainFamily(chain: string): ChainFamily {
  if (chain === 'bitcoin') return 'bitcoin';
  return 'evm';
}

// ─── Token balance (single token, atomic units) ────────────────────────────

export interface TokenBalance {
  total: string;
  allSpendable: string;
}

export interface BalanceOpts {
  feeToken?: string;
  feeReserve?: string;
}

/** Get balance for a single token. For BTC, tokenAddress is ignored. For EVM, omit tokenAddress for native. */
export async function getTokenBalance(
  chain: string,
  address: string,
  tokenAddress?: string,
  opts?: BalanceOpts,
): Promise<TokenBalance> {
  if (getChainFamily(chain) === 'bitcoin') {
    return getBtcBalance(address, getSdk());
  }
  if (!tokenAddress) {
    return getEvmNativeBalance(chain, address);
  }
  return getEvmTokenBalance(chain, address, tokenAddress, opts?.feeToken, opts?.feeReserve);
}

// ─── Chain balances (all tokens on a chain, raw atomic) ─────────────────────

export type { ChainBalanceRaw } from './evm.js';

/** Get all balances for all chains (or a specific chain). Returns raw atomic values. */
export async function getAllBalances(
  address: string,
  opts?: BalanceOpts & { chain?: string; chainFamily?: ChainFamily },
): Promise<Record<string, ChainBalanceRaw>> {
  const routes = await getRoutes();
  let chains = opts?.chain ? [opts.chain] : getUniqueChains(routes);
  if (opts?.chainFamily) {
    chains = chains.filter(c => getChainFamily(c) === opts.chainFamily);
  }

  const { getEvmChainBalancesRaw } = await import('./evm.js');

  const entries = await Promise.all(
    chains.map(async (chain): Promise<[string, ChainBalanceRaw]> => {
      try {
        if (getChainFamily(chain) === 'bitcoin') {
          const bal = await getTokenBalance(chain, address);
          return [chain, {
            address,
            balance: bal.total,
            allSpendable: bal.allSpendable,
          }];
        }

        const chainTokens = getTokensForChain(chain, routes);
        return [chain, await getEvmChainBalancesRaw(chain, address, chainTokens, opts)];
      } catch {
        return [chain, { address, error: true }];
      }
    }),
  );

  return Object.fromEntries(entries);
}

// ─── Address derivation ─────────────────────────────────────────────────────

export async function deriveAddress(chain: string, key: string): Promise<string> {
  if (getChainFamily(chain) === 'bitcoin') return deriveBtcAddress(key);
  return deriveEvmAddress(key);
}

// ─── Signer resolution ──────────────────────────────────────────────────────

export type BtcSigner = Awaited<ReturnType<typeof resolveBtcSigner>>;
export type EvmSigner = ReturnType<typeof resolveEvmSigner>;

export async function resolveSigner(
  chain: string,
  key: string,
): Promise<BtcSigner | EvmSigner> {
  if (getChainFamily(chain) === 'bitcoin') return resolveBtcSigner(key);
  return resolveEvmSigner(key, chain);
}

/** Resolve private key from explicit flag or env var based on chain family. */
export function resolvePrivateKey(
  chain: string,
  privateKey?: string,
  config?: { bitcoinPrivateKey?: string; evmPrivateKey?: string },
): string | undefined {
  return privateKey ?? (getChainFamily(chain) === 'bitcoin' ? config?.bitcoinPrivateKey : config?.evmPrivateKey);
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

// Re-export for direct access
export { getBtcBalance, deriveBtcAddress, resolveBtcSigner } from './bitcoin.js';
export {
  getEvmNativeBalance,
  getEvmTokenBalance,
  deriveEvmAddress,
  resolveEvmSigner,
  NATIVE_GAS_BUFFER,
} from './evm.js';
