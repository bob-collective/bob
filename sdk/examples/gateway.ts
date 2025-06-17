import { Network, XverseConnector } from '@gobob/sats-wagmi';
import { Address, createPublicClient, createWalletClient, http, PublicClient, Transport, zeroAddress } from 'viem';
import { bob } from 'viem/chains';

import { GatewaySDK } from '../src/gateway';

const BOB_TBTC_V2_TOKEN_ADDRESS = '0xBBa2eF945D523C4e2608C9E1214C2Cc64D4fc2e2';

export async function swapBtcForToken(evmAddress: Address) {
    const publicClient = createPublicClient({
        chain: bob,
        transport: http(),
    });

    const walletClient = createWalletClient({
        chain: bob,
        transport: http(),
        account: zeroAddress, // Use connected address here
    });
    const btcSigner = new XverseConnector(Network.mainnet);

    const gatewaySDK = new GatewaySDK('bob'); // or "mainnet"

    const quote = await gatewaySDK.getQuote({
        fromChain: 'bitcoin',
        fromToken: 'BTC',
        fromUserAddress: 'bc1qafk4yhqvj4wep57m62dgrmutldusqde8adh20d',
        toChain: 'bob',
        toUserAddress: evmAddress,
        toToken: BOB_TBTC_V2_TOKEN_ADDRESS, // or "tBTC"
        amount: 10000000, // 0.1 BTC
        gasRefill: 10000, // 0.0001 BTC,
    });

    const onrampTx = await gatewaySDK.executeQuote(quote, {
        walletClient,
        publicClient: publicClient as PublicClient<Transport>,
        btcSigner,
    });

    console.log(`Success! Txid = ${onrampTx}`);

    const offrampQuote = await gatewaySDK.getQuote({
        fromChain: 'bob',
        fromToken: BOB_TBTC_V2_TOKEN_ADDRESS, // or "tBTC"
        toChain: 'bitcoin',
        toUserAddress: 'bc1qafk4yhqvj4wep57m62dgrmutldusqde8adh20d',
        toToken: 'BTC',
        amount: 10000000, // 0.1 BTC
    });

    const offrampTx = await gatewaySDK.executeQuote(offrampQuote, {
        walletClient,
        publicClient: publicClient as PublicClient<Transport>,
    });

    console.log(`Success! Txid = ${offrampTx}`);
}
