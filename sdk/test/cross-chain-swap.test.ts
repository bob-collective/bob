import { assert, describe, it } from 'vitest';
import { CrossChainSwapGatewayClient } from '../src/gateway/cross-chain-swap';
import { createPublicClient, createWalletClient, http, PublicClient, Transport, zeroAddress } from 'viem';
import { bob } from 'viem/chains';
import { privateKeyToAccount } from 'viem/accounts';
import { Hex } from 'viem';
import { parseEther } from 'viem';
import { ScureBitcoinSigner } from '../src/gateway/utils';

describe('Cross Chain Swap Tests', () => {
    it('should get an onramp quote using the cross chain swap gateway client and execute it', async () => {
        const client = new CrossChainSwapGatewayClient();

        const quote = await client.getQuote({
            fromChain: 'bitcoin',
            fromToken: 'bitcoin',
            toChain: bob.id,
            toToken: '0x0000000000000000000000000000000000000000',
            fromUserAddress: 'bc1q6tgkjx4pgc5qda52fsgeuvjrhml5nuawwplejq',
            toUserAddress: '0xEf7Ff7Fb24797656DF41616e807AB4016AE9dCD5',
            amount: 4000,
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

    it('should get an offramp quote using the cross chain swap gateway client and execute it', async () => {
        const client = new CrossChainSwapGatewayClient();
        const quote = await client.getQuote({
            toChain: 'bitcoin',
            toToken: 'bitcoin',
            fromChain: bob.id,
            fromToken: '0x0000000000000000000000000000000000000000',
            toUserAddress: 'bc1q6tgkjx4pgc5qda52fsgeuvjrhml5nuawwplejq',
            fromUserAddress: '0xEf7Ff7Fb24797656DF41616e807AB4016AE9dCD5',
            amount: parseEther('0.001'),
        });

        console.log('quote', quote);

        // Verify we got an offramp quote
        assert.ok(quote.data, 'Should have offramp quote');
        assert.ok(quote.params, 'Should have quote params');

        const publicClient = createPublicClient({
            chain: bob,
            transport: http(),
        });

        const walletClient = createWalletClient({
            chain: bob,
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
