import { EsploraClient, ScureBitcoinSigner, type BitcoinSigner } from '@gobob/bob-sdk';
import { getSdk } from '../config.js';

/** Bitcoin balance with total and maximum spendable amounts in satoshis. */
export interface TokenBalance {
  total: string;
  allSpendable: string;
}

/**
 * Fetch Bitcoin balance and maximum spendable amount for an address.
 * Uses Esplora for total balance and Gateway SDK for spendable calculation.
 * 
 * @param address - Bitcoin address to query
 * @param sdk - Gateway SDK instance for max spendable calculation
 * @returns Total balance and all spendable amount in satoshis
 */
export async function getBtcBalance(
  address: string,
  sdk: ReturnType<typeof getSdk>,
): Promise<TokenBalance> {
  const esplora = new EsploraClient();
  const [bal, maxSpendable] = await Promise.all([
    esplora.getBalance(address),
    sdk.getMaxSpendable(address),
  ]);
  const total = BigInt(bal.total).toString();
  const allSpendable = maxSpendable.amount.amount;
  return { total, allSpendable };
}

/**
 * Derive a Bitcoin P2WPKH (bech32) address from a private key.
 * @param key - Private key in WIF or hex format
 * @returns P2WPKH address starting with "bc1q"
 */
export function deriveBtcAddress(key: string): Promise<string> {
  const signer = ScureBitcoinSigner.fromKey(key);
  return signer.getP2WPKHAddress();
}

/**
 * Create a Bitcoin signer with address for signing PSBTs.
 * @param key - Private key in WIF or hex format
 * @returns Object with derived address and signer instance
 */
export async function resolveBtcSigner(
  key: string,
): Promise<{ address: string; signer: BitcoinSigner }> {
  const signer = ScureBitcoinSigner.fromKey(key);
  const address = await signer.getP2WPKHAddress();
  return { address, signer };
}
