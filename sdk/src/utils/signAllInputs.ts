import { Transaction } from '@scure/btc-signer';
import { hex, base64 } from '@scure/base';
import { BitcoinNetworkType, signTransaction } from 'sats-connect';

interface PsbtInputAccounts {
    address: string;
    signingIndexes: number[];
}

async function signPsbt(psbtHex: string, psbtInputAccounts: PsbtInputAccounts[]): Promise<string> {
    // https://docs.xverse.app/sats-connect/bitcoin-methods/signpsbt
    return new Promise((resolve, reject) => {
        const psbtBase64 = base64.encode(hex.decode(psbtHex));

        return signTransaction({
            payload: {
                network: { type: BitcoinNetworkType.Testnet },
                message: 'Sign Transaction',
                psbtBase64: psbtBase64,
                broadcast: false,
                inputsToSign: psbtInputAccounts,
            },
            onFinish: (response) => {
                resolve(hex.encode(base64.decode(response.psbtBase64)));
            },
            onCancel: () => reject(new Error('Canceled')),
        });
    });
}

async function signAllInputs(paymentAddress: string, psbtBase64: string) {
    const unsignedTx = Transaction.fromPSBT(base64.decode(psbtBase64));

    // Determine how many inputs to sign
    const inputLength = unsignedTx.inputsLength;
    const inputsToSign = Array.from({ length: inputLength }, (_, i) => i);

    // Sign all inputs
    const psbt = unsignedTx.toPSBT(0);
    const psbtHex = hex.encode(psbt);

    // Sign all inputs with the payment address
    const signedPsbtHex = await signPsbt(psbtHex, [
        {
            address: paymentAddress,
            signingIndexes: inputsToSign,
        },
    ]);

    const signedTx = Transaction.fromPSBT(hex.decode(signedPsbtHex));

    signedTx.finalize();

    return signedTx.hex;
}

export { signAllInputs };
