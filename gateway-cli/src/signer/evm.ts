import {
  createWalletClient,
  createPublicClient,
  http,
  type Hex,
} from "viem";
import { privateKeyToAccount } from "viem/accounts";
import { readFileSync, existsSync } from "node:fs";
import { Wallet } from "ethers";
import { SignerError, ExternalSigner } from "./btc.js";

const errorMessage = (err: unknown): string => err instanceof Error ? err.message : String(err);

// ─── Types ────────────────────────────────────────────────────────────────────

export interface EvmTxRequest {
  to: string;
  data: string;
  value: string;
  chainId: number;
}

export type EvmSignerSpec =
  | { type: "private-key"; key: string }
  | { type: "external"; command: string }
  | { type: "unsigned" };

export interface EvmSignerOptions {
  privateKey?: string;
  envPrivateKey?: string;
  keystorePath?: string;
  keystorePassword?: string;
  externalSignerCmd?: string;
  unsigned: boolean;
}

export interface NonceClearOptions {
  maxWaitMs: number;
  intervalMs: number;
}

// ─── Keystore decryption ─────────────────────────────────────────────────────

export async function decryptKeystore(
  keystorePath: string,
  password: string,
): Promise<string> {
  if (!existsSync(keystorePath)) {
    throw new Error(`keystore file not found: ${keystorePath}`);
  }
  const json = readFileSync(keystorePath, "utf-8");
  try {
    const wallet = await Wallet.fromEncryptedJson(json, password);
    return wallet.privateKey;
  } catch (err) {
    const msg = errorMessage(err);
    if (msg.toLowerCase().includes("invalid") || msg.toLowerCase().includes("mac")) {
      throw new Error("invalid keystore password");
    }
    throw new Error(`failed to decrypt keystore: ${msg}`);
  }
}

// ─── Private key signing + broadcast ─────────────────────────────────────────

export async function signAndBroadcastEvm(
  tx: EvmTxRequest,
  privateKey: string,
  rpcUrl: string,
): Promise<string> {
  const keyHex = (privateKey.startsWith("0x") ? privateKey : `0x${privateKey}`) as Hex;
  const account = privateKeyToAccount(keyHex); // throws if invalid
  const transport = http(rpcUrl);

  const publicClient = createPublicClient({ transport });
  const walletClient = createWalletClient({ account, transport });

  const [nonce, gasEstimate, { maxFeePerGas, maxPriorityFeePerGas }] = await Promise.all([
    publicClient.getTransactionCount({ address: account.address }),
    publicClient.estimateGas({
      account: account.address,
      to: tx.to as Hex,
      data: tx.data as Hex,
      value: BigInt(tx.value || "0"),
    }),
    publicClient.estimateFeesPerGas(),
  ]);

  // Apply 1.3x gas buffer (required for LayerZero/gateway cross-chain calls)
  const gasLimit = (gasEstimate * 130n) / 100n;

  const hash = await walletClient.sendTransaction({
    chain: null,
    to: tx.to as Hex,
    data: tx.data as Hex,
    value: BigInt(tx.value || "0"),
    chainId: tx.chainId,
    nonce,
    gas: gasLimit,
    maxFeePerGas,
    maxPriorityFeePerGas,
  });

  return hash;
}

// ─── Nonce wait ───────────────────────────────────────────────────────────────

export async function waitForNonceClear(
  getLatestCount: () => Promise<bigint>,
  getPendingCount: () => Promise<bigint>,
  opts: NonceClearOptions,
): Promise<boolean> {
  const deadline = Date.now() + opts.maxWaitMs;
  while (Date.now() < deadline) {
    const [latest, pending] = await Promise.all([getLatestCount(), getPendingCount()]);
    if (pending <= latest) return true;
    await new Promise((r) => setTimeout(r, opts.intervalMs));
  }
  return false;
}

// ─── Signer resolver ─────────────────────────────────────────────────────────

export async function resolveEvmSigner(opts: EvmSignerOptions): Promise<EvmSignerSpec> {
  if (opts.unsigned) return { type: "unsigned" };

  const directKey = opts.privateKey ?? opts.envPrivateKey;
  if (directKey) return { type: "private-key", key: directKey };

  if (opts.keystorePath) {
    const password = opts.keystorePassword ?? "";
    const key = await decryptKeystore(opts.keystorePath, password);
    return { type: "private-key", key };
  }

  if (opts.externalSignerCmd) return { type: "external", command: opts.externalSignerCmd };

  throw new Error(
    "no signer configured for EVM.\n" +
    "  Set EVM_PRIVATE_KEY, EVM_SIGNER, or pass --private-key / --keystore.\n" +
    "  Use --unsigned to output the unsigned transaction.",
  );
}

// ─── Sign via resolved spec ───────────────────────────────────────────────────

export async function signEvmWithSpec(
  spec: EvmSignerSpec,
  txPayload: EvmTxRequest,
  rpcUrl: string,
): Promise<string> {
  if (spec.type === "private-key") {
    return signAndBroadcastEvm(txPayload, spec.key, rpcUrl);
  }
  if (spec.type === "external") {
    const ext = new ExternalSigner(spec.command);
    return ext.sign(JSON.stringify(txPayload));
  }
  throw new Error("signEvmWithSpec called with unsigned spec");
}
