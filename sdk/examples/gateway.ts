import { GatewayApiClient } from "../src/gateway";
import { AddressType, getAddressInfo } from "bitcoin-address-validation";
import { createTransfer } from "../src/wallet/utxo";
import { Transaction } from '@scure/btc-signer';

const BOB_TBTC_V2_TOKEN_ADDRESS = "0xBBa2eF945D523C4e2608C9E1214C2Cc64D4fc2e2";

export async function swapBtcForToken(evmAddress: string) {
    const gatewayClient = new GatewayApiClient("https://onramp-api-mainnet.gobob.xyz");

    const amount = 10000; // 0.0001 BTC
    const { fee: _fee, onramp_address: onrampAddress, bitcoin_address: bitcoinAddress, gratuity: _gratuity } = await gatewayClient.getQuote(
        BOB_TBTC_V2_TOKEN_ADDRESS,
        amount
    );

    const orderId = await gatewayClient.createOrder(onrampAddress, evmAddress, amount);

    const tx = await createTxWithOpReturn("bc1qafk4yhqvj4wep57m62dgrmutldusqde8adh20d", bitcoinAddress, amount, evmAddress);

    // NOTE: relayer should broadcast the tx
    await gatewayClient.updateOrder(orderId, tx.toHex());

}

async function createTxWithOpReturn(fromAddress: string, toAddress: string, amount: number, opReturn: string, fromPubKey?: string): Promise<Buffer> {
    const addressType = getAddressInfo(fromAddress).type;

    // Ensure this is not the P2TR address for ordinals (we don't want to spend from it)
    if (addressType === AddressType.p2tr) {
        throw new Error('Cannot transfer using Taproot (P2TR) address. Please use another address type.');
    }

    // We need the public key to generate the redeem and witness script to spend the scripts
    if (addressType === (AddressType.p2sh || AddressType.p2wsh)) {
        if (!fromPubKey) {
            throw new Error('Public key is required to spend from the selected address type');
        }
    }

    const unsignedTx = await createTransfer(
        'mainnet',
        addressType,
        fromAddress,
        toAddress,
        amount,
        fromPubKey,
        opReturn,
    );

    const psbt = unsignedTx.toPSBT(0);

    const signedTx = Transaction.fromPSBT(psbt);

    return Buffer.from(signedTx.extract())
}
