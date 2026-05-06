import { isAddress, type Address } from "viem";
import { getSdk } from "../config.js";

/** Bitcoin address prefixes for detecting BTC vs EVM addresses. */
const BTC_PREFIXES = ["1", "3", "bc1", "tb1"];

/** Check if an address string looks like a Bitcoin address based on prefix. */
function looksLikeBtcAddress(addr: string): boolean {
  return BTC_PREFIXES.some(p => addr.startsWith(p));
}

/**
 * Handle the orders command: list all orders for an EVM address.
 * Validates address format and rejects BTC addresses with helpful error.
 * Returns orders sorted by timestamp (most recent first).
 * 
 * @param opts - EVM address to look up orders for
 * @returns Array of orders sorted by timestamp descending
 * @throws Error if address is invalid format or is a BTC address
 */
export async function handleOrders(opts: { address: Address }) {
  // UX-5: Distinguish invalid format vs BTC address vs valid EVM address
  if (!isAddress(opts.address)) {
    if (looksLikeBtcAddress(opts.address)) {
      throw new Error("BTC address lookups are not supported. Provide an EVM address.");
    }
    throw new Error(`Invalid address format: '${opts.address}'`);
  }
  const sdk = getSdk();
  const { orders } = await sdk.getOrders({ userAddress: opts.address });

  // UX-6: Sort orders by timestamp descending (most recent first)
  return [...orders].sort((a, b) => b.timestamp - a.timestamp);
}
