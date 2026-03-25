import { isAddress, type Address } from "viem";
import { getSdk } from "../config.js";

// UX-5: BTC address prefixes for better error messages
const BTC_PREFIXES = ["1", "3", "bc1", "tb1"];

function looksLikeBtcAddress(addr: string): boolean {
  return BTC_PREFIXES.some(p => addr.startsWith(p));
}

export async function handleOrders(opts: { address: Address }) {
  // UX-5: Distinguish invalid format vs BTC address vs valid EVM address
  if (!isAddress(opts.address)) {
    if (looksLikeBtcAddress(opts.address)) {
      throw new Error("BTC address lookups are not supported. Provide an EVM address.");
    }
    throw new Error(`Invalid address format: '${opts.address}'`);
  }
  const sdk = getSdk();
  const orders = await sdk.getOrders(opts.address);

  // UX-6: Sort orders by timestamp descending (most recent first)
  return [...orders].sort((a, b) => b.timestamp - a.timestamp);
}
