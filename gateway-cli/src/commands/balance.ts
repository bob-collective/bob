import { getAllBalances } from '../chains/index.js';
import { formatAllBalances } from '../output.js';
import type { BalanceJson } from '../output.js';

export interface BalanceOptions {
  chain?: string;
  feeToken?: string;
  feeReserve?: string;
}

export async function handleBalance(address: string, opts: BalanceOptions): Promise<BalanceJson> {
  const raw = await getAllBalances(address, opts);
  return formatAllBalances(raw);
}
