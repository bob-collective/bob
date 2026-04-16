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

/** Chain family type for distinguishing Bitcoin and EVM-compatible chains. */
export type ChainFamily = 'bitcoin' | 'evm';

/**
 * Determine the chain family (bitcoin or evm) for a given chain name.
 * All chains except "bitcoin" are treated as EVM-compatible.
 */
export function getChainFamily(chain: string): ChainFamily {
  if (chain === 'bitcoin') return 'bitcoin';
  return 'evm';
}

// ─── Chain balances ─────────────────────────────────────────────────────────

export type { ChainBalance } from './evm.js';

/**
 * Fetch balances for all chains (or filtered by chain/family).
 * Returns raw atomic values (satoshis for BTC, wei for EVM tokens).
 * For EVM chains, deducts estimated gas costs from spendable balance.
 * 
 * @param address - Wallet address to query
 * @param opts - Optional filters: chain list, chain family, fee token/reserve
 * @returns Map of chain names to balance data
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

/**
 * Derive a wallet address from a private key for the specified chain.
 * For Bitcoin, derives P2WPKH (bech32) address. For EVM, derives standard address.
 */
export async function deriveAddress(chain: string, key: string): Promise<string> {
  if (getChainFamily(chain) === 'bitcoin') return deriveBtcAddress(key);
  return deriveEvmAddress(key);
}

// ─── Signer resolution ──────────────────────────────────────────────────────

/** Bitcoin signer type with address and signer object. */
export type BtcSigner = Awaited<ReturnType<typeof resolveBtcSigner>>;
/** EVM signer type with address, wallet client, and public client. */
export type EvmSigner = Awaited<ReturnType<typeof resolveEvmSigner>>;

/**
 * Resolve a signer (Bitcoin or EVM) for a given chain and private key.
 * @param chain - Chain name to determine signer type
 * @param key - Private key (WIF for BTC, hex for EVM)
 */
export async function resolveSigner(
  chain: string,
  key: string,
): Promise<BtcSigner | EvmSigner> {
  if (getChainFamily(chain) === 'bitcoin') return resolveBtcSigner(key);
  return resolveEvmSigner(key, chain);
}

/**
 * Resolve private key from explicit parameter or environment config.
 * Priority: explicit privateKey argument > env var based on chain family.
 */
export function resolvePrivateKey(
  chain: string,
  privateKey?: string,
  config?: { bitcoinPrivateKey?: string; evmPrivateKey?: string },
): string | undefined {
  return privateKey ?? (getChainFamily(chain) === 'bitcoin' ? config?.bitcoinPrivateKey : config?.evmPrivateKey);
}

// ─── Recipient resolution ───────────────────────────────────────────────────

/**
 * Resolve the recipient address for a swap.
 * Priority: explicit address argument > derive from configured private key > throw error.
 * 
 * @param chain - Destination chain name
 * @param explicit - Optional explicit recipient address
 * @param config - Config with private keys for derivation
 * @throws Error if no recipient provided and no private key configured
 */
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

/**
 * Build a registration payload for linking a transaction to an order.
 * Format depends on swap type: onramp (BTC→EVM), offramp (EVM→BTC), or layerZero (EVM→EVM).
 */
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
