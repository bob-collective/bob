import { type Hex, zeroAddress } from "viem";
import type { ResolvedAsset } from "../util/input-resolver.js";
import { parseAmount } from "../util/input-resolver.js";
import { resolveSendAsset } from "../util/asset-resolver.js";
import { loadConfig } from "../config.js";
import {
  validateRecipient, getChainFamily, resolvePrivateKey, deriveAddress, resolveSigner,
  type BtcSigner, type EvmSigner,
} from "../chains/index.js";
import { sendEvm, buildUnsignedEvmTx, nativeSweepAmount, getEvmBalances, CHAIN_IDS } from "../chains/evm.js";
import { sendBtc, buildBtcPsbt, estimateBtcSweepAmount } from "../chains/bitcoin.js";
import type { SendSuccessJson, SendUnsignedJson, Logger } from "../output.js";

const isNative = (address: string) => address.toLowerCase() === zeroAddress;

export interface SendOptions {
  asset: string;
  amount: string;
  to: string;
  privateKey?: string;
  btcFeeRate?: number;
  unsigned: boolean;
  wait: boolean;
  timeout?: number;
}

export type SendResult =
  | { type: "sent"; data: SendSuccessJson }
  | { type: "unsigned"; data: SendUnsignedJson };

/**
 * Convert the --amount value into atomic units for the resolved asset.
 * `ALL` delegates to the injected `spendable` callback (chain-specific balance lookup).
 */
export async function resolveSendAmount(
  opts: Pick<SendOptions, "amount">,
  asset: ResolvedAsset,
  _senderAddress: string | undefined,
  deps: { spendable: () => Promise<bigint> },
): Promise<bigint> {
  const parsed = await parseAmount(opts.amount, asset.symbol, asset.decimals);
  if (parsed.type === "all") {
    const all = await deps.spendable();
    if (all === 0n) throw new Error(`No ${asset.symbol} balance available to send.`);
    return all;
  }
  return BigInt(parsed.atomicUnits);
}

