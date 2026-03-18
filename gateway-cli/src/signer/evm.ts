import {
  createWalletClient,
  createPublicClient,
  http,
  type WalletClient,
  type PublicClient,
  type Hex,
  type LocalAccount,
} from "viem";
import { privateKeyToAccount } from "viem/accounts";
import { readFileSync, existsSync } from "node:fs";
import { Wallet } from "ethers";
import { ExternalSigner } from "./btc.js";

const errorMessage = (err: unknown): string => err instanceof Error ? err.message : String(err);

// ─── Types ────────────────────────────────────────────────────────────────────

export type EvmSignerResult =
  | { walletClient: WalletClient; publicClient: PublicClient }
  | { unsigned: true };

export interface EvmSignerOptions {
  privateKey?: string;
  envPrivateKey?: string;
  keystorePath?: string;
  keystorePassword?: string;
  externalSignerCmd?: string;
  unsigned: boolean;
  rpcUrl?: string;
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

// ─── Helpers ─────────────────────────────────────────────────────────────────

function normaliseHexKey(key: string): Hex {
  return (key.startsWith("0x") ? key : `0x${key}`) as Hex;
}

function createClients(account: LocalAccount, rpcUrl: string): { walletClient: WalletClient; publicClient: PublicClient } {
  const transport = http(rpcUrl);
  return {
    walletClient: createWalletClient({ account, transport }),
    publicClient: createPublicClient({ transport }),
  };
}

/**
 * Create a custom viem LocalAccount backed by an external signer command.
 *
 * The external command receives a JSON payload on stdin containing the
 * serialized transaction and must return the signed transaction hex on stdout.
 * This preserves the existing external signer protocol.
 */
function createExternalAccount(command: string): LocalAccount {
  const ext = new ExternalSigner(command);

  // We create a minimal custom account. The signTransaction method delegates
  // to the external command. signMessage and signTypedData are not supported.
  return {
    address: "0x0000000000000000000000000000000000000000" as `0x${string}`,
    type: "local",
    source: "custom",
    publicKey: "0x" as Hex,

    async signMessage() {
      throw new Error("external signer does not support signMessage");
    },

    async signTransaction(tx) {
      const result = await ext.sign(JSON.stringify(tx));
      return result as Hex;
    },

    async signTypedData() {
      throw new Error("external signer does not support signTypedData");
    },

    // nonceManager is not needed — nonce management is handled externally
  } satisfies LocalAccount;
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

/**
 * Resolve EVM signer options into a viem WalletClient + PublicClient pair,
 * or `{ unsigned: true }` when no signing is needed.
 *
 * Resolution order:
 * 1. `--private-key` flag (hex, 0x-prefixed)
 * 2. `EVM_PRIVATE_KEY` env var
 * 3. `--keystore <path>` + `--password`
 * 4. `EVM_SIGNER` external command
 * 5. `--unsigned`
 */
export async function resolveEvmSigner(opts: EvmSignerOptions): Promise<EvmSignerResult> {
  if (opts.unsigned) return { unsigned: true };

  const directKey = opts.privateKey ?? opts.envPrivateKey;
  if (directKey) {
    if (!opts.rpcUrl) {
      throw new Error("EVM_RPC_URL is required when using a private key for EVM signing.");
    }
    const account = privateKeyToAccount(normaliseHexKey(directKey));
    return createClients(account, opts.rpcUrl);
  }

  if (opts.keystorePath) {
    if (!opts.rpcUrl) {
      throw new Error("EVM_RPC_URL is required when using a keystore for EVM signing.");
    }
    const password = opts.keystorePassword ?? "";
    const key = await decryptKeystore(opts.keystorePath, password);
    const account = privateKeyToAccount(normaliseHexKey(key));
    return createClients(account, opts.rpcUrl);
  }

  if (opts.externalSignerCmd) {
    const rpcUrl = opts.rpcUrl ?? "";
    const account = createExternalAccount(opts.externalSignerCmd);
    return createClients(account, rpcUrl);
  }

  throw new Error(
    "no signer configured for EVM.\n" +
    "  Set EVM_PRIVATE_KEY, EVM_SIGNER, or pass --private-key / --keystore.\n" +
    "  Use --unsigned to output the unsigned transaction.",
  );
}
