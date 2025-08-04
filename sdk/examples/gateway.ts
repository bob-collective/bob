import { Network, XverseConnector } from '@gobob/sats-wagmi';
import {
    Address,
    createPublicClient,
    createWalletClient,
    encodeAbiParameters,
    encodeFunctionData,
    http,
    parseAbi,
    parseAbiParameters,
    PublicClient,
    Transport,
    zeroAddress,
    parseEther,
} from 'viem';
import { bob } from 'viem/chains';
import { GatewaySDK, parseBtc } from '../src/gateway';

export async function swapBtcForToken(evmAddress: Address) {
    const publicClient = createPublicClient({
        chain: bob,
        transport: http(),
    });

    // Example – replace with the EOA you want to sign with
    const walletClient = createWalletClient({
        chain: bob,
        transport: http(),
        account: zeroAddress, // Use connected account here
    });
    const btcSigner = new XverseConnector(Network.mainnet);

    const gatewaySDK = new GatewaySDK(bob.id);

    const quote = await gatewaySDK.getQuote({
        fromChain: 'bitcoin',
        fromToken: 'BTC',
        fromUserAddress: 'bc1qafk4yhqvj4wep57m62dgrmutldusqde8adh20d',
        toChain: 'bob',
        toUserAddress: evmAddress,
        toToken: 'wBTC',
        amount: parseBtc('0.1'), // BTC
        gasRefill: parseEther('0.00001'), // ETH
    });

    const onrampTx = await gatewaySDK.executeQuote(quote, {
        walletClient,
        publicClient: publicClient as PublicClient<Transport>,
        btcSigner,
    });

    console.log(`Onramp success! Txid = ${onrampTx}`);

    const offrampQuote = await gatewaySDK.getQuote({
        fromChain: 'bob',
        fromToken: 'wBTC',
        toChain: 'bitcoin',
        toUserAddress: 'bc1qafk4yhqvj4wep57m62dgrmutldusqde8adh20d',
        toToken: 'BTC',
        amount: parseBtc('0.1'), // BTC
    });

    const offrampTx = await gatewaySDK.executeQuote(offrampQuote, {
        walletClient,
        publicClient: publicClient as PublicClient<Transport>,
    });

    console.log(`Offramp success! Txid = ${offrampTx}`);
}

function generateMessageForMulticallHandler(
    userAddress: Address,
    aaveAddress: Address,
    depositAmount: bigint,
    depositCurrency: Address,
    aaveReferralCode: number
) {
    const approveFunction = 'function approve(address spender, uint256 value)';
    const depositFunction = 'function deposit(address asset, uint256 amount, address onBehalfOf, uint16 referralCode)';

    const erc20Interface = parseAbi([approveFunction]);
    const aaveInterface = parseAbi([depositFunction]);

    const approveCalldata = encodeFunctionData({
        abi: erc20Interface,
        functionName: 'approve',
        args: [aaveAddress, depositAmount],
    });
    const depositCalldata = encodeFunctionData({
        abi: aaveInterface,
        functionName: 'deposit',
        args: [depositCurrency, depositAmount, userAddress, aaveReferralCode],
    });

    return encodeAbiParameters(
        parseAbiParameters('((address target, bytes callData, uint256 value)[], address fallbackRecipient)'),
        [
            [
                [
                    { target: depositCurrency, callData: approveCalldata, value: 0n },
                    { target: aaveAddress, callData: depositCalldata, value: 0n },
                ],
                userAddress,
            ],
        ]
    );
}

export async function onrampAndDeposit(evmAddress: Address) {
    const publicClient = createPublicClient({
        chain: bob,
        transport: http(),
    });

    const walletClient = createWalletClient({
        chain: bob,
        transport: http(),
        account: evmAddress,
    });

    const btcSigner = new XverseConnector(Network.mainnet);

    const gatewaySDK = new GatewaySDK(bob.id);

    const quote = await gatewaySDK.getQuote({
        fromChain: 'bitcoin',
        fromToken: 'BTC',
        fromUserAddress: 'bc1qafk4yhqvj4wep57m62dgrmutldusqde8adh20d',
        toChain: 'bob',
        toUserAddress: evmAddress,
        toToken: 'wBTC',
        amount: parseBtc('0.1'), // BTC
        gasRefill: parseEther('0.00001'), // ETH
        message: generateMessageForMulticallHandler(
            evmAddress,
            '0x35B3F1BFe7cbE1e95A3DC2Ad054eB6f0D4c879b6', // Avalon pool
            10000000n,
            '0xd6890176e8d912142AC489e8B5D8D93F8dE74D60', // WBTC-AToken-BOB
            0
        ),
    });

    const onrampTx = await gatewaySDK.executeQuote(quote, {
        walletClient,
        publicClient: publicClient as PublicClient<Transport>,
        btcSigner,
    });

    console.log(`Onramp success! Txid = ${onrampTx}`);
}
