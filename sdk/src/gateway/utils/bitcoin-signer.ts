import * as btc from '@scure/btc-signer';
import { base64 } from '@scure/base';
import { mnemonicToSeedSync } from 'bip39';
import { HDKey } from '@scure/bip32';
import { BitcoinSigner } from '../types';

/**
 * Bitcoin signer implementation from seed phrase using scure-btc-signer
 */
export class ScureBitcoinSigner implements BitcoinSigner {
    private privateKey: Uint8Array;

    constructor(privateKeyHex: string) {
        const cleanPrivateKey = privateKeyHex.startsWith('0x') ? privateKeyHex.slice(2) : privateKeyHex;
        this.privateKey = new Uint8Array(Buffer.from(cleanPrivateKey, 'hex'));
    }

    /**
     * Create a Bitcoin signer from a seed phrase (BIP39 mnemonic)
     * @param seedPhrase The BIP39 mnemonic seed phrase
     * @param derivationPath The derivation path (e.g., "m/84'/0'/0'/0/0")
     * @returns Promise resolving to a new ScureBitcoinSigner instance
     */
    static async fromSeedPhrase(seedPhrase: string, derivationPath: string): Promise<ScureBitcoinSigner> {
        try {
            const seed = mnemonicToSeedSync(seedPhrase);
            const hdkey = HDKey.fromMasterSeed(seed);
            const childKey = hdkey.derive(derivationPath);
            if (!childKey.privateKey) {
                throw new Error('Failed to derive private key from seed phrase');
            }
            return new ScureBitcoinSigner(Buffer.from(childKey.privateKey).toString('hex'));
        } catch (error) {
            throw new Error(`Failed to create signer from seed phrase: ${error}`);
        }
    }

    /**
     * Sign all inputs in a PSBT
     *
     * @param psbtBase64 Base64 encoded PSBT string
     * @returns Promise resolving to the signed transaction hex
     */
    async signAllInputs(psbtBase64: string): Promise<string> {
        try {
            const tx = btc.Transaction.fromPSBT(base64.decode(psbtBase64));

            // Sign all inputs
            for (let i = 0; i < tx.inputsLength; i++) {
                tx.signIdx(this.privateKey, i);
            }

            tx.finalize();

            return tx.hex;
        } catch (error) {
            throw new Error(`Failed to sign PSBT with scure-btc-signer: ${error}`);
        }
    }

    /**
     * Get the P2WPKH address for this signer
     */
    async getP2WPKHAddress(): Promise<string> {
        return btc.getAddress('wpkh', this.privateKey) as string;
    }
}
