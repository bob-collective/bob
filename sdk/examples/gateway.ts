import { Network, XverseConnector } from '@gobob/sats-wagmi';
import { Address, createPublicClient, createWalletClient, http, PublicClient, Transport, zeroAddress } from 'viem';
import { bob } from 'viem/chains';

import { GatewaySDK } from '../src/gateway';
import { OnRampExecuteQuoteParams } from '../src/gateway/types';

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

    const quoteParams: OnRampExecuteQuoteParams = {
        fromChain: 'bitcoin',
        fromToken: 'BTC',
        fromUserAddress: 'bc1qafk4yhqvj4wep57m62dgrmutldusqde8adh20d',
        toChain: 'bob',
        toUserAddress: evmAddress,
        toToken: BOB_TBTC_V2_TOKEN_ADDRESS, // or "tBTC"
        amount: 10000000, // 0.1 BTC
        gasRefill: 10000, // 0.0001 BTC,
    };

    const txid = await gatewaySDK.executeQuote(
        quoteParams,
        walletClient,
        publicClient as PublicClient<Transport>,
        btcSigner
    );

    console.log(`Success! Txid = ${txid}`);
}
