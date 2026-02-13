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
