import type { RegisterTxV3 } from '@gobob/bob-sdk';
import { isValidBtcAddress } from '@gobob/bob-sdk';
import { isAddress } from 'viem';
import { getSdk } from '../config.js';
import { getRoutes, getUniqueChains, getTokensForChain } from '../util/route-provider.js';
import { getBtcBalance, deriveBtcAddress, resolveBtcSigner } from './bitcoin.js';
import {
  deriveEvmAddress,
  resolveEvmSigner,
} from './evm.js';
import {
  deriveTronAddress,
  resolveTronSigner,
  getTronBalances,
} from './tron/signer.js';
import { isValidTronAddress } from './tron/addresses.js';
import type { ChainBalance } from './evm.js';

// ─── Chain family registry ──────────────────────────────────────────────────

/** Chain family type for distinguishing Bitcoin, Tron, and EVM-compatible chains. */
export type ChainFamily = 'bitcoin' | 'evm' | 'tron';

/**
 * Determine the chain family for a given chain name.
 */
export function getChainFamily(chain: string): ChainFamily {
  if (chain === 'bitcoin') return 'bitcoin';
  if (chain === 'tron') return 'tron';
  return 'evm';
}

/**
 * Validate that a recipient address matches the asset's chain family.
 */
export function validateRecipient(chain: string, to: string): void {
  const family = getChainFamily(chain);
  if (family === 'bitcoin') {
    if (!isValidBtcAddress(to)) throw new Error(`--to "${to}" is not a valid Bitcoin address.`);
    return;
  }
  if (family === 'tron') {
    if (!isValidTronAddress(to)) throw new Error(`--to "${to}" is not a valid Tron address.`);
    return;
  }
  if (!isAddress(to, { strict: false })) throw new Error(`--to "${to}" is not a valid EVM address.`);
}

export function privateKeyEnvVar(chain: string): string {
  const family = getChainFamily(chain);
  if (family === 'bitcoin') return 'BITCOIN_PRIVATE_KEY';
  if (family === 'tron') return 'TRON_PRIVATE_KEY';
  return 'EVM_PRIVATE_KEY';
}

export function addressesMatch(a: string, b: string, chain: string): boolean {
  if (getChainFamily(chain) === 'evm') return a.toLowerCase() === b.toLowerCase();
  return a === b;
}

// ─── Chain balances ─────────────────────────────────────────────────────────

export type { ChainBalance } from './evm.js';

/**
 * Fetch balances for all chains (or filtered by chain/family).
 */
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

        if (getChainFamily(chain) === 'tron') {
          const chainTokens = await getTokensForChain(chain, routes);
          return [chain, await getTronBalances(address, chainTokens, { includeNative: true })];
        }

        const chainTokens = await getTokensForChain(chain, routes);
        return [chain, await getEvmBalances(chain, address, chainTokens, { ...opts, includeNative: true })];
      } catch (err) {
        const msg = err instanceof Error ? err.message : String(err);
        console.error(`Warning: ${chain}: ${msg}`);
        return [chain, { address, error: true, errorMessage: msg }];
      }
    }),
  );

  return Object.fromEntries(entries);
}

// ─── Address derivation ─────────────────────────────────────────────────────

export async function deriveAddress(chain: string, key: string): Promise<string> {
  const family = getChainFamily(chain);
  if (family === 'bitcoin') return deriveBtcAddress(key);
  if (family === 'tron') return deriveTronAddress(key);
  return deriveEvmAddress(key);
}

// ─── Signer resolution ──────────────────────────────────────────────────────

export type BtcSigner = Awaited<ReturnType<typeof resolveBtcSigner>>;
export type EvmSigner = Awaited<ReturnType<typeof resolveEvmSigner>>;
export type TronSigner = Awaited<ReturnType<typeof resolveTronSigner>>;

export async function resolveSigner(
  chain: string,
  key: string,
): Promise<BtcSigner | EvmSigner | TronSigner> {
  const family = getChainFamily(chain);
  if (family === 'bitcoin') return resolveBtcSigner(key);
  if (family === 'tron') return resolveTronSigner(key);
  return resolveEvmSigner(key, chain);
}

export function resolvePrivateKey(
  chain: string,
  privateKey?: string,
  config?: { bitcoinPrivateKey?: string; evmPrivateKey?: string; tronPrivateKey?: string },
): string | undefined {
  if (privateKey) return privateKey;
  const family = getChainFamily(chain);
  if (family === 'bitcoin') return config?.bitcoinPrivateKey;
  if (family === 'tron') return config?.tronPrivateKey;
  return config?.evmPrivateKey;
}

export async function resolveRecipient(
  chain: string,
  explicit: string | undefined,
  config: { bitcoinPrivateKey?: string; evmPrivateKey?: string; tronPrivateKey?: string },
): Promise<string> {
  if (explicit) return explicit;
  const key = resolvePrivateKey(chain, undefined, config);
  if (!key) {
    throw new Error(`--recipient is required when no destination wallet is configured.\n  Set ${privateKeyEnvVar(chain)} or pass --recipient <address>.`);
  }
  return deriveAddress(chain, key);
}

export function buildRegisterPayload(
  srcChain: string,
  dstChain: string,
  orderId: string,
  txId: string,
): RegisterTxV3 {
  if (getChainFamily(srcChain) === 'bitcoin') {
    return { onramp: { orderId, bitcoinTxHex: txId } };
  }
  if (getChainFamily(dstChain) === 'bitcoin') {
    return { offramp: { orderId, srcChain, srcTxHash: txId } };
  }
  return { tokenSwap: { orderId, srcChain, srcTxHash: txId } };
}

export { getBtcBalance, deriveBtcAddress, resolveBtcSigner } from './bitcoin.js';
export {
  getEvmBalances,
  deriveEvmAddress,
  resolveEvmSigner,
} from './evm.js';
export {
  deriveTronAddress,
  resolveTronSigner,
  getTronBalances,
} from './tron/signer.js';
export { getTokenMetadata } from './tokens.js';
