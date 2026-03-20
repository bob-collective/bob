import { isAddress } from 'viem';
import { getBtcAddressInfo, BtcAddressType } from '@gobob/bob-sdk';
import { getAllBalances, deriveAddress } from '../chains/index.js';
import { loadConfig } from '../config.js';
import { formatAllBalances } from '../output.js';
import type { BalanceJson } from '../output.js';

export interface BalanceOptions {
  chain?: string;
  feeToken?: string;
  feeReserve?: string;
}

function classifyAddress(addr: string): "bitcoin" | "evm" {
  if (isAddress(addr, { strict: false })) return "evm";
  try {
    const info = getBtcAddressInfo(addr);
    if (info.type === BtcAddressType.p2wpkh) return "bitcoin";
  } catch {}
  throw new Error(`Unsupported address format "${addr}". Expected an EVM address (0x...) or a P2WPKH Bitcoin address (bc1q...).`);
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

  // Classify addresses and query relevant chains
  const results: BalanceJson = {};
  for (const addr of addresses) {
    const family = classifyAddress(addr);
    const raw = await getAllBalances(addr, {
      ...opts,
      chainFamily: family,
    });
    Object.assign(results, formatAllBalances(raw));
  }

  return results;
}
