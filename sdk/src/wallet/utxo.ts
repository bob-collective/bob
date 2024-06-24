import { Transaction, Script, selectUTXO, TEST_NETWORK, NETWORK, p2wpkh, p2sh } from '@scure/btc-signer';
import { hex } from "@scure/base";
import { AddressType } from 'bitcoin-address-validation';

import { DefaultEsploraClient, UTXO } from '../esplora';

export type BitcoinNetworkName = 'mainnet' | 'testnet';

const bitcoinNetworks: Record<BitcoinNetworkName, typeof NETWORK> = {
  mainnet: NETWORK,
  testnet: TEST_NETWORK
};

export const getBtcNetwork = (name: BitcoinNetworkName) => {
  return bitcoinNetworks[name];
};

type Output = { address: string; amount: bigint } | { script: Uint8Array; amount: bigint };

export interface Input {
  txid: string;
  index: number;
  witness_script?: Uint8Array;
  redeemScript?: Uint8Array;
  witnessUtxo?: {
    script: Uint8Array;
    amount: bigint;
  };
  nonWitnessUtxo?: Uint8Array;
}

export async function createTransfer(
  network: BitcoinNetworkName,
  addressType: AddressType,
  fromAddress: string,
  toAddress: string,
  amount: number,
  publicKey?: string,
  opReturnData?: string,
  confirmationTarget: number = 3,
): Promise<Transaction> {
  const esploraClient = new DefaultEsploraClient(network);

  // NOTE: esplora only returns the 25 most recent UTXOs
  // TODO: change this to use the pagination API and return all UTXOs
  const [confirmedUtxos, feeRate] = await Promise.all([
    esploraClient.getAddressUtxos(fromAddress),
    esploraClient.getFeeEstimate(confirmationTarget)
  ]);

  if (confirmedUtxos.length === 0) {
    throw new Error('No confirmed UTXOs');
  }

  // To construct the spending transaction and estimate the fee, we need the transactions for the UTXOs
  let possibleInputs: Input[] = [];

  await Promise.all(
    confirmedUtxos.map(async (utxo) => {
      const hex = await esploraClient.getTransactionHex(utxo.txid);

      const transaction = Transaction.fromRaw(Buffer.from(hex, 'hex'), { allowUnknownOutputs: true });

      const input = getInputFromUtxoAndTx(network, utxo, transaction, addressType, publicKey);

      possibleInputs.push(input);
    })
  );

  const outputs: Output[] = [
    {
      address: toAddress,
      amount: BigInt(amount)
    }
  ];

  if (opReturnData) {
    // Strip 0x prefix from opReturn
    if (opReturnData.startsWith('0x')) {
      opReturnData = opReturnData.slice(2);
    }
    outputs.push({
      // OP_RETURN https://github.com/paulmillr/scure-btc-signer/issues/26
      script: Script.encode(['RETURN', hex.decode(opReturnData)]),
      amount: BigInt(0)
    })
  }

  // Outsource UTXO selection to btc-signer
  // https://github.com/paulmillr/scure-btc-signer?tab=readme-ov-file#utxo-selection
  // default = exactBiggest/accumBiggest creates tx with smallest fees, but it breaks
  // big outputs to small ones, which in the end will create a lot of outputs close to dust.
  const transaction = selectUTXO(possibleInputs, outputs, 'default', {
    changeAddress: fromAddress, // Refund surplus to the payment address
    feePerByte: BigInt(Math.ceil(feeRate)), // round up to the nearest integer
    bip69: true, // Sort inputs and outputs according to BIP69
    createTx: true, // Create the transaction
    network: getBtcNetwork(network),
    allowUnknownOutputs: true, // Required for OP_RETURN
    allowLegacyWitnessUtxo: true // Required for P2SH-P2WPKH
  });

  if (!transaction || !transaction.tx) {
    throw new Error('Failed to create transaction. Do you have enough funds?');
  }

  return transaction.tx;
}

// Using the UTXO and the transaction, we can construct the input for the transaction
export function getInputFromUtxoAndTx(
  network: BitcoinNetworkName,
  utxo: UTXO,
  transaction: Transaction,
  addressType: AddressType,
  pubKey?: string
): Input {
  // The output containts the necessary details to spend the UTXO based on the script type
  // Under the hood, @scure/btc-signer parses the output and extracts the script and amount
  const output = transaction.getOutput(utxo.vout);

  // For p2sh, we additionally need the redeem script. This cannot be extracted from the transaction itself
  // We only support P2SH-P2WPKH
  // TODO: add support for P2WSH
  // TODO: add support for P2SH-P2PKH
  let redeemScript = {};

  if (addressType === AddressType.p2sh) {
    const inner = p2wpkh(Buffer.from(pubKey!, 'hex'), getBtcNetwork(network));
    redeemScript = p2sh(inner);
  }

  // For the redeem and witness script, we need to construct the script mixin
  const scriptMixin = {
    ...redeemScript
  };


  const nonWitnessUtxo = {
    nonWitnessUtxo: Buffer.from(transaction.hex, 'hex')
  };
  const witnessUtxo = {
    witnessUtxo: {
      script: output.script!,
      amount: output.amount!
    }
  };
  const witnessMixin = transaction.hasWitnesses ? witnessUtxo : nonWitnessUtxo;

  // Construct inputs based on the script type
  const input = {
    txid: utxo.txid,
    index: utxo.vout,
    ...scriptMixin, // Maybe adds the redeemScript and/or witnessScript
    ...witnessMixin // Adds the witnessUtxo or nonWitnessUtxo
  };

  return input;
}
