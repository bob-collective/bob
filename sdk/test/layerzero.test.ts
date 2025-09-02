import { assert, describe, it } from 'vitest';
import { LayerZeroClient, LayerZeroGatewayClient } from '../src/gateway/layerzero';
import { createPublicClient, createWalletClient, http, PublicClient, Transport, zeroAddress } from 'viem';
import { base, bob } from 'viem/chains';
import { BitcoinSigner } from '../src/gateway/types';
import * as btc from '@scure/btc-signer';
import { base64 } from '@scure/base';
import { mnemonicToSeedSync } from 'bip39';
import { HDKey } from '@scure/bip32';
import { Hex } from 'viem';
import { privateKeyToAccount } from 'viem/accounts';

describe('LayerZero Tests', () => {
    it('should get chains', async () => {
        const client = new LayerZeroClient();
        const chains = await client.getSupportedChains();
        assert.containSubset(chains, [
            'ethereum',
            'bob',
            'base',
            'bera',
            'unichain',
            'avalanche',
            'sonic',
            'aptos',
            'bsc',
            'soneium',
            'telos',
            'swell',
            'optimism',
            'sei',
        ]);

        assert.equal(await client.getEidForChain('ethereum'), '30101');
        assert.equal(await client.getEidForChain('bob'), '30279');
        assert.equal(await client.getEidForChain('base'), '30184');
        assert.equal(await client.getEidForChain('bera'), '30362');
        assert.equal(await client.getEidForChain('unichain'), '30320');
        assert.equal(await client.getEidForChain('avalanche'), '30106');
        assert.equal(await client.getEidForChain('sonic'), '30332');
        assert.equal(await client.getEidForChain('aptos'), '30108');
        assert.equal(await client.getEidForChain('bsc'), '30102');
        assert.equal(await client.getEidForChain('soneium'), '30340');
        assert.equal(await client.getEidForChain('telos'), '30199');
        assert.equal(await client.getEidForChain('swell'), '30335');
        assert.equal(await client.getEidForChain('optimism'), '30111');
        assert.equal(await client.getEidForChain('sei'), '30280');

        assert.equal(await client.getOftAddressForChain('ethereum'), '0x0555e30da8f98308edb960aa94c0db47230d2b9c');
        assert.equal(await client.getOftAddressForChain('bob'), '0x0555e30da8f98308edb960aa94c0db47230d2b9c');
        assert.equal(await client.getOftAddressForChain('base'), '0x0555e30da8f98308edb960aa94c0db47230d2b9c');
        assert.equal(await client.getOftAddressForChain('bera'), '0x0555e30da8f98308edb960aa94c0db47230d2b9c');
        assert.equal(await client.getOftAddressForChain('unichain'), '0x0555e30da8f98308edb960aa94c0db47230d2b9c');
        assert.equal(await client.getOftAddressForChain('avalanche'), '0x0555e30da8f98308edb960aa94c0db47230d2b9c');
        assert.equal(await client.getOftAddressForChain('sonic'), '0x0555e30da8f98308edb960aa94c0db47230d2b9c');
        assert.equal(
            await client.getOftAddressForChain('aptos'),
            '0xef3a1c7d6aa1a531336433e48d7b1d9b46c5bedc69f3db291df4e39bef4708e2'
        );
        assert.equal(await client.getOftAddressForChain('bsc'), '0x0555e30da8f98308edb960aa94c0db47230d2b9c');
        assert.equal(await client.getOftAddressForChain('soneium'), '0x0555e30da8f98308edb960aa94c0db47230d2b9c');
        assert.equal(await client.getOftAddressForChain('telos'), '0x0555e30da8f98308edb960aa94c0db47230d2b9c');
        assert.equal(await client.getOftAddressForChain('swell'), '0x0555e30da8f98308edb960aa94c0db47230d2b9c');
        assert.equal(await client.getOftAddressForChain('optimism'), '0xc3f854b2970f8727d28527ece33176fac67fef48');
        assert.equal(await client.getOftAddressForChain('sei'), '0x0555e30da8f98308edb960aa94c0db47230d2b9c');
    }, 120000);

    it('should get an onramp quote and execute it', async () => {
        const client = new LayerZeroGatewayClient(bob.id);

        const quote = await client.getQuote({
            fromChain: 'bitcoin',
            fromToken: 'bitcoin',
            toChain: 'base',
            toToken: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c',
            fromUserAddress: 'bc1q6tgkjx4pgc5qda52fsgeuvjrhml5nuawwplejq',
            toUserAddress: '0x2A7f5295ac6e24b6D2ca78d82E3cbf01dDA52745',
            amount: 3000,
        });

        console.log('quote', quote);

        const publicClient = createPublicClient({
            chain: bob,
            transport: http(),
        });

        const walletClient = createWalletClient({
            chain: bob,
            transport: http(),
            account: zeroAddress,
        });

        const btcSignerFromSeed = await ScureBitcoinSigner.fromSeedPhrase(process.env.MNEMONIC!, "m/84'/0'/0'/0/0");
        console.log('P2WPKH Address: ', await btcSignerFromSeed.getP2WPKHAddress());

        const txHash = await client.executeQuote({
            quote,
            walletClient,
            publicClient: publicClient as PublicClient<Transport>,
            btcSigner: btcSignerFromSeed,
        });

        console.log(txHash);
    }, 120000);

    it('should get an offramp quote and execute it', async () => {
        const client = new LayerZeroGatewayClient(bob.id);
        const layerZeroClient = new LayerZeroClient();

        const quote = await client.getQuote({
            fromChain: 'base',
            fromToken: (await layerZeroClient.getOftAddressForChain('base')) as string, // WBTC on base
            toChain: 'bitcoin',
            toToken: 'bitcoin',
            fromUserAddress: '0x2A7f5295ac6e24b6D2ca78d82E3cbf01dDA52745', // EVM address (sender)
            toUserAddress: 'bc1q6tgkjx4pgc5qda52fsgeuvjrhml5nuawwplejq', // Bitcoin address (recipient)
            amount: 17000,
        });

        console.log('quote', quote);

        // Verify we got an offramp quote
        assert.ok(quote.offrampQuote, 'Should have offramp quote');
        assert.ok(quote.params, 'Should have quote params');

        const publicClient = createPublicClient({
            chain: base,
            transport: http(),
        });

        const walletClient = createWalletClient({
            chain: base,
            transport: http(),
            account: privateKeyToAccount(process.env.PRIVATE_KEY as Hex),
        });

        const btcSignerFromSeed = await ScureBitcoinSigner.fromSeedPhrase(process.env.MNEMONIC!, "m/84'/0'/0'/0/0");
        console.log('P2WPKH Address: ', await btcSignerFromSeed.getP2WPKHAddress());

        const txHash = await client.executeQuote({
            quote,
            walletClient,
            publicClient: publicClient as PublicClient<Transport>,
            btcSigner: btcSignerFromSeed,
        });

        console.log(txHash);
    }, 120000);
});

/**
 * Bitcoin signer implementation from seed phrase using scure-btc-signer
 */
class ScureBitcoinSigner implements BitcoinSigner {
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
