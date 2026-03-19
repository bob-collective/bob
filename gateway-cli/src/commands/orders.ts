import type { Address } from "viem";
import { getSdk } from "../config.js";

export async function handleOrders(opts: { address: Address }) {
  const sdk = getSdk();
  return sdk.getOrders(opts.address);
}
