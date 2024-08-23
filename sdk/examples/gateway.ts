import { GatewayQuoteParams, GatewaySDK } from "../src/gateway";
import { base64 } from '@scure/base';
import { Transaction } from '@scure/btc-signer';

const BOB_TBTC_V2_TOKEN_ADDRESS = "0xBBa2eF945D523C4e2608C9E1214C2Cc64D4fc2e2";

export async function swapBtcForToken(evmAddress: string) {
    const gatewaySDK = new GatewaySDK("bob"); // or "mainnet"

    const quoteParams: GatewayQuoteParams = {
        fromChain: "Bitcoin",
        fromUserAddress: "bc1qafk4yhqvj4wep57m62dgrmutldusqde8adh20d",
        toChain: "BOB",
        toUserAddress: evmAddress,
        toToken: BOB_TBTC_V2_TOKEN_ADDRESS, // or "tBTC"
        amount: 10000000, // 0.1 BTC
        gasRefill: 10000, // 0.0001 BTC
    };
    const quote = await gatewaySDK.getQuote(quoteParams);

    const { uuid, psbtBase64 } = await gatewaySDK.startOrder(quote, quoteParams);

    // NOTE: up to implementation to sign PSBT here!
    const tx = Transaction.fromPSBT(base64.decode(psbtBase64!));

    // NOTE: relayer broadcasts the tx
    await gatewaySDK.finalizeOrder(uuid, tx.hex);
}
