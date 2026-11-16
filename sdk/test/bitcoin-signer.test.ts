import { describe, expect, it } from 'vitest';
import { ScureBitcoinSigner } from '../src/gateway';

// BIP84 test vector: https://github.com/bitcoin/bips/blob/master/bip-0084.mediawiki#test-vectors
const BIP84_MNEMONIC = 'abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about';
const BIP84_FIRST_RECEIVE_PATH = "m/84'/0'/0'/0/0";
const BIP84_FIRST_RECEIVE_ADDRESS = 'bc1qcr8te4kr609gcawutmrza0j4xv80jy8z306fyu';

describe('ScureBitcoinSigner', () => {
    it('derives the BIP84 first receive address from a seed phrase', async () => {
        const signer = await ScureBitcoinSigner.fromSeedPhrase(BIP84_MNEMONIC, BIP84_FIRST_RECEIVE_PATH);

        expect(await signer.getP2WPKHAddress()).toBe(BIP84_FIRST_RECEIVE_ADDRESS);
    });

    it('wraps a derivation failure', async () => {
        await expect(ScureBitcoinSigner.fromSeedPhrase(BIP84_MNEMONIC, 'not-a-path')).rejects.toThrow(
            'Failed to create signer from seed phrase'
        );
    });
});
