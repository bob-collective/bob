import { getSdk, getApi } from "../config.js";
import { buildRegisterPayload } from "../chains/index.js";

/**
 * Register a Bitcoin transaction for an onramp order — recovery when the SDK's
 * automatic registration fails. EVM-source orders are rejected before the request
 * goes out: a no-op call would report success on an order nothing had reconciled.
 *
 * @param opts - Order ID and the raw signed Bitcoin transaction (hex) or its txid
 * @throws Error if the order does not originate on Bitcoin
 */
export async function handleRegister(opts: { orderId: string; txid: string }) {
  const sdk = getSdk();
  const order = await sdk.getOrder(opts.orderId);

  const registerTx = buildRegisterPayload(
    order.srcInfo.chain,
    opts.orderId,
    opts.txid,
  );

  return getApi().registerTxV3({ registerTxV3: registerTx });
}
