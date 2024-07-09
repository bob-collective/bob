import { BitcoinNetworkName } from "./utxo";

const BTC_MAINNET_REGEX = /\b([13][a-km-zA-HJ-NP-Z1-9]{25,34}|bc1[ac-hj-np-zAC-HJ-NP-Z02-9]{11,71})\b/;
const BTC_TESTNET_REGEX = /\b([2mn][a-km-zA-HJ-NP-Z1-9]{25,34}|tb1[ac-hj-np-zAC-HJ-NP-Z02-9]{11,71})\b/;

const mainnetRegex = new RegExp(BTC_MAINNET_REGEX);
const testnetRegex = new RegExp(BTC_TESTNET_REGEX);

export const isValidBtcAddress = (network: BitcoinNetworkName, address: string): boolean => {
  switch (network) {
    case 'mainnet':
      return mainnetRegex.test(address);
    case 'testnet':
      return testnetRegex.test(address);
    default:
      throw new Error(`Invalid bitcoin network configured: ${network}. Valid values are: mainnet | testnet.`);
  }
};
