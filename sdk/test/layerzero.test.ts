import { assert, describe, it } from 'vitest';
import { LayerZeroClient, LayerZeroGatewayClient } from '../src/gateway/layerzero';
import { createPublicClient, createWalletClient, http, PublicClient, Transport, zeroAddress } from 'viem';
import { bob } from 'viem/chains';
import { ScureBitcoinSigner } from '../src/gateway/utils';

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
    }, 120000);

    it('should get a quote and execute it', async () => {
        const client = new LayerZeroGatewayClient(bob.id);

        const quote = await client.getQuote({
            fromChain: 'bitcoin',
            fromToken: 'bitcoin',
            toChain: 'soneium',
            toToken: '0x0555E30da8f98308EdB960aa94C0Db47230d2B9c',
            fromUserAddress: 'bc1q6tgkjx4pgc5qda52fsgeuvjrhml5nuawwplejq',
            toUserAddress: '0x2A7f5295ac6e24b6D2ca78d82E3cbf01dDA52745',
            amount: 3000,
        });

        const publicClient = createPublicClient({
            chain: bob,
            transport: http(),
        });

        const walletClient = createWalletClient({
            chain: bob,
            transport: http(),
            account: zeroAddress,
        });

        // Uncomment this so test passes without mnemonic set
        // const btcSignerFromSeed = await ScureBitcoinSigner.fromSeedPhrase(process.env.MNEMONIC!, "m/84'/0'/0'/0/0");
        // console.log('P2WPKH Address: ', await btcSignerFromSeed.getP2WPKHAddress());

        // const txHash = await client.executeQuote({
        //     quote,
        //     walletClient,
        //     publicClient: publicClient as PublicClient<Transport>,
        //     btcSigner: btcSignerFromSeed,
        // });

        // console.log(txHash);
    }, 120000);
});
