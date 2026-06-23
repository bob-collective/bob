import { EsploraClient, ScureBitcoinSigner, createBitcoinPsbt, estimateTxFee, getBalance, type BitcoinSigner } from '@gobob/bob-sdk';
import { getSdk } from '../config.js';

/** Bitcoin dust threshold in satoshis (matches the SDK's selectUTXO default). */
const BTC_DUST_SATS = 546n;

/** Bitcoin balance with total and maximum spendable amounts in satoshis. */
export interface TokenBalance {
  total: string;
  allSpendable: string;
}

/**
 * Fetch Bitcoin balance and maximum spendable amount for an address.
 * Uses Esplora for total balance and Gateway SDK for spendable calculation.
 * 
 * @param address - Bitcoin address to query
 * @param sdk - Gateway SDK instance for max spendable calculation
 * @returns Total balance and all spendable amount in satoshis
 */
export async function getBtcBalance(
  address: string,
  sdk: ReturnType<typeof getSdk>,
): Promise<TokenBalance> {
  const esplora = new EsploraClient();
  const [bal, maxSpendable] = await Promise.all([
    esplora.getBalance(address),
    sdk.getMaxSpendable(address),
  ]);
  const total = BigInt(bal.total).toString();
  const allSpendable = maxSpendable.amount.amount;
  return { total, allSpendable };
}

/**
 * Derive a Bitcoin P2WPKH (bech32) address from a private key.
 * @param key - Private key in WIF or hex format
 * @returns P2WPKH address starting with "bc1q"
 */
export function deriveBtcAddress(key: string): Promise<string> {
  const signer = ScureBitcoinSigner.fromKey(key);
  return signer.getP2WPKHAddress();
}

/**
 * Create a Bitcoin signer with address for signing PSBTs.
 * @param key - Private key in WIF or hex format
 * @returns Object with derived address and signer instance
 */
export async function resolveBtcSigner(
  key: string,
): Promise<{ address: string; signer: BitcoinSigner }> {
  const signer = ScureBitcoinSigner.fromKey(key);
  const address = await signer.getP2WPKHAddress();
  return { address, signer };
}

// ─── Send (direct transfer) ───────────────────────────────────────────────────

/** Convert a base64-encoded PSBT (as produced by createBitcoinPsbt) to hex (as expected by signAllInputs). */
export function psbtBase64ToHex(base64: string): string {
  return Buffer.from(base64, "base64").toString("hex");
}

/**
 * Build an unsigned PSBT (base64) for a P2WPKH send. UTXO selection, fee, and change
 * are handled by the SDK. Public key is not required for P2WPKH senders.
 *
 * `strategy` defaults to `'default'` (subset selection + change). For a full-balance
 * sweep pass `'all'` so the PSBT spends every UTXO, matching the `'all'` fee estimate
 * used to derive the sweep amount.
 */
export function buildBtcPsbt(
  from: string,
  to: string,
  amount: bigint,
  feeRate?: number,
  strategy: "all" | "default" = "default",
): Promise<string> {
  return createBitcoinPsbt(from, to, Number(amount), undefined, undefined, feeRate, undefined, undefined, strategy);
}

/**
 * Compute the sweepable BTC amount for `--amount ALL`: the confirmed safe-UTXO
 * balance minus the network fee to spend every UTXO into a single output.
 *
 * Both values come from the SDK's own UTXO machinery over the SAME safe-UTXO set
 * (`getBalance` and `estimateTxFee` both use `findSafeUtxos`), so `confirmed - fee`
 * is the amount `createBitcoinPsbt` can then drain into one recipient output. The fee
 * estimate forces the `'all'` strategy, matching `buildBtcPsbt(..., "all")` on the build
 * side so the actual tx spends exactly the UTXOs the fee assumed (no subset/change drift).
 *
 * @returns The atomic sweep amount in satoshis.
 * @throws if the balance cannot cover the fee (result at or below dust).
 */
export async function estimateBtcSweepAmount(from: string, feeRate?: number): Promise<bigint> {
  const [{ confirmed }, fee] = await Promise.all([
    getBalance(from),
    estimateTxFee(from, undefined, undefined, undefined, feeRate, undefined, undefined, "all"),
  ]);
  return btcSweepAmount(confirmed, fee ?? 0n);
}

/** Sweep amount = confirmed balance minus fee, rejecting results at or below dust. Pure. */
export function btcSweepAmount(confirmed: bigint, fee: bigint): bigint {
  const sweep = confirmed - fee;
  if (sweep <= BTC_DUST_SATS) {
    throw new Error(`BTC balance (${confirmed} sats) is too low to cover the sweep fee (${fee} sats). Nothing to send.`);
  }
  return sweep;
}

/**
 * Execute a direct BTC transfer: build PSBT → sign (finalize) → broadcast.
 * @returns The broadcast transaction id.
 */
export async function sendBtc(
  signer: BitcoinSigner,
  from: string,
  to: string,
  amount: bigint,
  feeRate?: number,
  strategy: "all" | "default" = "default",
): Promise<string> {
  const psbtBase64 = await buildBtcPsbt(from, to, amount, feeRate, strategy);
  if (!signer.signAllInputs) throw new Error("Bitcoin signer cannot sign PSBTs (missing signAllInputs).");
  const signedTxHex = await signer.signAllInputs(psbtBase64ToHex(psbtBase64));
  return new EsploraClient().broadcastTx(signedTxHex);
}
