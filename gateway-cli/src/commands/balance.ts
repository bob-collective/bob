import { isAddress } from 'viem';
import { isValidBtcAddress } from '@gobob/bob-sdk';
import { getAllBalances, deriveAddress, getChainFamily } from '../chains/index.js';
import { getRoutes, getUniqueChains } from '../util/route-provider.js';
import { resolveChain } from '../util/input-resolver.js';
import { loadConfig } from '../config.js';
import { formatAllBalances } from '../output.js';
import type { BalanceJson } from '../output.js';

export interface BalanceOptions {
  chain?: string[];
  feeToken?: string;
  feeReserve?: string;
  nonZero?: boolean;
}

function classifyAddress(addr: string): "bitcoin" | "evm" {
  if (isAddress(addr, { strict: false })) return "evm";
  if (isValidBtcAddress(addr)) return "bitcoin";
  throw new Error(`Unsupported address format "${addr}". Expected an EVM address (0x...) or a Bitcoin address.`);
}

export async function handleBalance(addresses: string[], opts: BalanceOptions): Promise<BalanceJson> {
  // If no addresses provided, derive from env var keys
  if (addresses.length === 0) {
    const config = loadConfig();
    if (config.bitcoinPrivateKey) {
      addresses.push(await deriveAddress("bitcoin", config.bitcoinPrivateKey));
    }
    if (config.evmPrivateKey) {
      addresses.push(await deriveAddress("evm", config.evmPrivateKey));
    }
    if (addresses.length === 0) {
      throw new Error("No addresses provided and no private keys configured.\n  Pass an address or set BITCOIN_PRIVATE_KEY / EVM_PRIVATE_KEY.");
    }
  }

  if (opts.chain?.length) {
    const routes = await getRoutes();
    const knownChains = getUniqueChains(routes).sort();
    for (const raw of opts.chain) {
      const resolved = resolveChain(raw);
      if (!knownChains.includes(resolved)) {
        console.warn(`Warning: unknown chain '${raw}'. Known chains: ${knownChains.join(", ")}`);
        process.exitCode = 1;
      }
    }
    // Resolve aliases in the chain list passed to getAllBalances
    opts = { ...opts, chain: opts.chain.map(c => resolveChain(c)) };
  }

  // Classify addresses and query relevant chains
  const results: BalanceJson = {};
  for (const addr of addresses) {
    const family = classifyAddress(addr);

    if (opts.chain?.length) {
      for (const chain of opts.chain) {
        const chainFamily = getChainFamily(chain);
        if (family === "bitcoin" && chainFamily === "evm") {
          console.warn(`Warning: BTC address '${addr}' is not valid for EVM chain '${chain}'`);
          process.exitCode = 1;
        } else if (family === "evm" && chainFamily === "bitcoin") {
          console.warn(`Warning: EVM address '${addr}' is not valid for Bitcoin chain '${chain}'`);
          process.exitCode = 1;
        }
      }
    }

    const raw = await getAllBalances(addr, {
      ...opts,
      chainFamily: family,
    });
    Object.assign(results, formatAllBalances(raw));
  }

  if (opts.nonZero) {
    for (const [chain, data] of Object.entries(results)) {
      if (data.error) { delete results[chain]; continue; }
      // Filter zero-balance tokens within the chain
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
