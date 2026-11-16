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

    // A wrong word count used to derive an unintended key silently. Spacing counts too: trimming would derive a
    // different key than earlier versions did for the same input, so such a phrase is rejected instead.
    it.each([
        ['an extra word', `${BIP84_MNEMONIC} abandon`],
        ['a trailing space', `${BIP84_MNEMONIC} `],
        ['a double space', BIP84_MNEMONIC.replace(' ', '  ')],
    ])('rejects a phrase with %s', async (_case, phrase) => {
        await expect(ScureBitcoinSigner.fromSeedPhrase(phrase, BIP84_FIRST_RECEIVE_PATH)).rejects.toThrow(
            'Invalid mnemonic'
        );
    });

    it('wraps a derivation failure', async () => {
        await expect(ScureBitcoinSigner.fromSeedPhrase(BIP84_MNEMONIC, 'not-a-path')).rejects.toThrow(
            'Failed to create signer from seed phrase'
        );
    });
});
