import { Transaction } from '@scure/btc-signer';
import { BitcoinSigner } from '../types';
import { base64, hex } from '@scure/base';

interface ReownWalletProvider {
    signPSBT: (params: {
        psbt: string;
        broadcast: boolean;
        signInputs: Array<{ index: number; address: string; sighashTypes: number[] }>;
    }) => Promise<{ psbt: string }>;
}

export class ReownWalletAdapter implements BitcoinSigner {
    walletProvider: ReownWalletProvider;
    userAddress: string;

    constructor(walletProvider: ReownWalletProvider, userAddress: string) {
        this.walletProvider = walletProvider;
        this.userAddress = userAddress;
    }

    async signAllInputs(psbtHex: string): Promise<string> {
        const psbtBytes = hex.decode(psbtHex);
        const unsignedTx = Transaction.fromPSBT(psbtBytes);

        // Determine how many inputs to sign
        const inputLength = unsignedTx.inputsLength;
        const inputsToSign = Array.from({ length: inputLength }, (_, i) => i);

        // Use Reown's signPSBT method
        const result = await this.walletProvider.signPSBT({
            psbt: base64.encode(psbtBytes),
            broadcast: false,
            signInputs: inputsToSign.map((input) => ({ index: input, address: this.userAddress, sighashTypes: [0] })),
        });

        // Finalize and return hex. Some wallets return an already finalized PSBT.
        const signedTx = Transaction.fromPSBT(base64.decode(result.psbt));
        if (!signedTx.isFinal) {
            signedTx.finalize();
        }
        return signedTx.hex;
    }
}
