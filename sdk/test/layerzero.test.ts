import { assert, describe, it } from 'vitest';
import { LayerZeroClient, LayerZeroGatewayClient } from '../src/gateway/layerzero';
import { createPublicClient, createWalletClient, http, PublicClient, Transport, zeroAddress } from 'viem';
import { base, bob, optimism } from 'viem/chains';
import { BitcoinSigner } from '../src/gateway/types';
import * as btc from '@scure/btc-signer';
import { base64 } from '@scure/base';
import { mnemonicToSeedSync } from 'bip39';
import { HDKey } from '@scure/bip32';
import { Hex } from 'viem';
import { privateKeyToAccount } from 'viem/accounts';

describe('LayerZero Tests', () => {
    it.skip('should get chains', async () => {
        const client = new LayerZeroClient();
        const chainsInfo = await client.getSupportedChainsInfo();

        const chainNames = chainsInfo.map((chain) => chain.name);
        assert.containSubset(chainNames, [
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

        const findChain = (name: string) => chainsInfo.find((chain) => chain.name === name);

        assert.equal(findChain('ethereum')?.eid, '30101');
        assert.equal(findChain('bob')?.eid, '30279');
        assert.equal(findChain('base')?.eid, '30184');
        assert.equal(findChain('bera')?.eid, '30362');
        assert.equal(findChain('unichain')?.eid, '30320');
        assert.equal(findChain('avalanche')?.eid, '30106');
        assert.equal(findChain('sonic')?.eid, '30332');
        assert.equal(findChain('aptos')?.eid, '30108');
        assert.equal(findChain('bsc')?.eid, '30102');
        assert.equal(findChain('soneium')?.eid, '30340');
        assert.equal(findChain('telos')?.eid, '30199');
        assert.equal(findChain('swell')?.eid, '30335');
        assert.equal(findChain('optimism')?.eid, '30111');
        assert.equal(findChain('sei')?.eid, '30280');

        assert.equal(findChain('ethereum')?.oftAddress, '0x0555e30da8f98308edb960aa94c0db47230d2b9c');
        assert.equal(findChain('bob')?.oftAddress, '0x0555e30da8f98308edb960aa94c0db47230d2b9c');
        assert.equal(findChain('base')?.oftAddress, '0x0555e30da8f98308edb960aa94c0db47230d2b9c');
        assert.equal(findChain('bera')?.oftAddress, '0x0555e30da8f98308edb960aa94c0db47230d2b9c');
        assert.equal(findChain('unichain')?.oftAddress, '0x0555e30da8f98308edb960aa94c0db47230d2b9c');
        assert.equal(findChain('avalanche')?.oftAddress, '0x0555e30da8f98308edb960aa94c0db47230d2b9c');
        assert.equal(findChain('sonic')?.oftAddress, '0x0555e30da8f98308edb960aa94c0db47230d2b9c');
        assert.equal(
            findChain('aptos')?.oftAddress,
            '0xef3a1c7d6aa1a531336433e48d7b1d9b46c5bedc69f3db291df4e39bef4708e2'
        );
        assert.equal(findChain('bsc')?.oftAddress, '0x0555e30da8f98308edb960aa94c0db47230d2b9c');
        assert.equal(findChain('soneium')?.oftAddress, '0x0555e30da8f98308edb960aa94c0db47230d2b9c');
        assert.equal(findChain('telos')?.oftAddress, '0x0555e30da8f98308edb960aa94c0db47230d2b9c');
        assert.equal(findChain('swell')?.oftAddress, '0x0555e30da8f98308edb960aa94c0db47230d2b9c');
        assert.equal(findChain('optimism')?.oftAddress, '0xc3f854b2970f8727d28527ece33176fac67fef48');
        assert.equal(findChain('sei')?.oftAddress, '0x0555e30da8f98308edb960aa94c0db47230d2b9c');
    }, 120000);

    it.skip('should get an onramp quote and execute it', async () => {
        const client = new LayerZeroGatewayClient(bob.id);

        const quote = await client.getQuote({
            fromChain: 'bitcoin',
            fromToken: 'bitcoin',
            toChain: 'ethereum',
            toToken: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c',
            fromUserAddress: 'bc1q6tgkjx4pgc5qda52fsgeuvjrhml5nuawwplejq',
            toUserAddress: '0x2A7f5295ac6e24b6D2ca78d82E3cbf01dDA52745',
            amount: 4000,
            l0OriginFinalityBuffer: 10000,
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

    it.skip('should get an offramp quote and execute it', async () => {
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
        assert.ok(quote.data, 'Should have offramp quote');
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

    it.skip('should get a layerzero send quote and execute it', async () => {
        const client = new LayerZeroGatewayClient(base.id);

        const quote = await client.getQuote({
            fromChain: 'base',
            fromToken: (await client.getSupportedChainsInfo()).find((chain) => chain.name === 'base')
                ?.oftAddress as string,
            toChain: 'optimism',
            toToken: (await client.getSupportedChainsInfo()).find((chain) => chain.name === 'optimism')
                ?.oftAddress as string,
            fromUserAddress: '0xEf7Ff7Fb24797656DF41616e807AB4016AE9dCD5',
            toUserAddress: '0xEf7Ff7Fb24797656DF41616e807AB4016AE9dCD5',
            amount: 100,
        });

        console.log('quote', quote);

        const publicClient = createPublicClient({
            chain: base,
            transport: http(),
        });

        const walletClient = createWalletClient({
            chain: base,
            transport: http(),
            account: privateKeyToAccount(process.env.PRIVATE_KEY as Hex),
        });

        const txHash = await client.executeQuote({
            quote,
            walletClient,
            publicClient: publicClient as PublicClient<Transport>,
        });

        console.log(txHash);
    }, 120000);

    it('should get chain id for eid', async () => {
        const client = new LayerZeroClient();

        const optimismEid = 30111;

        assert.equal(await client.getChainId(optimismEid), optimism.id);
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
