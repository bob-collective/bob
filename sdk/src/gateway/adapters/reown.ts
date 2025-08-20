import { Transaction } from '@scure/btc-signer';
import { BitcoinSigner } from '../types';
import { base64 } from '@scure/base';
import { Address } from 'viem';

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

    async signAllInputs(psbtBase64: string): Promise<string> {
        const unsignedTx = Transaction.fromPSBT(base64.decode(psbtBase64));

        // Determine how many inputs to sign
        const inputLength = unsignedTx.inputsLength;
        const inputsToSign = Array.from({ length: inputLength }, (_, i) => i);

        // Use Reown's signPSBT method
        const result = await this.walletProvider.signPSBT({
            psbt: psbtBase64,
            broadcast: false,
            signInputs: inputsToSign.map((input) => ({ index: input, address: this.userAddress, sighashTypes: [0] })),
        });

        // Finalize and return hex
        const signedTx = Transaction.fromPSBT(base64.decode(result.psbt));
        signedTx.finalize();
        return signedTx.hex;
    }
}
