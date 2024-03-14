import * as bitcoin from "bitcoinjs-lib";

import { DummySigner, RemoteSigner } from "./signer";
import { CommitTxData, createCommitTxData } from "./commit";
import { createRevealTx, customFinalizer, signRevealTx } from "./reveal";
import { Inscription } from "../inscription";

export { RemoteSigner };

/**
 * Estimate the virtual size of a 1 input 1 output reveal tx.
 */
function estimateTxSize(
  network: bitcoin.Network,
  publicKey: Buffer,
  commitTxData: CommitTxData,
  toAddress: string,
  amount: number,
) {
  const psbt = new bitcoin.Psbt({ network });

  const { scriptTaproot, tapLeafScript } = commitTxData;
  psbt.addInput({
    hash: Buffer.alloc(32, 0),
    index: 0,
    witnessUtxo: {
      value: amount,
      script: scriptTaproot.output!,
    },
    tapLeafScript: [tapLeafScript],
  });

  psbt.addOutput({
    value: amount,
    address: toAddress,
  });

  psbt.signInput(0, new DummySigner(publicKey));
  psbt.finalizeInput(0, customFinalizer(commitTxData));

  const tx = psbt.extractTransaction();
  return tx.virtualSize();
}

/**
 * Inscribe some data on Bitcoin using the remote signer.
 *
 * @param signer - Implementation to interact with Bitcoin and sign the PSBT.
 * @param toAddress - The address to receive the inscription.
 * @param feeRate - Fee rate of the Bitcoin network (satoshi / byte).
 * @param inscription - Data to inscribe in the witness of the reveal transaction.
 * @param postage - Amount of postage to include in the inscription.
 * @returns Promise which resolves to the reveal transaction.
 */
export async function inscribeData(
  signer: RemoteSigner,
  toAddress: string,
  feeRate: number,
  inscription: Inscription,
  postage = 10000,
) {
  const bitcoinNetwork = await signer.getNetwork();
  const publicKey = Buffer.from(await signer.getPublicKey(), "hex");

  const commitTxData = createCommitTxData(bitcoinNetwork, publicKey, inscription);

  const revealTxSize = estimateTxSize(bitcoinNetwork, publicKey, commitTxData, toAddress, postage);

  // https://github.com/ordinals/ord/blob/ea1c7c8f73e1c30df547000ac7ccd82051cb60af/src/subcommand/wallet/inscribe/batch.rs#L501
  const revealFee = revealTxSize * feeRate;
  // https://github.com/ordinals/ord/blob/ea1c7c8f73e1c30df547000ac7ccd82051cb60af/src/subcommand/wallet/inscribe/batch.rs#L327
  const commitTxAmount = revealFee + postage;

  const commitAddress = commitTxData.scriptTaproot.address!;
  const commitTxId = await signer.sendToAddress(commitAddress, commitTxAmount);
  const commitTx = await signer.getTransaction(commitTxId);

  const scriptPubKey = bitcoin.address.toOutputScript(commitAddress, bitcoinNetwork);
  const commitUtxoIndex = commitTx.outs.findIndex(out => out.script.equals(scriptPubKey));

  const commitTxResult = {
    tx: commitTx,
    outputIndex: commitUtxoIndex,
    outputAmount: commitTxAmount,
  };

  const revealPsbt = createRevealTx(
    bitcoinNetwork,
    commitTxData,
    commitTxResult,
    toAddress,
    postage,
  );

  const revealTx = await signRevealTx(
    signer,
    commitTxData,
    revealPsbt
  );

  return revealTx;
}
