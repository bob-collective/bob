import * as bitcoinjsLib from "bitcoinjs-lib";

import { DummySigner, RemoteSigner } from "./signer";
import { CommitTxData, createCommitTxData, createTextInscription } from "./commit";
import { createRevealTx, customFinalizer, signRevealTx } from "./reveal";

export { RemoteSigner };

/**
 * Estimate the virtual size of a 1 input 1 output reveal tx.
 */
function estimateTxSize(
  network: bitcoinjsLib.Network,
  publicKey: Buffer,
  commitTxData: CommitTxData,
  toAddress: string,
  amount: number,
) {
  const psbt = new bitcoinjsLib.Psbt({ network });

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
 * Inscribe some text data on Bitcoin using the remote signer.
 *
 * @param signer - Implementation to interact with Bitcoin and sign the PSBT.
 * @param toAddress - The address to receive the inscription.
 * @param feeRate - Fee rate of the Bitcoin network (satoshi / byte).
 * @param text - Data to inscribe in the witness of the reveal transaction.
 * @param postage - Amount of postage to include in the inscription.
 * @returns Promise which resolves to the reveal transaction.
 */
export async function inscribeText(
  signer: RemoteSigner,
  toAddress: string,
  feeRate: number,
  text: string,
  postage = 10000,
) {
  const bitcoinNetwork = await signer.getNetwork();
  const publicKey = Buffer.from(await signer.getPublicKey(), "hex");

  const inscription = createTextInscription(text);
  const commitTxData = createCommitTxData(bitcoinNetwork, publicKey, inscription);

  const revealTxSize = estimateTxSize(bitcoinNetwork, publicKey, commitTxData, toAddress, postage);

  // https://github.com/ordinals/ord/blob/ea1c7c8f73e1c30df547000ac7ccd82051cb60af/src/subcommand/wallet/inscribe/batch.rs#L501
  const revealFee = revealTxSize * feeRate;
  // https://github.com/ordinals/ord/blob/ea1c7c8f73e1c30df547000ac7ccd82051cb60af/src/subcommand/wallet/inscribe/batch.rs#L327
  const commitTxAmount = revealFee + postage;

  const commitAddress = commitTxData.scriptTaproot.address!;
  const commitTxId = await signer.sendToAddress(commitAddress, commitTxAmount);
  const commitUtxoIndex = await signer.getUtxoIndex(commitAddress, commitTxId);

  const commitTxResult = {
    txId: commitTxId,
    sendUtxoIndex: commitUtxoIndex,
    sendAmount: commitTxAmount,
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
