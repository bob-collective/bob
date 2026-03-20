import type { RegisterTx } from '@gobob/bob-sdk';
import { formatBtc } from '@gobob/bob-sdk';
import { formatUnits } from 'viem';
import { getSdk, BTC_DECIMALS } from '../config.js';
import { getEnrichedRoutes, getNativeToken, getUniqueChains, getTokensForChain } from '../util/route-provider.js';
import { getBtcBalance, deriveBtcAddress, resolveBtcSigner } from './bitcoin.js';
import {
  getEvmNativeBalance,
  getEvmTokenBalance,
  deriveEvmAddress,
  resolveEvmSigner,
} from './evm.js';
import type { BalanceJson } from '../output.js';

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

// ─── Chain balances (all tokens on a chain, formatted) ─────────────────────

/** Get all balances for all chains (or a specific chain). One route fetch, no duplicates. */
export async function getAllBalances(
  address: string,
  opts?: BalanceOpts & { chain?: string },
): Promise<BalanceJson> {
  const routes = await getEnrichedRoutes();
  const chains = opts?.chain ? [opts.chain] : getUniqueChains(routes);

  const entries = await Promise.all(
    chains.map(async (chain): Promise<[string, BalanceJson[string]]> => {
      try {
        if (getChainFamily(chain) === 'bitcoin') {
          const bal = await getTokenBalance(chain, address);
          return [chain, {
            address,
            balance: formatBtc(BigInt(bal.total)),
            allSpendable: formatBtc(BigInt(bal.allSpendable)),
          }];
        }

        const nt = getNativeToken(chain);
        const chainTokens = getTokensForChain(chain, routes);
        const [nativeBal, ...tokenBals] = await Promise.all([
          getEvmNativeBalance(chain, address),
          ...chainTokens.map(t => getEvmTokenBalance(chain, address, t.address, opts?.feeToken, opts?.feeReserve).then(bal => ({
            symbol: t.symbol, address: t.address,
            balance: formatUnits(BigInt(bal.total), t.decimals),
            allSpendable: formatUnits(BigInt(bal.allSpendable), t.decimals),
          }))),
        ]);

        return [chain, {
          address,
          native: { symbol: nt.symbol, balance: formatUnits(BigInt(nativeBal.total), nt.decimals), allSpendable: formatUnits(BigInt(nativeBal.allSpendable), nt.decimals) },
          tokens: tokenBals,
        }];
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
