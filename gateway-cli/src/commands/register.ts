import { getSdk } from "../config.js";
import type { RegisterTx } from "@gobob/bob-sdk";

export async function handleRegister(opts: { orderId: string; txid: string }) {
  const sdk = getSdk();
  const order = await sdk.api.getOrder({ id: opts.orderId });

  let registerTx: RegisterTx;
  if (order.srcInfo.chain === "bitcoin") {
    registerTx = { onramp: { orderId: opts.orderId, bitcoinTxHex: opts.txid } };
  } else if (order.dstInfo.chain === "bitcoin") {
    registerTx = { offramp: { orderId: opts.orderId, evmTxhash: opts.txid } };
  } else {
    registerTx = { layerZero: { orderId: opts.orderId, evmTxhash: opts.txid } };
  }

  return sdk.api.registerTx({ registerTx });
}
