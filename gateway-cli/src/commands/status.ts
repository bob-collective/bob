import { getSdk } from "../config.js";

/**
 * Handle the status command: fetch order status by ID.
 * @param opts - Order ID to check
 * @returns Order status data from Gateway API
 */
export async function handleStatus(opts: { orderId: string }) {
  const sdk = getSdk();
  return sdk.api.getOrder({ id: opts.orderId });
}
