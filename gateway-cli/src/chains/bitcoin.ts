import { EsploraClient, ScureBitcoinSigner, type BitcoinSigner } from '@gobob/bob-sdk';
import { getSdk } from '../config.js';
import type { TokenBalance } from './index.js';

export async function getBtcBalance(
  address: string,
  sdk: ReturnType<typeof getSdk>,
): Promise<TokenBalance> {
  const esplora = new EsploraClient();
  const [bal, maxSpendable] = await Promise.all([
    esplora.getBalance(address),
    sdk.getMaxSpendable(address),
  ]);
  const total = BigInt(bal.confirmed + bal.unconfirmed).toString();
  const allSpendable = String(maxSpendable.amount.amount);
  return { total, allSpendable };
}

export async function deriveBtcAddress(key: string): Promise<string> {
  const signer = ScureBitcoinSigner.fromKey(key);
  return signer.getP2WPKHAddress();
}

export async function resolveBtcSigner(
  key: string,
): Promise<{ address: string; signer: BitcoinSigner }> {
  const signer = ScureBitcoinSigner.fromKey(key);
  const address = await signer.getP2WPKHAddress();
  return { address, signer };
}
