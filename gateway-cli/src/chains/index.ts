import type { RegisterTx } from '@gobob/bob-sdk';
import { getSdk } from '../config.js';
import { getRoutes, getUniqueChains, getTokensForChain } from '../util/route-provider.js';
import { getBtcBalance, deriveBtcAddress, resolveBtcSigner } from './bitcoin.js';
import {
  deriveEvmAddress,
  resolveEvmSigner,
} from './evm.js';
import type { ChainBalance } from './evm.js';

// ─── Chain family registry ──────────────────────────────────────────────────

export type ChainFamily = 'bitcoin' | 'evm';

export function getChainFamily(chain: string): ChainFamily {
  if (chain === 'bitcoin') return 'bitcoin';
  return 'evm';
}

// ─── Chain balances ─────────────────────────────────────────────────────────

export type { ChainBalance } from './evm.js';

/** Get all balances for all chains (or a specific chain). Returns raw atomic values. */
export async function getAllBalances(
  address: string,
  opts?: { feeToken?: string; feeReserve?: string; chain?: string[]; chainFamily?: ChainFamily },
): Promise<Record<string, ChainBalance>> {
  const routes = await getRoutes();
  let chains = opts?.chain?.length ? opts.chain : getUniqueChains(routes);
  if (opts?.chainFamily) {
    chains = chains.filter(c => getChainFamily(c) === opts.chainFamily);
  }

  const { getEvmBalances } = await import('./evm.js');

  const entries = await Promise.all(
    chains.map(async (chain): Promise<[string, ChainBalance]> => {
      try {
        if (getChainFamily(chain) === 'bitcoin') {
          const bal = await getBtcBalance(address, getSdk());
          return [chain, {
            address,
            balance: bal.total,
            allSpendable: bal.allSpendable,
          }];
        }

        const chainTokens = getTokensForChain(chain, routes);
        return [chain, await getEvmBalances(chain, address, chainTokens, { ...opts, includeNative: true })];
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
export type EvmSigner = Awaited<ReturnType<typeof resolveEvmSigner>>;

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

// ─── Recipient resolution ───────────────────────────────────────────────────

/** Resolve the recipient address: explicit flag > derived from private key > error. */
export async function resolveRecipient(
  chain: string,
  explicit: string | undefined,
  config: { bitcoinPrivateKey?: string; evmPrivateKey?: string },
): Promise<string> {
  if (explicit) return explicit;
  const family = getChainFamily(chain);
  const key = resolvePrivateKey(family, undefined, config);
  if (!key) {
    const envVar = family === 'bitcoin' ? 'BITCOIN_PRIVATE_KEY' : 'EVM_PRIVATE_KEY';
    throw new Error(`--recipient is required when no destination wallet is configured.\n  Set ${envVar} or pass --recipient <address>.`);
  }
  return deriveAddress(chain, key);
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
  getEvmBalances,
  deriveEvmAddress,
  resolveEvmSigner,
} from './evm.js';
