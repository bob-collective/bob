import * as btc from '@scure/btc-signer';
import { hex } from '@scure/base';
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
     * Create a Bitcoin signer from a WIF-encoded private key
     * @param wif WIF-encoded private key string
     * @returns A new ScureBitcoinSigner instance
     */
    static fromWIF(wif: string): ScureBitcoinSigner {
        const decoded = btc.WIF().decode(wif);
        return new ScureBitcoinSigner(Buffer.from(decoded).toString('hex'));
    }

    /**
     * Create a Bitcoin signer from a private key in any supported format (hex or WIF).
     * Auto-detects the format.
     * @param key Private key as hex string (with or without 0x prefix) or WIF-encoded
     * @returns A new ScureBitcoinSigner instance
     */
    static fromKey(key: string): ScureBitcoinSigner {
        const stripped = key.startsWith('0x') ? key.slice(2) : key;
        if (/^[0-9a-fA-F]{64}$/.test(stripped)) {
            return new ScureBitcoinSigner(stripped);
        }
        try {
            return ScureBitcoinSigner.fromWIF(key);
        } catch (e) {
            throw new Error('Invalid private key: expected 64-char hex (with optional 0x prefix) or WIF-encoded key', {
                cause: e,
            });
        }
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
            throw new Error(`Failed to create signer from seed phrase: ${error}`, { cause: error });
        }
    }

    /**
     * Sign all inputs in a PSBT
     *
     * @param psbtHex Hex encoded PSBT string
     * @returns Promise resolving to the signed transaction hex
     */
    async signAllInputs(psbtHex: string): Promise<string> {
        try {
            const tx = btc.Transaction.fromPSBT(hex.decode(psbtHex));

            // Sign all inputs
            for (let i = 0; i < tx.inputsLength; i++) {
                tx.signIdx(this.privateKey, i);
            }

            tx.finalize();

            return tx.hex;
        } catch (error) {
            throw new Error(`Failed to sign PSBT with scure-btc-signer: ${error}`, { cause: error });
        }
    }

    /**
     * Get the P2WPKH address for this signer
     */
    async getP2WPKHAddress(): Promise<string> {
        return btc.getAddress('wpkh', this.privateKey) as string;
    }
}
