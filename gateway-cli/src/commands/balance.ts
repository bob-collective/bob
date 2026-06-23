import { isAddress } from 'viem';
import { isValidBtcAddress } from '@gobob/bob-sdk';
import { getAllBalances, deriveAddress, getChainFamily } from '../chains/index.js';
import { isValidTronAddress } from '../chains/tron/addresses.js';
import { getRoutes, getUniqueChains } from '../util/route-provider.js';
import { resolveChain } from '../util/input-resolver.js';
import { loadConfig } from '../config.js';
import { formatAllBalances } from '../output.js';
import type { BalanceJson } from '../output.js';

/** Balance command options for filtering and formatting. */
export interface BalanceOptions {
  chain?: string[];
  feeToken?: string;
  feeReserve?: string;
  nonZero?: boolean;
}

function classifyAddress(addr: string): 'bitcoin' | 'evm' | 'tron' {
  if (isValidBtcAddress(addr)) return 'bitcoin';
  if (isValidTronAddress(addr)) return 'tron';
  if (isAddress(addr, { strict: false })) return 'evm';
  throw new Error(`Unsupported address format "${addr}". Expected a Bitcoin (bc1...), Tron (T...), or EVM (0x...) address.`);
}

export async function handleBalance(addresses: string[], opts: BalanceOptions): Promise<BalanceJson> {
  if (addresses.length === 0) {
    const config = loadConfig();
    if (config.bitcoinPrivateKey) {
      addresses.push(await deriveAddress('bitcoin', config.bitcoinPrivateKey));
    }
    if (config.evmPrivateKey) {
      addresses.push(await deriveAddress('ethereum', config.evmPrivateKey));
    }
    if (config.tronPrivateKey) {
      addresses.push(await deriveAddress('tron', config.tronPrivateKey));
    }
    if (addresses.length === 0) {
      throw new Error('No addresses provided and no private keys configured.\n  Pass an address or set BITCOIN_PRIVATE_KEY / EVM_PRIVATE_KEY / TRON_PRIVATE_KEY.');
    }
  }

  if (opts.chain?.length) {
    const routes = await getRoutes();
    const knownChains = getUniqueChains(routes).sort();
    for (const raw of opts.chain) {
      const resolved = resolveChain(raw);
      if (!knownChains.includes(resolved)) {
        console.warn(`Warning: unknown chain '${raw}'. Known chains: ${knownChains.join(', ')}`);
        process.exitCode = 1;
      }
    }
    opts = { ...opts, chain: opts.chain.map(c => resolveChain(c)) };
  }

  const results: BalanceJson = {};
  for (const addr of addresses) {
    const family = classifyAddress(addr);

    if (opts.chain?.length) {
      for (const chain of opts.chain) {
        const chainFamily = getChainFamily(chain);
        if (family !== chainFamily) {
          console.warn(`Warning: ${family} address '${addr}' is not valid for ${chainFamily} chain '${chain}'`);
          process.exitCode = 1;
        }
      }
    }

    const raw = await getAllBalances(addr, {
      ...opts,
      chainFamily: family,
    });
    const formatted = formatAllBalances(raw);
    for (const [chain, data] of Object.entries(formatted)) {
      const key = chain in results ? `${chain} (${addr})` : chain;
      results[key] = data;
    }
  }

  if (opts.nonZero) {
    for (const [chain, data] of Object.entries(results)) {
      if (data.error) { delete results[chain]; continue; }
      if (data.tokens) {
        data.tokens = data.tokens.filter(t => parseFloat(t.balance) > 0);
      }
      const hasBalance = data.balance !== undefined && parseFloat(data.balance) > 0;
      const hasNative = data.native !== undefined && parseFloat(data.native.balance) > 0;
      const hasTokens = (data.tokens?.length ?? 0) > 0;
      if (!hasBalance && !hasNative && !hasTokens) delete results[chain];
    }
  }

  return results;
}
