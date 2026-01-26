export interface BitcoinSigner {
    signAllInputs?: (psbtBase64: string) => Promise<string>;
    sendBitcoin?: (params: {
        from: string;
        to: string;
        value: string;
        opReturn?: string;
        isSignet?: boolean;
    }) => Promise<string>;
}
