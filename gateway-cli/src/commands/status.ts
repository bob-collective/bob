import { getSdk } from "../config.js";

export async function handleStatus(opts: { orderId: string }) {
  const sdk = getSdk();
  return sdk.api.getOrder({ id: opts.orderId });
}
