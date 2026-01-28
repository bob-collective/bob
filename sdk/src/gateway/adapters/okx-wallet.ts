import { EsploraClient } from '../../esplora';
import { BitcoinSigner } from '../types';
import { getAddressInfo } from '../../wallet';

interface OkxWalletProvider {
    // https://web3.okx.com/build/dev-docs/sdks/chains/bitcoin/provider#send
    send: (params: { from: string; to: string; value: string; memo?: string }) => Promise<{ txhash: string }>;
}

export class OkxWalletAdapter implements BitcoinSigner {
    walletProvider: OkxWalletProvider;

    constructor(walletProvider: OkxWalletProvider) {
        this.walletProvider = walletProvider;
    }

    async sendBitcoin({
        from,
        to,
        value,
        opReturn,
        isSignet,
    }: {
        from: string;
        to: string;
        value: string;
        opReturn?: string;
        isSignet?: boolean;
    }): Promise<string> {
        const { txhash } = await this.walletProvider.send({
            from,
            to,
            value,
            memo: opReturn,
        });

        const network = getAddressInfo(to, isSignet || false).network;
        const esploraClient = new EsploraClient(network);
        return esploraClient.getTransactionHex(txhash);
    }
}
