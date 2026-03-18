import { ScureBitcoinSigner } from "@gobob/bob-sdk";
import type { BitcoinSigner } from "@gobob/bob-sdk";
import { spawn } from "node:child_process";
import { createHash } from "node:crypto";

// ─── Error types ─────────────────────────────────────────────────────────────

export class SignerError extends Error {
  constructor(
    public readonly command: string,
    public readonly stderr: string,
    message?: string,
  ) {
    super(message || `Signer command "${command}" failed: ${stderr}`);
    this.name = "SignerError";
  }
}

// ─── Types ────────────────────────────────────────────────────────────────────

export type { BitcoinSigner };

export type BtcSignerResult =
  | { signer: BitcoinSigner; address?: string }
  | { unsigned: true };

export interface BtcSignerOptions {
  privateKey?: string;
  envPrivateKey?: string;
  externalSignerCmd?: string;
  unsigned: boolean;
}

// ─── WIF decoding ────────────────────────────────────────────────────────────

const BASE58_ALPHABET = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

function base58Decode(str: string): Buffer {
  let num = 0n;
  for (const c of str) {
    const idx = BASE58_ALPHABET.indexOf(c);
    if (idx < 0) throw new Error(`invalid base58 character: '${c}'`);
    num = num * 58n + BigInt(idx);
  }
  // Convert bigint to bytes
  const hex = num.toString(16).padStart(2, "0");
  const rawBytes = Buffer.from(hex.length % 2 ? "0" + hex : hex, "hex");

  // Count leading '1's → leading zero bytes
  let leadingZeros = 0;
  for (const c of str) {
    if (c === "1") leadingZeros++;
    else break;
  }

  return Buffer.concat([Buffer.alloc(leadingZeros), rawBytes]);
}

function base58CheckDecode(str: string): { version: number; payload: Buffer } {
  const raw = base58Decode(str);
  if (raw.length < 5) throw new Error("base58check: too short");

  const payload = raw.slice(0, raw.length - 4);
  const checksum = raw.slice(raw.length - 4);

  const hash = createHash("sha256").update(payload).digest();
  const hash2 = createHash("sha256").update(hash).digest();

  if (!hash2.subarray(0, 4).equals(checksum)) {
    throw new Error("base58check: invalid checksum");
  }

  return { version: payload[0], payload: payload.slice(1) };
}

/**
 * Decode a WIF-encoded private key to a hex string.
 * WIF format: 1 byte version + 32 bytes key [+ 1 byte compression flag] + 4 bytes checksum
 */
export function decodeWif(wif: string): string {
  const { payload } = base58CheckDecode(wif);
  // payload is 32 bytes (uncompressed) or 33 bytes (compressed, last byte = 0x01)
  if (payload.length === 33 && payload[32] === 0x01) {
    return payload.subarray(0, 32).toString("hex");
  }
  if (payload.length === 32) {
    return payload.toString("hex");
  }
  throw new Error(`invalid WIF key length: expected 32 or 33 bytes, got ${payload.length}`);
}

// ─── Private key helper ──────────────────────────────────────────────────────

/**
 * Normalise a user-supplied private key (hex or WIF) into a hex string
 * suitable for `ScureBitcoinSigner`.
 */
function normalisePrivateKey(key: string): string {
  // Try hex first (with or without 0x prefix)
  const stripped = key.startsWith("0x") ? key.slice(2) : key;
  if (/^[0-9a-fA-F]{64}$/.test(stripped)) {
    return stripped;
  }

  // Try WIF
  try {
    return decodeWif(key);
  } catch {
    // fall through
  }

  throw new Error(
    "invalid private key: must be 32-byte hex (64 chars) or WIF-encoded",
  );
}

// ─── External signer ─────────────────────────────────────────────────────────

export class ExternalSigner {
  constructor(private command: string) {}

  async sign(input: string): Promise<string> {
    return new Promise((resolve, reject) => {
      const parts = this.command.split(/\s+/);
      const cmd = parts[0];
      const args = parts.slice(1);
      const child = spawn(cmd, args, { stdio: ["pipe", "pipe", "pipe"] });

      let stdout = "";
      let stderr = "";

      child.stdout.on("data", (data: Buffer) => {
        stdout += data.toString();
      });
      child.stderr.on("data", (data: Buffer) => {
        stderr += data.toString();
      });
      child.on("error", (err: Error) => {
        reject(new SignerError(this.command, stderr || err.message));
      });
      child.on("close", (code: number | null) => {
        if (code !== 0) reject(new SignerError(this.command, stderr));
        else resolve(stdout.trim());
      });

      child.stdin.write(input);
      child.stdin.end();
    });
  }
}

/**
 * Wrap an external command as a `BitcoinSigner`.
 * The command receives PSBT hex on stdin and must return signed tx hex on stdout.
 */
function externalBitcoinSigner(command: string): BitcoinSigner {
  const ext = new ExternalSigner(command);
  return {
    signAllInputs: (psbtHex: string) => ext.sign(psbtHex),
  };
}

// ─── Signer resolver ─────────────────────────────────────────────────────────

export async function resolveBtcSigner(opts: BtcSignerOptions): Promise<BtcSignerResult> {
  if (opts.unsigned) return { unsigned: true };

  const key = opts.privateKey ?? opts.envPrivateKey;
  if (key) {
    const hexKey = normalisePrivateKey(key);
    const signer = new ScureBitcoinSigner(hexKey);
    const address = await signer.getP2WPKHAddress();
    return { signer, address };
  }

  if (opts.externalSignerCmd) {
    return { signer: externalBitcoinSigner(opts.externalSignerCmd) };
  }

  throw new Error(
    "no signer configured for Bitcoin.\n" +
    "  Set BITCOIN_PRIVATE_KEY, BITCOIN_SIGNER, or pass --private-key.\n" +
    "  Use --unsigned to output the PSBT without signing.",
  );
}

// ─── Sign via resolved result ────────────────────────────────────────────────

/**
 * Sign a PSBT using the resolved signer result.
 * Accepts PSBT as base64 (legacy) or hex. Internally converts to hex for the SDK.
 */
export async function signBtcWithResult(
  result: BtcSignerResult,
  psbtBase64: string,
): Promise<string> {
  if ("unsigned" in result) {
    throw new Error("signBtcWithResult called with unsigned result");
  }

  if (!result.signer.signAllInputs) {
    throw new Error("signer does not implement signAllInputs");
  }

  // Convert base64 PSBT to hex for the SDK's signAllInputs interface
  const psbtHex = Buffer.from(psbtBase64, "base64").toString("hex");
  return result.signer.signAllInputs(psbtHex);
}
