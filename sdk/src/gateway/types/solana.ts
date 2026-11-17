/**
 * A Solana wallet, passed to `executeQuote` as `walletClient` for an order whose source chain is Solana.
 *
 * Typed on raw bytes so the SDK takes no `@solana/*` dependency: the app wraps its own wallet. With
 * `@solana/web3.js` and a wallet-adapter or Reown AppKit provider, for example:
 *
 * ```ts
 * const walletClient: SolanaWallet = {
 *     address,
 *     signAndSendTransaction: (transaction) =>
 *         provider.sendTransaction(VersionedTransaction.deserialize(transaction), connection),
 * };
 * ```
 */
export interface SolanaWallet {
    /** Base58 public key that signs the transaction and pays its fee. */
    address: string;
    /**
     * Sign the unsigned `VersionedTransaction` Gateway built (its wire bytes), broadcast it, and resolve with its
     * Base58 signature. The wallet must not add signers: the transaction is checked to need only this one.
     */
    signAndSendTransaction(transaction: Uint8Array): Promise<string>;
}

/** A signature's status, as `getSignatureStatuses` reports it. */
export interface SolanaSignatureStatus {
    confirmationStatus?: 'processed' | 'confirmed' | 'finalized' | null;
    /** Set when the transaction failed on-chain. */
    err: unknown;
}

/**
 * Read access to a Solana cluster, passed to `executeQuote` as `publicClient` for an order whose source chain is
 * Solana. It confirms the sent transaction. A `@solana/web3.js` `Connection` satisfies it as-is.
 */
export interface SolanaConnection {
    /** Statuses in the order of `signatures`; `null` for a signature the cluster has not seen. */
    getSignatureStatuses(signatures: string[]): Promise<{ value: (SolanaSignatureStatus | null)[] }>;
}
