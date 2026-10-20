import { EsploraClient } from '../../esplora';
import { BitcoinBroadcastError, BitcoinSigner } from '../types';
import { getAddressInfo } from '../../wallet';

/** Esplora can lag the wallet's broadcast, so fetching the hex is retried: 1s, 2s, 4s then 8s apart. */
const HEX_FETCH_ATTEMPTS = 5;
const HEX_FETCH_BASE_DELAY_MS = 1000;

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
        // Resolved before sending, so that only fetching the hex can fail once the payment is out.
        const network = getAddressInfo(to, isSignet || false).network;
        const esploraClient = new EsploraClient(network);

        const { txhash } = await this.walletProvider.send({
            from,
            to,
            value,
            memo: opReturn,
        });

        for (let attempt = 1; ; attempt++) {
            try {
                return await esploraClient.getTransactionHex(txhash);
            } catch (error) {
                if (attempt === HEX_FETCH_ATTEMPTS) {
                    throw new BitcoinBroadcastError(
                        txhash,
                        `Sent Bitcoin transaction ${txhash}, but could not fetch its raw hex`,
                        { cause: error }
                    );
                }

                await new Promise((resolve) => setTimeout(resolve, HEX_FETCH_BASE_DELAY_MS * 2 ** (attempt - 1)));
            }
        }
    }
}
