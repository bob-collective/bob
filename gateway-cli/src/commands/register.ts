import { getSdk } from "../config.js";
import { buildRegisterPayload } from "../chains/index.js";

export async function handleRegister(opts: { orderId: string; txid: string }) {
  const sdk = getSdk();
  const order = await sdk.api.getOrder({ id: opts.orderId });

  const registerTx = buildRegisterPayload(
    order.srcInfo.chain,
    order.dstInfo.chain,
    opts.orderId,
    opts.txid,
  );

  return sdk.api.registerTx({ registerTx });
}