export async function handleSend(opts: SendOptions, log: Logger): Promise<SendResult> {
  const config = loadConfig();
  const asset = await resolveSendAsset(opts.asset);
  const family = getChainFamily(asset.chain);

  validateRecipient(asset.chain, opts.to);

  const key = resolvePrivateKey(asset.chain, opts.privateKey, config);
  const senderAddress = key ? await deriveAddress(asset.chain, key) : undefined;

  // ALL and BTC --unsigned require knowing the sender, which (no --sender flag) means a key.
  const needsSender = opts.amount.trim().toUpperCase() === "ALL" || (opts.unsigned && family === "bitcoin");
  if (needsSender && !senderAddress) {
    throw new Error(`A signing key is required for this operation.\n  Set ${family === "bitcoin" ? "BITCOIN_PRIVATE_KEY" : "EVM_PRIVATE_KEY"} or pass --private-key.`);
  }

  // Resolve signer up-front for the signed path (also gives EVM publicClient for ALL/fees).
  const signer = !opts.unsigned
    ? (key ? await resolveSigner(asset.chain, key)
           : (() => { throw new Error(`No signer configured.\n  Set ${family === "bitcoin" ? "BITCOIN_PRIVATE_KEY" : "EVM_PRIVATE_KEY"} or pass --private-key, or use --unsigned.`); })())
    : (key ? await resolveSigner(asset.chain, key).catch(() => null) : null);

  const feeRate = opts.btcFeeRate ?? config.btcFeeRate;

  // ── amount resolution (with chain-specific ALL spendable) ──
  const amount = await resolveSendAmount(opts, asset, senderAddress, {
    spendable: async () => {
      if (family === "bitcoin") {
        return estimateBtcSweepAmount(senderAddress!, feeRate);
      }
      if (!signer) throw new Error("An EVM RPC connection is required for --amount ALL. Set a working RPC or remove --unsigned.");
      const evmSigner = signer as EvmSigner;
      if (isNative(asset.address)) {
        const [balance, fees] = await Promise.all([
          evmSigner.publicClient.getBalance({ address: senderAddress as `0x${string}` }),
          evmSigner.publicClient.estimateFeesPerGas(),
        ]);
        return nativeSweepAmount(balance, fees.maxFeePerGas ?? fees.gasPrice ?? 0n);
      }
      const bal = await getEvmBalances(asset.chain, senderAddress!, [{ address: asset.address, symbol: asset.symbol, decimals: asset.decimals }]);
      return BigInt(bal.tokens![0].balance);
    },
  });

  // ── unsigned path ──
  if (opts.unsigned) {
    if (family === "bitcoin") {
      const psbtBase64 = await buildBtcPsbt(senderAddress!, opts.to, amount, feeRate);
      return { type: "unsigned", data: { unsigned: true, chain: asset.chain, asset: asset.symbol, amount: amount.toString(), to: opts.to, psbtBase64 } };
    }
    const from = senderAddress ?? zeroAddress;
    const tx = buildUnsignedEvmTx(asset, opts.to, amount, from, CHAIN_IDS[asset.chain]);
    return { type: "unsigned", data: { unsigned: true, chain: asset.chain, asset: asset.symbol, amount: amount.toString(), to: opts.to, tx } };
  }

  // ── signed path ──
  log.progress(`Sending ${amount} ${asset.symbol} (atomic) to ${opts.to} on ${asset.chain}...`);
  let txId: string;
  if (family === "bitcoin") {
    txId = await sendBtc((signer as BtcSigner).signer, senderAddress!, opts.to, amount, feeRate);
  } else {
    txId = await sendEvm(signer as EvmSigner, asset, opts.to, amount);
  }
  log.progress(`✓ Broadcast: ${txId}`);

  if (!opts.wait) {
    return { type: "sent", data: { asset: asset.symbol, chain: asset.chain, amount: amount.toString(), to: opts.to, txId, status: "broadcast" } };
  }

  // ── wait for 1 confirmation ──
  const timeoutMs = opts.timeout ? opts.timeout * 1000 : config.timeoutMs;
  if (family === "bitcoin") {
    await waitForBtcConfirmation(txId, timeoutMs, log);
  } else {
    try {
      await (signer as EvmSigner).publicClient.waitForTransactionReceipt({ hash: txId as Hex, timeout: timeoutMs });
    } catch (err) {
      throw new Error(`Timed out waiting for confirmation of ${txId}. The tx was broadcast and may still confirm — check a block explorer. ${err instanceof Error ? err.message : String(err)}`);
    }
  }
  log.progress(`✓ Confirmed: ${txId}`);
  return { type: "sent", data: { asset: asset.symbol, chain: asset.chain, amount: amount.toString(), to: opts.to, txId, status: "confirmed" } };
}

/** Poll Esplora until the tx has at least one confirmation, or throw on timeout.
 *
 * Step 3b verification: `getTransaction` is confirmed present on EsploraClient.prototype
 * and returns the full mempool.space tx object with shape `{ status: { confirmed: boolean } }`.
 * No change needed from the brief.
 */
async function waitForBtcConfirmation(txId: string, timeoutMs: number, log: Logger): Promise<void> {
  const { EsploraClient } = await import("@gobob/bob-sdk");
  const esplora = new EsploraClient();
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    try {
      const tx = await esplora.getTransaction(txId);
      if (tx?.status?.confirmed) return;
    } catch {
      // tx may not be indexed yet; keep polling
    }
    const remaining = deadline - Date.now();
    log.progress(`  Waiting for confirmation... (${Math.round(remaining / 1000)}s left)`);
    await new Promise(r => setTimeout(r, Math.min(15_000, remaining)));
  }
  throw new Error(`Timed out waiting for confirmation of ${txId}. The tx may still confirm — check a block explorer.`);
}
