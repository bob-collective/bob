import { EsploraClient, ScureBitcoinSigner, type BitcoinSigner } from '@gobob/bob-sdk';
import { getSdk } from '../config.js';

export interface TokenBalance {
  total: string;
  allSpendable: string;
}

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

export function deriveBtcAddress(key: string): Promise<string> {
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
