export interface BitcoinSigner {
    signAllInputs?: (psbtHex: string) => Promise<string>;
    sendBitcoin?: (params: {
        from: string | null | undefined;
        to: string;
        value: string;
        opReturn?: string;
        isSignet?: boolean;
    }) => Promise<string>;
}

/**
 * Thrown by a {@link BitcoinSigner} whose wallet broadcast the payment before something else failed, e.g. fetching
 * its raw hex: `txid` names the payment that is already out, so a caller does not send it twice. Recognized by
 * `name`, so an adapter built against another copy of the SDK can throw its own.
 */
export class BitcoinBroadcastError extends Error {
    readonly name = 'BitcoinBroadcastError';

    readonly txid: string;

    constructor(txid: string, message: string, options?: ErrorOptions) {
        super(message, options);
        this.txid = txid;
    }
}
